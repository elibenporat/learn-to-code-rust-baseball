# Learn To Code With Rust and Baseball - Chapter 3 : Player Info

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

## Important Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## The Evolution from the Gameday API to the Stats API

Over a decade ago, MLBAM released the amazing gameday pitch by pitch data as a publicly available xml application-programmer interface (API). Recently, they added a notice on the XML files which reads as follows:

>NOTICE: This file is no longer actively supported. Please use the MLB Stats API [http://statsapi.mlb.com/docs/)](http://statsapi.mlb.com/docs/) as an alternative.  Copyright 2020 MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here [http://gdx.mlb.com/components/copyright.txt](http://gdx.mlb.com/components/copyright.txt)

The MLB Stats API is everything the old XML API was, but better. We'll be exploring this API in depth.

## Adding Functionality - Crates.io

Rust has a philosophy of a minimal standard library. This means that whereas other languages might have a lot of things built in to the language, Rust encourages the use of 3rd party libraries. This is mostly because things that are built into a language are hard to change, which makes them hard to improve. Thus, things are only built in once they've gone through a rigorous process.

If you need to do a task, always start with Rust's official package repository [https://crates.io](crates.io). In Rust, "crates" are packages. You can also use the [htpps://lib.rs](lib.rs) for more advanced searching of the repository.

## A Library for Getting Data from the Internet

There are a lot of popular web clients available. They include [reqwest](https://crates.io/crates/reqwest), [surf](https://crates.io/crates/surf) and [Isahc](https://crates.io/crates/isahc), as well as many others. Reqwest is the most popular of these, however, Isahc is easier to use, so we'll use it for now.

## Adding a Dependency / Library

Adding a library to your rust program is really easy. All we need to do is edit the Cargo.toml file. Simply look for this:

```toml
[dependencies]
```

Then simply add ```isahc = "0.9"``` to the line below.
That's it.

Your ```[dependencies]``` section should look like this:

```toml
[dependencies]
isahc = "0.9"
```

## The Player API

We're going to look at Mike Trout. You can view Mike Trout's info at [https://statsapi.mlb.com/api/v1/people/545361](https://statsapi.mlb.com/api/v1/people/545361). Firefox has a great JSON viewer, so I recommend opening the link in Firefox. You'll notice that the data has a very simle structure.

At the top level we have a ```copyright``` object and a ```people``` object. The people object contains a list of people. In this case, we've only asked for one People, so the list has a size of 1. The API can also give you a lot of other info, via "hydrations", which we will cover in a later chapter.

## Downloading the player file into our program

Open up the main.rs file in your text editor. Delete the "hello, baseball" line in the  We're going to add 4 lines of code between the ```{}``` after the  ```fn main()```:

```rust
    use isahc::prelude::*;

    let mut response = isahc::get("https://statsapi.mlb.com/api/v1/people/545361").unwrap();
    let mike_trout_bio = response.text().unwrap();

    println!("Mike Trout's Bio: {}", mike_trout_bio);
```

Now, go back to your terminal and type ```cargo run```.

You'll notice that this takes a little longer than our "Hello, baseball!" example from Chapter 2. Cargo is downloading Isahc, as well as its dependencies, and compiling it all for you. Easily using other people's software is one of the key features of Rust. In technical terms, this means that Rust **composes** extremely well.

The quality of these community developed and maintained crates is exceptional. Reliable, performant and easy to use.

Let's go through each line of code.

```rust
    use isahc::prelude::*;
```

This line pulls in all the functions that isahc defines in its ```prelude```. The ```::``` is a [scope resolution operator](https://en.wikipedia.org/wiki/Scope_resolution_operator) which means that ```prelude``` belongs to ```isahc```. You can think of this as a way to differentiate between similar names. For example, say we have two players named Mike. In order to differentiate the two, we'd call them ```piazza::mike``` and ```trout::mike```.

```rust
    let mut response = isahc::get("https://statsapi.mlb.com/api/v1/people/545361").unwrap();
```

We define a ```response``` variable, which needs to be mutable (mut is short for mutable). Mutable is a programmer's way of saying something can change. In Rust, variables are immutable (cannot be changed) by default; we must explicitly state which variables need to be mutable. This is one of the most important concepts in Rust, which we'll dig into later when we discuss unique vs shared access. If you are interested in further reading, I'll point you to [Matt Brubeck's post](https://limpet.net/mbrubeck/2019/02/07/rust-a-unique-perspective.html).

We use the ```get``` function to request a ```Response``` from the internet. Asking for data from the internet is an operation that might fail. Rust wants you to be very explicit about how to handle the potential failure since ```Error``` handling is very important. Any operation which might run into a problem, such as a network request, returns a ```Result``` type, which **wraps** the thing you wanted.

The ```get``` function returns a ```Result``` which is either a ```Response``` or an ```Error```. We can also say that ```Result``` wraps around the ```Response```. In a "proper" program, we'd explicitly handle the error. For now, we will simply ```.unwrap()``` the ```Response``` from the ```Result``` wrapper. We are telling Rust that we want the program to crash if it runs into a problem. This is a complex topic, so don't worry if it doesn't quite make sense yet. We'll run into this a LOT.

```rust
    let mike_trout_bio = response.text().unwrap();
```

We now use the ```response``` to make another request from the internet, asking for the ```Body``` of the document, which we'll read as text. This too could fail and returns a ```Result``` which we'll ```.unwrap()``` for now. We'll store the text in the ```mike_trout_bio``` variable.

```rust
    println!("Mike Trout's Bio: {}", mike_trout_bio);
```

Now that we have the text, we can print it out. We use the ```println``` macro to print a line. The ```{}``` curly braces are replaced by whatever comes after the comma. This is very similar to how Python does it.

## Summary

We got Mike Trout's bio from the MLB Stats API and printed it to our terminal.You should now see his entire bio printed in your terminal.
