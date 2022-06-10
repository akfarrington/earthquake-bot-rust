use dotenv::dotenv as dot;
use dotenv_codegen::dotenv;
// use reqwest::Client;

use crate::cwb_api::structs::Response;

pub mod impls;
pub mod structs;

/// the two urls for getting earthquake information
const STORES: [&str; 2] = ["E-A0015-001", "E-A0016-001"];
/// the url root for the Taiwan CWB api
const URL_ROOT: &str = "https://opendata.cwb.gov.tw/api/v1/rest/datastore/";

/// get both urls in a vector
/// arguments: last_time: string (this one needs the T)
fn get_earthquake_urls(last_time: Option<String>) -> Vec<String> {
    dot().ok();
    let api_key = dotenv!("CWB_API_KEY");

    // if passing a time, this part will have the get argument for time
    // if None, it'll be empty, meaning the api will return the latest earthquake
    let time_part = match last_time {
        Some(t) => format!("&timeFrom={}", t),
        None => "".to_string(),
    };

    STORES
        .iter()
        .map(|store| {
            format!(
                "{base_url}{store}?Authorization={api_key}{last_time}",
                base_url = URL_ROOT,
                store = store,
                api_key = api_key,
                last_time = time_part
            )
        })
        .collect()
}

/// convenience struct to hold both responses (so it's an array of responses)
#[derive(Debug)]
pub struct BothResponses(pub [structs::Response; 2]);

impl BothResponses {
    /// this will take the two urls in this mod, then it'll get two `Response`s
    /// arguments:
    /// * last_time: Option<String>
    /// returns:
    /// * Result<Self, Box<dyn Error::error>>
    pub async fn new_from_last_time(last_time: Option<String>) -> Result<Self, reqwest::Error> {
        // get the two urls (there will always be two)
        let both_urls = get_earthquake_urls(last_time);

        // get both `Response`s
        // block_on will block the running thread until the async function has completed
        let big = Response::new_from_url(both_urls[0].clone()).await?;
        let small = Response::new_from_url(both_urls[1].clone()).await?;

        Ok(Self([big, small]))
    }

    /// search the two responses for any Earthquakes stored in them, then return the Earthquake
    /// structs
    pub fn get_all_earthquakes(&self) -> Vec<structs::Earthquake> {
        let mut all_eqs = Vec::new();

        // iterate the big and small eq returns
        // (this is in the even the server returns multiple earthquakes per
        // api endpoint, which has never happened in my experience, but still worth
        // checking for)
        for resp in self.0.iter() {
            for eq in &resp.records.earthquake {
                all_eqs.push(eq.clone());
            }
        }

        all_eqs
    }
}
