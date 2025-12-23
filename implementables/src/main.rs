use std::ops::{AddAssign, DivAssign};

#[derive(Debug, Clone, Copy)]
struct Count {
    value: i32,
}

impl AddAssign for Count {
    fn add_assign(&mut self, other: Count) {
        self.value += other.value;
    }
}

impl DivAssign for Count {
    fn div_assign(&mut self, other: Count) {
        if other.value == 0 {
            panic!("Cannot divide by zero");
        }
        self.value /= other.value;
    }
}

fn main() {
    println!("=== Implementing Operator Traits ===\n");

    let mut c1 = Count { value: 1 };
    let c2 = Count { value: 5 };
    let c3 = Count { value: 2 };
    let c4 = Count { value: 0 };

    println!("Initial value: {:?}", c1);

    c1 += c2;
    println!("After adding 5: {:?}", c1);

    c1 /= c3;
    println!("After dividing by 2: {:?}", c1);

    // This will panic - uncomment to test error handling
    // c1 /= c4;
    println!("Would panic if divided by: {:?}", c4);
}
