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

        }

        pub fn remove () {

        }

        pub fn print () {

        }

        pub fn print_faction () {

        }
    }
}
