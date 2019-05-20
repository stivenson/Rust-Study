struct Man {
	name: &'static str,
	cellphone: i64,
	is_alive: bool,
}

trait Person {
	fn new(name: &'static str, cellphone: &i64) -> Self;
	fn name(&self) -> &'static str;
	fn is_alive(&self) -> &bool;
	fn cellphone(&self) -> &i64;

	fn get_current_age(days_of_live: &i64) -> i64 {
		return days_of_live / 365;
	}
}

impl Man {
	fn get_full_name_pet(&self, name: &'static str, last_name: &String) -> String {
		return name.to_string() + " " + &last_name.to_string();
	}

	fn get_data_city(&self) -> String {
		return String::from("Cúcuta");
	}
}

impl Person for Man {
	fn new(name: &'static str, cellphone: &i64) -> Man {
		Man {
			name: name,
			cellphone: *cellphone,
			is_alive: true,
		}
	}
	fn name(&self) -> &'static str {
		return self.name;
	}
	fn cellphone(&self) -> &i64 {
		return &self.cellphone;
	}
	fn is_alive(&self) -> &bool {
		return &self.is_alive;
	}
}

fn main() {
	let cellphone: i64 = 3102531280;
	let days_alive: i64 = 10585;
	let man: Man = Person::new("Stivenson", &cellphone);
	let last_name_pet = "Rodriguez".to_string();

	println!("Name: {:?}", man.name()); // method
	println!("Cellphone: {:?}", man.cellphone()); // method
	println!("City: {:?}", man.get_data_city()); // method
	println!("Current age: {:?}", Man::get_current_age(&days_alive)); // associated function
	println!(
		"Full name of pet: {:?}",
		man.get_full_name_pet("Sibonei", &last_name_pet)
	); // method
}
