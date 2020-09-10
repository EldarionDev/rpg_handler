pub mod armies {    
    pub fn process_command () {
        loop {
            println!("Please input an army command or help to see available commands or back to go back!");
            let mut input   =   String::new();
            std::io::stdin().read_line(&mut input).expect("Error parsing input!");
            let input : &str    =   input.trim();
            if input == "help" {
                println!("Available commands: 'add', 'remove', 'add_unit', 'remove_unit', 'print'");
            } else if input == "back" {
                break;
            } else if input == "add" {
                execute::add();
            } else if input == "remove" {
                execute::remove();
            } else if input == "add_unit" {
                execute::add_unit();
            } else if input == "remove_unit" {
                execute::remove_unit();
            } else if input == "print" {
                execute::print();
            } else {
                println!("Invalid command. Use 'help' to see available.");
            }
        }
        return;
    }

    mod execute {
        
        pub fn add () {

        }

        pub fn remove () {

        }

        pub fn add_unit () {

        }

        pub fn remove_unit () {

        }

        pub fn print () {

        }
    }
}
