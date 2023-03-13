use std::io;

fn main() {
    let mileage: u32;
    loop {
        let mut input_line = String::new();

        println!("Give a mileage: ");
    
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line !");

        match input_line.trim().parse() {
            Ok(num) => {
                mileage = num;
                break;
            },
            Err(_) => {
                println!("Input not an integer");
                continue;
            } 
        };
    };

    println!("Mileage given : {mileage}");
} 