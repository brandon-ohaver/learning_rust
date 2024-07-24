# <ins>Chapter 7 Section 4: Bringing Paths into Scope with the `use` Keyword</ins>

#### Having to write out the paths to call functions can feel inconvenient and repetitive. In previous examples, whether we chose the absolute or relative path to the `add_to_waitlist` function, everytime we wanted to call `add_to_waitlist` we had to specyify `front_of_house` and `hosting` too. Fortunately, there's a way to simplify this process: we can create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope.

#### In the following example, we bring the `crate:front_of_house::hosting` module into the scope of the `eat_at_restaurant` function so we only have to specify `hosting::add_to_waitlist` to call the `add_to_waitlist` function in `eat_at_restaurant`:

```
mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	hosting::add_to_waitlist();
}
```

#### The compiler error shows that the shortcut no longer applies within the `customer` module:

```
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
  --> src/lib.rs:11:9
   |
11 |         hosting::add_to_waitlist();
   |         ^^^^^^^ use of undeclared crate or module `hosting`
   |
help: consider importing this module through its public re-export
   |
10 +     use crate::hosting;
   |

warning: unused import: `crate::front_of_house::hosting`
 --> src/lib.rs:7:5
  |
7 | use crate::front_of_house::hosting;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

For more information about this error, try `rustc --explain E0433`.
warning: `restaurant` (lib) generated 1 warning
error: could not compile `restaurant` (lib) due to 1 previous error; 1 warning emitted
```

#### Notice there's also a warning that the `use` is no longer used in its scope! To fix this problem, move the `use` within the `customer` module too, or reference the shortcut in the parent module with `super::hosting` within the child `customer` module.

## <ins>Creating Idiomatic `use` Paths</ins>

#### In the previous example, you might have wondered why we specified `use crate::front_of_house::hosting` and then called `hosting::add_to_waitlist` in `eat_at_restaurant` rather than specifying the `use` path all the way out to the `add_to_waitlist` function to achieve the same result, as shown in the following example:

```
mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
	add_to_waitlist();
}
```

#### Although both examples accomplish the same task, the first example is the idiomatic way to bring a function into scope with `use`. Bringing the function's parent module into scope with `use` means we have to specify the parent module when calling the function. Specifying the parent module when calling the function makes it clear that the function isn't locally defined while still minimizing repetition of the full path. The code in the most recent example is unclear as to where `add_to_waitlist` is defined.

#### On the other hand, when bringing in structs, enums, and other items with `use`, it's idiomatic to specify the full path. The following example shows the idiomatic way to bring the standard library's `HashMap` struct into the scope of a binary crate.

```
use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	map.insert(1, 2);
}
```

#### There's no strong reason behind this idiom: it's just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.

#### The exception to this idiom is if we're bringing two items with the same name into scope with `use` statements, because Rust doesn't allow that. The next example shows how to bring two `Result` types into scope that have the same name but different parent modules and how to refer to them.

```
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
	// --snip--
}

fn function2() -> io::Result<()> {
	// --snip--
}
```

#### As you can see, using the parent modules distinguishes the two `Result` types. If instead we specified `use std::fmt::Result` and `use std::io::Result`, we'd have two `Result` types in the same scope and Rust wouldn't know which one we meant when we used `Result`.

## <ins>Providing New Names with the `as` Keyword

#### There's another solution to the problem of bringing two types of the same name into the same scope with `use`: after the path, we can specify `as` and a new local name, or *alias*, for the type. The following code shows another way to write the code example from before by renaming one of the two `Result` types using `as`.

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

#### In the second `use` statement, we chose the new name `IoResult` for the `std::io::Result` type, which won't conflict with the `Result` from `std::fmt` that we've also brought into scope. Both this and the prior example are considered idiomatic, so the choice is up to you!

## <ins>Re-exporting Names with `pub use`</ins>

#### When we bring a name into scope with the `use` keyword, the name available in the new scope is private. To enable the code that calls our code to refer to that name as if it had been defined in that code's scope, we can combine `pub` and `use`. This technique is called *re-exporting* because we're bringing on item into scope but also making that item available for others to bring into their sope.

#### The following code example shows a change to an older example with the use of `pub use` instead of `use`:

```
mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
	hosting::add_to_waitlist();
}
```

#### Before this change, external code would have to call the `add_to_waitlist` function by using the path `restaurant::front_of_house::hosting::add_to_waitlist()`, which also would have required the `front_of_house` module be marked as `pub`. Now that this `pub use` has re-exported the `restaurant::hosting::add_to_waitlist()` instead.

#### Re-exporting is useful when the internal structure of your code is different from how programmers calling your code would think about the domain. For example, in this restaurant methaphor, the people running the restaurant probably won't think about the parts of the restaurant in those terms. With `pub use`, we can write our code with one structure but expose a different structure. Doing so makes our library well organized for programmers working on the library and programmers calling the library. We'll look at another example of `pub use` and how it affects your crate's documentation in Chapter 14.

## <ins>Using External Packages</ins>

#### In Chapter 2, we programmed a guessing game project that used an external package called `rand` to get random numbers. TO use `rand` in our project, we added this line to *Cargo.toml*:

```
rand = "0.8.5"
```

#### Adding `rand` as a dependency in *Cargo.toml* tells Cargo to download the `rand` package and any dependencies from *crates.io* and make `rand` available to our project.

#### Then, to bring `rand` definitions into the scope of our package, we added a `use` line starting with the name of the crate, `rand`, and listed the items we wanted to bring into scope. Recall that in Chapter 2, we brought the `Rng` trait into scope and called teh `rand::thread_rng` function:

```
use rand::Rng;

fn main() {
	let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

#### Members of the Rust community have made many packages available at *crates.io*, and pulling any of them into your packages involves these same steps, listing them in your package's *Cargo.toml* file and using `use` to bring items from their crates into scope.

#### Note that the standard `std` library is also a crate that's external to our package. Because the standard library is shipped with the Rust language, we don't need to change *Cargo.toml* to include `std`. But we do need to refer to it with `use` to bring items from there into our package's scope. For example, with `HashMap` we would use this line:

```
use std::collections::HashMap;
```

#### This is an absolute path starting with `std`, the name of the standard library.

## <ins>Using Nested Paths to Clean Up Large `use` Lists</ins>

#### If we're using multiple items defined in the same crate or same module, listing each item on its own line can take up a lot of vertical space on our files. For exmaple, these two `use` statements we had in the Guessing Game bring items from the `std` into scope:

```
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

#### Instead, we can use nested paths to bring the same items into scope in one line. We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ, as shown below:

```
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

#### In bigger programs, bringing many items into scope from the same crate or module using nested paths can reduce the number of separate `use` statements needed by a lot!

#### We can use a nested path at any level in a path, which is useful when combining two `use` statements that share a subpath. For example, the following code example shows two `use` statements: one that brings `std::io` into scope and one that brings `std::io::Write` into scope:

```
use std::io;
use std::io::Write;
```

#### The common parts of these two paths is `std::io`, and that's the complete first path. TO merge these two paths into one `use` statement, we can use `self` in the nested path, as shown below:

```
use std::io::{self, Write};
```

#### This line brings `std::io` and `std::io::Write` into scope.

## <ins>The Glob Operator</ins>

#### If we want to bring *all* public items defined in a path into scope, we can specify that path followed by the `*` glob operator:

```
use std::collections::*;
```

#### This `use` statement brigns all public items defined in `std::collections` into the current scope. Be careful when using the glob operator! Glob can make it harder to tell what names are in scope and where a name used in your program was defined.

#### The glob operator is often used when testing to bring everything under test into the `tests` module; This module will be discussed in Chapter 11. The glob operator is alos sometimes used as part of the prelude pattern.
