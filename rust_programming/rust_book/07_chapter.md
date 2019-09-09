# Managing growing projects

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

## Packages and Crates

_crate root_ -- source file that rust compiler starts and makes up the root module of your crate.
_package_ -- one of more crates that provide functionality (contains a _Cargo.toml_ file).

Cargo follows convention that src/main.rs is root of binary with same name as package.

Cargo knows a package directory contains src/lib.rs.  Cargo passes crate root files to `rustc` to build library or binary.

If a package has _src/main.rs_ and _src/lib.rs_ it has two crates: a library and a binary, both with the same name as the package.  A package can have many binary crates by placing files in the _src/bin_ dir with each file as a separate binary crate.

> Keeping a crate’s functionality in its own scope clarifies whether particular functionality is defined in our crate or the rand crate and prevents potential conflicts. For example, the rand crate provides a trait named Rng. We can also define a struct named Rng in our own crate. Because a crate’s functionality is namespaced in its own scope, when we add rand as a dependency, the compiler isn’t confused about what the name Rng refers to. In our crate, it refers to the struct Rng that we defined. We would access the Rng trait from the rand crate as rand::Rng.

## Defining MOdules to Control Scope and Privacy
defining modules
```
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
module tree
```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### Pathing

A path can take two forms:

- An absolute path starts from a crate root by using a crate name or a literal crate.
- A relative path starts from the current module and uses self, super, or an identifier in the current module.
Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

```
// ! this code doesn't compile
// because the modules are private
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
need to make mods public to be able to reference them by using ::
```
// compiles
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```

strucs and enums can also be public

use super for relative paths 

```
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```

## Bring paths into scope with `use`

```
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
> Bringing the function’s parent module into scope with use so we have to specify the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path. instead of 
e.g.
```
use parent

pub fn foo() {
  parent::child();
}
```
vs
```
use parent::child

pub fn foo() {
  // looks locally defined
  child();
}
```

### Using `as`

```
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### Reexporting

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
This allows external code to use hosting::add_to_waitlist

### Using nested paths to clean up `use` lists

```
use std::cmp::Ordering;
use std::io;
```
can be used as
```
use std::{cmp::Ordering, io};
```

### Glob
brings all public items defined in a path into scope.
```
use std::collections::*;
```
## Separating Modules into different files


:: can follow file directories with modules and files of the same name.























