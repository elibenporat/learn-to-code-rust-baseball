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

Let's define a very basic Country enum:

```rust
#[derive(Debug, Deserialize)]
enum Country {
    Canada,
    USA,
    #[serde(other)]
    Other,
}
```

The ```#[serde(other)]``` line means that anything that doesn't match to Canada or USA will be put into the "Other" bucket. We could of course list out all the possible countries, however we're over-simplifying here in order to teach the concept.

Let's update our Person struct to include the new Country enum:

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
    birth_country: Country,
}
```

Let's also change the ```let mut response``` line to look like this:

```rust
let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015").unwrap();
```

Let's do another ```cargo run``` and you should now see both Mike Trout and Joey Votto's bio.
