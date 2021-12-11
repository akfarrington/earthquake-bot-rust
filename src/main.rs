use crate::cwb_api::BothResponses;
use crate::db::EqDb;

use std::thread::sleep;
use std::time::Duration;

mod cwb_api;
mod db;
mod taiwan_map;
mod time;
mod tweet;

const WAIT_BETWEEN_API_CALLS: u64 = 1;
const ERROR_WAIT_MINUTES: u64 = 3;

#[tokio::main]
async fn main() {
    let eq_db = startup_checks();

    // just here in case I want to set a time by myself for testing
    // eq_db.store_last_time("2021-10-24T13:00:00".to_string());

    let mut times_run = 0;

    println!("last time (for api calls): {}", eq_db.get_last_time());

    loop {
        // get the last time from the database
        let last_time = eq_db.get_last_time();
        // get data from 2 api endpoints
        let two_res = BothResponses::new_from_last_time(Some(last_time)).await;

        // handle the error in case there is an issue with the api
        if let Ok(responses) = two_res {
            // extract all earthquakes from the responses
            let all_eqs = responses.get_all_earthquakes();

            // process each earthquake
            for eq in all_eqs {
                eq.tweet().await.unwrap();
                eq.update_last_time(&eq_db);
            }

            // sleep
            sleep(Duration::from_secs(60 * WAIT_BETWEEN_API_CALLS));
        } else if let Err(r_error) = two_res {
            // there was an error, so I guess take a break for some time
            println!("error: {}", r_error);
            println!("error, waiting {} minute(s)", ERROR_WAIT_MINUTES);
            sleep(Duration::from_secs(60 * ERROR_WAIT_MINUTES));
        }

        // increment the times run variable, and print if it has run 60 * x times
        if times_run % 60 == 0 {
            println!("looped 60 times (about one hour)");
        } else if times_run == 0 {
            println!("first run successful");
        }
        times_run += 1;
    }
}

fn startup_checks() -> EqDb {
    // make sure certain files are around
    // check_for_file(".env");
    check_for_file(taiwan_map::EPI_PIC_LOC);
    check_for_file(taiwan_map::MAP_LOC);

    // connect to database, and panic if it fails, otherwise return the EqDb struct
    if let Ok(eq_db) = EqDb::new() {
        // check if a last_time exists, and if not add it. if it fails, panic
        eq_db.check_last_time_create_if_not_exist();
        eq_db
    } else {
        panic!("error when connecting/creating the database");
    }
}

fn check_for_file(file_name: &str) {
    if !std::path::Path::new(file_name).exists() {
        panic!("{} wasn't found", file_name);
    }
}
