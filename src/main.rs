
use std::{error::Error, fs::File, io::BufReader};
use distane::position::{Positions,PositionWIthDistance};

const DEFAULT_LAT: f64 = 45.13222;
const DEFAULT_LONG: f64 = 13.5914833;

fn main() -> std::io::Result<()> {
    let file = File::open("sample.csv")?;
    let buf_reader = BufReader::new(file);
    let results: Vec<Positions> = Vec::new();
    let mut alarm_positions: Vec<PositionWIthDistance> = Vec::new();

    /* if let Err(err) = read_from_csv(buf_reader, results) {
        println!("error running example: {}", err);
        process::exit(1);
    } */

    match distane::position::read_from_csv(buf_reader, results) {
        Ok(out) => {
            for position in out {
                let pos = PositionWIthDistance {
                    position: position.clone(),
                    distance: distane::position::calc_distance(
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
            println!("{}", position);
        }
    }

    Ok(())
}
