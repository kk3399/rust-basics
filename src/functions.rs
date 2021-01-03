pub fn run() {
    println!("{}", greet("mut", "is cool"));
    println!("sum {}", sum(-1, 1));

    //closure
    let i3 = 1;
    let csum = |i1: i32, i2: i32| i1 + i2 + i3;
    println!("csum {}", csum(-1, 1));
}

fn sum(i1: i32, i2: i32) -> i32 {
    return i1 + i2;
}

fn greet(s1: &str, s2: &str) -> String {
    let mut s = String::from(s1);
    s.push_str(" ");
    s.push_str(s2);
    return s;
}
