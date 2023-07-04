#![allow(non_snake_case)]
mod BundleDownloadVideo;
mod SingleDownloadVideo;
use text_io::read;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Youtube Video Downloader.");
    println!("Enter one of the options: ");
    println!("A. Bundle Download Youtube Links");
    println!("B. Single Download Youtube Links");
    println!("C. Don't know");
    println!("D. Don't know");
    print!("-----> ");
    let option: char = read!();

    if option == 'A' {
        BundleDownloadVideo::bundle_download().await?;
    }
    else if option == 'B' {
        SingleDownloadVideo::single_download().await?;
    }
    else if option == 'D' {
        let file: String = std::fs::read_to_string("Test.txt")?;
        file.lines().for_each(|lines| println!("\n{}", lines));
    }
    Ok(())
}
