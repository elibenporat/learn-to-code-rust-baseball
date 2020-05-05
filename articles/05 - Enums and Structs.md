# Learn To Code With Rust and Baseball - Chapter 4 : Enums and Structs

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization. We're going to dig much deeper here.

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

## The Country Enum

If we wanted to, we could list all the countries that have produced baseball players. Listing all the possibilities, is referred to as **enumerating** all the possibilities. With the Person ```struct``` above, we had a situation where each Person had an id **AND** a name **AND** a birth city, etc. ```Struct```s are used to group together fields that are "AND"s. ```Enum```s (short for enumerations) group together fields that you must pick precisely one of; they group together fields that are **OR**s.

```Enum```s are best when we can enumerate all the variants. For example, a batter can bat "Left", "Right" or "Switch". The full, enumerated, list of possible values are one of those 3. By creating an ```enum```, we are encoding this into our type system. We'll now be able to leverage this knowledge later in our program. We could just encode it as text (aka a ```String```), like we did with the city and country fields. Hard coding it will help us down the line, which we'll cover in later chapters.

Let's enumerate the BatSideCode and BatSideDescription:

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
```

We've now created two ```enum```s that can only ever be 1 of 3 things. If we ever need to refer to one of the variants, we'll use the syntax ```BatSideCode::R``` or ```BatSideDescription::Right```. This uses the same ```::``` syntax we discussed with the ```use``` statement.

Now that we can add a ```BatSide``` struct:

```rust
#[derive(Debug, Deserialize)]
struct BatSide {
    code: BatSideCode,
    description: BatSideDescription,
}
```

We created a new ```struct``` which now has the two ```enum```s as its fields. We can now add this to our Person ```struct```:

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
    bat_side: BatSide,
}
```

You'll notice that we now have a mixture of different types. We have some integers (```u32``` and ```u16```), as well as ```String```s and the custom type we defined (```BatSide```).

Let's now change the ```let mut response``` line to look like this:

```rust
let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015").unwrap();
```

Let's do another ```cargo run```; you should now see both Mike Trout and Joey Votto's bios.

## Let's Make an Error

Part of the beginners journey in Rust includes making mistakes, and having the compiler explain to you all the things you did wrong. This happens a lot early on, but subsides once you get the hang of things. First, let's run into the issue where SerDe expects a field, but can't find it. We'll get this error by adding Franmil Reyes to our query. Let's change the ```let mut response``` line again:

```rust
let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015,614177").unwrap();
```

Do another cargo run in your terminal and you should get a message like this:

> thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("missing field `birthStateProvince`", line: 146, column: 3)', src\main.rs:48:37

The Rust compiler is telling us that it ran into an Error because it was looking for the field ```birthStateProvince``` but couldn't find it. It also relates to Line 48, column 37 in our main.rs file (your specific location might be different). If you go to your browser and look at what is produced, you'll note that Franmil Reyes does not have any ```birthStateProvince``` field Specifically, it is pointing to this line:

```rust
let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();
```

Even more specifically, it is pointing to the ```.unwrap()``` part. When we unwrap a value from a ```Result``` ```enum```, the program will crash and tell you where the program was. There are two ways we can fix this, we can fix the thing that caused the error, or we can add logic that will handle the error. For now, we'll just fix the error.

## The Option Enum

```Option``` is a key concept in Rust. An ```Option``` is simply a way to state that something might not be there. It is either ```Some```thing or ```None```. In your ```Person``` struct, we'll find the ```birth_state_province: String,``` line and change it to:

```rust
birth_state_province: Option<String>,
```

We wrapped the ```String``` in an ```Option``` in order to state that the item may or may not be there. The ```<``` and ```>``` are called "angle brackets" and are used to wrap one type around another type. This is why Rust uses the .unwrap() syntax to 

Cargo run and everything should now work. If you look closely at the printout, you'll see that Franmil Reyes has this in his bio: ```birth_state_province: None,```. This means that there was no value found for him.

## Summary

You should have some comfort with ```Struct```s and ```Enum```s. Don't fret if it hasn't fully sunk in yet, these are bedrock concepts, so you'll see them a lot going forward. Everything in Rust revolves around ```Struct```s for grouping things together, and ```Enum```s for specifying that something can only ever be one item on a list.

We also skimmed the surface of things that can go wrong.
