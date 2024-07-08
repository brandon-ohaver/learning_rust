# Chapter 3 Section 1: Variables and Mutability

## Defined variables are initialized as immutable. This makes it so that the programmer is nudged into writing code in a way that takes advantage of the safety and easy concurrency that Rust offers. However, variables are still able to be made mutable. In order to make a variable mutable, add `mut` in front of the `let` in variable declaration.

## Constants can be declared by using `const` instead of `let`

## The idea of "shadowing" is when a variable is redeclared with a new value and when compiled the first use of the variable is overshadowed by the redeclared use of the variable making it so the first time the variable is declared is not used
