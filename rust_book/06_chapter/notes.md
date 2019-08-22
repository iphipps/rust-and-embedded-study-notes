# Enums and Pattern Matching

Rust's enums are most similar to _algebraic data types_ in functional languages, like F#, OCaml and Haskell

## Defining enums

### Ip address example
IP address can be v4 or v6 address but not both at the same time.

```
enum IpAddrKind {
  V4,
  V6,
}
```
V4 and V6 are _variants_ of the enum.

```
// both are same type IpAddrKing also takes double colon
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;


enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};

fn route(ip_kind: IpAddrKing){}
```

more descriptive types
```
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
```

the stand libary already has IpAddr, because this is done so frequently.

### New example

methods are created the same way.
```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();

```

### The `Option` Enum

Advantages over null values.  Rust has no null feature.  Not because null isn't useful but because
language implementatinos of null are error prone. Assuming a variable is not null is dangerous and commonplace

The `Option<T>` is defined by the standard library:

```
enum Option<T> {
  Some(T),
  None,
}
```

Use it like this:
```
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

Generally, when using Option you want to handle each variant.

That's where match comes in

## Match

`match` is a powerful control flow operator.

the compiler confirms that all possible cases are handled.

analogy: match is like a coin sorting machine.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // these are referrred to as arms.  a pattern and some code
        // separates by =>
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Patterns bound to values

```
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

```

### Matching with `Option<T>`
```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```


> Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.

### Exhaustive

The compiler will check for holes.  If one missing, the compiling will complain. e.g. option must have a None condition

```
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    // _ matches everything else
    _ => (),
}
```

## Concise Control Flow with if let

for when an more simple than match

Instead of 
```
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}
````
consider this
```
if let Some(3) = some_u8_value {
    println!("three");
}
```
