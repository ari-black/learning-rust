
// Declare a constant named `STARTING_MISSILES` and set it to `8` (the type is `i32`)
const STARTING_MISSILES: i32 = 8;

// Declare a constant named `READY_AMOUNT` and set it to `2` (also `i32`).
const READY_AMOUNT: i32 = 2;


fn main() {

    // Declare the variable `missiles` and initialize it to `8`
    // let mut missiles: i32 = STARTING_MISSILES;
    // let missiles: i32 = STARTING_MISSILES;

    // Declare the variable `ready` and initialize it to `2`
    // let ready: i32 = READY_AMOUNT;

    // Try binding the variables all at once on one line using a pattern 
    // (parentheses and commas) -- can you figure out where `mut` goes?
    // let (mut missiles, ready) : (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    let (missiles, ready) : (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);

    // Add another variable to your program *but don't use it*.
    // let new_variable = 7;

    // Try modifying a constant in `main()` (for example, `READY_AMOUNT = 1`). 
    // What does the error look like?
    // READY_AMOUNT = 4;

    // Change the `println!(...)` at the end of `main()` to:
    // `println!("Firing {} of my {} missiles...", ready, missiles);`
    println!("Firing {} of my {} missiles...", ready, missiles);

    // After the `println!(...)`, subtract `ready` from `missiles`
    // missiles = missiles - ready;

    // Add a second `println!(...)` to the end.
    // Instead of changing missiles, use the value 
    // `missiles - ready` directly in the second `println!(...)`
    println!("{} missiles left", missiles - ready);
}
