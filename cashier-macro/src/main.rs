use std::io;

// macro called cashier
macro_rules! cashier {
    () => { // without arguments
        println!("Now is necessary that you enter a quantity.");
    };
    ($($x: expr),+) => { // 1 to n arguments
        {
            let values: [u32;4] = [50000, 20000, 10000, 2000]; 
            let mut total: u64 = 0;
            let mut _i: usize = 0;
            $( // cycle
                let number: u64 = $x
                    .trim()
                    .parse()
                    .expect("Wanted a number");
                total = total + (number * values[_i] as u64); // logic of operation
                _i += 1; // "_" to omit alert of use
            )+
            println!("Current Total {:?}", &total);
        }
    };
}

fn main() {
    let mut bills50000 = String::new();
    println!("Amount of bills required of $50000 COP and press enter (eg. 4): ");
    io::stdin().read_line(&mut bills50000).expect("Failed to read line");
    cashier!(&mut bills50000);

    let mut bills20000 = String::new();
    println!("Amount of bills required of $20000 COP and press enter (eg. 3): ");
    io::stdin().read_line(&mut bills20000).expect("Failed to read line");
    cashier!(&mut bills50000, &mut bills20000);

    let mut bills10000 = String::new();
    println!("Amount of bills required of $10000 COP and press enter (eg. 3): ");
    io::stdin().read_line(&mut bills10000).expect("Failed to read line");
    cashier!(&mut bills50000, &mut bills20000, &mut bills10000);

    let mut bills2000 = String::new();
    println!("Amount of bills required of $2000 COP and press enter (eg. 3): ");
    io::stdin().read_line(&mut bills2000).expect("Failed to read line");
    cashier!(&mut bills50000, &mut bills20000, &mut bills10000, &mut bills2000);
}