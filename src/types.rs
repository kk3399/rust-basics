/*
primitive types =>

unsigned ints - u8, u16, u32, u64, u128
signed ints - i8, i16, i32, i64, i128
float - f32, f64
boolean - bool
character - char
tuples
arrays


*/

fn type_name_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}
pub fn run() {
    let i = 1000 * 1000 * 1000;
    let j = 2.0;
    let is_alive = true;
    println!("{:?}", (i, j, is_alive, '\u{1F600}'));

    type_name_of(&i);
    type_name_of(&j);
    type_name_of(&is_alive);
    type_name_of("test");
}
