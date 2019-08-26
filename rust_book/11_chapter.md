# Automated Tests

## Writing Tests

Test functions do the following:
1. Set up needed data/state.
2. Runs code you want to test.
3. Assert results are as exxpected.

`test`, `should_panic` and macros.

### Anatomy of test

`cargo test` will run the tests

```
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

(check the file)[./adder/src/lib.rs]

## Controlling how tests are run

### Running Tests in parallel or consecutively

`$ cargo test -- --test-threads=1` runs consecutly
default is in parellel

### Showing function output

`cargo test -- --nocapture` will allow you to see output captured in run functions
like println! statemtents

### Running subset of tests by name

`cargo test add` will run all tests that match add.  e.g. add_two, add_three etc.
No test name can be duplicated so if you want to run 1 test only pass the full name of the test
or a name close enough to not match other tests

### Ignoring test
```
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

### Running ignored tests
`cargo test -- -- ignored`


## Test organization

### Unit tests

convention is to create a module named `tests` in each file and annotate with `cgf(test)`

### The Tests Module and #[cfg(test)]

Tells rust to compile and run test on `cargo test` not on `cargo build`

### Private functions can be tested in the same file

```
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

### Integration Tests

Integration tests are external.

#### The _tests_ dir

You can add integration tests to their own test dir and run `cargo test`
