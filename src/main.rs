use distane::position::{PositionWIthDistance, Positions};
use std::{fs::File, io::BufReader};
//const DEFAULT_LAT: f64 = 45.13222;
//const DEFAULT_LONG: f64 = 13.5914833;
const DEFAULT_LAT: f64 = 43.0534;
const DEFAULT_LONG: f64 = 16.456683;

fn main() -> std::io::Result<()> {
    let args = distane::args::parse_args();
    let file = File::open(args.file)?;
    let buf_reader = BufReader::new(file);
    let results: Vec<Positions> = Vec::new();
    let mut alarm_positions: Vec<PositionWIthDistance> = Vec::new();

    match distane::position::read_from_csv(buf_reader, results) {
        Ok(out) => {
            let mut pos_no: usize = 0;
            for position in out {
                pos_no = pos_no + 1;
                let pos = PositionWIthDistance {
                    position: position.clone(),
                    distance: distane::position::calc_distance(
                        DEFAULT_LAT,
                        DEFAULT_LONG,
                        position.lat,
                        position.lon,
                    ),
                };
                alarm_positions.push(pos);
            }
            println!("{} positions in file", pos_no);
        }
        Err(err) => {
            println!("Error : {}", err)
        }
    }

    let mut pos_no = 0;

    for position in alarm_positions {
        
        if position.distance >= args.radius as f64 {
            pos_no +=1;
            println!("{}", position);
        }
    }
    println!("{} of positions in alarm", pos_no);
    

    Ok(())
}
