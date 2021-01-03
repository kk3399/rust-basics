pub fn run() {
    //primitive valeus are shared by default (probably memory is shared internally?)
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("{:?}", (arr1, arr2));

    //for non primitive values once a variable is assigned to a new variable, the old one can not be used
    // to share data, need to use reference instead of assigning to a new variable to create a copy
    // explicit copy is needed if sharing isnt what is desired
    let vec1 = vec![1, 2, 3];
    let mut vec2 = vec1;
    println!("{:?}", (vec2));
    // println!("{:?}", (vec1)); // this line throws an error "value borrowed here after move"

    let vec3 = &vec2;
    // vec2[0] = 9; // not allowing chages here
    println!("{:?}", (&vec2, vec3));
}
