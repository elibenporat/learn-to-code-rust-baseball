# Learn To Code With Rust and Baseball - Chapter 7 : Iterator Magic

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization. We're going to dig much deeper here.

Chapter 5 dug into `struct`s and `enum`s, and introduced the `Option` type.

Chapter 6 introduced how to convert our `PersonTemp` temporary struct into our final `Person` struct.

Chapter 7 introduced Iterators. We're going to continue with Iterators, and use them to calculate a few things.

## Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## Where We Left Off

We ended Chapter 7 off with a simple `Iterator` that processed our list of player IDs and converted them into a list of `Person`s. We'll now use this list and run some simple analytics on it.

## Filter

We can run **a lot** of different things (adapters) on an iterator, not just the `.map()` and `.collect()` adapters that we introduced. You can even write your own! Let's add another basic one, the `.filter()`.

Before we run the filter, we'll need to make sure the thing we're filtering on can be tested for equality. In Rust, any feature we want, we need to explicitly opt in to. We'll need to change our `SideCode` and `SideDescription` structs to automatically include `Eq` and `PartialEq`.

```rust
#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
enum SideCode {
    R,
    L,
    S,
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
enum SideDescription {
    Right,
    Left,
    Switch,
    Either,
}
```

Now we can create a simple iterator that counts the number of left handed hitters in our vector:

```rust
let lhh_persons = persons.iter()
    .filter(|person| person.bat_side_code == SideCode::L)
    .count();
```

Let's also change our `dbg!` line to:

```rust
dbg!(lhh_persons);
```

Cargo run should give you the following output:

```bash
 Finished dev [unoptimized + debuginfo] target(s) in 1.55s
     Running `target\debug\fangraphs-learn-to-code.exe`
[src\main.rs:38] lhh_persons = 24
```

We won't actually need this, go ahead and delete the `let lhh_persons` iterator lines as well as the `dbg!(lhh_persons);` lines.

## The Relationship Between Height And Weight

There is a misconception out there that Rust is lacking when it comes to data science. In my opinion, this couldn't be farther from the truth. We're going to create a multiple linera regression model, all in pure Rust.

First, we'll need to convert the height from the text format that looks like `6'2"` into inches.

Let's create a function which takes in a `String` and returns a machine word (`usize`). This function will look rather complex at first, but we'll break it down piece by piece.

```rust
fn to_inches (h: String) -> usize {
    h.split("'").into_iter()
        .map(|h| h.replace('"',"").trim().parse().unwrap_or(0))
        .enumerate()
        .map(|(n, h)| h * 12usize.pow(1-n as u32))
        .sum()
}
```

Let's go through it line by line.

```rust
fn to_inches (h: String) -> usize {
```

This is our function signature. It takes a `String` which we'll call `h`.

```rust
h.split("'").into_iter()
```

We then take that `String` and split it into separate `String`s, before and after the `'`. Then we turn it into an iterator.

```rust
.map(|h| h.replace('"',"").trim().parse().unwrap_or(0))
```

This runs a `.map()` on each of the parts to perform the following steps:

* `Replace` any `"` with a blank (basically delete)
* `Trim` (remove) any whitespace
* Attempt to `parse` (convert) into a number.
* `Unwrap` the `parse` attempt, if unsuccesful just set it to 0

```rust
.enumerate()
```

This changes our iterator so that each item is enumerated (0, 1, 2 etc.). We need this since we want to mutiply the first number by 12 and the second number by 1. We'll do this with a little math, since 12<sup>1</sup> = 12 and 12<sup>0</sup> = 1.

```rust
.map(|(n, h)| h * 12usize.pow(1-n as u32))
```

This part is a little technical. After the first `.map()` our `String` slices were `parse`d into `usize`s. Then we enumerated it so that we created a list that looks something like this:

(0, 6)\
(1, 2)

To capture this, we call the first number `n` and the second number `h`. After that it's just math. `12usize` is simply because the compiler wants the 12 to be the same type as the `h`. If this feels weird, it should. It takes some getting used to.

## Let's Prepare our Data to Run a Regression Model

We'll need to create three separate vectors, one each for the height, weight and id. Let's start with the height first:

### The Height Vector

```rust
let height: Vec<f64> = persons.iter()
    .filter(|person| person.height.is_some() && person.weight.is_some())
    .map(|person| to_inches(person.height.clone().unwrap()) as f64)
    .collect();
```

We're creating a vector of 64 bit floating point numbers (`f64`s). We're going to `borrow` the `persons` vector we created earlier. If we were going to take ownership of it, we'd use `into_iter`.

The `is_some()` is a function that tests whether an `Option` has `Some`thing or is `None`. In our case, we want to make sure the `person` has both a height and a weight. We've stated that id is always there, so we don't need to check for that.

Since we're only `borrow`ing the list, we have to make a clone (sort of like a copy) of the height string, we do this with the `.clone().`. Finally, we collect it all into a vector called 'height'.

### The Weight Vector

```rust
let weight: Vec<f64> = persons.iter()
    .filter(|person| person.height.is_some() && person.weight.is_some())
    .map(|person| person.weight.unwrap() as f64)
    .collect();
```

This code is quite similar to the height vector, except we didn't need to process the height with our `to_height` function. Also, we didn't need to `.clone()` the value. This is because integers implement `Copy`, which is a fancy way of saying, it's really quick and easy to copy it, so Rust just does it for you in the background, without you asking. We're skimming the surface here of borrowing and ownership only. These concepts won't fully make sense until much later.

### The ID Vector

```rust
let ids: Vec<f64> = persons.iter()
    .filter(|person| person.height.is_some() && person.weight.is_some())
    .map(|person| person.id as f64)
    .collect();
```

## Summary

We explored iterators and used them to take one vector and create 3 different vectors. We also used an iterator to convert a height `String` into a number. In Chapter 9, we'll build a multiple linear regression model using these data!

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

    let players: Vec<u32> = vec![
        400124, 400180, 425844, 434564, 435043, 435261,
        435638, 444541, 444886, 446359, 451506, 474029,
        476127, 488722, 501660, 501785, 501922, 502083,
        502154, 518963, 519412, 534584, 534708, 534730,
        543041, 543044, 543059, 543211, 543809, 543819,
        543936, 544253, 545346, 554430, 572227, 581662,
        581683, 594943, 595025, 600350, 605148, 605200,
        605530, 605543, 607389, 608339, 621107, 621545,
        623698, 623967, 626925, 630242, 641470, 641558,
        642066, 643299, 643327, 643335, 650638, 656643,
        657020, 657446, 661457, 661521, 656716, 661827,
        663672, 663749, 664215, 664686, 661841, 666923,
        668676, 668678, 669174, 669342, 667493, 669733,
        673863, 676422, 676646, 676913, 670097, 685358,
        689787,
    ];

    fn to_person (text: &str) -> Person {
        let person_temp: Players = serde_json::from_str(text).unwrap();
        person_temp.people[0].clone().into()
    }

    let persons: Vec<Person> = players.into_iter()
        .map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}", player))
        .map(|url| isahc::get(url).unwrap().text().unwrap())
        .map(|text| to_person(&text) )
        .collect();

    fn to_inches (h: String) -> usize {
        h.split("'").into_iter()
            .map(|h| h.replace('"',"").trim().parse().unwrap_or(0))
            .enumerate()
            .map(|(n, h)| h * 12usize.pow(1-n as u32))
            .sum()
    }

    let height: Vec<f64> = persons.iter()
        .filter(|person| person.height.is_some() && person.weight.is_some())
        .map(|person| to_inches(person.height.clone().unwrap()) as f64)
        .collect();

    let weight: Vec<f64> = persons.iter()
        .filter(|person| person.height.is_some() && person.weight.is_some())
        .map(|person| person.weight.unwrap() as f64)
        .collect();

    let ids: Vec<f64> = persons.iter()
        .filter(|person| person.height.is_some() && person.weight.is_some())
        .map(|person| person.id as f64)
        .collect();

    #[derive(Debug, Deserialize)]
    struct Players {
        people: Vec<PersonTemp>,
    }

    #[derive(Debug, Deserialize, Clone)]
    #[serde(rename_all="camelCase")]
    struct PersonTemp {
        id: u32,
        full_name: String,
        height: Option<String>,
        weight: Option<u16>,
        birth_date: Option<String>,
        mlb_debut_date: Option<String>,
        birth_city: Option<String>,
        birth_state_province: Option<String>,
        birth_country: Option<String>,
        bat_side: Side,
        pitch_hand: Side,
    }

    #[derive(Debug)]
    struct Person {
        id: u32,
        full_name: String,
        height: Option<String>,
        weight: Option<u16>,
        birth_date: Option<String>,
        mlb_debut_date: Option<String>,
        birth_city: Option<String>,
        birth_state_province: Option<String>,
        birth_country: Option<String>,
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

    #[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
    enum SideCode {
        R,
        L,
        S,
    }

    #[derive(Debug, Deserialize, Clone, Eq, PartialEq)]
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
}
```

We don't have any debug output right now, so run `cargo check` to make sure that your code can compile.
