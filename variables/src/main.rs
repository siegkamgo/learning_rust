// Variables

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// Constants (Always immutable)

// const HOURS_IN_A_YEAR: u32 = 365 * 24;
// fn main() {
//     println!("{HOURS_IN_A_YEAR}");
    
// }


// Shadowing

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
