use std::io;
use csv::Reader;
use serde::Deserialize;

#[derive(Deserialize)]
struct Thetas {
    t0: f64,
    t1: f64,
}

fn get_thetas() -> Thetas {
    let rdr = Reader::from_path("thetas.csv");
    match rdr {
        Ok(mut f) => {
            let mut iter = f.deserialize();

            if let Some(result) = iter.next() {
                match result {
                    Ok(t) => return t,
                    Err(_e) => return Thetas { t0: 0.0, t1: 0.0 },
                }
            } else {
                Thetas {
                    t0: 0.0,
                    t1: 0.0,
                }
            }
        },
        Err(e) => {
            println!("Error : {}", e);
            return Thetas { t0: 0.0, t1: 0.0 }
        }
    }
}

fn main() {
    let mileage: i64;
    let thetas: Thetas = get_thetas();

    loop {
        let mut input_line = String::new();

        println!("Give a mileage: ");
    
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line !");

        match input_line.trim().parse() {
            Ok(num) => {
                if num >= 0 {
                    mileage = num;
                    break;
                } else {
                    println!("Input must be positive");
                    continue;
                }
            },
            Err(_) => {
                println!("Input not an integer");
                continue;
            }
        };
    };

    println!("Mileage given : {mileage}");
    println!("Price of a car with {} mileage : {}", mileage, (thetas.t0 + (thetas.t1 * mileage as f64)) as i64);
} 