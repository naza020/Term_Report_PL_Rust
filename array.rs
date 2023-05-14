fn main(){
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;
    for x in array {
        print!("{x} ");
    }
    println!("Array of array = {:?}", array);

    //Vec
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Array of vec = {:?}", vec);
    vec.extend([1, 2, 3]);
    for x in &vec {
        println!("{x}");
    }
}