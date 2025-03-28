use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

fn input(prompt: &str) -> i32 {
    loop {
        // outputs prompt based on the text
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        // takes the typed number as input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        // checks if it's a valid number
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Please input a valid number!");
                continue;
            }
        }
    }
}

fn math(first: i32, operand: &str, second: i32) -> String {
    // calculates the result based on the inputted numbers and operand
    match operand {
        "+" | "add" => (first + second).to_string(),
        "-" | "sub" | "min" => (first - second).to_string(),
        "*" | "times" | "multi" => (first * second).to_string(),
        "/" | ":" | "div" => (first as f32 / second as f32).to_string(),
        "^" | "pow" => (first.pow(second as u32)).to_string(),
        _ => "Please input a valid operand!".to_string(),
    }
}

fn main() {
    println!("A simple calculator!");

    'cal_loop: loop {
        // takes the first number and second number as input
        let first_num = input("Input first number: ");
        let second_num = input("Input second number: ");

        // takes operand as input for calculation method
        print!("Input operand: ");
        io::stdout().flush().unwrap();

        let mut operand = String::new();
        io::stdin()
            .read_line(&mut operand)
            .expect("Reading failed!");

        let operand = operand.trim().to_lowercase();

        // calculates the result
        let result = math(first_num, &operand, second_num);

        // displays the operand
        let operand_sign = match operand.as_str() {
            "+" | "add" => "+",
            "-" | "sub" | "min" => "-",
            "*" | "times" | "multi" => "*",
            "/" | ":" | "div" => "/",
            "^" | "pow" => "^",
            _ => "?",
        };

        // prints the inputted numbers, operand, and the result
        println!("{} {} {} = {}", first_num, operand_sign, second_num, result);

        // calculate again confirmation
        'again_loop: loop {
            print!("Calculate again? (y/n, press Enter for y): ");
            io::stdout().flush().unwrap();

            let mut again_input = String::new();
            io::stdin()
                .read_line(&mut again_input)
                .expect("Reading failed!");
            let again = again_input.trim().to_lowercase();

            match again.as_str() {
                // continues the loop if confirmed
                "" | "y" | "yes" => continue 'cal_loop,

                // closes the calculator
                "n" | "no" => {
                    print!("Closing...");
                    io::stdout().flush().unwrap();
                    sleep(Duration::from_secs(2));
                    break 'cal_loop;
                }
                // repeats the confirmation loop if input is not valid
                _ => {
                    println!("Input must be 'y' or 'n'!");
                    continue 'again_loop;
                }
            }
        }
    }
}
