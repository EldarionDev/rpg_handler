mod armies;
mod events;
mod factions;
mod regions;

fn process_input(input: &str) {
    if input == "armies" {
        armies::armies::process_command();
    } else if input == "events" {
        events::events::process_command();
    } else if input == "factions" {
        factions::factions::process_command();
    } else if input == "regions" {
        regions::regions::process_command();
    } else {
        println!("Invalid command, use 'help' to see available.");
        return;
    }
}

fn main() {
    println!(
        "A RPG Handler written by Eldarion for the Reforged community, inspired by Finrod's RP."
    );
    loop {
        println!("Please enter a command to continue or help to show all available commands or quit to quit.");
        let mut input_string = String::new();
        std::io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to parse input!");
        let trimmed_string = input_string.trim();
        if trimmed_string == "help" {
            println!("Available commands: 'armies', 'events', 'factions', 'regions'");
        } else if trimmed_string == "quit" {
            break;
        } else {
            process_input(trimmed_string);
        }
    }
    return;
}
