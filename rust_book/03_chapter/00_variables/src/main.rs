fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of mutated variable x is: {}", x);
    // prints 6

    let shadowable = 5;
    let shadowable = shadowable + 1;
    let shadowable = shadowable + 2;

    println!("The value of shadowable is {}", shadowable);
    // prints 8

    let mut spaces = "     ";
    // spaces = spaces.len();
    // above commented code returns error because we mutated type.

    println!("{}", spaces.len());
    // println needed to remove the unused of inital var
    spaces = "hello";
    // reassign to remove the doesnt need to be mutable warning
    println!("{}", spaces);
    // println needed to remove the unused
    // warning of the new variable value
}
