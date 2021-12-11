use super::structs::{Earthquake, Response};
use crate::db::EqDb;
use crate::time::CwbTime;
// use reqwest::{Client, Error};
use reqwest::Error;

impl Response {
    /// pass a url to create a new CWB EQ response struct
    pub async fn new_from_url(url: String) -> Result<Self, Error> {
        let json = reqwest::get(&url).await?;
        let s: Self = json.json().await?;

        // before returning, sleep for a second to make sure I don't
        // accidentally hit the CWB servers too often/quickly
        std::thread::sleep(std::time::Duration::from_secs(1));
        Ok(s)
    }
}

impl Earthquake {
    /// checks if the earthquake is more recent than the time stored in the database
    /// as last_time, and if so update the db
    pub fn update_last_time(&self, db: &EqDb) {
        let mut eq_time = CwbTime::new_time_string_without_t(&self.earthquake_info.origin_time);

        let last_time_string = db.get_last_time();

        let last_time = CwbTime::new_time_string_with_t(&last_time_string);

        // now check if the eq is more recent than the last_time stored in the db
        if eq_time.get_date_time() >= last_time.get_date_time() {
            println!(
                "updating db with this eq time: {}",
                &self.earthquake_info.origin_time
            );

            // save this as a new time
            db.store_last_time(eq_time.add_seconds(1).format_with_t());
        }
    }
}
