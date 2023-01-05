use crate::cwb_api::structs::Earthquake;

use dotenv_codegen::dotenv;
use egg_mode::media::{media_types, upload_media};
use egg_mode::tweet::DraftTweet;
use egg_mode::KeyPair;
use egg_mode::Token::Access;
use log::info;

impl Earthquake {
    /// this implementation of `Earthquake` uses the info in the struct, gathers the CWB
    /// report content string for the tweet, then uses both epicenter and station intensity
    /// info to make taiwan map
    pub async fn tweet(&self) -> Result<(), Box<dyn std::error::Error>> {
        // get tweet text
        let mut text = format!("{} #台灣 #地震 #Taiwan #earthquake", self.report_content);

        // if there's a massive earthquake, need to shorten the text, otherwise egg-mode panics :(
        if self.report_content.len() > 200 {
            text = format!(
                "{} #台灣 #地震 #Taiwan #earthquake",
                &self.report_content[..200]
            );
        }

        // prepare the image for upload
        self.mark_image_with_eq_data();
        let img_64 = std::fs::read("temp.png")?;

        // create a token for twitter access
        let consumer = KeyPair::new(dotenv!("API_KEY"), dotenv!("API_SECRET_KEY"));
        let access = KeyPair::new(dotenv!("ACCESS_TOKEN"), dotenv!("ACCESS_TOKEN_SECRET"));
        let token = Access { consumer, access };

        // draft the tweet
        let mut tweet = DraftTweet::new(text);

        info!("drafted the tweet");

        // upload the file
        // let pic_handle = upload_media(img_64, &media_types::image_png(), &token).await?;
        let pic_handle = upload_media(&img_64, &media_types::image_png(), &token).await?;

        info!("uploaded media: {:?}", pic_handle.id.clone());

        // add media to tweet
        tweet.add_media(pic_handle.id.clone());

        // // this isn't working. When uploaded, the images appear to already be in the progress: None
        // // state, so maybe it's unnecessary. (this code was included in the docs)
        // for ct in 0..=60_u32 {
        //     match get_status(pic_handle.id.clone(), &token).await?.progress {
        //         None | Some(ProgressInfo::Success) => {
        //             println!("\nMedia successfully processed");
        //             break;
        //         }
        //         Some(ProgressInfo::Pending(_)) | Some(ProgressInfo::InProgress(_)) => {
        //             print!(".");
        //             tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        //         }
        //         Some(ProgressInfo::Failed(err)) => Err(err)?,
        //     }
        //     if ct == 60 {
        //         Err("Error: timeout")?
        //     }
        // }

        let sent = tweet.send(&token).await?;
        info!("tweet id: {}\n{}", sent.response.id, sent.response.text);

        // tweeted, so remove the temporary file
        let _ = std::fs::remove_file("temp.png");

        Ok(())
    }
}
