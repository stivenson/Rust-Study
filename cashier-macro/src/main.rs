use std::io;

const VALUES_COP: &[i32; 4] = &[50_000, 20_000, 10_000, 2_000];

// Declarative macro for calculating cashier totals
macro_rules! cashier {
    () => {
        println!("Please enter a quantity.");
    };
    ($($x: expr),+) => {{
        let mut total: u64 = 0;
        let mut index: usize = 0;
        $(
            let number: u64 = $x
                .trim()
                .parse()
                .expect("Expected a valid number");
            total += number * VALUES_COP[index] as u64;
            index += 1;
        )+
        println!("Current Total: ${}", total);
        total
    }};
}

fn main() {
    println!("=== Colombian Peso Cashier Calculator ===\n");

    let mut bills50000 = String::new();
    println!("Amount of $50,000 COP bills (e.g. 4): ");
    io::stdin()
        .read_line(&mut bills50000)
        .expect("Failed to read line");
    cashier!(&bills50000);

    let mut bills20000 = String::new();
    println!("\nAmount of $20,000 COP bills (e.g. 3): ");
    io::stdin()
        .read_line(&mut bills20000)
        .expect("Failed to read line");
    cashier!(&bills50000, &bills20000);

    let mut bills10000 = String::new();
    println!("\nAmount of $10,000 COP bills (e.g. 3): ");
    io::stdin()
        .read_line(&mut bills10000)
        .expect("Failed to read line");
    cashier!(&bills50000, &bills20000, &bills10000);

    let mut bills2000 = String::new();
    println!("\nAmount of $2,000 COP bills (e.g. 3): ");
    io::stdin()
        .read_line(&mut bills2000)
        .expect("Failed to read line");
    cashier!(&bills50000, &bills20000, &bills10000, &bills2000);
}