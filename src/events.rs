pub mod events {

    pub fn process_command () {
        loop {
            println!("Please input an event command or help to see available commands or back to go back!");
            let mut input   =   String::new();
            std::io::stdin().read_line(&mut input).expect("Error parsing input!");
            let input : &str    =   input.trim();
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
        
        pub fn add () {
            println!("Please enter the name of the new event!");
            let mut event_name  =   String::new();
            std::io::stdin().read_line(&mut event_name).expect("Error parsing input!");

            println!("Please enter an event duration!");
            let mut event_duration_string   =   String::new();
            std::io::stdin().read_line(&mut event_duration_string).expect("Error parsing input!");
            let event_duration: u32 =   event_duration_string.parse().unwrap();
            
            println!("Please enter the number of effects you want to add!");
            let mut effect_count_string =   String::new();
            std::io::stdin().read_line(&mut effect_count_string).expect("Error parsing input!");
            let effect_count: u32   =   match effect_count_string.parse() {
                Ok(effect_count) => effect_count,
                Err(e) => {
                    println!("Please enter a valid number! {}", e);
                    0
                }
            };
            
            let effect_affects: Vec<String>;
            let effect_strengths: Vec<i32>;
            let effect_count: u32;
            for number in 1..effect_count {
                        
            }
        }

        pub fn remove () {

        }

        pub fn print () {

        }

        pub fn print_faction () {

        }
    }
}
