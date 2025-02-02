# <ins>Chapter 5 Section 3: Method Syntax</ins>

#### *Methods* are similar to functions: we declare them with the `fn` keyword and a name, they can have parameters and a return value, and they contain some code that's run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object, covered in Chapter 6 and Chapter 17, respectively), and their first parameter is always `self`, which represents the instance of the struct the method is being called on.

## <ins> Defining Methods</ins>

#### Let's change the `area` function that has a `Rectangle` instance as a parameter and instead make an `area` method defined on the `Rectangle` struct, as shown below:

```
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!(
		"The are of the rectangle is {} square pixels.",
		rect1.area()
	);
}
```

#### To define the function within the context of `Rectangle`, we start an `impl` (implementation) block for `Rectangle`. Everything within this `impl` block will be associated with the `Rectangle` type. Then we move the `area` function within the `impl` curly brackets and change the first (and in this case, only) parameter to be `self` in the signature and everywhere within the body. In `main` where we called the `area` function and passed `rect1` as an argument, we can instead use *method syntax* to call the `area` method on our `Rectangle` instance. The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.

#### In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`. The `&self` is actualy short for `self: &Self`. Within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for. Methods must have a parameter named `self` of type `Self` for their first parameter, so Rust lets you abbreviate this with only the name `self` in the first parameter spot. Note that we will need to use the `&` in front of the `self` shorthand to indicate that this method borrows the `Self` instance, just as we did in `rectangle: &Rectangle`. Methods can take ownership of `self`, borrow `self` immutably, as we've done here, or borrow `self` mutably, just as they can any other parameter.

#### We chose `&self` here for the same reason we used `&Rectangle` in the function version: we don't want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we've called the method on as part of what the methdo does, we'd use `&mut self` as the first parameter. Having a method that takes ownership of the instance by using just `self` as the first parameter is rare; this technique is usually used when the method transforms `self` into something else and you want to prevent the caller from using the original instance after the transformation.

#### The main reason for using methods instead of functions, in addition to providing method syntax and not having to repeat the type of `self` in every method's signature, is for organization. We've put all the things we can do with an instance of a type in one `impl` block rather than making future users of our code search for capabilities of `Rectangle` in various palces in the library we provide.

#### Note that we can choose to give a method the same name as one of the struct's fields. for example, we can define a method on `Rectangle` that is also named `width`:

```
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn width(&self) -> bool {
		self.width > 0
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	if rect1.width() {
		println!("The rectangle has a nonzero width: it is {}", rect1.width);
	}
}
```

#### Here, we're choosing to make the `width` method return `true` if the value in the instance's `width` field is greater than `0` and `false` if the value is `0`: we can use a field within a method of the same name for any purpose. In `main`, when we follow `rect1.width` with parentheses, Rust knows we mean the method `width`. When we don't use parentheses, Rust knows we mean the field `width`.

#### Often, but not always, when we give a method the same name as a field we want it to only return the value in the field and do nothing else. Methods like this are called *getters*, and Rust does not implement them automatically for struct fields as some other languages do. Getters are useful because you can make the field private but the method public, and thus enable read-only access to that field as part of the type's public API. Chapter 7 will discuss what public and private are and how to designate a field or method as public or private.

## <ins>Methods with More Parameters</ins>

#### Let's practice using methods by implementing a second method on the `Rectangle` struct. This time we want an instance of `Rectangle` to take another instance of `Rectangle` and return `true` if the second `Rectangle` can fit completely within `self` (the first `Rectangle`); otherwise, it should return `false` That is, once we've define the `can_hold` method, we want to be able to write the program shown below:

```
fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};

	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};

	println!("Can rect1 hold rect2? {}, rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}, rect1.can_hold(&rect3));
}
```

#### The expected output would look like the following because both dimensions of `rect2` are smaller than the dimensions of `rect1`, but `rect3` is wider than `rect1`:

```
Can rect1 hold rect2? true
Can rect1 hold rect3? false
```

#### We know we want to define a method, so it will be within the `impl Rectangle` block. The method name will be `can_hold`, and it will take an immutable borrow of another `Rectangle` as a parameter. We can tell what the type of the parameter will be by looking at the code that calls the method: `rect1.can_hold(&rect2)` passed in `&rect2`, which is an immutable borrow to `rect2`, an instance of `Rectangle`. This makes sense because we only need to read `rect2` (rather than write, which would mean we'd need a mutable borrow), and we want `main` to retain ownership of `rect2` so we can use it again after calling the `can_hold` method. The return value of `can_hold` will be a Boolean, and the implementation will check whether the width and height of `self` are greater than the width and height of the other `Rectangle`, respectively. Let's add the new `can_hold` method to the `impl` block to our code from above:

```
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		other.width < self.width && other.height < self.height
	}
}
```

#### When we run this code with the `main` function, we'll get our desired output. Methods can take multiple parameters that we add to the signature after the `self` parameter, and those parameters work just like parameters in functions.

## <ins>Associated Functions</ins>

####  All functions defined within an `impl` block are called *associated functions* because they're associated with the type anmed after the `impl`. We can define associated functions that don't have `self` as their first parameter (and thus are not methods) because they don't need an instance of the type to work with. We've already used on function like this: the `String::from` function that's defined on the `String` type.

#### Associated functions that aren't methods are often used for constructors that will return a new instance of the struct. THese are often called `new`, but `new` isn't a special name and isn't built into the language. For exampel, we could choose to provide an associate function named `square` that would have one dimension parameter and use that as both width and height, thus making it easier to create a squar `Rectangle` rather than having to specify the same value twice:

```
impl Rectangle {
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}
```

#### The `Self` keywords in the return type and in the body of the function are aliases for the type that appears after the `impl` keyword, which in this case is `Rectangle`.

#### To call this associated function, we use the `::` syntax with the struct name; `let sq = Rectangle::square(3);` is an example. This function is namespaced by the struct: the `::` syntax is used for both associated functions and namespaces created by modules. Modules will be discussed in Chapter 7.

## <ins>Multiple `impl` Blocks</ins>

#### Each struct is allowed to have multiple `impl` blocks. For example, the example below is equivalent to our complete `impl` block for `Rectangle`:

```
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

impl Rectangle {
	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.widht && self.height > other.height
	}
}
```

#### There's no reason to separate these methods into multiple `impl` blocks here, but this is valid syntax. We'll see a case in which multiple `impl` blocks are useful in Chapter 10, in which generic types and traits are discussed.

# <ins>Summary</ins>

#### Structs let you create custom types that are meaningful for your domain. By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. In `impl` blocks, you can define functions that are associated with your type, and methods are a kind of associated function that let you specify the behavior that instances of your structs have.

#### But structs aren't the only way you can create custom types: let's turn to Rust's enum feature to add another tool to our toolbox.
