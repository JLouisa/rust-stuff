// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::collections::HashMap;

mod clear_terminal {
    #[cfg(not(target_os = "windows"))]
    pub fn clear_terminal() {
        std::process::Command::new("clear").status().unwrap();
    }

    #[cfg(target_os = "windows")]
    pub fn clear_terminal() {
        std::process::Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
    }
}

#[derive(Debug)]
enum Menu {
    AddBills,
    ViewBills,
    RemoveBills,
    UpdateBills,
    Total,
    Invalid,
}
impl Menu {
    fn select(buffer: &str) -> Option<Menu> {
        match buffer {
            "add" | "1" => Some(Menu::AddBills),
            "view" | "2" => Some(Menu::ViewBills),
            "remove" | "3" => Some(Menu::RemoveBills),
            "update" | "4" => Some(Menu::UpdateBills),
            "total" | "5" => Some(Menu::Total),
            _ => Some(Menu::Invalid),
        }
    }
}

enum Text {
    Main,
    View,
    Add,
    Remove,
    Update,
    UpdateBill,
    UpdateName,
    UpdateAmount,
    ViewName,
    ViewAmount,
    Instructions,
    Empty,
}
impl Text {
    fn print(text: Text) {
        match text {
            Text::Main => {
                println!("== Manage Bills ==");
                println!("   1. Add bills");
                println!("   2. View bills");
                println!("   3. Remove bills");
                println!("   4. Update bills");
                println!("   5. Total bills");
                println!(" ");
                println!("Enter selection:");
            }
            Text::View => println!("== View Bills =="),
            Text::Add => println!("== Add Bills =="),
            Text::Remove => println!("== Remove Bill =="),
            Text::UpdateBill => println!("== Update Bill =="),
            Text::Update => println!("Which bill do you want to update?"),
            Text::UpdateName => println!(" Update Name "),
            Text::UpdateAmount => println!(" Update Amount "),
            Text::ViewName => println!("Enter bill name:"),
            Text::ViewAmount => println!("Enter bill amount:"),
            Text::Instructions => println!("> Main Menu: 'exit' or 'back'"),
            Text::Empty => println!(" "),
        }
    }
    fn calc_total(bills: &HashMap<String, f64>) -> f64 {
        let mut total = 0.00;
        for value in bills.values() {
            total += value
        }
        let formatted_float = format!("{:.2}", total);
        let truncated_float: f64 = formatted_float.parse().unwrap();
        return truncated_float;
    }
    fn print_total(arr: &HashMap<String, f64>) {
        if arr.len() > 0 {
            println!("Total: €{:?}", Text::calc_total(arr))
        }
    }
    fn print_bills(bills: &HashMap<String, f64>) {
        let mut i = 0;
        for (key, value) in bills.iter() {
            println!("{}. Name: {}, amount: €{}", i + 1, key, value);
            i += 1;
        }
    }
}

#[derive(Debug)]
struct Bills {
    name: String,
    amount: f64,
}
impl Bills {
    // Create a bill
    fn new(name: String, amount: f64) -> Self {
        Self { name, amount }
    }
    // Add bill to vec!
    fn add(self, bills: &mut HashMap<String, f64>) {
        bills.insert(self.name, self.amount);
    }
}

mod user_input {
    use std::io;

    pub fn user_input() -> (Result<usize, io::Error>, String) {
        let mut buffer_name = String::new();
        let bill_name = io::stdin().read_line(&mut buffer_name);
        let buffer_name = buffer_name.trim().to_lowercase();
        (bill_name, buffer_name)
    }

    pub mod main_menu {
        use crate::*;

        pub fn input() -> Menu {
            let bill_name = user_input::user_input();
            if bill_name.0.is_ok() {
                let selection = match Menu::select(&bill_name.1) {
                    Some(Menu::AddBills) => Menu::AddBills,
                    Some(Menu::ViewBills) => Menu::ViewBills,
                    Some(Menu::RemoveBills) => Menu::RemoveBills,
                    Some(Menu::Invalid) => Menu::Invalid,
                    Some(Menu::UpdateBills) => Menu::UpdateBills,
                    Some(Menu::Total) => Menu::Total,
                    None => Menu::Invalid,
                };
                return selection;
            } else {
                return Menu::Invalid;
            }
        }
    }

    pub mod add_bill {
        use crate::*;

        pub fn input() -> String {
            let input = user_input::user_input();
            let input = if input.0.is_ok() {
                input.1
            } else {
                println!("Something went wrong");
                "".to_string()
            };
            return input;
        }
        pub fn input_float() -> f64 {
            let input = user_input::user_input();
            let input = if input.0.is_ok() {
                input.1
            } else {
                println!("Something went wrong");
                "".to_string()
            };
            let input_float = input.parse::<f64>();
            let input_float: f64 = if input_float.is_ok() {
                match input_float {
                    Ok(result) => result,
                    _ => 0.00,
                }
            } else {
                println!("Something went wrong");
                0.00
            };
            // Restrict float to 2 decimals
            let formatted_float = format!("{:.2}", input_float);
            let truncated_float: f64 = formatted_float.parse().unwrap();
            return truncated_float;
        }
    }
}

mod section {
    pub mod main {
        use crate::*;

        pub fn main() {
            // Clear Terminal
            clear_terminal::clear_terminal();
            Text::print(Text::Main);
        }
    }
    pub mod add_bill {
        use crate::*;

        pub fn bill(bills: &mut HashMap<String, f64>) {
            loop {
                // Clear Terminal
                clear_terminal::clear_terminal();

                // Print bills
                Text::print(Text::Add);
                Text::print(Text::Empty);
                if bills.len() > 0 {
                    Text::print_bills(&bills);
                    Text::print(Text::Empty);
                }
                Text::print(Text::Instructions);
                Text::print(Text::Empty);
                Text::print(Text::ViewName);

                let name = user_input::add_bill::input();
                if name.as_str() == "exit" || name.as_str() == "back" {
                    return;
                }
                Text::print(Text::ViewAmount);

                let amount = user_input::add_bill::input_float();

                let bill = Bills::new(name, amount);
                Bills::add(bill, bills);
            }
        }
    }
    pub mod view_bill {
        use crate::*;

        pub fn view(bills: &mut HashMap<String, f64>) {
            // Clear Terminal
            clear_terminal::clear_terminal();

            // View
            Text::print(Text::View);
            Text::print_bills(&bills);
            Text::print(Text::Empty);
            Text::print_total(&bills);
            Text::print(Text::Empty);

            Text::print(Text::Instructions);

            loop {
                let input = user_input::add_bill::input();
                if input.as_str() == "exit" || input.as_str() == "back" {
                    return;
                } else {
                    println!("Invalid input")
                }
            }
        }
    }
    pub mod remove_bill {
        use crate::*;

        pub fn remove(bills: &mut HashMap<String, f64>) {
            loop {
                // Clear Terminal
                clear_terminal::clear_terminal();

                Text::print(Text::Remove);
                Text::print_bills(bills);
                Text::print(Text::Empty);
                Text::print_total(bills);
                Text::print(Text::Empty);
                Text::print(Text::Instructions);

                let input = user_input::add_bill::input();
                if input.as_str() == "exit" || input.as_str() == "back" {
                    return;
                }

                bills.remove(&input);
            }
        }
    }
    pub mod update_bill {
        use crate::*;

        pub fn update(bills: &mut HashMap<String, f64>) {
            loop {
                // Clear Terminal and Print Text
                clear_terminal::clear_terminal();
                Text::print(Text::UpdateBill);
                Text::print_bills(bills);
                Text::print(Text::Empty);
                Text::print_total(bills);
                Text::print(Text::Empty);
                Text::print(Text::Instructions);
                Text::print(Text::Empty);
                Text::print(Text::Update);

                // Get user input for the bill name to update
                let input = user_input::add_bill::input();
                if input.as_str() == "exit" || input.as_str() == "back" {
                    return;
                }

                if bills.contains_key(&input) {
                    Text::print(Text::UpdateName);
                    let new_name = user_input::add_bill::input();
                    Text::print(Text::UpdateAmount);
                    let new_amount = user_input::add_bill::input_float();

                    bills.remove(&input);
                    bills.insert(new_name, new_amount);
                } else {
                    println!("Bill not found, please try again.");
                }
            }
        }
    }
}

fn main() {
    // Create vec! for the bills
    let mut hash_bills: HashMap<String, f64> = HashMap::new();

    loop {
        // Print main menu to screen
        section::main::main();

        // User Input to navigate main menu
        let user_input = user_input::main_menu::input();

        match user_input {
            Menu::AddBills => section::add_bill::bill(&mut hash_bills),
            Menu::ViewBills => section::view_bill::view(&mut hash_bills),
            Menu::RemoveBills => section::remove_bill::remove(&mut hash_bills),
            Menu::UpdateBills => section::update_bill::update(&mut hash_bills),
            Menu::Total => section::view_bill::view(&mut hash_bills),

            Menu::Invalid => println!("Invalid choice"),
        };
    }
}

//* Testing
#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_total() {
        let mut arr = HashMap::new();

        arr.insert("car".to_string(), 50.00);
        arr.insert("phone".to_string(), 20.00);
        arr.insert("home".to_string(), 30.00);

        let result = Text::calc_total(&mut arr);
        let expected = 100.0;
        assert_eq!(result, expected, "Result should be 100.00")
    }

    #[test]
    fn check_total2() {
        let mut arr = HashMap::new();

        arr.insert("car".to_string(), 2917.35);
        arr.insert("phone".to_string(), 11.40);
        arr.insert("home".to_string(), 71.25);

        let result = Text::calc_total(&mut arr);
        let expected = 3000.00;
        assert_eq!(result, expected, "Result should be 3000.00")
    }
}
