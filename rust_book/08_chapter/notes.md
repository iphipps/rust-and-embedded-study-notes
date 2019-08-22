# Common Collections

Rust's standard library data structures.

- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

## Vectors

`let v: Vec<i32> = Vec::new();`

can only store values of the same type.
Rust can often infer type. Use the `vec!` macro.
`let v = vec![1, 2, 3];`

### Updating Vectors

```
let mut v - Vec::new();
v.push(5);
v.push(8);
```

### Dropping a Vector drops its elements

like all other structs. once out of scope, it is dropped

### Reading elements of Vectors

use `get`

```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

or use brackets
`let elem = &v[0]`

for if they don't exist .get will pass None and not panic, but if you use [] you'll crash the program.

```
// this fails because mutable and immutable references in the same scope
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];
// here you're trying to push on a vector that has an immudable reference first
v.push(6);

println!("The first element is: {}", first);
```

### Iterating

```
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

read the value

```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

- is the dereference operator; required for changing the value

### Enum for storing multiple Types

The variants of an enum are defined under the same enum type, so we can store elements of different types in a vector.

```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Rust just needs to know what types will be in the vector at compile time.
This therefore needs to be explicitly set and not just inferred during
instantiation.

Check the api on Vec. There is .push .pop etc.

## Strings

Core language string type is the string slice `str` or the borrowed form `&str`. String literals are in the programs binary and are string slices.

The String type is part of the standard library. It is growable, mutable, owned, UTF-8 encoded.

There are other string types in standard library: `OsString, OsStr, CString, CStr`.

### Creating strings

`let mut s = String::new();`

```

let data = "initial contents";

// this is a display trait
let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

formatting

```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
At this point, s will be tic-tac-toe. With all of the + and " characters, it’s difficult to see what’s going on. For more complicated string combining, we can use the format! macro:
```

vs

```
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

#### Rust does not provide indices on strings

why?
String is a wrapper over Vec<u8>
`let len = String::from("Здравствуйте").len();`
each unicode in scalar value is 2 bytes, so the above code len == 24
that is, the index into a strings' bytes will not always correlate to a valid unicode scalar value.

#### Bytes and scalar values and grapheme clusters

> Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

Rust allows differrent ways of interpreting raw string data.

Lastly, string get character options are expected to always take a constant time but that isn't possible to guarantee
given the diffent interpretations of string data.

### Slicing strings

Be careful when creating slices using indices, because of the byte inconsistencies above.

### Iterating over strings

for c in "नमस्ते".chars() {
println!("{}", c);
}

or

for b in "नमस्ते".bytes() {
println!("{}", b);
}

### String summary

Strings are complicated. Most other languages present less complexity with Strings.

In Rust, programmers need to be more deliberate about how they handle UTF-8 data.

## Hash Maps

HashMap<K, V> stores keys of type K to values of type V via a _hashing function_ which deals with how these keys and values are
stored in memory.

Other languages handle this, but often call it, dictionary, associative array, hash, map, object, hash table, etc.

### Create a new hash map

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

Like vectors, hash map memory is stored on heap.

Like vectors, hash maps are homogeneous: all of the keys must have the same type, and all of the values must have the same type.

### `Collect` on tuples

```
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

### Hash Maps and Ownership

For types with `Copy` like i32, values are copied into the hashmap. For owned values like `String`, values are moved
and the hash map becomes the owner of those values.

```
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

### Accessing values

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

### Iterating over hash maps

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Updating hash maps
You can overwrite the old value with new, keep the old value disregard new, only add new if key doesnt have a value or combine in some way.

#### Overwriting

```
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

#### Insert if key has no value

```
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
// prints {"Yellow": 50, "Blue": 10}
```

#### Update based on old value

```
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
// {"world": 2, "hello": 1, "wonderful": 1}
```

### Hashing algorithmn

The hashing algorithmn that rust uses is not the fastest
That decision prioritized security/safety (DDOS resiliance) over performance


