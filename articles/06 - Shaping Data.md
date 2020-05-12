# Learn To Code With Rust and Baseball - Chapter 6 : Shaping Data

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization. We're going to dig much deeper here.

Chapter 5 dug into `struct`s and `enum`s, and introduced the `Option` type.

In this chapter, we'll explore how to shape our data. This is a very powerful feature of rust. If you prefer to skip to the end, go to the [Our Story So Far](#our-story-so-far) section at the bottom.

## Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## Adding Some Fields to Capture

We left out the **pitch_hand** field, so let's do a quick "refactor" (change our code) first.

The `BatSideCode` and `BatSideDescription` can be used for both the **bat_side** and the **pitch_hand** fields. Let's go ahead and change those two structs to make them more general.

Old version:

```rust
    #[derive(Debug, Deserialize)]
    enum BatSideCode {
        R,
        L,
        S,
    }

    #[derive(Debug, Deserialize)]
    enum BatSideDescription {
        Right,
        Left,
        Switch,
    }

    #[derive(Debug, Deserialize)]
    struct BatSide {
        code: SideCode,
        description: SideDescription,
    }
```

Revised version:

```rust
    #[derive(Debug, Deserialize)]
    enum SideCode {
        R,
        L,
        S,
    }

    #[derive(Debug, Deserialize)]
    enum SideDescription {
        Right,
        Left,
        Switch,
        Either,
    }

    #[derive(Debug, Deserialize)]
    struct Side {
        code: SideCode,
        description: SideDescription,
    }
```

We removed the word "Bat" from all of the fields. Also, the ```SideDescription``` adds an "Either" which is used to describe switch pitchers, such as Pat Venditte.

Do a quick Cargo Run. You should see the following error message:

```bash
error[E0412]: cannot find type `BatSideCode` in this scope
  --> src\main.rs:45:15
   |
29 | /     enum SideCode {
30 | |         R,
31 | |         L,
32 | |         S,
33 | |     }
   | |_____- similarly named enum `SideCode` defined here
...
45 |           code: BatSideCode,
   |                 ^^^^^^^^^^^ help: an enum with a similar name exists: `SideCode`
```

I've left out the similar Error for `BatSideDesciption`. The compiler is telling us that on Line 45, we said that the **code** field should look for a `BatSideCode`, but it can't find it. It helpfully tells us that there is an `enum` that looks quite similar.

Let's fix this by changing this:

```rust
#[derive(Debug, Deserialize)]
struct BatSide {
    code: BatSideCode,
    description: BatSideDescription,
}
```

To this:

```rust
#[derive(Debug, Deserialize)]
struct BatSide {
    code: SideCode,
    description: SideDescription,
}
```

You'll also need to change the **bat_side** field in the `Person` struct from `bat_side: BatSide` to `bat_side: Side`.

The helpful compiler messages literally hold your hand if you ever need to make a lot of changes to (refactor) your code. This is an *amazing* feature of Rust.

Do a cargo run and make sure everything works.

## Updating Our Person Struct

We can now add **pitch_hand** to our `Person` by adding one line:

```rust
pitch_hand: Side,
```

Notice that it has the same type as the **bat_side** field, the custom type `Side` that we created.

## Caputuring Data vs "Shaped"/"Final" Data

Our `Person` `struct` has a field called **bat_side** that has nested items, which is another way of saying that it is a field that has two fields inside of it. Later, when we output the data to CSV format, we'll need our `struct` to be "flat". Which means we'll want to take something that looks like this:

```rust
struct PersonTemp {
    id: u32,
    bat_side: Side,
}
```

and transform it into something that looks like this:

```rust
struct Person {
    id: u32,
    bat_side_code: SideCode,
    bat_side_description: SideDescription
}
```

In other words, we want to pull out the `BatSideCode` and `BatSideDescription` from the **bat_side** field and create a flat structure, with no nested items. To do this, we'll need two structs - one for capturing/de-serializing the data, as well as one for our final output. We'll call them `PersonTemp` and `Person` respectively.

The `Person` struct we've been building is actually the capturing struct, so let's go ahead and rename that to `PersonTemp`. You'll also need to change (for now) your `Players` struct as well:

```rust
#[derive(Debug, Deserialize)]
struct Players {
    people: Vec<PersonTemp>,
}
```

## Brief Overview of Traits

We're going to write code which tells Rust how to convert one type `From` another type. To do this, we need to describe how the type we created changes. In Rust, any "behaviour"s that apply to a type, are defined in `Trait`s. We'll be covering this in more detail later; for now think of `Trait`s as a way to describe what a type can do, rather than what it looks like.

## The From Trait

The Rust standard library defines a `From` Trait. Any type that wants to **implement** from/to behaviour needs to follow the format of the `From` Trait. From the example above, if we wanted to convert From `PersonTemp` Into `Person` we'd write something like this

```rust
impl From<PersonTemp> for Person {
    fn from( person_temp: PersonTemp ) -> Person {
        // Conversion logic goes here
    }
}
```

The `impl` keyword is a way of saying that we are **implementing** the `From` behaviour for our `Person` type, when we are converting `From` the `PersonTemp` struct.

The implementation contains a function called `from` which takes in a `PersonTemp` and outputs a `Person`. This enforces a **contract** that every time you make this conversion, you must give it a `PersonTemp` and you are **guaranteed** to get back a `Person`.

## Our Output Struct: Person

Before we can describe how to go From `PersonTemp` to Person, we'll need to define `Person`. It will look very similar to our original `Person` struct which we renamed to `PersonTemp`.

```rust
#[derive(Debug)]
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
    bat_side_code: SideCode,
    bat_side_description: SideDescription,
    pitch_hand_code: SideCode,
    pitch_hand_description: SideDescription,
}
```

There are two key differences. First, we don't need SerDe for this struct, so we only need `#[derive(Debug)]`. Second, we took the bat_side and pitch_hand fields and flattened them out. Now all we have to do is say how one converts to the other.

## From PersonTemp For Person

We're start by saying we're **implementing** the From PersonTemp for Person.

```rust
impl From<PersonTemp> for Person {
}
```

The From trait expects a function `from` that does the actual logic. More specifically, because we said its From PersonTemp for Person, we need a function that take a PersonTemp as input and produces a Person as output.
Let's add that function:

```rust
impl From<PersonTemp> for Person {
    fn from (person_temp: PersonTemp) -> Person {
        // Conversion code will go here
    }
}
```

Remember that `person_temp: PersonTemp` means that we're defining a variable `person_temp` which will have the type `PersonTemp`.

In order to "return" a `Person` as output, we'll add a `Person {}` block inside the function, with no `:` at the end. It will look like this:

```rust
impl From<PersonTemp> for Person {
    fn from (person_temp: PersonTemp) -> Person {
        Person {
            // Person definition goes here
        }
    }
}
```

Then we add in all the conversion logic:

```rust
impl From<PersonTemp> for Person {
    fn from (person_temp: PersonTemp) -> Person {
        Person {
            id: person_temp.id,
            full_name: person_temp.full_name,
            height: person_temp.height,
            weight: person_temp.weight,
            birth_date: person_temp.birth_date,
            mlb_debut_date: person_temp.mlb_debut_date,
            birth_city: person_temp.birth_city,
            birth_state_province: person_temp.birth_state_province,
            birth_country: person_temp.birth_country,
            bat_side_code: person_temp.bat_side.code,
            bat_side_description: person_temp.bat_side.description,
            pitch_hand_code: person_temp.pitch_hand.code,
            pitch_hand_description: person_temp.pitch_hand.description,
        }
    }
}
```

For the `id` field, we tell it to pull the `id` field from the struct we fed into the function. We then repeat this logic for most of the other fields.

When we get to the `bat_side_code` field, we tell it to look for the `code` field inside of the `bat_side` field and then repeat for the next 3 fields.

Delete this line:

```rust
dbg!(bio_deserialized);
```

And replace it with these lines:

```rust
let from = bio_deserialized.people[0].clone();
dbg!(&from);

let into: Person = from.into();
dbg!(&into);
```

Cargo run and you should see your initial PersonTemp as well as the Person it transformed into. Scroll down to the bottom of the page to see the full printout.

## Summary

This was a more technical chapter, where we touched upon Traits, and how to convert data from one shape to another. We'll be coming back to this topic again and again, so don't fret if it hasn't fully sunk in yet (it didn't for me until I was 6 months in).

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
        people: Vec<PersonTemp>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all="camelCase")]
    struct PersonTemp {
        id: u32,
        full_name: String,
        height: String,
        weight: u16,
        birth_date: String,
        mlb_debut_date: String,
        birth_city: String,
        birth_state_province: Option<String>,
        birth_country: String,
        bat_side: Side,
        pitch_hand: Side,
    }

    #[derive(Debug)]
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
        bat_side_code: SideCode,
        bat_side_description: SideDescription,
        pitch_hand_code: SideCode,
        pitch_hand_description: SideDescription,
    }

    impl From<PersonTemp> for Person {
        fn from (person_temp: PersonTemp) -> Person {
            Person {
                id: person_temp.id,
                full_name: person_temp.full_name,
                height: person_temp.height,
                weight: person_temp.weight,
                birth_date: person_temp.birth_date,
                mlb_debut_date: person_temp.mlb_debut_date,
                birth_city: person_temp.birth_city,
                birth_state_province: person_temp.birth_state_province,
                birth_country: person_temp.birth_country,
                bat_side_code: person_temp.bat_side.code,
                bat_side_description: person_temp.bat_side.description,
                pitch_hand_code: person_temp.pitch_hand.code,
                pitch_hand_description: person_temp.pitch_hand.description,
            }
        }
    }

    #[derive(Debug, Deserialize, Clone)]
    enum SideCode {
        R,
        L,
        S,
    }

    #[derive(Debug, Deserialize, Clone)]
    enum SideDescription {
        Right,
        Left,
        Switch,
        Either,
    }

    #[derive(Debug, Deserialize, Clone)]
    struct Side {
        code: SideCode,
        description: SideDescription,
    }

    let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();

    let from = bio_deserialized.people[0].clone();
    dbg!(&from);

    let into: Person = from.into();
    dbg!(&into);
}
```

`cargo run` will output in your terminal:

```bash
    Finished dev [unoptimized + debuginfo] target(s) in 1.95s
     Running `target\debug\fangraphs-learn-to-code.exe`
[src\main.rs:93] &from = PersonTemp {
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
    bat_side: Side {
        code: R,
        description: Right,
    },
    pitch_hand: Side {
        code: R,
        description: Right,
    },
}
[src\main.rs:96] &into = Person {
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
    bat_side_code: R,
    bat_side_description: Right,
    pitch_hand_code: R,
    pitch_hand_description: Right,
}
```
