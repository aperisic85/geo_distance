use serde::Deserialize;
use std::f64::consts::PI;
use std::{error::Error, fs::File, io::BufReader};

#[derive(Debug, Deserialize, Clone)]
pub struct Positions {
    pub date: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone)]
pub struct PositionWIthDistance {
    pub position: Positions,
    pub distance: f64,
}

impl std::fmt::Display for PositionWIthDistance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Udaljenost: {:.3} m. Vrijeme: {}",
            self.distance, self.position.date
        )
    }
}
fn to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn calc_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // polumjer zemlje u km
    let dlat = to_radians(lat2 - lat1);
    let dlon = to_radians(lon2 - lon1);

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}

pub fn read_from_csv(
    b: BufReader<File>,
    mut output: Vec<Positions>,
) -> Result<Vec<Positions>, Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(b);
    for result in rdr.deserialize() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let positions: Positions = result?;
        output.push(positions);
        //println!("{:?}", positions);
    }
    Ok(output)
}
