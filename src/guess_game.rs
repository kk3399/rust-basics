use std::io;
use std::io::Write;

pub fn run() {
    println!("pick a number between 1 and 100 (inclusive)");
    let (mut l, mut r) = (1, 100);
    let mut mid;
    while l < r {
        mid = (l + r) >> 1;
        print!("is your number greater than {}? (enter y/n)", mid);
        io::stdout().flush();
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<String>() {
            Ok(choice) => {
                if choice == "y" {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            Err(..) => println!("cant parse: {}", trimmed),
        };
    }
    println!("your number is {}", l);
}
