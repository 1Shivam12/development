fn main() {
    let temp_array = [0,40 , 100, -200];
    for temp in temp_array {
        let converted_temp_to_c = convert_temperature_to(temp, "c");
        println!("Convert {temp}F to Celsius: {converted_temp_to_c}C");
        let converted_temp_to_f = convert_temperature_to(temp, "f");
        println!("Convert {temp}C to Celsius: {converted_temp_to_f}F")
    }
}

fn convert_temperature_to(temperature: i32, target_unit: &str) -> i32 {
    let formatted_unit = target_unit.to_lowercase();
    if formatted_unit == "f" {
        (temperature * 9/5) + 32
    }
    else if formatted_unit == "c" {
        (temperature - 32) * 5/9
    }
    else {
        temperature
    }
}
