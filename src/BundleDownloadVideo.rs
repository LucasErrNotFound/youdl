#![allow(non_snake_case)]
use std::error::Error;
use text_io::read;
use rustube::{Id, VideoFetcher};

pub async fn bundle_download() -> Result<(), Box<dyn Error>> {
    print!("Enter the number of YouTube links: ");
    let youtube_link_numbers: usize = read!();
    let mut youtube_links = Vec::new();
    let mut video_titles = Vec::new();

    for _ in 0..youtube_link_numbers {
        print!("\nEnter a YouTube link: ");
        let youtube_link: String = read!();
        let id = Id::from_raw(&youtube_link)?;
        let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;

        youtube_links.push(youtube_link.trim().to_string());
        video_titles.push(descrambler.video_title().to_owned());
    }

    let mut handles = vec![];

    for (index, link) in youtube_links.into_iter().enumerate() {
        let title = video_titles[index].clone();
        let handle = tokio::spawn(async move {
            let _ = rustube::download_best_quality(&link).await;
            title
        });
        handles.push(handle);
    }

    for handle in handles {
        let title = handle.await?;
        println!("Downloaded video: {:?}", title);
    }
    Ok(())
}
