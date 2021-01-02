pub fn run() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4];
    println!(
        "{:?}; len = {}; capacity = {}",
        nums,
        nums.len(),
        nums.capacity()
    );
    println!("{:?}", nums[nums.len() - 1]);
    nums[3] = 4;
    println!("{:?}", nums[nums.len() - 1]);
    nums.push(5);
    println!(
        "{:?}; len = {}; capacity = {}",
        nums,
        nums.len(),
        nums.capacity()
    );

    //size in bytes
    println!("size of nums = {}", std::mem::size_of_val(&nums));

    let slice: &[i32] = &nums[0..3];
    println!("slice =  {:?}", slice);

    let reslice: &[i32] = &slice[1..2];
    println!("reslice = {:?}", reslice);
}
