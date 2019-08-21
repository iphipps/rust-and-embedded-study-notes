# Understanding Ownership

Rust is memory safe without a gardbage collector.
`borrowing, slices and data in memory`

## What is ownership?

It refers to what memory management system is being used at different times in the program.

### Stack vs heap
Stack -- lifo - like a stack of plates
Stack must have a known, fixed size.

Heap -- -- like a restaurant seating chart.
Anything that has variable size must be stored here.  Heap is less organized. 
OS finds space that is big enough, marks it as being used and returns a pointer. (_allocating_ on the heap) 


Heap is slower, but can be faster if locations are close to each other; the processor doesn't need to jump around as much.

For functions, arguments, including pointers to heap locations AND local variable are pushed to the stack.  After the function runs, stack vars are popped.

### Ownership rules

* values have an _owner_ valiable
* there can only be one owner
* value is dropped when owner goes out of scope

### Variable scope

```
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
} 
```

* When s comes into scope, it is valid.
* It remains valid until it goes out of scope.

### Memory and Allocation and The `String` Type 

string literals are immutable and their value is known at compile time
string is stored on the heap and can be mutated
```
// ::from requests memore it needs.
let mut s = String::from("hello");

// traditionally GC will remove the old s heap memory.  
// In rust though, an allocate must be 1-1 with a free
s.push_str(", world!"); // push_str() appends a literal to a String

println!("{}", s); // This will print 'hello, world!'
// enter drop
```


>Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.


```
let s1 = String::from("hello");
let s2 = s1;
```
In the above example

pointer to table
s1
||name|value||
|prt|table|
|length|5|
|capacity|5|

table in heap
||index|value||
|0|h|
|1|e|
|2|l|
|3|l|
|4|o|

when s2=s1

s2 is a new pointer to the same table in the heap.

when that function goes out of scope, a _double free_ error occurs.
you cannot free memory twice.

to avoid this rust moves s1 into s2. that is, s1 is invalidated.

###  Variables and Date Interactions

#### Clone

For deep copying of heap data.
```
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
// works as expected
```

#### Stack-Onlt Data: Copy
```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```
They have none size, so there's no different between deep and shallow copying.

Here are some of the types that are Copy:

* All the integer types, such as u32.
* The Boolean type, bool, with values true and false.
* All the floating point types, such as f64.
* The character type, char.
* Tuples, if they only contain types that are also Copy. For example, (i32, i32) is Copy, but (i32, String) is not.

### Function Ownership

```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

returning passes ownership

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```
tuples can pass

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```











