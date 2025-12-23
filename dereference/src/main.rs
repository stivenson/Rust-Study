use std::ops::{Deref, DerefMut};

//////////////////////// Basic Example ////////////////////////
fn change_phrase_value(phrase: &mut String) {
    let other_phrase = "and born in Cúcuta city";

    // Following the pointer (dereferencing) to modify the value
    *phrase = format!("{} {}", phrase, other_phrase);
}

fn with_basic_use_dereference() {
    let mut phrase = String::from("James Rodriguez is a colombian player");
    change_phrase_value(&mut phrase);
    println!("{}", phrase);
    assert_eq!(
        phrase,
        "James Rodriguez is a colombian player and born in Cúcuta city"
    );
}



/////////////////// With Deref Trait //////////////////////////////

struct DeRefJames<T> {
    phrase: T,
}

impl<T> Deref for DeRefJames<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.phrase
    }
}

fn with_deref_trait_use() {
    let mut james_deref = DeRefJames {
        phrase: "James Rodriguez is a colombian player".to_string(),
    };
    let other_phrase = "and born in Cúcuta city";

    // Using Deref to access the inner value
    james_deref.phrase = format!("{} {}", *james_deref, other_phrase);

    println!("{}", *james_deref);
    assert_eq!(
        "James Rodriguez is a colombian player and born in Cúcuta city",
        &*james_deref
    );
}




/////////////////// With DerefMut Trait //////////////////////////////

impl<T> DerefMut for DeRefJames<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.phrase
    }
}

fn with_derefmut_trait_use() {
    let mut james_deref = DeRefJames {
        phrase: "James Rodriguez is a colombian player".to_string(),
    };
    let other_phrase = "and born in Cúcuta city";

    // Using DerefMut to modify the inner value directly
    *james_deref = format!("{} {}", *james_deref, other_phrase);

    println!("{}", *james_deref);
    assert_eq!(
        "James Rodriguez is a colombian player and born in Cúcuta city",
        &*james_deref
    );
}

fn main() {
    println!("=== Dereference Examples ===\n");

    // Basic Example
    println!("1. Basic dereference:");
    with_basic_use_dereference();

    // With Deref Trait
    println!("\n2. Deref trait:");
    with_deref_trait_use();

    // With DerefMut Trait
    println!("\n3. DerefMut trait:");
    with_derefmut_trait_use();
}
