## 00 Variables

make vars mutable if you want to change them

`let mut x = 5;`
`let y = 6;`
`const MAX_POINTS: u32 = 100_000;`

const need type defs

check the (file)[./00_variables/src/main.rs] for better descriptions 

## 01 Data Types

static typed

### Scalars

integers, floating point numbers, booleans, characters

#### Integer types

Signed or unsigned and bit
u8, i8
8,  16, 32, 64, 128, arch
arch becomes isize usize

> Signed and unsigned refer to whether it’s possible for the number to be negative or positive—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using two’s complement representation.

#### Floating point types

numbers with decimal types

f64 is default
 f32 = single precision
 f64 = double precision

 mathematics
 ```
 fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

#### Booleans

true false
```
let t = true;
let f: bool = false;
```

#### Character Type

> Note that char literals are specified with single quotes, as opposed to string literals, which use double quotes.

```
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

char is 4 bytes and unicode scalar value so it can be more than ASCII

#### Compound types
groups

##### Tuple

Can define type, store together and destructure (ref)[./01_data_types/src/main.rs#7]

Can also reference by index. (ref)[./01_data_types/src/main.rs#10]

##### Array

must match type
stored on stack not heap

## 03 Functions

Can declare arg type and return type.

> You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

## 04 Comments

Comments use // and /* */ C style
There are also documentation  comments but that's later

## Control Flow

if else, else if
```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

One note that if else is different than JavaScript.  Conditions must be boolean and not truthy nor falsey.

Also conditions can be in assignments

```
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```
but types need to match


#### loop while and for

loop and if breaks are so common that rust has while
```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```
vs
```
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

You can also loop through arrays

```
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```
