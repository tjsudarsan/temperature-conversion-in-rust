use std::io;

fn main() { 
    loop {
        println!("Select the converstion method below:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Fahrenheit to Celsius");
        println!("Enter number (1 or 2): ");

        let mut method = String::new();
        io::stdin().read_line(&mut method).expect("Input Error");
        let method: u8 = match method.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid degree! Enter again\n");
                continue;
            }
        };

        if method > 2 {
            println!("Invalid method!\n");
            continue;
        }
        
        loop {
            let mut degree = String::new();

            if method == 1 {
                println!("\nEnter the temperature in Celsius:");
            } else {
                println!("\nEnter the temperature in Fahrenheit:");
            }
            io::stdin().read_line(&mut degree).expect("Input Error");
    
            let degree: f64 = match degree.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid degree!\n");
                    continue;
                }
            };
            
            let result: f64;
            if method == 1 {
                result = (degree * 1.8000) + 32.0;
            } else {
                result = (degree - 32.0)/1.8000;
            }

            println!("The converted result is {}", result);
            break;
        }
        break;
    }
}
