pub fn run() {
    //simple print line
    println!("Hello");

    //placeholder
    println!("{}", 1);

    //indexed placeholder
    println!("{0} {0} {0} {1}", "test", "stop");

    //named arguments
    println!("{t} {t} {t} {s}", t = "test", s = "stop");

    //traits
    println!("{:b} {:x}", 10, 10);

    //debug trait
    println!("{:?}", (1, true, "hello world"));
}
