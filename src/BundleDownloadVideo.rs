#![allow(non_snake_case)]
use std::error::Error;
use text_io::read;
use rustube::{Id, VideoFetcher};

pub async fn bundle_download() -> Result<(), Box<dyn Error>> {
    print!("Enter the number of YouTube links: ");
    let YoutubeLinkCount: usize = read!();
    let mut YoutubeLinks = Vec::new();
    let mut VideoTitles = Vec::new();

    for _ in 0..YoutubeLinkCount {
        print!("\nEnter a YouTube link: ");
        let YoutubeURL: String = read!();
        let ID = Id::from_raw(&YoutubeURL)?;
        let descrambler = VideoFetcher::from_id(ID.into_owned())?.fetch().await?;

        YoutubeLinks.push(YoutubeURL.trim().to_string());
        VideoTitles.push(descrambler.video_title().to_owned());
    }

    let mut handles = vec![];

    for (index, link) in YoutubeLinks.into_iter().enumerate() {
        let title = VideoTitles[index].clone();
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
