pub mod regions {
    
    pub fn process_command () {
        loop {
            println!("Please input an army command or help to see available commands or back to go back!");
            let mut input   =   String::new();
            std::io::stdin().read_line(&mut input).expect("Error parsing input!");
            let input : &str    =   input.trim();
            if input == "help" {
                println!("Available commands: 'print', 'move_army', 'print_beseiged', 'change_owner', 'add_building',
                'remove_building'");
            } else if input == "back" {
                break;
            } else if input == "print" {
                execute::print();
            } else if input == "move_army" {
                execute::move_army();
            } else if input == "print_beseiged" {
                execute::print_beseiged();
            } else if input == "change_owner" {
                execute::change_owner();
            } else if input == "add_building" {
                execute::add_building();
            } else if input == "remove_building" {
                execute::remove_building();
            } else {
                println!("Invalid command. Use 'help' to see available.");
            }
        }
        return;
    }
    
    pub mod execute {
        
        pub fn print () { 

        }
        
        pub fn move_army () {

        }

        pub fn print_beseiged () {

        }

        pub fn change_owner () {

        }

        pub fn add_building () {

        }

        pub fn remove_building () { 

        }
    }
}
