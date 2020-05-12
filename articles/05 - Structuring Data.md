# Learn To Code With Rust and Baseball - Chapter 5 : Structuring Data

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization; we're going to expand on that here.

Now that our code is getting a little longer, we're including an "[Our Story So Far](#our-story-so-far)" section at the end, which will include a snapshot of all the code you need.

## Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## Expanding the Person Struct

Let's add a few fields to the Person ```struct```. Right now we only have the ```id``` field. Let's go and pull in a few more. We'll update the struct to look like this:

```rust
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
    birth_country: String,
}
```

You'll notice there's a new line there:

```rust
#[serde(rename_all="camelCase")]
```

This line is crucial. The data format that the API uses is JSON, which likes to separate words by capitalizing allTheWordsAfterTheFirstWord. This is called "camelCase". Rust demands that you split_up_words_with_underscores. This is called "snake_case". Snake case is a lot easier to read, since humans are used to splitting up words with spaces, rather than capital letters. This magical line tells SerDe to interpret all the source fields as camelCase and map them to their snake_case equivalents.

We also introduced a new type, the ```String``` type. We'll discuss ```String```s a lot as we go. For now, think of it as text that can be any length.

Flip to your terminal and do another ```cargo run```. You should see a few more fields in your print out.

## Enums = Short, Complete Lists

A lot of data are short, complete lists. You're either a Parent or a Child. Today is either Sunday, Monday, Tuesday, etc. For these types of data, we want to explicitly list all the possibilities in an exhaustive (complete) manner.

Listing all the possibilities, is referred to as **enumerating** all the possibilities. With the Person ```struct``` above, we had a situation where each Person had an id **AND** a name **AND** a birth city, etc. ```Struct```s are used to group together fields that are "AND"s. `Enum`s (short for enumerations) group together fields that you must pick precisely one of; they group together fields that are **OR**s. Today is Sunday OR Monday.

```Enum```s are best when we can enumerate all the variants. For example, a batter can bat "Left", "Right" or "Switch". The full, enumerated, list of possible values are one of those 3. By creating an ```enum```, we are encoding this into our type system. We'll now be able to leverage this knowledge later in our program. We could just encode it as text (aka a ```String```), like we did with the city and country fields. Hard coding it will help us down the line, which we'll cover in later chapters.

## The BatSideCode and BatSideDescription Enums

Let's enumerate the BatSideCode and BatSideDescription:

```rust
#[derive(Debug, Deserialize, Clone)]
enum BatSideCode {
    R,
    L,
    S,
}

#[derive(Debug, Deserialize, Clone)]
enum BatSideDescription {
    Right,
    Left,
    Switch,
}
```

We've now created two `enum`s that can only ever be 1 of 3 things. If we ever need to refer to one of the variants, we'll use the syntax `BatSideCode::R` or `BatSideCode::L`. This uses the same ```::``` syntax we discussed with the ```use``` statement.

Now that we can add a `BatSide` struct, which contains the two `enum`s we just created.

```rust
#[derive(Debug, Deserialize, Clone)]
struct BatSide {
    code: BatSideCode,
    description: BatSideDescription,
}
```

We created a new ```struct``` which now has the two ```enum```s as its fields. We can now add this to our Person ```struct```:

```rust
#[derive(Debug, Deserialize, Clone)]
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
    birth_country: String,
    bat_side: BatSide,
}
```

You'll notice that we now have a mixture of different types. We have some integers (`u32` and `u16`), as well as `String`s and the custom type we defined (`BatSide`).

We also added the `Clone` instruction to our `derive`. This will make our lives easier in future chapters.

Let's now change the `let mut response` line to look like this:

```rust
let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015").unwrap();
```

Let's do another ```cargo run```, you should now see two bios: Mike Trout's and Joey Votto's.

## Let's Make an Error

Part of the journey in Rust includes making mistakes, and having the compiler explain to you all the things you did wrong. This happens a lot early on, but subsides once you get the hang of things. First, let's run into the issue where SerDe expects a field, but can't find it. We'll get this error by adding Franmil Reyes to our query. Let's change the ```let mut response``` line again:

```rust
let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015,614177").unwrap();
```

Do another cargo run in your terminal and you should get a message like this:

```bash
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("missing field `birthStateProvince`", line: 146, column: 3)', src\main.rs:48:37
```

The Rust compiler is telling us that it ran into an Error because it was looking for the field `birthStateProvince` but couldn't find it. It also relates to Line 48, column 37 in our main.rs file (your specific location might be different). If you go to your browser and look at what is produced, you'll note that Franmil Reyes does not have any `birthStateProvince` field. Specifically, it is pointing to this line:

```rust
let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
```

Even more specifically, it is pointing to the `.unwrap()` part. When we unwrap a value from a `Result` `enum`, the program will crash and tell you where the program was. There are two ways we can fix this, we can fix the thing that caused the error, or we can add logic that will handle the error. For now, we'll just fix the error.

## The Option Enum

`Option` is a key concept in Rust. An `Option` is simply a way to state that something might not be there. It is either `Some`thing or `None`. In your `Person` struct, find the `birth_state_province: String,` line and change it to:

```rust
birth_state_province: Option<String>,
```

We wrapped the ```String``` in an ```Option``` in order to state that the item may or may not be there. The `<` and `>` are called "angle brackets" and are used to **wrap** one type around another type. This is why Rust uses the .unwrap() syntax to take something out of an ```Option```.

Cargo run and everything should now work. If you look closely at the printout, you'll see that Franmil Reyes has this in his bio: ```birth_state_province: None,```, which means that there was no value found for him.

## Summary

You should have some comfort with ```Struct```s and ```Enum```s. These are bedrock concepts, and you'll see them a lot going forward. Rust programs are built with ```Struct```s for grouping things together, and ```Enum```s for specifying that something can only ever be one item on a list.

We also skimmed the surface of things that can go wrong. Error handling is a very deep topic that we'll chip away at as we go.

## Our Story So Far

Cargo.toml:

```toml
[package]
name = "fangraphs-learn-to-code"
version = "0.1.0"
authors = ["Learn To Code <learn-to-code@fangraphs.com>"]
edition = "2018"

[dependencies]
isahc = "0.9"
serde = {version = "1.0", features = ["derive"]}
serde_json = "1"
```

src/main.rs:

```rust
fn main() {
    use isahc::prelude::*;
    use serde::Deserialize;

    let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015,614177").unwrap();
    let mike_trout_bio = response.text().unwrap();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<Person>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all="camelCase")]
    struct Person {
        id: u32,
        full_name: String,
        height: String,
        weight: u16,
        birth_date: String,
        mlb_debut_date: String,
        birth_city: String,
        birth_state_province: Option<String>,
        birth_country: String,
        bat_side: BatSide,
    }

    #[derive(Debug, Deserialize, Clone)]
    enum BatSideCode {
        R,
        L,
        S,
    }

    #[derive(Debug, Deserialize, Clone)]
    enum BatSideDescription {
        Right,
        Left,
        Switch,
    }

    #[derive(Debug, Deserialize, Clone)]
    struct BatSide {
        code: BatSideCode,
        description: BatSideDescription,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
    dbg!(bio_deserialized);
}
```

`cargo run` will output in your terminal:

```bash
Finished dev [unoptimized + debuginfo] target(s) in 1.82s
     Running `target\debug\fangraphs-learn-to-code.exe`
[src\main.rs:49] bio_deserialized = Players {
    people: [
        Person {
            id: 545361,
            full_name: "Mike Trout",
            height: "6\' 2\"",
            weight: 235,
            birth_date: "1991-08-07",
            mlb_debut_date: "2011-07-08",
            birth_city: "Vineland",
            birth_state_province: Some(
                "NJ",
            ),
            birth_country: "USA",
            bat_side: BatSide {
                code: R,
                description: Right,
            },
        },
        Person {
            id: 458015,
            full_name: "Joey Votto",
            height: "6\' 2\"",
            weight: 220,
            birth_date: "1983-09-10",
            mlb_debut_date: "2007-09-04",
            birth_city: "Toronto",
            birth_state_province: Some(
                "ON",
            ),
            birth_country: "Canada",
            bat_side: BatSide {
                code: L,
                description: Left,
            },
        },
        Person {
            id: 614177,
            full_name: "Franmil Reyes",
            height: "6\' 5\"",
            weight: 275,
            birth_date: "1995-07-07",
            mlb_debut_date: "2018-05-14",
            birth_city: "Palenque",
            birth_state_province: None,
            birth_country: "Dominican Republic",
            bat_side: BatSide {
                code: R,
                description: Right,
            },
        },
    ],
}
```
