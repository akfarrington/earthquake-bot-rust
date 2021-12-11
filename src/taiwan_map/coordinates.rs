use crate::taiwan_map::{
    BASE_LAT, BASE_LONG, DEGREE_TO_LAT, DEGREE_TO_LONG, PIC_HEIGHT, PIC_WIDTH,
};

/// for each earthquake, the CWB will have stations with the intensity
/// as well as the the real life coordinates of each station
pub struct Coords {
    pub x: u32,
    pub y: u32,
    pub intensity: u8,
}

impl Coords {
    pub fn from_coordinates(long: f64, lat: f64, intensity: u8) -> Option<Self> {
        let y: i16 = PIC_HEIGHT as i16 - ((long - BASE_LONG) * DEGREE_TO_LONG) as i16;
        let x: i16 = ((lat - BASE_LAT) * DEGREE_TO_LAT) as i16;

        // check if the x or y are too low or high, and return None if so
        if x < 0 || x > PIC_WIDTH as i16 || y < 0 || y > PIC_HEIGHT as i16 {
            return None;
        }

        Some(Self {
            x: x as u32,
            y: y as u32,
            intensity,
        })
    }
}
