#![warn(clippy::all)]

use clap::{App, Arg};
use console::style;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod api;
mod downloader;
mod models;
mod sanitize_path;

#[tokio::main]
async fn main() {
    let matches = App::new("Rai-Play Downloader")
        .version("0.1.0")
        .author("Matteo Guarda <matteoguarda@tutanota.com>")
        .about("CLI scritto in Rust per scaricare video da Rai-Play.")
        .arg(
            Arg::with_name("url")
                .value_name("URL")
                .help("URL al video da scaricare")
                .required(true),
        )
        .arg(
            Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Disattiva di logging non necessario"),
        )
        .arg(
            Arg::with_name("mp4")
                .short("m")
                .long("mp4")
                .help("Al posto di scaricare il video come .ts, lo scarica come mp4"),
        )
        .arg(
            Arg::with_name("m3u8")
                .short("M")
                .long("m3u8")
                .help("Scarica solo il file .m3u8 master del video"),
        )
        .arg(
            Arg::with_name("infos")
                .short("i")
                .long("infos")
                .help("Scarica e salva in un file JSON delle informazioni sul video"),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();
    let verbose = matches.occurrences_of("quiet") != 1;
    let mp4 = matches.occurrences_of("mp4") == 1;
    let infos = matches.occurrences_of("infos") == 1;
    let m3u8 = matches.occurrences_of("m3u8") == 1;

    let mut video_infos = api::extract_video_url(url, Some(verbose))
        .await
        .expect("Non sono riuscito a scaricare le info sul video");
    let filename = sanitize_path::sanitize(&video_infos.infos.name, None, None);
    video_infos
        .fetch_all_segments(Some(verbose))
        .await
        .expect("Non sono riuscito a scaricare tutti i segmenti");

    if infos {
        let mut file = File::create(format!("{}.json", filename))
            .expect("Non sono riuscito a creare data.json");
        file.write_all(
            serde_json::to_string_pretty(&video_infos)
                .unwrap()
                .as_bytes(),
        )
        .expect("Non sono riuscito a scrivere su data.json");
        return;
    }

    if mp4 {
        downloader::download(&video_infos.mp4_url, Some(verbose))
            .await
            .expect(&format!("Non sono riuscito a scaricare {}", mp4));
        return;
    }

    println!("{}Seleziona la qualità:", if verbose { "\n" } else { "" });
    for (i, variant) in video_infos.m3u8_variants.iter().enumerate() {
        println!("  [{}] {}", style(i).cyan(), variant.resolution);
    }

    let i: usize = loop {
        let mut input = String::new();

        print!("{}", style("==> ").green());
        std::io::stdout()
            .flush()
            .expect("Non sono riuscito a flushare stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Non hai inserito una stringa valida");

        match input.trim().parse::<i32>() {
            Ok(num) => {
                let len = video_infos.m3u8_variants.len() as i32;
                if num >= 0 && num < len {
                    break num as usize;
                } else {
                    println!(
                        "\n{} Devi inserire un numero tra 0 e {}\n",
                        style(">>").red(),
                        len - 1
                    );
                    continue;
                }
            }
            Err(_) => {
                println!("\n{} Input non valido. Riprova\n", style(">>").red());
                continue;
            }
        }
    };

    println!();

    if m3u8 {
        video_infos.m3u8_variants[i as usize]
            .save_m3u8(Path::new(&format!("{}.m3u8", filename)), Some(verbose))
            .expect("Non sono riuscito a salvare il file .m3u8");
        return;
    }

    video_infos.m3u8_variants[i as usize]
        .download_ts(Path::new(&format!("{}.ts", filename)), Some(verbose))
        .await
        .expect("Non sono riuscito a scaricare il file .ts");
}
