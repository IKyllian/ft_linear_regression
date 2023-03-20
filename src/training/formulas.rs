use crate::Record;

pub fn estimate_price(t0: f64, t1: f64, mileage: f64) -> f64 {
    t0 + (t1 * mileage)
}

pub fn summ_t0(t0: f64, t1: f64, datas: &Vec<Record>) -> f64 {
    let mut value: f64 = 0.0;

    for data in datas {
        value += estimate_price(t0, t1, data.km) - data.price;
    }

    value
}

pub fn summ_t1(t0: f64, t1: f64, datas: &Vec<Record>) -> f64 {
    let mut value: f64 = 0.0;

    for data in datas {
        value += (estimate_price(t0, t1, data.km) - data.price) * data.km;
    }

    value
}