# <ins>Chapter 6 Section 3: Concise Control Flow with `if let`</ins>

#### The `if let` syntax lets you combine `if` and `let` into a less vebose way to handle values that match one pattern while ignoring the rest. Consider the program below that matches on an `Option<u8>` value in the `config_max` variable but only wants to execute code if the value is the `Some` variant.

```
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("The maximum is configured to be {max}),
    _ => (),
}
```

#### If teh value is `Some`, we print out the value in the `Some` variant by binding the value to the variable `max` in the patter. We don't want to do anything with the `None` value. To satisfy the `match` expression, we have to add `_ => ()` after processing just one variant, which is annoying boilerplate code to add.

#### Instead, we could write this in a shorter way using `if let`. The following code behaves the same as the `match` from above:

```
let config_max = Some(3u8);
if let Some(max) = config {
    println!("The maximum is configured to be {max}");
}
```

#### The syntax `if let` takes a pattern and an expression separated by an equal sign. It works the same was as a `match`, where the expression is given to the `match` and the pattern is its first arm. In this case, the pattern is `Some(max)`, and the `max` binds to the value inside the `Some`. We can then use `max` in the body of the `if let` block in the same way we used `max` in the corresponding `match` arm. The code in the `if let` block isn't run if the value doesn't matcht the pattern.

#### Using `if let` means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that `match` enforces. Choosing between `match` and `if let` depends on what you're doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

#### In other words, you can think of `if let` as syntax sugar for a `match` that runs code when the value matches one pattern and then ignores all other values.

#### We can include an `else` with an `if let`. The block of code that goes with the `else` is the same as the block of code that would go with the `_` case in the `match` expresson that is equivalent to the `if let` and `else`. Recall the `Coin` enum definition previously, where the `Quarter` variatn also hel a `UsState` value. If we wanted to count all non-quarter coins we see while announcing the state of the quarters, we could do that with a `match` expression, like this:

```
let mut count = 0;
match coin {
    CoinQuarter(state) => println!("State quarter from {state:?}!"),
    _ => count += 1;
}
```

#### Or we could use an `if let` and `else` expression, like this:

```
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {state:?}!");
} else {
    count += 1;
}
```

#### If you have a situation in which your program has logic that is too verbose to express using a `match` remember that `if let` is in your Rust toolbox.

# <ins>Summary</ins>

#### We've now covered how to use enums to create custom types that can be one of a set of enumerated values. We've shown how the standard library's `Option<T>` type helps you use the type system to prevent errors. When enum values have data inside them, you can use `match` of `if let` to extract and use those values, depending on how many cases you need to handle.

#### Your Rust program can now express concepts in your domain using structs and enums. Creating custom types to use in your API ensure type safety: the compiler will make certain your functions only get values of the type each function expects.

#### In order to provide a well-organized API to your users taht is straightforward to use and only exposes exactly what your users will need, let's now turn to Rust's modules.