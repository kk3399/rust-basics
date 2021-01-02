/*
a tuple can have atmost 12 elements
*/

pub fn run() {
    let student: (i32, &str, bool) = (1, "ruster", false);
    println!("{:?}", student);
}
