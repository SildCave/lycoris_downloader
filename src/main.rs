mod args;
mod fetcher;
mod parser;

use args::Args;
use fetcher::{download_file, get_base_website, get_video_urls};

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0")
        .default_headers(reqwest::header::HeaderMap::from_iter(vec![
            (
                reqwest::header::ACCEPT_LANGUAGE,
                "en-US,en;q=0.5".parse().unwrap(),
            ),
            (reqwest::header::ACCEPT_ENCODING, "json".parse().unwrap()),
        ]))
        .build()?;

    let url = url::Url::parse(&args.url)
        .map_err(|e| {
            eprintln!("Invalid URL: {}", e);
            std::process::exit(1);
        })
        .unwrap();

    let body = get_base_website(&client, &url).await.unwrap();
    let id = parser::find_id_in_base_website_source(&body).unwrap();

    let video_urls = get_video_urls(&client, id).await?;
    if video_urls.is_none() {
        eprintln!("Failed to get video URLs");
        std::process::exit(1);
    }
    let video_urls = video_urls.unwrap();

    let title = url.query_pairs().find(|(key, _)| key == "title").unwrap().1;
    let episode = url
        .query_pairs()
        .find(|(key, _)| key == "episode")
        .unwrap()
        .1;

    let download_message = format!("Downloading video: {}, episode: {}", title, episode);
    let download_finish_message = format!(
        "Finished downloading video: {}, episode: {}",
        title, episode
    );

    let mut output_path;

    if args.output_path.is_none() {
        output_path = std::path::PathBuf::from(".");
        output_path.push(format!("{}-{}.mp4", title, episode));
    } else {
        output_path = args.output_path.unwrap();
    }

    match args.quality {
        args::Quality::HD => {
            if video_urls.hd.is_none() {
                eprintln!("Failed to get HD video URL");
                std::process::exit(1);
            }
            download_file(
                &video_urls.hd.unwrap(),
                &output_path.to_string_lossy(),
                &download_message,
                &download_finish_message,
            )
            .await?;
        }
        args::Quality::FHD => {
            if video_urls.fhd.is_none() {
                eprintln!("Failed to get FHD video URL");
                std::process::exit(1);
            }
            download_file(
                &video_urls.fhd.unwrap(),
                &output_path.to_string_lossy(),
                &download_message,
                &download_finish_message,
            )
            .await?;
        }
        args::Quality::SD => {
            if video_urls.sd.is_none() {
                eprintln!("Failed to get SD video URL");
                std::process::exit(1);
            }
            download_file(
                &video_urls.sd.unwrap(),
                &output_path.to_string_lossy(),
                &download_message,
                &download_finish_message,
            )
            .await?;
        }
        args::Quality::Source => {
            if video_urls.source.is_none() {
                eprintln!("Failed to get Source video URL");
                std::process::exit(1);
            }
            download_file(
                &video_urls.source.unwrap(),
                &output_path.to_string_lossy(),
                &download_message,
                &download_finish_message,
            )
            .await?;
        }
    }

    Ok(())
}
