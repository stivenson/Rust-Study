
/// Traits 

// Trait to Visitor Pattern, (similar to an interface)
trait AnimalVisitor {
    fn meow(&mut self, cat: &Cat) -> String;
    fn bark(&mut self, dog: &Dog) -> String;
}

// Trait to all Animals, (similar to an interface)
trait AnimalData {
    fn cat(name: String, color: String) -> Cat {
        Cat {
            name,
            color,
        }
    }   
    fn dog(name: String, color: String, age: i32) -> Dog {
        Dog {
            name,
            color,
            age,
        }
    } 
    fn sound(&self, av: &mut AnimalVisitor) -> String;
    fn color(&self) -> &String;
    fn name(&self) -> &String;
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
    repetitions: i8,
}





/// Implementations (link methods to structs and link traits to structs) 

// Cat
impl Cat {
    fn get_description(&self) ->  String {
        format!(
            "This cat called {:?} and its color is {:?}", 
            &self.name, 
            &self.color
        )
    }
}

impl AnimalData for Cat {
    fn sound(&self, av: &mut AnimalVisitor) -> String {
        av.meow(&self)
    }
    fn color(&self) -> &String {
        &self.color
    }

    fn name(&self) -> &String {
        &self.name
    }
}

// Dog
impl Dog {
    fn get_description(&self) ->  String {
        format!(
            "This dog called {:?}, its color is {:?}, and is {:?} years old", 
            &self.name, 
            &self.color, 
            &self.age
        )
    }
}

impl AnimalData for Dog {
    fn sound(&self, av: &mut AnimalVisitor) -> String {
        av.bark(&self)
    }
    fn color(&self) -> &String {
        &self.color
    }

    fn name(&self) -> &String {
        &self.name
    }
}

// Visitor
impl AnimalVisitor for SoundsAnimals {

    fn meow(&mut self, cat: &Cat) -> String {

        let mut meows = String::from(format!("{} says: ", cat.name));
        for _r in 1..self.repetitions {
            &meows.push_str("moew! ");
        };
        meows.to_string()
    }
    fn bark(&mut self, dog: &Dog) -> String {
        let mut barks = String::from(format!("{} says: ", dog.name));
        for _r in 1..self.repetitions {
            &barks.push_str("wow! ");
        };
        barks.to_string()
    }
}



/// Main method to use all
fn main() {

    /// Explication of visitor pattern: https://es.wikipedia.org/wiki/Visitor_(patr%C3%B3n_de_dise%C3%B1o)

    let mut sounds_animal = SoundsAnimals {repetitions: 8}; // struct of visitor
    // Cat
    let cat = Cat {name: "Misifú".to_string(), color: "gray".to_string()}; // struct of cat
    println!("Cat's description: {:?}", cat.get_description()); // logic in own implementation
    println!("Cat's sound: {:?}", cat.sound(&mut sounds_animal)); // logic in visitor: differents logics in the visitor but the cat chooses which to use.

    // Dog
    let dog = Dog {name: "Siboney".to_string(), color: "white".to_string(), age: 7}; // struct of dog
    println!("Dog's description: {:?}", dog.get_description()); // logic in own implementation
    println!("Dog's sound: {:?}", dog.sound(&mut sounds_animal)); // logic in visitor: differents logics in the visitor but the dog chooses which to use.

}
