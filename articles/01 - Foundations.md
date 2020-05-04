# Learn To Code With Rust and Baseball - Chapter 1 : Foundations

If you've ever wanted to learn to write code, but didn't know where to begin, we hope this series will offer you an opportunity to dip your proverbial toes into the coding world. Learning to code can appear from the outside like a monumental task; before I began my own journey on the Rust learning curve, I didn't quite think I had it in me.

Before we begin, I feel it is important to clarify the primary goals of this series. They are as follows:

* **Pedagogy**: Our primary goal is to empower the reader to build her own code bases. We'll drive towards that by focusing early on on foundational elements of computer science, while attempting to strike a fine balance detail and complexity. I'll do my best not to assume any pre-existing knowledge.
* **Inspiration**: Software engineering is unique in the sense that it doesn't require years of expensive education to enter. Anyone, at any age, can participate, as long as they have a computer with an internet connection. My personal hope is to inspire some of you to try your hand at coding.
* **Baseball Themed**: The hardest challenge will be keeping the content balanced between core concepts, while still keeping it baseball themed. This is a baseball site after all. This may not be possible for the first few articles, but I'll do my best.

It is important to note that I am not an expert in programming language theory nor computer science. These articles will undergo technical review before publishing, however any errata are my responsibility.

## Why Rust and not Python or R

The vast majority of job openings posted on this site list requirements that include Python or R. This begs the question, why wouldn't we write this series focusing on a language that is more "mainstream"?

We want to teach something more foundational, and orders of magnitude more powerful. It boils down to the difference between dynamic languages (such as Python, R and Javascript) and static languages such as Rust and C/C++. Using Rust as our primary language will allow us to explore fundamental computer science (CS) principles that are hidden in dynamic languages. These concepts are important to understand, even if you end up using a higher level language later on.

## Low Level vs High Level Programming Languages

What is a "low level" language as compared to a "high level" language? Roughly speaking, it boils down to how many layers of abstraction above the physical hardware the language lies. For this reason, you'll see systems programming languages such as Rust, C/C++, COBOL and Fortran, referred to as "close to the metal". The lower the level of the language, the more access to the physical hardware you have. The higher the level you go, the more the language itself makes decisions for you.

## Computer Memory

In order to do any work, the computer needs to store information in fast, short-term memory. Once the work is done, it must clean up that memory, making space for other work. Memory is limited in size, so this is really important! There are essentially two ways to clean up: the programmer tells the program when to clean up memory, or a "garbage collector" comes and cleans out the garbage memory from time to time.

## Trade-offs and the Cost/Benefit of Abstraction

Computer science is all about trade-offs. In baseball, you can swap in a better offensive player, but you sacrifice some defense. In computer science, up until Rust, you either had to manually manage all your memory (say in C/C++) or use a much slower, garbage collected language. Rust's major breakthrough was its ability to manage memory through static analysis. Static analysis simply means that it reads your code and figures out all the memory management needed when you build your program.

The trade-off that Rust makes is that it requires you to understand a lot, before being able to be proficient in it. However, once you get past the initial learning curve, you now have the best of all worlds, performance, reliability and productivity.

In other words, Rust is the Mike Trout of programming languages.

## Bits & Bytes

At the absolute lowest level, the computer only sees a stream of bits, 0s and 1s. A byte is simply 8 bits. Most modern computers have processors that work on either 32 bits (4 bytes), or 64 bits (8 bytes). This size is referred to as a machine "word". In Rust, this is referred to as a ```usize``` or an ```isize```. In Rust, if we want to define a variable (something that holds data) we'd do it like this:

```rust
let year: usize = 2020;
```

We are telling the computer that we have a variable called ```year``` that will be one machine word in size (32 bits or 64 bits depending on the machine) and we want to set the value to 2020. The semicolon at the end signifies the end of the line.

## What is a Type?

Types are the foundation of everything. They group together bits and imbue them with semantic meaning. A "type" is simply a way for us to tell the computer what a group of bits represent. Are the data integers? Are they text? True/false values? Can they be negative, or are they always positive?

## Dynamic Types vs Static Types

In a dynamic language, you don't see any types. However, if you ask javascript to compare the numeric 5 with the text '5', you get some interesting behaviour.

```javascript
5 ==  '5' \\ true
5 === '5' \\ false
```

What's happening in the background is that the ```==``` version converts the text type of 5 into the number type 5. After the conversion, they are indeed the same. In the ```===``` version, aka strict equality, no type conversion is done, so the values are not the same, as the underlying bits are very different.

When we give a variable a "type", we're telling the computer how to interpret the bits.

## Types in Rust

For a more in-depth look at Rust types, read [Chapter 3.2 of the Rust Book](https://doc.rust-lang.org/book/ch03-02-data-types.html).

Let's split our most basic types into two main buckets: number types (including boolean true/false) and string types (text). For numbers they can be one of 4 things:

* Signed integer: Positive or negative integer.
* Unsigned integer: Positive integer.
* Floating point number: Positive or negative number with decimal points.
* Booleans: 0 for false, 1 for true.

Integers are always aligned to power of 2 bytes, so they are either 8, 16, 32, 64 or 128 bits in length. We use ```u``` to signify unsigned (positive) or ```i``` for integers that may positive or negative. So a 64 bit positive-only (unsigned) integer will be a ```u64``` compared to the signed version ```i64```.

Say we were looking to describe the initial state of a plate appearance. It would look something like this:

```rust
// Balls, strikes and outs have a maximum value of 4, 3 and 3
// We'll leverage the smallest positive type: u8
let mut balls: u8 = 0;
let mut strikes: u8 = 0;
let mut outs: u8 = 0;

// Each base either has a runner or not, we'll use booleans (true/false) for this.
let mut runner_1st: bool = false;
let mut runner_2nd: bool = false;
let mut runner_3rd: bool = false;

// MLB player IDs are 6 decimal digits long, so we need u32 types that have a max value large enough.
// We could store it as text, but the number type will be more efficient.
let batter_id: u32 = 54361;   // Mike Trout
let pitcher_id: u32 = 477132; // Clayton Kershaw

// Say we wanted to track how many runs ahead or behind a team was.
// An i8 will go from -128 to 127, which should be a large enough range for us.
// In this case, our plate appearance starts with the batting team down by 4 runs.
let mut runs_ahead: i8 = -4;
```

The ```//``` are comments. The ```mut``` keyword tells rust that this value might change later in the code. If Rust doesn't find anywhere else that the variable can change, it will warn you. We'll discuss this in much greater detail later, as this touches upon the fundamental concepts of ownership and borrowing.

## Conclusion

We've scratched the surface of computer science and Rust. In Part 2, we'll get you up and running in Rust, with our very own "Hello Baseball" application.
