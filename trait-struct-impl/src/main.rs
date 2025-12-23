struct Man {
    name: &'static str,
    cellphone: i64,
    is_alive: bool,
}

trait Person {
    fn new(name: &'static str, cellphone: i64) -> Self;
    fn name(&self) -> &'static str;
    fn is_alive(&self) -> bool;
    fn cellphone(&self) -> i64;

    // Associated function with default implementation
    fn get_current_age(days_of_live: i64) -> i64 {
        days_of_live / 365
    }
}

impl Man {
    fn get_full_name_pet(&self, name: &str, last_name: &str) -> String {
        format!("{} {}", name, last_name)
    }

    fn get_data_city(&self) -> &'static str {
        "Cúcuta"
    }
}

impl Person for Man {
    fn new(name: &'static str, cellphone: i64) -> Self {
        Man {
            name,
            cellphone,
            is_alive: true,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn cellphone(&self) -> i64 {
        self.cellphone
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }
}

fn main() {
    println!("=== Traits, Structs, and Implementations ===\n");

    let cellphone = 3_102_531_280i64;
    let days_alive = 10_585i64;
    let man = Man::new("Stivenson", cellphone);
    let last_name_pet = "Rodriguez";

    println!("Name: {}", man.name());
    println!("Cellphone: {}", man.cellphone());
    println!("City: {}", man.get_data_city());
    println!("Current age: {} years", Man::get_current_age(days_alive));
    println!(
        "Full name of pet: {}",
        man.get_full_name_pet("Sibonei", last_name_pet)
    );
    println!("Is alive: {}", man.is_alive());
}
