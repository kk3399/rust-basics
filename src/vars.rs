//vars are immutable (^_^)

pub fn run() {
    let one = 1;
    // one = 1; //throws error since the variable is immutable
    let mut two = one;
    two = two + 1;
    println!("two = {}", two);

    const TIMEOUT_MILLISECONDS: i32 = 1000;
    println!("TIMEOUT_MILLISECONDS = {}", TIMEOUT_MILLISECONDS);

    let (id, name) = (1, "rust");
    println!("{:?}", (id, name))
}
