mod coordinates;

use crate::cwb_api::structs::{Earthquake, IntensityConversionErrors};
use crate::taiwan_map::coordinates::Coords;
use image::{open, DynamicImage, GenericImage, GenericImageView};

const DEGREE_TO_LONG: f64 = 279.065;
const DEGREE_TO_LAT: f64 = 256.364;
const BASE_LONG: f64 = 21.8898;
const BASE_LAT: f64 = 119.2021;
const PIC_WIDTH: u32 = 950;
const PIC_HEIGHT: u32 = 1000;
const EQ_STATION_BOX_SIZE: u32 = 12;
const EQ_STATION_BOX_BORDER: u32 = 2;
const EPICENTER_ICON_SIZE: u32 = 40;

pub const MAP_LOC: &str = "img/eq-map.png";
pub const EPI_PIC_LOC: &str = "img/eq-epi.png";

impl Earthquake {
    /// this will just take an earthquake as an argument, then it'll
    /// return an EqMap with a completed image
    pub fn mark_image_with_eq_data(&self) {
        let mut map_image = open(MAP_LOC).expect("couldn't open eq-map.png");

        // first mark the epicenter (looks nicer with the station on top if they overlap)
        let epicenter_coords = Coords::from_coordinates(
            self.earthquake_info.epicenter.long,
            self.earthquake_info.epicenter.lat,
            0, // this won't matter, so marking it 0
        );
        if let Some(epicenter_coords) = epicenter_coords {
            mark_epicenter(&mut map_image, epicenter_coords);
        }

        // iterate through the areas, which I guess mean cities/counties
        for area in self.intensity.shaking_area.iter() {
            // iterate through the station in those areas
            for station in &area.eq_station {
                // get intensity (default to 1 if there's an error)
                let intensity = match station.convert_station_intensity_to_u8() {
                    Ok(num) => num,
                    Err(e) => {
                        // todo later just delete this since I don't really care if it works
                        // this is just here in case something breaks, but it likely won't break
                        // but in the meantime, just log some stuff if there are some errors
                        println!("There was an error with this intensity: {}", station.station_intensity);
                        match e {
                            IntensityConversionErrors::ParseError(ee) => {
                                println!("Error text: {}", ee);
                            },
                            IntensityConversionErrors::OutOfBounds(ee) => {
                                println!("Parsed to this value: {}", ee);
                            }
                        }

                        0
                    },
                };

                // get coordinates
                if let Some(coords) = Coords::from_coordinates(
                    station.station_lat,
                    station.station_long,
                    intensity,
                ) {
                    // *********
                    // the coordinates are within the image bounds
                    // *********
                    add_station_to_map(&mut map_image, coords);
                }
            }
        }

        let _ = map_image.save("temp.png");
    }
}

/// add a color coded square on the map to indicate the intensity of the shaking at the station
fn add_station_to_map(map_image: &mut DynamicImage, coordinates: Coords) {
    // make a black box that'll go behind the colored box
    let back_box_size = EQ_STATION_BOX_SIZE + (EQ_STATION_BOX_BORDER * 2);
    let box_start_x = coordinates.x - back_box_size / 2;
    let box_end_x = box_start_x + back_box_size;
    let box_start_y = coordinates.y - back_box_size / 2;
    let box_end_y = box_start_y + back_box_size;
    make_box(
        map_image,
        box_start_x,
        box_end_x,
        box_start_y,
        box_end_y,
        image::Rgba([0, 0, 0, 255]),
    );

    // do the same thing again, but with the regular sized box
    let box_size = EQ_STATION_BOX_SIZE;
    let box_start_x = coordinates.x - box_size / 2;
    let box_end_x = box_start_x + box_size;
    let box_start_y = coordinates.y - box_size / 2;
    let box_end_y = box_start_y + box_size;
    make_box(
        map_image,
        box_start_x,
        box_end_x,
        box_start_y,
        box_end_y,
        get_rgba_from_intensity(coordinates.intensity).expect("intensity out of expected range"),
    );
}

/// takes a start and end pixel for x and y, then makes a square with the color provided
fn make_box(
    map_image: &mut DynamicImage,
    box_start_x: u32,
    box_end_x: u32,
    box_start_y: u32,
    box_end_y: u32,
    color: image::Rgba<u8>,
) {
    // iterate through x and y and put a pixel
    for x in box_start_x..=box_end_x {
        for y in box_start_y..=box_end_y {
            if (0..PIC_WIDTH).contains(&x) && (0..PIC_HEIGHT).contains(&y) {
                map_image.put_pixel(x, y, color);
            }
        }
    }
}

fn get_rgba_from_intensity(intensity: u8) -> Option<image::Rgba<u8>> {
    match intensity {
        8 => Some(image::Rgba([13, 0, 140, 255])),
        7 => Some(image::Rgba([13, 0, 140, 255])),
        6 => Some(image::Rgba([240, 0, 255, 255])),
        5 => Some(image::Rgba([255, 0, 0, 255])),
        4 => Some(image::Rgba([255, 130, 0, 255])),
        3 => Some(image::Rgba([255, 220, 0, 255])),
        2 => Some(image::Rgba([0, 140, 0, 255])),
        1 => Some(image::Rgba([0, 190, 0, 255])),
        // 0 is if there's an error parsing the station intensity string
        0 => Some(image::Rgba([100, 100, 100, 255])),
        _ => None,
    }
}

fn mark_epicenter(map_image: &mut DynamicImage, coords: Coords) {
    let epi_image = open(EPI_PIC_LOC).expect("couldn't find eq-epi.png");

    let box_size = EPICENTER_ICON_SIZE;
    let box_start_x = coords.x - box_size / 2;
    let box_end_x = box_start_x + box_size;
    let box_start_y = coords.y - box_size / 2;
    let box_end_y = box_start_y + box_size;

    for (x_icon, x_map) in (box_start_x..box_end_x).enumerate() {
        for (y_icon, y_map) in (box_start_y..box_end_y).enumerate() {
            // skip this iteration if x or y aren't in the image bounds
            if !(0..PIC_WIDTH).contains(&x_map) || !(0..PIC_HEIGHT).contains(&y_map) {
                // println!("trying to write to {}, {} - skipping", x_map, y_map);
                continue;
            }

            // first get the pixel from the epicenter icon, check if it's transparent or not
            // then manually use its A value to make it transparent
            let mut icon_pixel = epi_image.get_pixel(x_icon as u32, y_icon as u32);
            let icon_rgba = icon_pixel;
            let map_rgba = map_image.get_pixel(x_map, y_map);

            if icon_rgba[3] == 0 {
                // this pixel is transparent, so no need to continue processing. Just
                // loop again
                continue;
            } else if icon_rgba[3] != 255 {
                // not completely transparent or opaque, so process
                // if A == 255, no processing, but will just directly copy to the map image.
                // get "weight" of both the map and icon. Use 255 as the max since it's the max
                let icon_weight: f64 = icon_rgba[3] as f64 / 255.0;
                let map_weight: f64 = 1.0 - icon_weight;

                // apply weights to the rgb values
                let target_r: u8 =
                    ((icon_rgba[0] as f64 * icon_weight) + (map_rgba[0] as f64 * map_weight)) as u8;
                let target_g: u8 =
                    ((icon_rgba[1] as f64 * icon_weight) + (map_rgba[1] as f64 * map_weight)) as u8;
                let target_b: u8 =
                    ((icon_rgba[2] as f64 * icon_weight) + (map_rgba[2] as f64 * map_weight)) as u8;
                let target_a: u8 = 255;

                // modify the target pixel to the target values
                icon_pixel = image::Rgba([target_r, target_g, target_b, target_a]);
            }

            map_image.put_pixel(x_map, y_map, icon_pixel);
        }
    }
}
