pub fn run() {
    let mut nums: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", nums);
    println!("{:?}", nums[nums.len() - 1]);
    nums[nums.len() - 1] = 4;
    println!("{:?}", nums[nums.len() - 1]);

    //size in bytes
    println!("size of nums = {}", std::mem::size_of_val(&nums));

    let slice: &[i32] = &nums[0..3];
    println!("slice =  {:?}", slice);

    let reslice: &[i32] = &slice[1..2];
    println!("reslice = {:?}", reslice);
}
