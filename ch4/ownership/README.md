# Chapter 4 Section 1: What is Ownership?

#### Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer's memory while running. Some languages have garbage collections that regularly looks for no-longer-used memory as the program runs; in other languages the programmer must explicitly allocate and free the memory. 

#### Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won't compile. None of the features of ownership will slow down your program while it's running.

#### Because ownership is a new concept for many programmers, it does take some time to get used to. The good news is that the more experienced you become with Rust and the rules of the ownership system, the easier you'll find it to naturally develop code that is safe and efficient. Keep at it!

#### When you understand ownership, you'll have a solid foundation for understanding the features that make Rust unique. In this chapter, you'll learn ownership by working through some examples that focus on a very common data structure: Strings!

## <ins>The Stack and the Heap</ins>

#### Many programming languages don't require you to think about the stack and the heap very often, but in a systems programming language like Rust, whether a value is on the stack of the heap affects how the language behaves and why you have to make certain decisions. Parts of ownership will be descibed in relation to the stack and the heap later in this chapter, so here is a brief explanation in preparation.

#### Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as *last in, first out*. Think of a stack of plates: when you add more plates, you put them on top of the pile, and when you need a plate, you take on off the top. Adding or removing plates from the middle or bottom wouldn't work as well! Adding data is called *pushing onto the stack*, and removing data is called *popping off the stack*. All the data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

#### The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a *pointer*, which is the address of that location. This process is called *allocating on the heap* and is sometimes abbreviated as just *allocating* (pushing values onto the stack is not considered allocating). Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer. THink of being seated at a restaurant. When you enter, you state the number of people in your group, and the host finds an empty table that fits everyone and leads you there. If someone in your group comes late, they can ask where you've been seated to find you.

#### Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work, because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.

#### Accessing data in the heap is slower than accessing data on the stack because you have to follow a pointer to get there. Contemporary processors are faster if they jump around less in memory. Continuing the analogy, consider a server at a restaurant taking orders from many tables. IT's most efficient to get all the orders at one table before moving on to the next table. Taking an order from table A, then an order from table B, then one from A again, and then one from B again would be a much slower process. By the same token, a processor can do its job better if it works on data that's close to other data (as it is on the stack) rather than farther away (as it can be on the heap).

#### When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function's local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

#### Keeping track of what parts of code are using what data on the heap, minimizing the amout of duplicate data on the heap, and cleaning up unused data on the heap so you don't run out of space are all problems that ownership addresses. Once you understand ownership, you won't need to think about the stack and the heap very often, but knowing that the main purpose of ownership is to manage heap data can help explain why it works the way it does.

## <ins>Ownership Rules</ins>

#### First, let's take a look at the ownership rules. Keep these rules in mind as we work through the examples  that illustrat them: 

* Each value in rust has an *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## <ins>Variable Scope</ins>

#### Now that we're past basic Rust syntax, we won't include the `fn main() {}` code in examples, so if you're following along, make sure to put the following examples inside a `main` function manually. As a result, our examples will be a bit more concise, letting us focus on the actual details rather than boilerplate code.

#### As a first example of ownership, we'll look at the *scope* of some variables. A scope is the range within a program for which an item is valid. Take the following variable:

```let s = "hello";```

#### The variable `s` refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it's declared until the end of the current *scope*. The following example shows a program with comments annotating where the variable `s` would be valid.

```
{						// s is not valid here, it's not yet declared
	let s = "hello";	// si is valid from this point forward

	// do stuff with s
}						// this scope is now over, and s is no longer valid
```

#### In other words, there are two important points in time here:

* When `s` comes *into* scope, it is valid.
* It remains valid until it goes *out of scope*.

#### At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages. Now we'll build on top of this understanding by introducing the `String` type.

## <ins>The `String` Type</ins>

#### To illustrate the rules of ownership, we need a data type that is more complex than those we convered in the **Data Types** section of Chapter 3. The types covered previously are of a known size, can be stored on the stack and popped off the stack when their scope is over, and can be quickly and trivially copied to make a new, independent instance if another part of code needs to use the same value in a different scope. But we want to look at data that is stored on the heap and explore how Rust knows when to clean up that data, and the `String` type is a great example.

#### We'll concentrate on the parts of `String` that relate to ownership. These aspects also apply to other complex data types, whether they are provided by the standard library or created by you. We'll disucss `String` more in depth in **Chapter 8**.

#### We've already seen string literals, where a string value is hardcoded into our program. String literals are convenient, but they aren't suitable for every situation in which we may want to use text. One reason is that they're immutable. Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? For these situations, Rust ahs a second string type, `String`. This type manages data allocated on the heap and as such is able to sotre an amount of text that is unknwon to us at compile time. You can create a `String` from a string literal using the `from()` function, like so:

```
let s = String::from("hello");
```

#### The double colon `::` operator allows us to namespace this particular `from` function under the `String` type rather than using some sort of name like `string_from`. We'll discuss this syntax more in the **Method Syntax** section of Chapter 5, and when we talk about namespacing with modules in "Paths for Referring to an Item in the Modules Tree" in Chapter 7.

#### This kind of string *can* be mutated:

```
let mut s = String::from("hello");

s.push_str(", world!"); // push_str() appends a literal to a String

println("{s}"); // This will print `hello, world!`
```

#### So, what's the difference here? Why can `String` but mutated but literals cannot? The difference is in how these two types deal with memory.

## <ins>Memory and Allocation</ins>

#### In the case of a string literal, we know the contents at compile time, so the text is hardcodes directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal's immutability. Unfortunately, we can't put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

#### With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

* The memory must be requested from the memory allocator at runtime.
* We need a way of returning this memory to the allocator when we're done with our `String`

#### That first part is done by us: when we call `String::from`, its implementation requests the memory it needs. This is pretty much universal in programming languages.

#### However, the seconds part is different. In languages with a *garbage collector (GC)*, the GC keeps track of and cleans up memory that isn't being used anymore, and we don't need to think about it. In most languages without a GC, it's our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we'll waste memory. If we do it too early, we'll have an invalid variable. If we do it twice, that's a bug too. We need to pair exaclty one `allocate` with exactly one `free`.

#### Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. Here's a version of our scope example from before using a `String` instead of a string literal:

```
{
	let s = String::from("hello"); 	// s is valid from this point forward

	// do stuff with s
}									// this scope is now over, and s is no longer valid
```

#### There is a natural point at which we can return the memory our `String` needs to the allocator: when `s` goes out of scope. When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it's where the author of `String` can put the code to return the memory. Rust calls the `drop` automatically at the closing curly bracket.

#### This pattern has a profound impact on the way Rust code is written. It may seem simple right now, but the behavior of coe can be unexpected in more complicated situations when we want to have multiple variables use the data we've allocated on the heap. Let's explore some of those situations now

## <ins>Variables and Data Interacting with Move</ins>

####
