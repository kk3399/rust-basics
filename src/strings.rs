/*
    primitive type: str - fixed size & immutable
    mutable type: String - heap allocated and grows
*/
pub fn run() {
    let mut test = String::from("a growable rust string");

    println!("{}", test.is_empty());
    println!(
        "Length of {0} = {1}, Capacity of {0} = {2}",
        test,
        test.len(),
        test.capacity()
    );

    test.push('\u{1F600}');
    println!(
        "Length of {0} = {1}, Capacity of {0} = {2}",
        test,
        test.len(),
        test.capacity()
    );

    test.push_str(" blah blah");
    println!(
        "Length of {0} = {1}, Capacity of {0} = {2}",
        test,
        test.len(),
        test.capacity()
    );

    for word in test.split_whitespace() {
        println!("{}", word);
    }

    let mut test1 = String::with_capacity(1);
    println!(
        "{}; len = {}; capacity = {}",
        test1,
        test1.len(),
        test1.capacity()
    );
    test1.push('a');
    println!(
        "{}; len = {}; capacity = {}",
        test1,
        test1.len(),
        test1.capacity()
    );
    test1.push('a');
    println!(
        "{}; len = {}; capacity = {}",
        test1,
        test1.len(),
        test1.capacity()
    );
}
