use std::io;

fn main() {
    println!("Temperature Conversion");

    let mode = select_mode();
    let temperature = read_temperature(mode.clone());

    let converted_temperature: f64;
    let degree_type: String;

    if mode == "C" {
        converted_temperature = c_to_f(temperature);
        degree_type = "Fahrenheit".to_string();
    } else {
        converted_temperature = f_to_c(temperature);
        degree_type = "Celcius".to_string();
    }

    println!("Your temperature is {} degrees {}", converted_temperature, degree_type);
}

fn read_temperature(mode: String) -> f64 {
    let temp: f64 = loop {
        println!("Enter your temperature in {}", mode);

        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature)
            .expect("failed to read line");

        match temperature.trim()
            .parse() {
                Ok(num) => break num,
                Err(_) => continue,
            }
    };

    return temp;
}

fn select_mode() -> String {
    let mode: String = loop {
        println!("Select your conversion");
        println!("\t[F]: F to C\n\t[C]: C to F");

        let mut mode = String::new();
        io::stdin().read_line(&mut mode)
            .expect("failed to read line");

        if mode.trim() == "F"  || mode.trim() == "C" {
            break mode;
        }
    };

    return mode.trim().to_string();
}

fn f_to_c(temperature: f64) -> f64 {
    (temperature - 32.0) * (5.0/9.0)
}

fn c_to_f(temperature: f64) -> f64 {
    temperature * (9.0/5.0) + 32.0
}
