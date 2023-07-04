#![allow(non_snake_case)]
use std::error::Error;
use text_io::read;
use rustube::{Id, VideoFetcher};

pub async fn single_download() -> Result<(), Box<dyn Error>> {
    print!("Enter Youtube Link to Download: ");
    let YoutubeLink: String = read!();
    let ID = Id::from_raw(&YoutubeLink)?;
    let descrambler = VideoFetcher::from_id(ID.into_owned())?
        .fetch().await?;

    let title = descrambler.video_title();
    let _ = rustube::download_best_quality(&YoutubeLink).await?;

    println!("\nThe video `{}` has been successfully downloaded.", title);
    Ok(())
}
