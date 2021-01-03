pub fn run() {
    let mut count = 0;
    loop {
        count += 1;
        println!("{}", count);
        if count == 10 {
            break;
        }
    }

    count = 0;
    while count != 10 {
        count += 1;
        println!("{}", count);
    }
}
