use serde::{Deserialize, Serialize};

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
    #[serde(rename(serialize = "earthquake", deserialize = "earthquake"))]
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
    #[serde(rename(serialize = "report_content", deserialize = "reportContent"))]
    pub report_content: String,
    #[serde(rename(serialize = "earthquake_info", deserialize = "earthquakeInfo"))]
    pub earthquake_info: EarthquakeInfo,
    #[serde(rename(serialize = "intensity", deserialize = "intensity"))]
    pub intensity: Intensity,
}

/// has information about the earthquake's time, lat, and long
/// includes:
/// * origin_time - the time the earthquake happened
/// * epicenter - the lat and long of the epicenter
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EarthquakeInfo {
    #[serde(rename(serialize = "origin_time", deserialize = "originTime"))]
    pub origin_time: String,
    #[serde(rename(serialize = "epicenter", deserialize = "epiCenter"))]
    pub epicenter: Epicenter,
}

/// information about the earthquake's intensity
/// includes
/// * shaking_area - information about each area's station's records
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Intensity {
    #[serde(rename(serialize = "shaking_area", deserialize = "shakingArea"))]
    pub shaking_area: Vec<ShakingArea>,
}

/// stores the information about the intensity of each station
/// includes:
/// * eq_station - information about each station's records
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShakingArea {
    #[serde(rename(serialize = "eq_station", deserialize = "eqStation"))]
    pub eq_station: Vec<EqStation>,
}

/// single station information
/// includes:
/// * station_intensity - this station's intensity reading
/// * station_lat - this station's location
/// * station_long - this station's location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EqStation {
    #[serde(rename(serialize = "station_intensity", deserialize = "stationIntensity"))]
    pub station_intensity: StationIntensity,
    #[serde(rename(serialize = "station_lat", deserialize = "stationLat"))]
    pub station_lat: LongLat,
    #[serde(rename(serialize = "station_long", deserialize = "stationLon"))]
    pub station_long: LongLat,
}

/// this station's intensity reading
/// includes:
/// value - u16 intensity reading
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StationIntensity {
    #[serde(rename(serialize = "value", deserialize = "value"))]
    pub value: u16,
}

/// a struct to serialize long/lat information
/// includes:
/// * value - f64 long/lat
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LongLat {
    #[serde(rename(serialize = "value", deserialize = "value"))]
    pub value: f64,
}

/// epicenter of the earthquake
/// includes:
/// * long - this.value is f64 of its location
/// * lat - this.value is f64 of its location
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Epicenter {
    #[serde(rename(serialize = "long", deserialize = "epiCenterLat"))]
    pub long: LongLat,
    #[serde(rename(serialize = "lat", deserialize = "epiCenterLon"))]
    pub lat: LongLat,
}
