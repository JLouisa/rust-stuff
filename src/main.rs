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

#[derive(Debug)]
enum Menu {
    AddBills,
    ViewBills,
    Invalid,
}
impl Menu {
    fn select(buffer: &str) -> Option<Menu> {
        match buffer {
            "add" | "1" => Some(Menu::AddBills),
            "view" | "2" => Some(Menu::ViewBills),
            _ => Some(Menu::Invalid),
        }
    }
}

enum Sec {
    First,
    Second,
    Begin,
    View,
    Instruction,
    Empty,
    Total,
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
    fn add(self, arr: &mut Vec<Bills>) {
        arr.push(self)
    }
    fn total(arr: &Vec<Bills>) -> f64 {
        let mut total = 0.00;
        arr.iter().for_each(|bill| total += bill.amount);
        return total;
    }
}

mod print {
    use crate::*;
    pub fn main_menu(sec: Sec) {
        match sec {
            Sec::Begin => {
                println!("== Manage Bills ==");
                println!("   1. Add bills");
                println!("   2. View bills");
                // println!("   2. Remove bills");
                // println!("   2. Update bills");
                // println!("   5. Total bills");
                println!(" ");
                println!("Enter selection:");
            }
            _ => println!("Invalid selection"),
        }
    }

    pub fn bill_menu(sec: Sec, total: Option<&str>) {
        match total {
            Some(total) => print!("Total: {}", total),
            None => match sec {
                Sec::Begin => println!("== Add Bills =="),
                Sec::First => println!("Enter bill name:"),
                Sec::Second => println!("Enter bill amount:"),
                Sec::View => println!("== View Bills =="),
                Sec::Instruction => println!("> Main Menu: 'exit' or 'back'"),
                Sec::Total => print!("Total: "),
                Sec::Empty => println!(" "),
            },
        }
    }

    pub fn bills_view(bills: &Vec<Bills>) {
        if bills.len() > 0 {
            bills
                .iter()
                .for_each(|bill| println!("Name: {:?}, amount: {:?}", bill.name, bill.amount))
        }
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
                    Some(Menu::Invalid) => Menu::Invalid,
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
            print::main_menu(Sec::Begin);
        }
    }
    pub mod add_bill {
        use crate::*;

        pub fn bill(bills: &mut Vec<Bills>) {
            loop {
                // Clear Terminal
                clear_terminal::clear_terminal();

                print::bill_menu(Sec::Begin, None);
                print::bill_menu(Sec::Empty, None);
                print::bill_menu(Sec::Instruction, None);
                // print::bill_menu(Sec::Empty, None);

                // Print bills
                print::bills_view(&bills);
                print::bill_menu(Sec::Empty, None);

                print::bill_menu(Sec::First, None);
                let name = user_input::add_bill::input();
                if name.as_str() == "exit" || name.as_str() == "back" {
                    return;
                }
                print::bill_menu(Sec::Second, None);
                let amount = user_input::add_bill::input_float();

                let bill = Bills::new(name, amount);
                bill.add(bills);
            }
        }
    }
    pub mod view_bill {
        use crate::*;

        pub fn view(bills: &mut Vec<Bills>) {
            // Clear Terminal
            clear_terminal::clear_terminal();
            // View
            print::bill_menu(Sec::View, None);
            print::bills_view(&bills);
            print::bill_menu(Sec::Empty, None);
            print::bill_menu(Sec::Total, None);
            let total = Bills::total(&bills);
            println!("{:?}", total);
            print::bill_menu(Sec::Empty, None);

            print::bill_menu(Sec::Instruction, None);

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
}

fn main() {
    // Create vec! for the bills
    let mut bills: Vec<Bills> = vec![];

    loop {
        // Print main menu to screen
        section::main::main();

        // User Input to navigate main menu
        let user_input = user_input::main_menu::input();

        match user_input {
            Menu::AddBills => section::add_bill::bill(&mut bills),
            Menu::ViewBills => section::view_bill::view(&mut bills),
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
        let mut arr = vec![];
        let bill1 = Bills {
            name: "car".to_string(),
            amount: 50.00,
        };
        let bill2 = Bills {
            name: "car".to_string(),
            amount: 20.00,
        };
        let bill3 = Bills {
            name: "car".to_string(),
            amount: 30.00,
        };
        arr.push(bill1);
        arr.push(bill2);
        arr.push(bill3);

        let result = Bills::total(&mut arr);
        let expected = 100.0;
        assert_eq!(result, expected, "Result should be 100.00")
    }

    #[test]
    fn check_total2() {
        let mut arr = vec![];
        let bill1 = Bills {
            name: "car".to_string(),
            amount: 2917.35,
        };
        let bill2 = Bills {
            name: "car".to_string(),
            amount: 11.40,
        };
        let bill3 = Bills {
            name: "car".to_string(),
            amount: 71.25,
        };
        arr.push(bill1);
        arr.push(bill2);
        arr.push(bill3);

        let result = Bills::total(&mut arr);
        let expected = 3000.00;
        assert_eq!(result, expected, "Result should be 3000.00")
    }
}
