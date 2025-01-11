use std::cmp::min;
use std::fs::File;
use std::io::Write;

use anyhow::Context;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{self, Client};
use serde_json::Value;
use url::Url;

pub async fn get_base_website(client: &Client, url: &Url) -> anyhow::Result<String> {
    let response = client.get(url.as_str()).send().await?;
    let body = response.text().await?;
    Ok(body)
}

#[derive(Debug, serde::Deserialize)]
pub struct FetchedVideoUrls {
    #[serde(rename = "HD")]
    pub hd: Option<String>,
    #[serde(rename = "FHD")]
    pub fhd: Option<String>,
    #[serde(rename = "SD")]
    pub sd: Option<String>,
    #[serde(rename = "SourceMKV")]
    pub source: Option<String>,
}

pub async fn get_video_urls(client: &Client, id: i64) -> anyhow::Result<Option<FetchedVideoUrls>> {
    let response = client
        .get(format!(
            "https://www.lycoris.cafe/api/getSecondaryLink?id={}",
            id
        ))
        .send()
        .await?;
    let body: Value = response.json().await?;

    let video_links = body
        .get("videoLink")
        .ok_or_else(|| anyhow::anyhow!("Failed to get video links"))?;
    let video_urls: FetchedVideoUrls = serde_json::from_value(video_links.clone())?;

    Ok(Some(video_urls))
}

pub async fn download_file(
    url: &str,
    path: &str,
    download_message: &str,
    download_finish_message: &str,
) -> anyhow::Result<()> {
    let download_message = download_message.to_string();
    let download_finish_message = download_finish_message.to_string();

    let client = reqwest::Client::new();
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .context(format!("Failed to send request to '{}'", &url))?;
    let total_size = res
        .content_length()
        .context(format!("Failed to get content length of '{}'", &url))?;

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})").unwrap()
        .progress_chars("#>-"));
    pb.set_message(download_message);

    // download chunks
    let mut file = File::create(path).context(format!("Failed to create file '{}'", path))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.context("Failed to get next chunk")?;
        file.write_all(&chunk)
            .context("Failed to write chunk to file")?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(download_finish_message);

    Ok(())
}
