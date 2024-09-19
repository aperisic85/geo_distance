use serde::Deserialize;
use std::f64::consts::PI;
use std::{error::Error, fs::File, io::BufReader};

const DEFAULT_LAT: f64 = 45.13222;
const DEFAULT_LONG: f64 = 13.5914833;

fn to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[derive(Debug, Deserialize, Clone)]
struct Positions {
    date: String,
    lat: f64,
    lon: f64,
}

#[derive(Debug, Clone)]
struct PositionWIthDistance {
    position: Positions,
    distance: f64,
}

fn calc_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // polumjer zemlje u km
    let dlat = to_radians(lat2 - lat1);
    let dlon = to_radians(lon2 - lon1);

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    r * c
}

fn read_from_csv(
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
fn main() -> std::io::Result<()> {
    let file = File::open("sample.csv")?;
    let buf_reader = BufReader::new(file);
    let results: Vec<Positions> = Vec::new();
    let mut alarm_positions: Vec<PositionWIthDistance> = Vec::new();

    /* if let Err(err) = read_from_csv(buf_reader, results) {
        println!("error running example: {}", err);
        process::exit(1);
    } */

    match read_from_csv(buf_reader, results) {
        Ok(out) => {
            for position in out {
                let pos = PositionWIthDistance {
                    position: position.clone(),
                    distance: calc_distance(
                        DEFAULT_LAT,
                        DEFAULT_LONG,
                        position.clone().lat,
                        position.clone().lon,
                    ),
                };
                alarm_positions.push(pos);
            }
        }
        Err(err) => {
            println!("Error : {}", err)
        }
    }

    for position in alarm_positions {
        if position.distance > 40.0 {
            println!("{:?}", position);
        }
    }

    Ok(())
}
