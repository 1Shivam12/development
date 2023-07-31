use std::io;

fn main() {
    // let temp_array = [0,40 , 100, -200];
    let mut conversion_type_input = String::new();
    println!("Choose Conversion Type: (1) = C to F, (2) = F to C");
    io::stdin().read_line(&mut conversion_type_input).expect("Failed To Read Input");

    // Convert to int
    let conversion_type: u8 = conversion_type_input.trim().parse().expect("Input Invalid");

    let [source_unit, target_unit] = match conversion_type {
        1 => ["C", "F"],
        2 => ["F", "C"],
        _ => ["Invalid", "Invalid"]
    };

    let mut temp_input = String::new();
    println!("Input the number you want to convert: ");
    io::stdin().read_line(&mut temp_input).expect("Failed to read temp input");

    // Convert Temperature
    let temp: i32 = temp_input.trim().parse().expect("Input Invalid");
    let converted_temp = convert_temperature_to(temp, conversion_type);
    println!("{temp} {source_unit} is converted to {converted_temp} {target_unit}");
}

fn convert_temperature_to(temperature: i32, result_type: u8) -> i32 {
    if result_type == 1 {
        (temperature * 9/5) + 32
    }
    else if result_type == 2 {
        (temperature - 32) * 5/9
    }
    else {
        temperature
    }
}
