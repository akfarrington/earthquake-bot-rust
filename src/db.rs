use crate::time::get_local_date_time_with_t;
use dotenv_codegen::dotenv;
use sled::{Db, Result as SledResult};

const CANT_ACCESS_LAST_TIME: &str = "failed to retrieve last_time from db";
const LAST_TIME_KEY: &str = "last_time";

/// open the database
fn open_connection() -> SledResult<Db> {
    let db_file = dotenv!("DATABASE_FILE");
    sled::open(db_file)
}

/// struct will hold the `sled::Db` thing
pub struct EqDb(Db);

impl EqDb {
    /// connect and return Result with EqDb struct
    pub fn new() -> Result<Self, ()> {
        match open_connection() {
            Ok(d) => Ok(EqDb(d)),
            Err(_) => Err(()),
        }
    }

    /// get the last_time from the db
    pub fn get_last_time(&self) -> String {
        // call .expect().expect() because it shouldn't be an error or none
        // (otherwise there is a real error and it needs to panic)
        let last_time = self
            .0
            .get(LAST_TIME_KEY.as_bytes())
            .expect(CANT_ACCESS_LAST_TIME)
            .expect(CANT_ACCESS_LAST_TIME);

        String::from(std::str::from_utf8(last_time.as_ref()).unwrap())
    }

    /// store last time in the database
    /// (`Earthquake` `update_last_time` should verify that the new time is newer
    /// than the old time before it stores a new time)
    pub fn store_last_time(&self, new_last_time: String) {
        let _ = self.0.insert(LAST_TIME_KEY, new_last_time.as_bytes());
    }

    /// check if the db has a last_time value, and if not, create one with the current time
    pub fn check_last_time_create_if_not_exist(&self) {
        let last_time = self.0.get(LAST_TIME_KEY);

        // check result, panic if it fails
        match last_time.expect(CANT_ACCESS_LAST_TIME) {
            Some(_) => {} // do nothing, it's already there
            None => {
                println!("Couldn't find a db folder, so creating a new one now.");
                let time_now = get_local_date_time_with_t();

                // notify that a new one was saved
                println!("Created using last_time = {}", time_now);

                // this should work, so just unwrap it
                // also, I already know the result is None, so drop it
                let _ = self.0.insert(LAST_TIME_KEY, time_now.as_bytes()).unwrap();
            }
        }
    }
}
