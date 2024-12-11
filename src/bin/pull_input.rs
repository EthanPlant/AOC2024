use std::{fs::{self, read}, io::ErrorKind};

use anyhow::{Context, Result};
use chrono::{Datelike, Utc};
use clap::Parser;
use reqwest::ClientBuilder;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = Utc::now().day())]
    day: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if !check_cached_input(args.day) {
        println!("Downloading input file for {}", args.day);
        download_input(args.day).await?;
    }

    Ok(())
}

fn check_cached_input(day: u32) -> bool {
    let file = format!("input/{day}.txt");
    match read(&file) {
        Ok(_) => true,
        Err(e) => e.kind() != ErrorKind::NotFound
    }
}

async fn download_input(day: u32) -> Result<()> {
    let url = format!("https://adventofcode.com/2024/day/{day}/input");
    let session = std::env::var("AOC_SESSION").context("Failed to find AOC session token")?;
    let client = ClientBuilder::new()
        .user_agent("https://github.com/EthanPlant/AOC2024 by plant.ethan@gmail.com")
        .build()
        .context("Failed to build http client")?;

    let request = client
        .get(url)
        .header("Cookie", format!("session={session}"))
        .build()
        .context("Failed to build request")?;

    let resp = client
        .execute(request)
        .await
        .context("Failed to execute http request")?
        .error_for_status()
        .context("Server returned error")?
        .text()
        .await
        .context("Failed to read http response body")?;

    fs::write(format!("input/{day}.txt"), resp).context("Failed to write input to file")?;
    Ok(())
}