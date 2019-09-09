# Error Handling

_recoverable_ -- reasonable to report the problem and retry -- like looking for a file not found.
_unrecoverable_ -- symptoms of bugs, like trying to access a location beyond the end of array.

Most languages don't make this distinction, and they'll handle the same way, but using exceptions.

Rust has Result<T, E> for recoverable errors and `panic!` macro that stops execution when it encounters the error.

## `panic!`

When panic occurs, the program unwinds the stack and cleans up the data. 
But this is a lot of work. Alternatively, you could abort the program without cleaning up.
Then the os will clean it up.

You can set these in Cargo.toml
```
[profile.release]
panic = 'abort'
```

You can set the code to panic.  Or it will panic when it does something, like accessing an index on a vec that doesn't exist.
You can run `RUST_BACKTRACE=1 cargo run` for a more verbose output.

## Recovering Errors with `Result`
Some methods return a result.  The compiler will warn us about that.
File::open returns a Result that you need to handle.

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}
```
#### Matching on different errors
```
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```


### Shortcuts for Panic on Error: `unwrap` and `expect`
Match can be cumbersome and verbose.  Result<T, E> has helper methods.

#### Unwrap
If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
If the Result is the Err variant, unwrap will call the panic! macro for us.
```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

#### Expect 
Works like unwrap but allows a messaging argument

```
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

### Recovering errors

```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

#### Shortcut for propagating errors

```
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
with the ? the Error gets propagated to the calling code.









## When to Panic
### Examples, Prototypes and Tests
When a big matcher on a Result distracts from what is going on.

### When you know more than the compiler

```
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

If we know the ip address will never change, but the compiler does not
like user input

### Guidelines for Error Handling
Panic when your code could end up in a bad state.  Invalid values, contradictory values or missing values, etc.
- The bad state is not something that’s expected to happen occasionally.
- Your code after this point needs to rely on not being in this bad state.
- There’s not a good way to encode this information in the types you use.

panic is also good if you're using a crate and have no control over the library code
