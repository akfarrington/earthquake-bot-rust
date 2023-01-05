use serde::{Deserialize, Serialize};
use crate::cwb_api::structs::IntensityConversionErrors::OutOfBounds;

/// All CWB data goes into this parent struct
/// includes:
/// * records - has one child, and all info below
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    #[serde(rename(serialize = "records", deserialize = "records"))]
    pub records: Records,
}

/// parent to earthquakes array
/// includes:
/// * earthquake - a vector of earthquakes returned by the server
#[derive(Serialize, Deserialize, Debug)]
pub struct Records {
    #[serde(rename(serialize = "earthquake", deserialize = "Earthquake"))]
    pub earthquake: Vec<Earthquake>,
}

/// information about a single earthquake
/// includes:
/// * report_content - a printable string (for tweeting)
/// * report_image_URI - uri to the CWB map
/// * web - web address for information about the earthquake
/// * earthquake_info - a parent for the earthquake's time and epicenter
/// * intensity - intensity of the epicenter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Earthquake {
    #[serde(rename(serialize = "report_content", deserialize = "ReportContent"))]
    pub report_content: String,
    #[serde(rename(serialize = "earthquake_info", deserialize = "EarthquakeInfo"))]
    pub earthquake_info: EarthquakeInfo,
    #[serde(rename(serialize = "intensity", deserialize = "Intensity"))]
    pub intensity: Intensity,
}

/// has information about the earthquake's time, lat, and long
/// includes:
/// * origin_time - the time the earthquake happened
/// * epicenter - the lat and long of the epicenter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EarthquakeInfo {
    #[serde(rename(serialize = "origin_time", deserialize = "OriginTime"))]
    pub origin_time: String,
    #[serde(rename(serialize = "epicenter", deserialize = "Epicenter"))]
    pub epicenter: Epicenter,
}

/// information about the earthquake's intensity
/// includes
/// * shaking_area - information about each area's station's records
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Intensity {
    #[serde(rename(serialize = "shaking_area", deserialize = "ShakingArea"))]
    pub shaking_area: Vec<ShakingArea>,
}

/// stores the information about the intensity of each station
/// includes:
/// * eq_station - information about each station's records
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShakingArea {
    #[serde(rename(serialize = "eq_station", deserialize = "EqStation"))]
    pub eq_station: Vec<EqStation>,
}

/// single station information
/// includes:
/// * station_intensity - this station's intensity reading
/// * station_lat - this station's location
/// * station_long - this station's location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EqStation {
    #[serde(rename(serialize = "station_intensity", deserialize = "SeismicIntensity"))]
    pub station_intensity: String,
    #[serde(rename(serialize = "station_lat", deserialize = "StationLatitude"))]
    pub station_lat: f64,
    #[serde(rename(serialize = "station_long", deserialize = "StationLongitude"))]
    pub station_long: f64,
}

/// epicenter of the earthquake
/// includes:
/// * long - this.value is f64 of its location
/// * lat - this.value is f64 of its location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Epicenter {
    #[serde(rename(serialize = "long", deserialize = "EpicenterLatitude"))]
    pub long: f64,
    #[serde(rename(serialize = "lat", deserialize = "EpicenterLongitude"))]
    pub lat: f64,
}

// start some implementations here

pub enum IntensityConversionErrors {
    OutOfBounds(u8),
    ParseError(String)
}

impl EqStation {
    pub fn convert_station_intensity_to_u8(&self) -> Result<u8, IntensityConversionErrors> {
        // Taiwan's station intensities can only go to 7, so bust the string to
        // chars and take the first one

        let intensity_chars = self.station_intensity.chars();

        let intensity_numbers: String = intensity_chars.filter(|c| c.is_numeric()).collect();

        let intensity = intensity_numbers.parse::<u8>().map_err(|e| IntensityConversionErrors::ParseError(e.to_string()))?;

        if (1..=7).contains(&intensity) {
            Ok(intensity)
        } else {
            Err(OutOfBounds(intensity))
        }
    }
}
