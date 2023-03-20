use std::error::Error;
use csv::Reader;
use std::vec::Vec;
use serde::{Deserialize, Serialize};

mod formulas;

#[derive(Deserialize)]
pub struct Record {
    km: f64,
    price: f64,
}

#[derive(Serialize)]
struct Thetas {
    t0: f64,
    t1: f64,
}

fn write_theta_values(t0: f64, t1: f64) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("./thetas.csv")?;
    writer.serialize(Thetas {
        t0,
        t1,
    })?;
    writer.flush()?;
    Ok(())
}

fn read_csv_file(path: &str, datas: &mut Vec<Record>) -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        datas.push(Record {km: record.km / 1000.0, price: record.price / 1000.0});
    }
    Ok(())
}

fn gradient_descent(datas: &Vec<Record>) {
    let mut t0: f64 = 0.0;
    let mut t1: f64 = 0.0;

    loop {
        let sum0 =  0.0001 * (formulas::summ_t0(t0, t1, datas) / (datas.len() as f64));
        let sum1 =  0.0001 * (formulas::summ_t1(t0, t1, datas) / (datas.len() as f64));

        if sum0.abs() < 0.000001 && sum1.abs() < 0.000001 {
            if let Err(e) = write_theta_values(t0 * 1000.0, t1) {
                println!("Error: {}", e)
            }
            return ;
        }

        t0 = t0 - sum0;
        t1 = t1 - sum1;
    }
}

fn main(){
    let mut datas: Vec<Record> = Vec::new();

    if let Err(e) = read_csv_file("../data.csv", &mut datas) {
        println!("Error: {}", e)
    }

    gradient_descent(&datas);
}
