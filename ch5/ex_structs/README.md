# <ins>Chapter 5 Section 2: An Example Program Using Structs</ins>

#### To understand when we might want to use structs, let's write a program that calculates the area of a rectangle. We'll start by using single variables, and then refactor the program until we're using structs instead.

#### Make a new binary project with Cargo called *rectangles* that will take the width and height of a rectangle specified in pixels and calculate the are of a rectangle. The code below shows a short program with one way of doing exactly that in our projects *src/main.rs*:

```
fn main() {
	let width1 = 30;
	let height1 = 50;

	println!(
		"The area of the rectangle is {} square pixels.",
		area(width1, height1)
	);
}

fn area(width: u32, height: u32) -> u32 {
	width * height
}
```

Now, run this program using `cargo run`:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
The area of the rectangle is 1500 square pixels.
```

#### This code succeeds in figuring out the area of the rectangle by calling the `area` function with each dimension, but we can do more to make this code clear and readable:

```
fn area(width: u32, height: u32) -> u32 {
```

#### The `area` function is supposed to calculate the area of one rectangle, but the function we wrote has two parameters, and it's not clear anywhere in our program that the parameters are related. It would be more readable and more manageable to group width and height together. We've already discussed one way we might do that in the tuples section of Chapter 3: by using tuples.

## <ins>Refactoring with Tuples</ins>

#### The following shows another version of our program that uses tuples

```
fn main() {
	let rect1 = (30, 50);

	println!(
		"The area of the rectangle is {} square pixels.",
		area(rect1)
	);
}

fn area(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}
```

#### In one way, this program is better. Tuples let us add a bit of structure, and we're now passing just one argument. But in another way, this version is less clear: tuples don't name their elements, so we have to index into the parts of the tuple, making our calculation less obvious.

#### Mixing up the width and height wouldn't matter for the area calculation, but if we want to draw the rectangle to on the screen, it would matter! We would have to keep in mind that `width` is the tuple index `0` and `height` is the tuple index `1`. This would be even harder for someone else to figure out and keep in mind if they were to use our code. Because we haven't conveyed the meaning of our data in our code, it's now easier to introduce errors.

## <ins>Refactoring with Structs: Adding More Meaning</ins>

#### We use structs to add meaning by labeling the data. We can transform the tuple we're using into a struct with a name for the whole as well as names for the parts, as shown below:

```
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!(
		"The area of the rectangle is {} square pixels.",
		area(&rect1)
	);
}

fn area(rectangle: &Rectangle) -> u32 {
	rectangle.width * rectangle.height
}
```

#### Here we've defined a struct and name it `Rectangle`. Inside the curly brackets, we defined the fields as `width` and `height`. both of which have type `u32`. Then, in `main`, we created a particular instance of `Rectangle` that has a width of `30` and a height of `50`.

#### Our `area` function is now defined with one parameter, which we've named `rectangle`, whose type is an immutable borrow of a struct `Rectangle` instance. as mentioned in Chapter 4, we want to borrow the struct rather than take ownership of it. This way, `main` retains its ownership and can continue using `rect1`, which is the reason we use the `&` in the function signature and where we call the function.

#### The `area` function accesses the `width` and `height` fields of the `Rectangle` instance (note that accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs). Our function signature for `area` now says exactly what we mean: calculate the are of `Rectangle`, using its `width` and `height` fields. This conveys that the width and height are related to each other, and it gives descripitive names to the values rather than using the tuple index values of `0` and `1`. This is a win for clarity.

## <ins>Adding Useful Functionality with Derived Traits</ins>

#### It'd be useful to be able to print an instance of `Rectangle` while we're debugging our program and see the values for all its fields. The code below tries using the `println! macro` as we have used in previous chapters. This won't work, however: 

```
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("rect1 is {}", rect1);
}
```

#### When we compile this code, we get an error with this core message:

```
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

#### The `prinln!` macro can do many kinds of formatting, and by default, the curly brackets tell `println!` to use formatting known as `Display`: output intended for direct end user consumption. The primitive types we've seen so far implement `Display` by default because there's only one way you'd want to show a `1` or any other primitive type to a user. But with structs, the way `println!` should format the output is less clear because there are more display possibilities: Do you want commas or not? Do you want to print the curly brackets? Should all the fields be shown? Due to this ambiguity, Rust doesn't try to guess what we want, and structs don't have a provided implementation of `Display` to use with `println!` and teh `{}` placeholder.

#### If we continue reading the errors, we'll find this helpful note:

```
   = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

#### Let's try it! The `println!` macro call will now look like `println!("rect1 is {:?}", rect1);`. Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`. The `Debug` trail enables us to print our struct in a way that is useful for developers so we can see its value while we're debugging our code.

#### Compile the code with this change. Drat! We still get an error:

```
error[E0277]: `Rectangle` doesn't implement `Debug`
```

#### But again, the compiler gives us a helpful note:

```
   = help: the trait `Debug` is not implemented for `Rectangle`
   = note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

#### Rust *does* include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute `#[derive(Debug)]` just before the struct definition, as shown in the code below:

```
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("rect1 is {rect1:?}");
}
```

#### Now when we run the program, we won't get any errors, and we'll see the following output:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle { width: 30, height: 50 }
```

#### Nice! It's not the prettiest out, but it shows the values of all the fields for this instance, which would definitely help during debugging. When we have larger structs, it's useful to have output that's a bit easier to read; in those cases we can use `{:#?}` instead of `{:?}` in the `println!` string. In this example, using the `{:#?}` style will output the following:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
rect1 is Rectangle {
    width: 30,
    height: 50,
}
```

#### Another way to print out a value using the `Debug` format is to use the `dbg! macro`, which takes ownership of an expression (as opposed to `println!`, which takes a reference), prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.

##### Note: Calling the `dbg!` macro prints to the standard error console stream (`stderr`), as opposed to `println!`, which prints to the standard output console stream (`stdout`). `stderr` and `stdout` will be discussed more in Chapter 12.

#### Here's an example where we're interested in the value that gets assigned to the `width` field, as well as the value of the whole struct in `rect1`:

```
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main() {
	let scale = 2;
	let rect1 = Rectangle {
		width: dbg!(30 * scale),
		height: 50,
	};

	dbg!(&rect1);
}
```

#### We can put `dbg!` around the expression `30 * scale` and , because `dbg!` returns ownership of the expression's value, the `width` field will get the same value as if we didn't have the `dbg!` call there. We don't want `dbg!` to take ownership of `rect1`, so we use a reference to `rect1` in the next call. Here's what the output of this example looks like:

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

#### we can see the first bit of output came from *src/main.rs* line 10 where we're debugging the expression `30 * scale`, and its resultant value is `60` (the `Debug` formatting implemented for integers is to print only their value). The `dbg!` call on line 14 of *src/main.rs* outputs the value fo `&rect1`, which is the `Rectangle` struct. This output uses the pretty `Debug` formatting of the `Rectangle` type. The `dbg!` macro can be really helpful when you're trying to figure out what your code is doing!

#### In addition to the `Debug` trait. Rust has provided a number of traits for us to use with the `derive` attribute that can add useful behavior to our custom types. Chapter 10 will cover hwo to implement these traits with custom behavior as well as how to create your own traits. There are also attributes other than `derive`.

#### Our `area` function is very specific: it only computes the area of rectangles. It would be helpful to tie this behavior more closeley to our `Rectangle` struct because it won't work with any other type. Let's look at how we can continue to refactor this code by turning the `area` function into an `area` method defined on our `Rectangle` type.
