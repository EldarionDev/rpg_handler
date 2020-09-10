pub mod events {

    pub fn process_command() {
        loop {
            println!("Please input an event command or help to see available commands or back to go back!");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error parsing input!");
            let input: &str = input.trim();
            if input == "help" {
                println!("Available commands: 'add', 'remove', 'print', 'print_faction'");
            } else if input == "back" {
                break;
            } else if input == "add" {
                execute::add();
            } else if input == "remove" {
                execute::remove();
            } else if input == "print" {
                execute::print();
            } else if input == "print_faction" {
                execute::print_faction();
            } else {
                println!("Invalid command. Use 'help' to see available.");
            }
        }
        return;
    }

    pub mod execute {

        pub fn add() {
            println!("Please enter the name of the new event!");
            let mut event_name = String::new();
            std::io::stdin()
                .read_line(&mut event_name)
                .expect("Error parsing input!");

            println!("Please enter an event duration!");
            let mut event_duration_string = String::new();
            std::io::stdin()
                .read_line(&mut event_duration_string)
                .expect("Error parsing input!");
            let event_duration: u32 = match event_duration_string.trim().parse() {
                Ok(event_duration) => event_duration,
                Err(e) => {
                    println!("Please insert a valid number! {}", e);
                    return;
                }
            };

            println!("Please enter the number of effects you want to add!");
            let mut effect_count_string = String::new();
            std::io::stdin()
                .read_line(&mut effect_count_string)
                .expect("Error parsing input!");
            let effect_count: usize = match effect_count_string.trim().parse() {
                Ok(effect_count) => effect_count,
                Err(e) => {
                    println!("Please enter a valid number! {}", e);
                    return;
                }
            };

            let mut effect_affects: Vec<String> = vec![String::new(); effect_count];
            let mut effect_strengths: Vec<i32> = vec![0; effect_count];
            for number in 1..effect_count {
                println!("Please enter the name of the {}. effect.", number);
                let mut affect = String::new();
                std::io::stdin()
                    .read_line(&mut affect)
                    .expect("Could not parse input!");
                effect_affects[number - 1] = affect.clone();
                println!("Please enter the strength of the effect.");
                let mut strength_string = String::new();
                std::io::stdin()
                    .read_line(&mut strength_string)
                    .expect("Could not parse input!");
                let strength: i32 = match strength_string.trim().parse() {
                    Ok(strength) => strength,
                    Err(e) => {
                        println!("Please enter a valid number! {}", e);
                        return;
                    }
                };
                effect_strengths[number - 1] = strength;
            }

            let mut repetition_string = String::new();
            std::io::stdin()
                .read_line(&mut repetition_string)
                .expect("Could not parse input!");
            let repetition: u32 = match repetition_string.trim().parse() {
                Ok(repetition) => repetition,
                Err(e) => {
                    println!("Please enter a valid number! {}", e);
                    return;
                }
            };
        }

        pub fn remove() {}

        pub fn print() {}

        pub fn print_faction() {}
    }
}
