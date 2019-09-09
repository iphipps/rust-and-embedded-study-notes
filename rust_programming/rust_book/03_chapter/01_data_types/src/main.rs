fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let five_hundre = tup.0;
    println!("Indexes can be used as well, {}", five_hundre);

    let an_array: [i32, 5] = [1, 2, 3, 4, 5];
    // you don't need to set type and length, can be inferred
    // but must match type 
    
    // let a = [3; 5];
    // is same as
    // let a = [3, 3, 3, 3, 3];
}
