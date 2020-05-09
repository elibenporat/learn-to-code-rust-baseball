# Learn To Code With Rust and Baseball - Chapter 4 : De-serialization

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

## Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## What are you going to build in this chapter?

Some people prefer to learn by first seeing the end result and than diving into the details. If you are one of them you can simply copy paste the code below to see the final output of this chapter.

Cargo.toml
```
[package]
name = "fangraphs-learn-to-code"
version = "0.1.0"
authors = ["you <you@gmail.com>"]
edition = "2018"

[dependencies]
isahc = "0.9"
serde = {version = "1.0", features = ["derive"]}
serde_json = "1"
```

src/main.rs
```rust
fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;

    let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<Person>,
    }

    #[derive(Debug, Deserialize)]
    enum Country {
        Canada,
        USA,
        #[serde(other)]
        Other,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all="camelCase")]
    struct Person {
        id: u32,
        full_name: String,
        height: String,
        weight: u16,
        birth_date: String,
        mlb_debut_date: String,
        birth_city: String,
        birth_state_province: String,
        birth_country: Country,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
    dbg!(bio_deserialized);
}
```

`cargo run` will output the following:

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/fangraphs-learn-to-code`
[src/main.rs:36] bio_deserialized = Players {
    people: [
        Person {
            id: 545361,
            full_name: "Mike Trout",
            height: "6\' 2\"",
            weight: 235,
            birth_date: "1991-08-07",
            mlb_debut_date: "2011-07-08",
            birth_city: "Vineland",
            birth_state_province: "NJ",
            birth_country: USA,
        },
        Person {
            id: 458015,
            full_name: "Joey Votto",
            height: "6\' 2\"",
            weight: 220,
            birth_date: "1983-09-10",
            mlb_debut_date: "2007-09-04",
            birth_city: "Toronto",
            birth_state_province: "ON",
            birth_country: Canada,
        },
    ],
}

```

## Serialization and De-serialization

The task of taking in data and bringing it into your program is called de-serialization. The process of taking data from your program and writing it to disk, is called serialization. If you are doing any (de)serializing in Rust, you most certainly want to use the [Serde](https://crates.io/crates/serde) crate. Serde is so fantastic, it borders on magical.

## Adding Serde as a Dependency

To add Serde to our program, we'll add ```serde = {version = "1.0", features = ["derive"]}``` to our ```[dependencies]``` section in our Cargo.toml file. We'll also need to add ```serde_json = "1"```. The section should look like this now:

```toml
[dependencies]
isahc = "0.9"
serde = {version = "1.0", features = ["derive"]}
serde_json = "1"
```

We now have exactly three crates that we are depending on. Isahc to grab stuff from the network and Serde + serde_json to convert it into a data format we can use.

## Declarative De-serialization

In order to use the data, we need to tell our program what the data looks like. Serde will then handle all of the logic and magically convert the data into something we can use. We don't need to define everything we want - we only need to define the parts we need. We'll build up our structure piece by piece.

## Structs

A ```struct``` is simply a type that you define yourself, which can group together other ```struct```s and types. You define a ```struct``` by writing something like this:

```rust
struct Baseball {
}
```

Specifically, we define it the keyword ```struct``` followed by the name of the ```struct```. The ```{``` ```}``` braces contain all the fields that the ```struct``` will have. In this super simple example, we've defined a ```struct``` with no fields.

We're going to build our ```struct```ure up piece by piece. If we look at the printout of Trout's bio in the terminal, we'll see there are two objects at the root (top-most) level. There's the "copyright" object which is the same for every player, as well as a "people" object.

The ```people``` object contains a list of people. We can tell that it's a list, since it begins with a square bracket ```[```. A curly brace ```{``` indicates a single object, rather than a list. In Rust, we call lists ```Vectors```, abbreviated to ```Vec```.

We'll need to define two ```struct```s, one for the parent object and one for the people object. We don't need to pull in the copyright notice into our ```struct```.

## The Players Struct

We'll define our Players ```struct``` as follows:

```rust
#[derive(Debug, Deserialize)]
struct Players {
    people: Vec<Person>,
}
```

We're calling it ```Players``` and not ```Player``` since the structure could have multiple players, we've simply only asked for one. Let's go through this line by line.

```rust
#[derive(Debug, Deserialize)]
```

The ```derive``` macro tells the Rust compiler what features the struct needs. In our case, we need the ability to print it out in our terminal (```debug```) as well as ```deserialize``` data to our struct. While we could theoretically enable everything under the sun for each struct by default, this would be extremely wasteful. In Python and JavaScript, every "object" you create, will come with a ton of stuff that you get "automatically". This is great, since you don't have to worry about any of that. However, it also means that you are forced to carry that around with you. Rust, will always force you to "opt-in" to most things that you want. Right now all we need are ```debug``` and ```deserialize```. We can easily add more later on.

```rust
struct Players {
    people: Vec<Person>,
}
```

This defines a ```struct``` which we've called ```Players```. Inside of ```Players``` we have a field called ```people```, which is a list of ```Person```s.

The ```Vec<Person>``` should be read as a Vector (list), whose items all have the type ```Person```. In a systems programming language, every list must always be a list of the exact same type. Put another way, it must be a list of things that all have the exact same structure. We haven't yet defined the ```Person``` struct, so let's go ahead and define that in the most minimal way possible:

```rust
#[derive(Debug, Deserialize)]
struct Person {
    id: u32,
}
```

We're leaving out pretty much all other data. We're defining a structure called ```Person``` that (for now) has only one field, a 32 bit unsigned (always positive) integer. We no longer need this:

```rust
println!("Mike Trout's Bio: {}", mike_trout_bio);
```

Go ahead and delete that line.

Now we are all ready for some serde magic:

```rust
let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
dbg!(bio_deserialized);
```

Once we've set up our struct, all we have to do is specify the type and everything else falls into place. There are 3 parts to it, let's break it down:

First, ```let bio_deserialized: Players``` is saying that we want a variable called ```bio_deserialized``` and we want it to have the structure of ```Players```.

Then, we used ```serde_json```'s ```from_str()``` function. We passed in the text stored in ```mike_trout_bio``` into the ```from_str()``` function. The ```&``` means we are passing a reference to the value, rather than the value itself. We'll get into references later; for now just think of it as we're **borrowing** the value, rather than taking ownership. The concepts of ownership and borrowing are fundamental to understanding Rust; we'll dive into them once we've built up enough concepts to wrap them around.

Lastly, since the de-serialization could fail, it returns a ```Result```, which is **either** the ```Players``` struct you wanted, or an ```Error```. We ```.unwrap()``` the ```Players``` from the ```Result``` and put the value into ```bio_deserialized```. If there was an ```Error```, our program will crash. Error handling is a rather large topic which we'll cover much later. For now, we'll just ```.unwrap()``` everything.

```rust
dbg!(bio_deserialized);
```

The last line simply prints out the debug value of ```bio_deserialized``` to our terminal.

Your code should now look like this:

```rust
fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;

    let mut response = isahc::get("https://statsapi.mlb.com/api/v1/people/545361").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<Person>,
    }

    #[derive(Debug, Deserialize)]
    struct Person {
        id: u32,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
    dbg!(bio_deserialized);
}
```

Go to your terminal and type ```cargo run```. You should see a very simple print out.

## Summary

We built up a very simple structure that allowed us to capture a player bio. In chapter 5, we'll expand on this.
