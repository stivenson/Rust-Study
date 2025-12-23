/// Traits

// Visitor pattern trait (similar to an interface)
trait AnimalVisitor {
    fn meow(&mut self, cat: &Cat) -> String;
    fn bark(&mut self, dog: &Dog) -> String;
}

// Animal trait with factory methods and common behavior
trait AnimalData {
    fn cat(name: String, color: String) -> Cat {
        Cat { name, color }
    }

    fn dog(name: String, color: String, age: i32) -> Dog {
        Dog { name, color, age }
    }

    fn sound(&self, av: &mut dyn AnimalVisitor) -> String;
    fn color(&self) -> &str;
    fn name(&self) -> &str;
}






/// Structs (Similar to entities or resources)

struct Dog {
    name: String,
    color: String,
    age: i32,
}

struct Cat {
    name: String,
    color: String,
}

struct SoundsAnimals {
    repetitions: u8,
}





/// Implementations (link methods to structs and traits to structs)

// Cat
impl Cat {
    fn get_description(&self) -> String {
        format!(
            "This cat is called {} and its color is {}",
            self.name, self.color
        )
    }
}

impl AnimalData for Cat {
    fn sound(&self, av: &mut dyn AnimalVisitor) -> String {
        av.meow(self)
    }

    fn color(&self) -> &str {
        &self.color
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Dog
impl Dog {
    fn get_description(&self) -> String {
        format!(
            "This dog is called {}, its color is {}, and is {} years old",
            self.name, self.color, self.age
        )
    }
}

impl AnimalData for Dog {
    fn sound(&self, av: &mut dyn AnimalVisitor) -> String {
        av.bark(self)
    }

    fn color(&self) -> &str {
        &self.color
    }

    fn name(&self) -> &str {
        &self.name
    }
}

// Visitor implementation
impl AnimalVisitor for SoundsAnimals {
    fn meow(&mut self, cat: &Cat) -> String {
        let mut meows = format!("{} says: ", cat.name);
        for _ in 1..self.repetitions {
            meows.push_str("meow! ");
        }
        meows
    }

    fn bark(&mut self, dog: &Dog) -> String {
        let mut barks = format!("{} says: ", dog.name);
        for _ in 1..self.repetitions {
            barks.push_str("woof! ");
        }
        barks
    }
}

/// Main method demonstrating the visitor pattern
fn main() {
    println!("=== Visitor Pattern Example ===\n");
    // More info: https://en.wikipedia.org/wiki/Visitor_pattern

    let mut sounds_animal = SoundsAnimals { repetitions: 8 };

    // Cat
    let cat = Cat {
        name: "Misifú".to_string(),
        color: "gray".to_string(),
    };
    println!("Cat's description: {}", cat.get_description());
    println!("Cat's sound: {}\n", cat.sound(&mut sounds_animal));

    // Dog
    let dog = Dog {
        name: "Siboney".to_string(),
        color: "white".to_string(),
        age: 7,
    };
    println!("Dog's description: {}", dog.get_description());
    println!("Dog's sound: {}", dog.sound(&mut sounds_animal));
}
