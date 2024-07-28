const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    let _unused = 42; // but it's fine because it's prefixed with an underscore

    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);
    // READY_AMOUNT = 1; // E0070: cannot assign to this expression
}
