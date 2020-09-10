pub mod factions {
    
    pub fn process_command () {
        loop {
            println!("Please input an army command or help to see available commands or back to go back!");
            let mut input   =   String::new();
            std::io::stdin().read_line(&mut input).expect("Error parsing input!");
            let input : &str    =   input.trim();
            if input == "help" {
                println!("Available commands: 'print_balance', 'print_military_strength', 'print_economic_strength',
                'print_upkeep_total', 'print_leader', 'print_members', 'change_leader', 'add_member', 'remove_member'");
            } else if input == "back" {
                break;
            } else if input == "print_balance" {
                execute::print_balance();
            } else if input == "print_military_strength" {
                execute::print_military_strength();
            } else if input == "print_economic_strength" {
                execute::print_economic_strength();
            } else if input == "print_upkeep_total" {
                execute::print_upkeep_total();
            } else if input == "print_leader" {
                execute::print_leader();
            } else if input == "print_members" {
                execute::print_members();
            } else if input == "change_leader" {
                execute::change_leader();
            } else if input == "add_member" {
                execute::add_member();
            } else if input == "remove_member" {
                execute::remove_member();
            } else {
                println!("Invalid command. Use 'help' to see available.");
            }
        }
        return;
    }

    pub mod execute {
        
        pub fn print_balance () {

        }

        pub fn print_military_strength () {

        }

        pub fn print_economic_strength () {

        }

        pub fn print_upkeep_total () {

        }

        pub fn print_leader () {

        }

        pub fn print_members () {

        }

        pub fn change_leader () {

        }

        pub fn add_member () {

        }

        pub fn remove_member () {

        }
    }
}
