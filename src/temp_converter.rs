use std::io;

pub fn temp_converter() {
    loop {
        let mut celsius = String::new();

        println!("Please input your temorature in fahranheit or 'q' to terminate program:");

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        if celsius.trim() == "q" {
            println!("Terminating program...");
            break;
        }

        let celsius: f32 = match celsius.trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius: f32 = (celsius - 32.0) * 5.0 / 9.0;

        println!("The temperature in celsius is: {celsius}");
    }
}
