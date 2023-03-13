use std::error::Error;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    km: i64,
    price: i64,
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
}
