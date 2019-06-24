use std::ops::{Deref, DerefMut};


//////////////////////// Basic Example ////////////////////////
fn change_phrase_value (phrase: &mut String) {
    let other_phrase = "and born in Cúcuta city".to_string();

    // Following the pointer (with dereferencing) to find the value and change it.
    *phrase = format!("{} {}", &phrase, other_phrase); 
}


fn with_basic_use_dereference () {
    let mut phrase = String::from("James Rodriguez is a colombian player");
    change_phrase_value(&mut phrase);
    println!("{:?}", &phrase);
    assert_eq!(phrase, String::from("James Rodriguez is a colombian player and born in Cúcuta city")); // Test
}



/////////////////// With Deref Trait //////////////////////////////

struct DeRefJames<T> {
    phrase: T
}

impl<T> Deref for DeRefJames<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.phrase
    }
}

fn with_deref_trait_use () {
    let mut james_deref = DeRefJames { phrase: "James Rodriguez is a colombian player".to_string() };
    let other_phrase = "and born in Cúcuta city".to_string();

    // Using the struct with Deref implementation, to find the value and change it.
    james_deref.phrase = format!("{} {}", *james_deref, other_phrase);

    println!("{:?}", *james_deref);
    assert_eq!("James Rodriguez is a colombian player and born in Cúcuta city", *james_deref);
}




/////////////////// With DerefMut Trait //////////////////////////////

impl<T> DerefMut for DeRefJames<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.phrase
    }
}

fn with_derefmut_trait_use () {
    let mut james_deref = DeRefJames { phrase: "James Rodriguez is a colombian player".to_string() };
    let other_phrase = "and born in Cúcuta city".to_string();

    // Using the struct with Deref implementation, to find the value and change it.
    *james_deref = format!("{} {}", *james_deref, other_phrase);

    println!("{:?}", *james_deref);
    assert_eq!("James Rodriguez is a colombian player and born in Cúcuta city", *james_deref);
}





fn main() {

    // Basic Example
    with_basic_use_dereference();


    // With Deref Trait
    with_deref_trait_use();


    // with Deref Trait
    with_derefmut_trait_use();

}
