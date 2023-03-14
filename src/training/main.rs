use std::error::Error;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Record {
    km: i64,
    price: i64,
}

#[derive(Serialize)]
struct Thetas {
    t0: i64,
    t1: i64,
}

fn write_theta_values(t0: i64, t1: i64) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("./thetas.csv")?;
    writer.serialize(Thetas {
        t0,
        t1,
    })?;
    writer.flush()?;
    Ok(())
}

fn read_csv_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("Km: {} - Price: {}", record.km, record.price);
    }
    Ok(())
}

fn main(){
    if let Err(e) = read_csv_file("../data.csv") {
        println!("Error: {}", e)
    }

    if let Err(e) = write_theta_values(32, 43) {
        println!("Error: {}", e)
    }
}
