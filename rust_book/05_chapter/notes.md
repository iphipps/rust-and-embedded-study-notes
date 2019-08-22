# Structs
## Defining structs

```
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
### Instantiating field/name structs
If the instance is mutable, all fields are mutable, not just some fields
```
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```
Uses dot notation

allows field shorthand
```
fn build_user(email: String, username: String) -> User {
   User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```


allows spreading

```
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```


### Tuple Structs
```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```
Color and Point are different types even though they look the same.


Note: It is possible for structs to store references to data owned by something else, but that requires _lifetimes_.

e.g. life time annotation
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
but let's save that for later.

## Example program

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}
// is an immutable borrow of a struct Rectangle instance
// we want to borrow rather than take ownership.  main retains ownership.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```


derive debug annotation and print format options
```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!("rect1 is {:?}", rect1);
}
```

>Our area function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closely to our Rectangle struct, because it won’t work with any other type. Let’s look at how we can continue to refactor this code by turning the area function into an area method defined on our Rectangle type.

## Methods

Methods are like functions but methods are different from functions in that they’re defined within the context of a struct (or an enum or a trait object, which we cover in Chapters 6 and 17, respectively), and their first parameter is always self, which represents the instance of the struct the method is being called on.

### Defining Methods

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// implementation block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```
Methods can take ownership of self, borrow self immutably as we’ve done here, or borrow self mutably, just as they can any other parameter.
In the example above, we've choisen `&self` because we don't want to take ownership, it's read only.  But if we want to change it, we'll use `&mut self` as first param.  Having methods take ownership is rare, but could be useful if we want to transform self into something else and prevent the calling from using the original instance.


```
// each of these could be in their own
// impl Rectangle blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // associative function is called by Rectangle::square(arg)
    // it has no references to self
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
```

summary

> Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. Methods let you specify the behavior that instances of your structs have, and associated functions let you namespace functionality that is particular to your struct without having an instance available.

> But structs aren’t the only way you can create custom types: let’s turn to Rust’s enum feature to add another tool to your toolbox.


