#![warn(clippy::all)]

use crate::models::video;
use console::style;
use failure::{Error, Fail};
use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// URL di esempio a video su RaiPlay.
#[allow(dead_code)]
pub const RAI_PLAY_EXAMPLE_URLS: [&'static str; 6] = [
    "https://www.raiplay.it/video/2019/10/ulisse-il-piacere-della-scoperta-il-gattopardo---il-romanzo-della-sicilia-cbbcfc7a-c25c-476c-b396-a744aa1fe457.html",
    "https://www.raiplay.it/video/2020/02/sanremo-2020-vince-diodato-e2488135-54c5-4776-b846-af9d7c386e83.html",
    "https://www.raiplay.it/video/2020/02/sanremo-2020-serata-finale-francesco-gabbani-viceversa-37c8448b-824a-48e2-883f-dafaf972bc23.html",
    "https://www.raiplay.it/video/2017/12/FILM-Il-ragazzo-invisibile-91e80d6c-8488-4b5c-8b97-4f97f9dbf486.html",
    "https://www.raiplay.it/video/2020/02/narcos-iii-ep8-d0dab05f-b056-4c72-9994-c8738feb540f.html",
    "https://www.raiplay.it/video/2018/03/Nati-per-sopravvivere-E1-ada6827d-0551-4d2b-969b-acc6eb9cbda8.html"
];

#[derive(Fail, Debug)]
#[fail(display = "URL `{}` is not valid", _0)]
struct UrlNotValidError(String);

#[derive(Fail, Debug)]
#[fail(display = "M3U8 is not valid and it cannot be parsed")]
struct M3u8NotValidError;

#[derive(Debug, Serialize, Deserialize)]
pub struct M3u8VideoVariant {
    pub uri: String,
    pub bandwidth: String,
    pub resolution: String,
    segments: Option<Vec<M3u8VideoSegment>>,
    #[serde(skip)]
    pub m3u8_content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct M3u8VideoSegment {
    pub duration: f32,
    pub uri: String,
}

impl M3u8VideoVariant {
    pub fn new(
        uri: String,
        bandwidth: String,
        resolution: String,
        m3u8_content: Vec<u8>,
    ) -> M3u8VideoVariant {
        M3u8VideoVariant {
            uri,
            bandwidth,
            resolution,
            segments: None,
            m3u8_content,
        }
    }

    /// Scarica il vettore di segmenti e lo cachea nello struct, se tutto va bene
    /// ritorna una reference wrappata in un Some().
    pub async fn fetch_segments(
        &mut self,
        verbose: Option<bool>,
    ) -> Result<&Vec<M3u8VideoSegment>, Error> {
        let verbose = verbose.unwrap_or(false);

        if self.segments.is_none() {
            if verbose {
                print!("Ottenendo i segmenti M3U8 per la variante...");
            }
            let text = reqwest::get(&self.uri).await?.text().await?;
            let parsed = m3u8_rs::parse_media_playlist_res(text.as_bytes())
                .map_err(|_| M3u8NotValidError)?;

            self.segments = Some(
                parsed
                    .segments
                    .into_iter()
                    .map(|seg| M3u8VideoSegment {
                        duration: seg.duration,
                        uri: seg.uri,
                    })
                    .collect(),
            );

            if verbose {
                println!("{}", style(" fatto").green());
            }
        }

        Ok(self.segments.as_ref().unwrap())
    }

    /// Salva i dati M3U8 in un file.
    pub fn save_m3u8(&self, path: &Path, verbose: Option<bool>) -> Result<(), Error> {
        let verbose = verbose.unwrap_or(false);
        let mut file = File::create(path)?;
        file.write_all(&self.m3u8_content)?;
        if verbose {
            println!("M3U8 salvato in {:#?}", style(path).green());
        }
        Ok(())
    }

    /// Scarica tutti i segmenti e li concatena in un unico file .ts.
    pub async fn download_ts(&mut self, path: &Path, verbose: Option<bool>) -> Result<(), Error> {
        let segs = self.fetch_segments(verbose).await?;
        let verbose = verbose.unwrap_or(false);
        let segs_len = segs.len() as u64;
        let mut file = File::create(path)?;

        let progress_bar = {
            let pbar = ProgressBar::new(segs_len)
                .with_style(ProgressStyle::default_bar().template(
                "[{prefix} / {msg}] [{wide_bar:.cyan/blue}] segmento {pos} / {len} [{elapsed} .. {eta}]",
            ));

            pbar.set_prefix(&style("0").green().to_string());
            pbar.set_message("0");
            pbar.enable_steady_tick(1000);

            pbar
        };

        let mut total_content_len = 0;

        for seg in segs {
            let seg_data = reqwest::get(&seg.uri).await?;
            total_content_len += seg_data.content_length().unwrap_or(0);
            file.write(&mut seg_data.bytes().await?)?;
            // std::io::copy(&mut seg_data, &mut file)?;

            progress_bar.inc(1);
            progress_bar.set_prefix(&style(HumanBytes(total_content_len)).green().to_string());
            let predicted_total_size = total_content_len / progress_bar.position() * segs_len;
            progress_bar.set_message(&HumanBytes(predicted_total_size).to_string());
        }

        progress_bar.finish();
        if verbose {
            println!("TS salvato in {:#?}", style(path).green());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaiPlayVideoInfos {
    pub mp4_url: String,
    pub infos: video::RaiPlayVideo,
    pub m3u8_variants: Vec<M3u8VideoVariant>,
}

impl RaiPlayVideoInfos {
    /// Scarica i segmenti di tutte le varianti M3U8.
    pub async fn fetch_all_segments(&mut self, verbose: Option<bool>) -> Result<(), Error> {
        for seg in self.m3u8_variants.iter_mut() {
            seg.fetch_segments(verbose).await?;
        }
        Ok(())
    }
}

/// Estrae l'URL all'M3U8 del video o direttamente al suo MP4 (di qualità
/// sconosciuta).
pub async fn extract_video_url(
    url: &str,
    verbose: Option<bool>,
) -> Result<RaiPlayVideoInfos, Error> {
    let verbose = verbose.unwrap_or(false);

    let captures = {
        let re = Regex::new(r#"^http(s)?://(?:www\.)?raiplay\.it/video/\d{4}/\d{2}/[^\.]+\.html$"#)
            .unwrap();
        re.captures(url)
    };

    let is_https = match captures {
        Some(caps) => caps.get(1).is_some(),
        None => {
            return Err(Error::from_boxed_compat(Box::new(
                UrlNotValidError(url.to_string()).compat(),
            )))
        }
    };

    let req_url = if is_https {
        String::from(url)
    } else {
        let mut req_url = String::from(url);
        req_url.insert(4, 's');
        req_url
    };

    let json_url = {
        let mut json_url: Vec<&str> = req_url.split('.').collect();
        json_url.truncate(json_url.len() - 1);
        json_url.join(".") + ".json"
    };
    if verbose {
        print!("Ottenendo il JSON contenente le informazioni sul video...");
    }
    let rai_json_resp: video::RaiPlayVideo = reqwest::get(&json_url).await?.json().await?;
    if verbose {
        println!("{}", style(" fatto").green());
    }
    let m3u8_url = &rai_json_resp.video.content_url;

    let m3u8_variants = {
        if verbose {
            print!("Ottenendo le varianti M3U8...");
        }
        let client = reqwest::Client::builder().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.106 Safari/537.36").build()?;
        let resp = client.get(m3u8_url).send().await?;
        let m3u8_text = resp.text().await?;
        let parsed = m3u8_rs::parse_master_playlist_res(m3u8_text.as_bytes())
            .map_err(|_| M3u8NotValidError)?;

        let variants = parsed
            .variants
            .into_iter()
            .filter_map(|var| match var.resolution {
                Some(res) => Some(M3u8VideoVariant::new(
                    var.uri,
                    var.bandwidth,
                    res,
                    m3u8_text.as_bytes().to_vec(),
                )),
                None => None,
            })
            .collect();

        if verbose {
            println!("{}", style(" fatto").green());
        }
        variants
    };

    let mp4_url = {
        if verbose {
            print!("Ottenendo l'URL MP4 del video...");
        }
        let client = reqwest::Client::new();
        let mp4_url = client.head(m3u8_url).send().await?.url().to_string();
        if verbose {
            println!("{}", style(" fatto").green());
        }
        mp4_url
    };

    Ok(RaiPlayVideoInfos {
        m3u8_variants,
        infos: rai_json_resp,
        mp4_url,
    })
}
