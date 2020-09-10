pub mod armies {
    pub fn process_command() {
        loop {
            println!("Please input an army command or help to see available commands or back to go back!");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error parsing input!");
            let input: &str = input.trim();
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

        pub fn add() {
            println!("Please enter the army name you want to add!");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error parsing input!");
            let army_name = input.trim();
        }

        pub fn remove() {
            println!("Please enter the name of the army you want to remove!");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error parsing input!");
            let army_name = input.trim();
        }

        pub fn add_unit() {
            println!("Please enter the name of the army you want to add an unit to!");
            let mut input_army = String::new();
            std::io::stdin()
                .read_line(&mut input_army)
                .expect("Error parsing input!");
            let army_name = input_army.trim();

            println!("Please enter the name of the unit you want to add!");
            let mut input_unit = String::new();
            std::io::stdin()
                .read_line(&mut input_unit)
                .expect("Error parsing input!");
            let unit_name = input_unit.trim();
        }

        pub fn remove_unit() {
            println!("Please enter the name of the army you want to remove an unit from!");
            let mut input_army = String::new();
            std::io::stdin()
                .read_line(&mut input_army)
                .expect("Error parsing input!");
            let army_name = input_army.trim();

            println!("Please enter the name of the unit you want to remove!");
            let mut input_unit = String::new();
            std::io::stdin()
                .read_line(&mut input_unit)
                .expect("Error parsing input!");
            let unit_name = input_unit.trim();
        }

        pub fn print() {
            println!("Please enter the name of the army you want to print!");
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Error parsing input!");
            let army_name = input.trim();
        }
    }
}
