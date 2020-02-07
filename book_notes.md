# Rust Book Notes

## 3. Common Programming Concepts

### 3.2 Data Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### 3.3 Functions

Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

```
let y = 6; is a statement.

Expressions evaluate to something and make up most of the rest of the code that you’ll write in Rust. Consider a simple math operation, such as 5 + 6, which is an expression that evaluates to the value 11
```

Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.

### 3.4 Control Flow

Unlike languages such as Ruby and JavaScript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.

One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. However, you might need to pass the result of that operation to the rest of your code. To do this, you can add the value you want returned after the break expression you use to stop the loop; that value will be returned out of the loop so you can use it.

All five array values appear in the terminal, as expected. Even though index will reach a value of 5 at some point, the loop stops executing before trying to fetch a sixth value from the array.

But this approach is error prone; we could cause the program to panic if the index length is incorrect. It’s also slow, because the compiler adds runtime code to perform the conditional check on every element on every iteration through the loop.

More importantly, with `for` over `while` to loop over an array, we’ve now increased the safety of the code and eliminated the chance of bugs that might result from going beyond the end of the array or not going far enough and missing some items.

The safety and conciseness of for loops make them the most commonly used loop construct in Rust. Even in situations in which you want to run some code a certain number of times, as in the countdown example that used a while loop in Listing 3-3, most Rustaceans would use a for loop. The way to do that would be to use a Range, which is a type provided by the standard library that generates all numbers in sequence starting from one number and ending before another number.

## 4. Understanding Ownership

Ownership is Rust’s most unique feature, and it enables Rust to make memory safety guarantees without needing a garbage collector.

Rust’s central feature is ownership. Although the feature is straightforward to explain, it has deep implications for the rest of the language.

All programs have to manage the way they use a computer’s memory while running.

- Some languages have garbage collection that constantly looks for no longer used memory as the program runs;
- in other languages, the programmer must explicitly allocate and free the memory.

Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.

In many programming languages, you don’t have to think about the stack and the heap very often. But in a systems programming language like Rust, whether a value is on the stack or the heap has more of an effect on how the language behaves and why you have to make certain decisions.

Ownership rules:

- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

## 6. Enums and Pattern Matching

### 6.2 The match Control Flow Operator

Combining match and enums is useful in many situations. You’ll see this pattern a lot in Rust code: match against an enum, bind a variable to the data inside, and then execute code based on it. It’s a bit tricky at first, but once you get used to it, you’ll wish you had it in all languages. It’s consistently a user favorite.

### 6.3 Concise Control Flow with if let

Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

If you have a situation in which your program has logic that is too verbose to express using a match, remember that if let is in your Rust toolbox as well.

Use enums to create custom types that can be one of a set of enumerated values. When enum values have data inside them, you can use match or if let to extract and use those values, depending on how many cases you need to handle.

## 7. Managing Growing Projects with Packages, Crates, and Modules

A package can contain multiple binary crates and optionally one library crate. As a package grows, you can extract parts into separate crates that become external dependencies.

Rust has a number of features that allow you to manage your code’s organization, including which details are exposed, which details are private, and what names are in each scope in your programs. These features, sometimes collectively referred to as the module system, and include:

    Packages: A Cargo feature that lets you build, test, and share crates
    Crates: A tree of modules that produces a library or executable
    Modules and use: Let you control the organization, scope, and privacy of paths
    Paths: A way of naming an item, such as a struct, function, or module

Crate: A crate is a binary or library.
The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.

Package: A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. A package must contain zero or one library crates, and no more. It can contain as many binary crates as you’d like, but it must contain at least one crate (either library or binary).

### 7.3 Paths for Referring to an Item in the Module Tree

Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

## 8. Common Collections

Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.

- A vector allows you to store a variable number of values next to each other.
- A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
- A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.

### 8.1

When you’re writing a program, if you don’t know the exhaustive set of types the program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object

### 8.2 Storing UTF-8 Encoded Text with Strings

New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8.

- Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.
- The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice. Therefore, Rust asks you to be more specific if you really need to use indices to create string slices.

But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.

Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library.

## 9. Error Handling

In many cases, Rust requires you to acknowledge the possibility of an error and take some action before your code will compile.

Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

### 9.2 Recoverable Errors with Result

This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values in Listing 9-6. If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

There is a difference between what the match expression and the ? operator do:

- error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another.
- When the ? operator calls the from function, the error type received is **converted into the error type defined in the return type of the current function**. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
- As long as each error type implements the from function to define how to convert itself to the returned error type, the ? operator takes care of the conversion automatically.

The ? operator can only be used in functions that have a return type of Result, because it is defined to work in the same way as the match expression

### 9.3 To `panic!` or Not to `panic!`

So how do you decide when you should call panic! and when you should return Result?

- When code panics, there’s no way to recover. You could call panic! for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision on behalf of the code calling your code that a situation is unrecoverable.
- When you choose to return a Result value, you give the calling code options rather than making the decision for it.
  - The calling code could choose to attempt to recover in a way that’s appropriate for its situation,
  - or it could decide that an Err value in this case is unrecoverable, so it can call panic! and turn your recoverable error into an unrecoverable one.

Therefore, returning Result is a good default choice when you’re defining a function that might fail.

unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.

#### Guidelines for Error Handling

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

- The bad state is not something that’s expected to happen occasionally.
- Your code after this point needs to rely on not being in this bad state.
- There’s not a good way to encode this information in the types you use.

When to use panic:

- If someone calls your code and passes in values that don’t make sense, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development.
- Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.
- Cases:
  - When your code performs operations on values, your code should verify the values are valid first and panic if the values aren’t valid.
  - This is mostly for safety reasons: attempting to operate on invalid data can expose your code to vulnerabilities. This is the main reason the standard library will call panic! if you attempt an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem.
  - Functions often have contracts:
    - their behavior is only guaranteed if the inputs meet particular requirements.
    - Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle.
    - In fact, there’s no reasonable way for calling code to recover; the calling programmers need to fix the code.
    - Contracts for a function, especially when a violation will cause a panic, should be explained in the API documentation for the function.

When to use Result:

- However, when failure is expected, it’s more appropriate to return a Result than to make a panic! call.
- Examples include a parser being given malformed data or an HTTP request returning a status that indicates you have hit a rate limit.
- In these cases, returning a Result indicates that failure is an expected possibility that the calling code must decide how to handle.

Rust’s error handling features are designed to help you write more robust code.

- The panic! macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values.
- The Result enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use Result to tell code that calls your code that it needs to handle potential success or failure as well.

## 10. Generic Types, Traits, and Lifetimes

In the same way that the function body can operate on an abstract list instead of specific values, generics allow code to operate on abstract types.

### 10.1 Generic Data Types

We can use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.

`fn largest<T>(list: &[T]) -> T {`

We read this definition as:

- the function `largest` is generic over some type T.
- This function has one parameter named list, which is a slice of values of type T. The largest function will return a value of the same type T.

Advice using generic type parameters:

- You can use as many generic type parameters in a definition as you want, but using more than a few makes your code hard to read.
- **When you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.**

When you recognize situations in your code with multiple struct or enum definitions that differ only in the types of the values they hold, you can avoid duplication by using generic types instead.

#### Performance of Code using Generics

- Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types.
- Rust accomplishes this by performing monomorphization of the code that is using generics at compile time.
  - Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
  - The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

### 10.2 Traits: Defining Shared Behavior

A trait tells the Rust compiler about functionality a particular type has and can share with other types.

- We can use traits to define shared behavior in an abstract way.
- We can use trait bounds to specify that a generic can be any type that has certain behavior.

#### Returning types that implements traits

The ability to return a type that is only specified by the trait it implements is especially useful in the context of closures and iterators

Benefits summary:

- Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior.
- The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.

### 10.3 Validating references with LifeTimes

TL;DR:

- every reference in Rust has a lifetime, which is the scope for which that reference is valid.
- Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
- We must annotate types when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
- Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

The concept of lifetimes is somewhat different from tools in other programming languages, arguably making lifetimes Rust’s most distinctive feature.

Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types. Most people use the name 'a. We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

#### Lifetime Elision

In Chapter 4 we had a function in Listing 4-9, which is shown again in Listing 10-26, that compiled without lifetime annotations.

```
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

```

The reason this function compiles without lifetime annotations is historical: in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because every reference needed an explicit lifetime. After writing a lot of Rust code, the Rust team found that Rust programmers were entering the same lifetime annotations over and over in particular situations. These situations were predictable and followed a few deterministic patterns. The developers programmed these patterns into the compiler’s code so the borrow checker could infer the lifetimes in these situations and wouldn’t need explicit annotations.

This piece of Rust history is relevant because it’s possible that more deterministic patterns will emerge and be added to the compiler. In the future, even fewer lifetime annotations might be required.

The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations:

- first rule is that each parameter that is a reference gets its own lifetime parameter.
- second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
- The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

#### The Static Lifetime

'static, which means that this reference can live for the entire duration of the program.

The text of the string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static

You might see suggestions to use the 'static lifetime in error messages:

- But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not.
- You might consider whether you want it to live that long, even if it could.
- Most of the time, the problem results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the 'static lifetime.

#### Generic Type Parameters, Trait Bounds & Lifetimes together

```
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

```

This is the longest function from Listing 10-22 that returns the longer of two string slices.

- But now it has an extra parameter named ann of the generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause.
- This extra parameter will be printed before the function compares the lengths of the string slices, which is why the Display trait bound is necessary.
- Because lifetimes are a type of generic, the declarations of the lifetime parameter 'a and the generic type parameter T go in the same list inside the angle brackets after the function name.

#### Summary

- Generic type parameters let you apply the code to different types.
- Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs.
- Lifetime annotations to ensure that this flexible code won’t have any dangling references.

And all of this analysis happens at compile time, which doesn’t affect runtime performance!

## 11. How To Write Tests

Correctness in our programs is the extent to which our code does what we intend it to do. Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove. Rust’s type system shoulders a huge part of this burden, but the type system cannot catch every kind of incorrectness. As such, Rust includes support for writing automated software tests within the language.

Eg:

- We write a function called add_two that adds 2 to whatever number is passed to it.
- This function’s signature accepts an integer as a parameter and returns an integer as a result.
- When we implement and compile that function, Rust does all the type checking and borrow checking that you’ve learned so far to ensure that, for instance, we aren’t passing a String value or an invalid reference to this function.
- But Rust can’t check that this function will do precisely what we intend, which is return the parameter plus 2 rather than, say, the parameter plus 10 or the parameter minus 50! That’s where tests come in.

### 11.1 How to write tests

The next part of the test output, which starts with Doc-tests adder, is for the results of any documentation tests. We don’t have any documentation tests yet, but Rust can compile any code examples that appear in our API documentation. This feature helps us keep our docs and our code in sync!

Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

If the value is false, the assert! macro calls the panic! macro, which causes the test to fail. Using the assert! macro helps us check that our code is functioning in the way we intend.

The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal. This macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely won’t be if our code is functioning as we intend.

Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

You can’t use the #[should_panic] annotation on tests that use Result<T, E>. Instead, you should return an Err value directly when the test should fail.

### 11.3 Test Organization

- If our project is a binary crate that only contains a src/main.rs file and doesn’t have a src/lib.rs file, we can’t create integration tests in the tests directory and bring functions defined in the src/main.rs file into scope with a use statement.
- Only library crates expose functions that other crates can use; binary crates are meant to be run on their own.

## 12. An I/O Project: Building a Command Line Program

### 12.3 Refactoring to Improve Modularity and Error Handling

The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

- Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
- As long as your command line parsing logic is small, it can remain in main.rs.
- When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in lib.rs
- Handling the error if run returns an error

This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs. The only code that remains in main.rs will be small enough to verify its correctness by reading it.

Note: Using primitive values when a complex type would be more appropriate is an anti-pattern known as primitive obsession.

## 13. Funtional Language Features: Iterators & Closures

### 13.1 Closures: Anonymous Functions that Can Capture Their Environment

- Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
- You can create the closure in one place and then call the closure to evaluate it in a different context.
- Unlike functions, closures can capture values from the scope in which they’re defined.

Specific usecase of where they might be helpful:

- We can create a struct that will hold the closure and the resulting value of calling the closure. The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. You may know this pattern as memoization or lazy evaluation.

Capturing environment with closures:

- Most of the time when specifying one of the Fn trait bounds, you can start with Fn and the compiler will tell you if you need FnMut or FnOnce based on what happens in the closure body.
- Iterators are where closures that can capture their environment are useful as function parameters.

### 13.2 Processing a Series of Items with Iterators

- In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up
- Iterators handle all the index-0-increment-+1 logic for you, cutting down on repetitive code you could potentially mess up. Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you can index into, like vectors.

#### The `Iterator` Trait and the `next` method:

Note:

- we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.
  - In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator.
  - We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.
- The values we get from the calls to next are immutable references to the values in the vector.
  - The iter method produces an iterator over immutable references.
  - If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
  - Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter

#### Methods that consume the iterator

- Methods that call next are called consuming adaptors, because calling them uses up the iterator.
  - One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator. As it iterates through, it adds each item to a running total and returns the total when iteration is complete.

#### Methods that Produce Other Iterators

- Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change iterators into different kinds of iterators.
- You can chain multiple calls to iterator adaptors to perform complex actions in a readable way. But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.
- To consume the iterator, we’ll use the collect method. This method consumes the iterator and collects the resulting values into a collection data type.

### 13.4 Comparing Performance: Loops Vs Iterators

- iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself.
- Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.

### Summary:

## 14.

### 14.2

#### Exporting a Convenient Public API with `pub use`

Problem:

- The structure of your public API is a major consideration when publishing a crate.
- People who use your crate are less familiar with the structure than you are and might have difficulty finding the pieces they want to use if your crate has a large module hierarchy.

Solution: In cases where there are many nested modules, re-exporting the types at the top level with pub use can make a significant difference in the experience of people who use the crate.

## 15. Smart Pointers

- A pointer is a general concept for a variable that contains an address in memory.
- The most common kind of pointer in Rust is a reference. References are indicated by the & symbol and borrow the value they point to
- Smart pointers, on the other hand, are data structures that not only act like a pointer but also have additional metadata and capabilities.
  - In Rust, the different smart pointers defined in the standard library provide functionality beyond that provided by references.
  - Eg: `reference counting` smart pointer type -> enables you to have multiple owners of data by keeping track of the number of owners and, when no owners remain, cleaning up the data.
- An additional difference between references and smart pointers is that:
  - references are pointers that only borrow data;
  - in contrast, in many cases, smart pointers own the data they point to.
- Smart Pointers example: `String`, `Vec<T>`.
  - Both these types count as smart pointers because they own some memory and allow you to manipulate it.
  - They also have metadata (such as their capacity) and extra capabilities or guarantees.
- Smart pointers are usually implemented using structs:
  - The characteristic that distinguishes a smart pointer from an ordinary struct is that smart pointers implement the `Deref` and `Drop` traits.
    - The `Deref` trait allows an instance of the smart pointer struct to behave like a reference so you can write code that works with either references or smart pointers.
    - The `Drop` trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.
- Common Smart Ponters in std:
  - `Box<T>` for allocating values on the heap.
  - `Rc<T>`, a reference counting type that enables multiple ownership.
  - `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time.

### 15.1 Using `Box<T>` to Point to Data on the Heap

- Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.
- Use in:
  - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
  - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    - transferring ownership of a large amount of data can take a long time because the data is copied around on the stack.
    - To improve performance in this situation, we can store the large amount of data on the heap in a box.
    - Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap.
  - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

#### Enabling Recursive Types with Boxes

- At compile time, Rust needs to know how much space a type takes up.
- One type whose size can’t be known at compile time is a recursive type, where a value can have as part of itself another value of the same type.
- Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types.
- Traits:
  - The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values to be treated like references.
  - When a `Box<T>` value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the `Drop` trait implementation.

### 15.2 Treating Smart Pointers like regular references with the `Deref` trait

- Implementing the Deref trait allows you to customize the behavior of the dereference operator, \* (as opposed to the multiplication or glob operator).
- By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

#### Following the Pointer to the Value with teh Dereference Operator

- A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.
- However, if we want to make an assertion about the value in y, we have to use \*y to follow the reference to the value it’s pointing to (hence dereference). Once we dereference y, we have access to the integer value y is pointing to that we can compare with 5.
- Without the Deref trait, the compiler can only dereference & references. The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.
- Rust substitutes the \* operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method. This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements Deref.
- The reason the `deref` method returns a reference to a value, and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary, is the ownership system. If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self.

#### Implicit Deref Coercions with Functions & Methods

- Deref coercion is a convenience that Rust performs on arguments to functions and methods.
- Deref coercion converts a reference to a type that implements `Deref` into a reference to a type that Deref can convert the original type into.
  - Input: a reference to a type that implements `Deref`
  - Output: reference to a type that Deref can convert the original type into.
- Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.
- When the Deref trait is defined for the types involved, Rust will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter’s type. The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

### 15.3 Running Code on Cleanup with the `Drop` Trait

- Drop, which lets you customize what happens when a value is about to go out of scope.
- You can use code specified in a Drop trait implementation in many ways to make cleanup convenient and safe: for instance, you could use it to create your own memory allocator!

#### Dropping a Value Early with `std::mem::drop`

- Occasionally, however, you might want to clean up a value early.
- One example is when using smart pointers that manage locks: you might want to force the drop method that releases the lock to run so other code in the same scope can acquire the lock.
- Rust doesn’t let you call the Drop trait’s drop method manually; instead you have to call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.

### 15.4 `Rc<T>`, the Reference Counted Smart Pointer

- There are cases when a single value might have multiple owners.
- For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.
- To enable multiple ownership, Rust has a type called `Rc<T>`, which is an abbreviation for reference counting.
  - The `Rc<T>` type keeps track of the number of references to a value which determines whether or not a value is still in use.
  - If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
- We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last.
- `Rc<T>` is only for use in single-threaded scenarios, not for multithreaded programs.
- The implementation of `Rc::clone` doesn’t make a deep copy of all the data like most types’ implementations of clone do.
- The call to `Rc::clone` only increments the reference count, which doesn’t take much time.
  - Deep copies of data can take a lot of time. By using `Rc::clone` for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count.
  - When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to `Rc::clone`.

### 15.5 `RefCell<T>` and the Interior Mutability Pattern

- Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.
- To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.

#### A use case for Interior Mutability: Mock Objects

- A test double is the general programming concept for a type used in place of another type during testing.
- Mock objects are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.

This section went over my head + don't think it will be that important in the beginning of my Rust journey.

#### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`

- A common way to use `RefCell<T>` is in combination with `Rc<T>`
- Recall that `Rc<T`> lets you have multiple owners of some data, but it only gives immutable access to that data. If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have multiple owners and that you can mutate!

### 15.4 Reference Cycles Can Leak Memory

- Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak).
- Preventing memory leaks entirely is not one of Rust’s guarantees in the same way that disallowing data races at compile time is, meaning memory leaks are memory safe in Rust.
- Creating reference cycles is not easily done, but it’s not impossible either.
  - If you have `RefCell<T>`values that contain `Rc<T>` values or similar nested combinations of types with interior mutability and reference counting, you must ensure that you don’t create cycles; you can’t rely on Rust to catch them.
  - Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and other software development practices to minimize.

## 16. Fearless Concurrency

- By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors.
  - Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime concurrency bug occurs, incorrect code will refuse to compile and present an error explaining the problem.
  - As a result, you can fix your code while you’re working on it rather than potentially after it has been shipped to production.
- Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between threads.
- Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages, because a higher-level language promises benefits from giving up some control to gain abstractions.
- However, lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware. Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

### 16. 1 Using Threads to Run Code Simultaneously

- Programming language-provided threads are known as green threads, and languages that use these green threads will execute them in the context of a different number of operating system threads. For this reason, the green-threaded model is called the M:N model: there are M green threads per N operating system threads, where M and N are not necessarily the same number.
- We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn`in a variable.
  - The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the join method on it, will wait for its thread to finish.
  - Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates.
- The `move` closure is often used alongside `thread::spawn` because it allows you to use data from one thread in another thread.
  - By adding the `move` keyword before the closure, we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    - The `move` keyword overrides Rust’s conservative default of borrowing; it doesn’t let us violate the ownership rules.

### 16.2 Using Message Passing to Transfer Data Between Threads

- increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.
- “Do not communicate by sharing memory; instead, share memory by communicating.”
- One major tool Rust has for accomplishing message-sending concurrency is the channel, a programming concept that Rust’s standard library provides an implementation of.
  - A channel in programming has two halves: a transmitter and a receiver.
    - The transmitter half is the upstream location where you put rubber ducks into the river, and the receiver half is where the rubber duck ends up downstream.
  - One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages.
  - A channel is said to be closed if either the transmitter or receiver half is dropped.
  - We create a new channel using the `mpsc::channel` function; `mpsc` stands for multiple producer, single consumer.
  -

#### Channels and Ownership Transference

- The ownership rules play a vital role in message sending because they help you write safe, concurrent code
- Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs

### 16.3 Shared-State Concurrency

#### Using Mutexes to Allow Access to Data from One Thread at a Time

- Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
- To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.
- Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels. However, thanks to Rust’s type system and ownership rules, you can’t get locking and unlocking wrong.

#### Multiple Ownership with Multiple Threads

- Unfortunately, `Rc<T>` is not safe to share across threads.
  - When `Rc<T>` manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped.
  - But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread.
  - This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it.

#### Atomic Reference Counting with `Arc<T>`

- Fortunately, `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.
  - atomics work like primitive types but are safe to share across threads.
  - Why all primitive types aren’t atomic and why standard library types aren’t implemented to use `Arc<T>` by default? The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to.
    - If you’re just performing operations on values within a single thread, your code can run faster if it doesn’t have to enforce the guarantees atomics provide.

#### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

- `Mutex<T>` provides interior mutability.
- In the same way we used `RefCell<T>` to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>` to mutate contents inside an `Arc<T>`.
- Another detail to note is that Rust can’t protect you from all kinds of logic errors when you use `Mutex<T>`. `Mutex<T>`comes with the risk of creating deadlocks.

### 16.4 Extensible Concurrency with the `Sync` and `Send` Traits

#### Allowing Transference of Ownership Between Threads with Send

- The `Send` market trait indicates that ownership of the the type implementing `Send` can be transferred between threads.
- Almost every Rust type is `Send`, but there are some exceptions, including `Rc<T>`

#### Allowing Access from Multiple Threads with Sync

- The `Sync` marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
  - Any type T is Sync if &T (a reference to T) is Send, meaning the reference can be sent safely to another thread.
- primitive types are Sync, and types composed entirely of types that are Sync are also Sync.

## 17. Object oriented programming

### 17.1 Characteristics of Object-Oriented Languages

#### Inheritance as a Type System and as Code Sharing

- If a language must have inheritance to be an object-oriented language, then Rust is not one. There is no way to define a struct that inherits the parent struct’s fields and method implementations.
- You choose inheritance for two main reasons:
  - reuse of code:
    - can share Rust code using default trait method implementations.
  - to enable a child type to be used in the same places as the parent type. This is also called polymorphism, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.
- Inheritance has recently fallen out of favor as a programming design solution in many programming languages because it’s often at risk of sharing more code than necessary.
  - Subclasses shouldn’t always share all characteristics of their parent class but will do so with inheritance.
  - This can make a program’s design less flexible.
  - It also introduces the possibility of calling methods on subclasses that don’t make sense or that cause errors because the methods don’t apply to the subclass.

For these reasons, Rust takes a different approach, using trait objects instead of inheritance.

### 17.2 Using Trait Objects That Allow for Values of Different Types

- sometimes we want our library user to be able to extend the set of types that are valid in a particular situation.
- At the time of writing the library, we can’t know and define all the types other programmers might want to create.
- A trait object points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime.
- To create a trait object by specifying some sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the relevant trait.
- The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway.

#### Trait Objects Perform Dynamic Dispatch

- when we use trait bounds on generics: the compiler generates nongeneric implementations of functions and methods for each concrete type that we use in place of a generic type parameter.
  - The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
- This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.
- In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.
- When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that is using trait objects, so it doesn’t know which method implemented on which type to call.
  - Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call.
  - There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.

#### Object Safety Is Required for Trait Objects

- You can only make object-safe traits into trait objects. A trait is object safe if all the methods defined in the trait have the following properties:
  - The return type isn’t `Self`.
  - There are no generic type parameters.

## 18. Patterns & Matching

- The downside of using if let expressions is that the compiler doesn’t check exhaustiveness, whereas with match expressions it does. If we omitted the last else block and therefore missed handling some cases, the compiler would not alert us to the possible logic bug.

### 18.2 Refutability: Whether a Pattern Might Fail to Match

Patterns come in two forms:

- refutable and
  - Patterns that can fail to match for some possible value are `refutable`.
  - The if let and while let expressions only accept refutable patterns, because by definition they’re intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure.
- irrefutable:
  - Patterns that will match for any possible value passed are irrefutable.
  - eg: `let x = 5` x matches anything and therefore cannot fail to match.
  - Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.

do need to be familiar with the concept of refutability so you can respond when you see it in an error message. In those cases, you’ll need to change either the pattern or the construct you’re using the pattern with, depending on the intended behavior of the code.

## 19.

### 19.1 Unsafe Rust

- Unsafe Rust exists because, by nature, static analysis is conservative.
- When the compiler tries to determine whether or not code upholds the guarantees, it’s better for it to reject some valid programs rather than accept some invalid programs.
- The downside is that you use it at your own risk: if you use unsafe code incorrectly, problems due to memory unsafety, such as null pointer dereferencing, can occur.
- Unsafe Rust ability:
  - Dereference a raw pointer
  - Call an unsafe function or method
  - Access or modify a mutable static variable
  - Implement an unsafe trait

#### Dereferencing a Raw Pointer

- Different from references and smart pointers, raw pointers:
  - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
  - Aren’t guaranteed to point to valid memory
  - Are allowed to be null
  - Don’t implement any automatic cleanup
- we can create raw pointers in safe code, but we can’t dereference raw pointers
- With all of these dangers, why would you ever use raw pointers?

  - One major use case is when interfacing with C code,

#### Calling an Unsafe Fucntion or Method

- Usecase for Unsafe code:
  - Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice; it only knows that we’re borrowing from the same slice twice.
  - Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping, but Rust isn’t smart enough to know this.
  - When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.

#### Using `extern` Functions to Call External Code

- your Rust code might need to interact with code written in another language.
- Rust has a keyword, `extern`, that facilitates the creation and use of a Foreign Function Interface (FFI).
- An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

### Accessing or Modifying a Mutable Static Variable

- Rust does support global variables but can be problematic with Rust’s ownership rules.
- If two threads are accessing the same mutable global variable, it can cause a data race.
- In Rust, global variables are called static variables.
- Constants and immutable static variables might seem similar, but a subtle difference is that values in a static variable have a fixed address in memory.
  - Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used.

### 19.2 Advanced Traits

#### Specifying Placeholder Types in Trait Definitions with Associated Types

3
33
32
21
9
35
2:15 minutes

## 20

33
60
24
2 hours

## Todo:

Need more resources/practise to understand:

1. Rust's ownership in depth (specifically failure cases)
2. Lifetime paramaters
3. Traits implementation
