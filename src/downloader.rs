#![warn(clippy::all)]

use console::style;
use failure::Error;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

/// Scarica un file usando `url`.
pub async fn download(url: &str, verbose: Option<bool>) -> Result<(), Error> {
    let verbose = verbose.unwrap_or(false);
    let url = reqwest::Url::parse(url)?;

    let file_name = match url
        .path_segments()
        .and_then(|segs| segs.last().map(String::from))
    {
        Some(fname) => fname,
        None => Uuid::new_v4().to_string(),
    };
    if verbose {
        println!("\nSalvo in: {}", style(&file_name).green());
    }

    let mut resp = {
        let client = reqwest::Client::new();
        client.get(url).send().await?
    };
    let headers = resp.headers();

    let ct_len = {
        let ct_len = headers.get(reqwest::header::CONTENT_LENGTH);
        let ct_len_str = ct_len.and_then(|len| len.to_str().ok());
        ct_len_str.and_then(|x| x.parse::<u64>().ok())
    };
    if verbose {
        match ct_len {
            Some(len) => println!(
                "Content-Length: {} ({})",
                style(len).green(),
                style(indicatif::HumanBytes(len)).cyan()
            ),
            None => println!("Content-Length: {}", style("unknown").red()),
        }
    }

    let ct_type = headers
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|t| t.to_str().ok());
    if verbose {
        match ct_type {
            Some(t) => println!("Content-Type: {}", style(t).green(),),
            None => println!("Content-Type: {}", style("unknown").red()),
        }
    }

    let progress_bar = match ct_len {
        Some(len) => ProgressBar::new(len).with_style(ProgressStyle::default_bar().template(
            "{wide_bar:.cyan/blue} {bytes:.green} / {total_bytes} [{elapsed} .. {eta:.cyan}]",
        )),
        None => ProgressBar::new(0).with_style(
            ProgressStyle::default_spinner()
                .template("[{elapsed_precise}] {spinner:.green} {bytes:.cyan}"),
        ),
    };
    let mut file = File::create(&file_name)?;

    while let Some(ref chunk) = resp.chunk().await? {
        file.write(chunk)?;
        progress_bar.inc(chunk.len() as u64);
    }

    progress_bar.finish();
    if verbose {
        println!("File salvato in {}", style(file_name).green());
    }
    Ok(())
}
