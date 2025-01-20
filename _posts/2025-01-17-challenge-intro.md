# Print a prompt

Below are the challenge instructions: 

```
"In this stage, you'll implement printing a shell prompt ($) and waiting for user input".
Notes:
There's a space after the $ character in the prompt.
Your program must not exit after printing $, it should wait for user input.
We'll handle reading commands and executing them in later stages, this stage only deals with printing the prompt.
```

To get us started, CodeCrafters gives us the relevant code and tells us to just uncomment some lines. This ends up being the result:

```rust
use std::io::{self, Write};
fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
}
```

Looks simple enough! However, there are some notable things happening here that illustrate one of Rust's main features, borrowing. I'll get to them in a second.

## Diving in Rust principles

The `main` function is always the first code that runs in every Rust executable program. Here we're just printing `$` and waiting for user input.
We're declaring some variables with the `let` keyword. One of these variables is declared as being *mutable* (`let mut`).
In Rust, variables are immutable by default. We can't change immutable variable's values once they have been declared.

Notice also we have a funky `std.read_line(&mut input).unwrap()`. 

Let's look at the documentation for `read_line`:

```rust
pub fn read_line(&self, buf: &mut String) -> io::Result<usize>
"""
Locks this handle and reads a line of input, appending it to the specified buffer.

For detailed semantics of this method, see the documentation on BufRead::read_line. In particular:

- Previous content of the buffer will be preserved. To avoid appending to the buffer, you need to clear it first.
- The trailing newline character, if any, is included in the buffer.
"""
```

`&mut input` is a mutable reference to `input`. A reference is similar to a pointer,
but unlike a pointer, "a reference is guaranteed to point to a valid value of a particular type for the life of that reference".
[[Rust book]](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing).

Whew! The way I think about this is that a pointer may point to *nothing*. However, a *reference* always refers to *something*. 

By default, references are immutable as well, and in Rust, declaring/creating a reference is called *borrowing*. To understand borrowing, we need to take a quick detour 
into *ownership*:

### Ownership

Ownership in Rust is a system that governs how memory is managed. These are the ownership rules in Rust:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Rust's ownership system enforces these rules at compile time.

Ownership is affected by scopes and moves. A scope is the range within a program for which an item is valid. Depending on data types and how they affect memory, variables are either copied or moved when assigned like this:

```rust
let x = 10;
let y = x;
```

Here, `x` is of type `i32`. Integers are of a known and fixed size, so here the assignment copies the value.
In other data types, such as with `String`, such an assignment does not copy the value. Instead, the pointer is copied, meaning there would now be two pointers pointing to the same memory.
However, this could cause memory safety issues: consider this

```rust
{
    let s1 = String::from("don't panic");
    let s2 = s1;
}
```

Once `s1` and `s2` go out of scope, the memory management system would try to free the same underlying memory. This is known as a double free error. Freeing memory twice can lead to memory corruption.
To prevent this, after `let s2 = s1`, Rust considers `s1` as no longer valid. `s1` is "moved" to `s2`. Variables are also moved when passed to functions, which is why we need references. References let us *borrow* variables so that their ownership does not change.

In our original code snippet, the `read_line` function is *borrowing* `input` through the mutable reference. It appends to `input` the contents of the line read from standard input.
If we had not used mutable references, `input`'s ownership would have been transferred into the `read_line` function (assuming `read_line` would look like `fn read_line(mut input: String)`) and dropped after it went out of scope (i.e. once the function is over).

## Unwrap

And finally, we have a funky `unwrap()`. To explain this we need to look at Rust's error handling.

In Rust, errors are classified into recoverable and unrecoverable errors. 
"Unrecoverable errors are always symptoms of bugs, and so we want to immediately stop the program." [[Error handling]](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
For recoverable errors, there is an enum type `Result<T,E>`. There is some funky stuff about generic types here, but for now all we need to know is that for functions that return a `Result<T,E>` type,
the rusty way of handling this type is to use the `match` expression:

```rust
fn main() {
    let number_str = "42a"; // Invalid number
    let parse_result = number_str.parse::<i32>(); // Try to parse the string as an integer

    match parse_result {
        Ok(number) => {
            println!("Parsed number successfully: {}", number);
        }
        Err(error) => {
            panic!("Failed to parse number: {}", error);
        }
    }
}
```

This code snippet ends up printing:

```
thread 'main' panicked at src/main.rs:10:13:
Failed to parse number: invalid digit found in string

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
``` 

because the string `"42a"` is not a valid integer. Essentially what this means is that when functions return the `Result<T,E>` type, we need to handle the fact that there is either an `Ok` value or an `Err`. 
This can sometimes be annoying, so Rust has the `unwrap` function, that will return the `Ok` value of the `Result<T,E>` type if it exists and panic otherwise. Because of this, the use of `unwrap()` is typically discouraged.
Luckily, there also exists `unwrap_or_else`, `unwrap_or` and `unwrap_or_default`.

# Conclusion

Well, that was a very lengthy blog post for a very simple code snippet... 
My goal here was to learn Rust at the same time as writing a program that satisfies the CodeCrafters requirements, but I think that if I do such deep-dives into Rust while 
trying to complete the challenge, I may never end up completing the challenge! I'll try to keep the Rust-related explanations to a minimum from now on, instead focusing more on the challenge itself.
