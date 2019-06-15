use std::ops::{AddAssign, DivAssign};

#[derive(Debug)]
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
            panic!("Second value is zero");
        }
        self.value /= other.value;
    }
}

fn main() {
    let mut c1 = Count { value: 1 };
    let c2 = Count { value: 5 };
    let c3 = Count { value: 2};
    let c4 = Count { value: 0 };

    c1 += c2;
    println!("{:?}", &c1);
    c1 /= c3;
    println!("{:?}", &c1);
    c1 /= c4;
    println!("{:?}", c1);
}
