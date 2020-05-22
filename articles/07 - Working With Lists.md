# Learn To Code With Rust and Baseball - Chapter 7 : Working with Lists

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization. We're going to dig much deeper here.

Chapter 5 dug into `struct`s and `enum`s, and introduced the `Option` type.

Chapter 6 introduced how to convert our `PersonTemp` temporary struct into our final `Person` struct. This was a more advanced chapter. You'll need to have the code up-do-date to continue following along, but we'll be reviewing these concepts going forward.

In this chapter, we're going to process our data, using iterators.

## Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## Working With Lists / Vectors / Arrays

Lists of things in Rust are either `array`s or `vector`s. The primary difference being that `array`s must have a set size when you build your program, whereas a `vector` can grow or shrink based on your what's happening while your code is running. Working with `vector`s is a **lot** easier, so we'll stick with `vector`s for the time being.

## All The Things In The Vector (List) Must Be The Same Size

If you've done some Python or JavaScript, you'll be used to being able to store anything in a list. The first item can be a number, the second a date and the third someone's name. In Rust (and other system languages), each item must be the same. That means you can have a list of integers, or a list of dates, but not a list that contains both. You could have a list of `enum`s, which in turn could be a date or an integer, but the `enum`s all need to be the same `enum`s.

## Creating a Vector

One way to create a `vector` in Rust goes like this:

```rust
let list_of_players : Vec<u32> = vec![545361,458015,614177];
```

First we say `let list_of_players` to name our list. Then we say `: Vec<u32>` to specify that it is a list of 32 bit unsigned (always positive) integers. If we wrote `: Vec<String>` we'd be creating a list of `String`s instead.

Then we feed it a list, using the `vec!` macro, with all the values we want separated by commas. Let's have some fun and create a list of all the batters that have first names that start with Z that have played in the majors (regular or spring training) since 2005. Copy-paste the following code into your main.rs:

```rust
let players: Vec<u32> = vec![
    400124, 400180, 425844, 434564, 435043, 435261,
    435638, 444541, 444886, 446359, 451506, 474029,
    476127, 488722, 501660, 501785, 501922, 502083,
    502154, 518963, 519412, 534584, 534708, 534730,
    543041, 543044, 543059, 543211, 543809, 543819,
    543936, 544253, 545346, 554430, 572227, 581662,
    581683, 584871, 594943, 595025, 600350, 605148,
    605200, 605530, 605543, 607389, 608339, 621107,
    621545, 623698, 623967, 626925, 630242, 641470,
    641558, 642066, 643299, 643327, 643335, 650638,
    656643, 656716, 657020, 657446, 661457, 661521,
    661827, 661841, 663672, 663749, 664215, 664686,
    666923, 667493, 668676, 668678, 669174, 669342,
    669733, 670097, 673863, 676422, 676646, 676913,
    685358, 689787,
];
```

You'll notice that we can break it into as many or few lines as we want. We can also leave a trailing comma after our last item.

Delete all of the following lines, we don't need them anymore:

```rust
let mut response = isahc::get("http://statsapi.mlb.com/api/v1/people/?personIds=545361,458015,614177").unwrap();
let mike_trout_bio = response.text().unwrap();

let bio_deserialized: Players = serde_json::from_str(&mike_trout_bio).unwrap();

let from = bio_deserialized.people[0].clone();
dbg!(&from);

let into: Person = from.into();
dbg!(&into);
```

It's every line that isn't a `struct` or `enum` definition, or the `use` declarations at the top. If you delete something you shouldn't have, you can always undo.

## Iterators Are AMAZING

Iterators are flat out amazing. Once you get the hang of it, you'll have mastered one of the most powerful tools a programmer can have. Computers are really good at doing the same thing over and over again. Iterators do exactly that - they're a way of saying "do all of these steps to all of these items".

Iterators are such a fundamental concept that we'll spend quite a bit of time on them. Let's start by turning our Vec of `players` into a Vec of urls. Here's the entire code to do that:

```rust
let urls: Vec<String> = players.into_iter()
    .map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}", player))
    .collect();
```

Let's break this down.

```rust
let urls: Vec<String>
```

We declare that we want a new vector of `String`s.

```rust
let urls: Vec<String> = players.into_iter()
```

We want that vector to be built out of the players vector. To do that we use `into_iter()` to turn it into an interator.

```rust
.map()
.collect()
```

The `.map()` applies logic to every item in the list and `.collect()` gathers them into the `Vec<String>` that you defined at the start.

## Map

Let's dig into the .map() part of the iterator. Map allows you to map the thing in your list to something else.

```rust
.map(|player|)
```

This takes the item from the list and gives it the temporay name `player`. If we wrote `.map(|xyz|)` it would do the exact same thing, but the name would be `xyz` instead of `player`.

```rust
.map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}", player))
```

This instructs the iterator to go through every item, call it player and apply the `format!` macro to each of the players in the list. The `format!` macro is a shortcut for creating text. It replaces the `{}` with the `player` id from our list.

Add a debug below:

```rust
dbg!(urls);
```

Cargo run should give you the following output:

```bash
[src\main.rs:27] urls = [
    "http://statsapi.mlb.com/api/v1/people/400124",
    "http://statsapi.mlb.com/api/v1/people/400180",
    "http://statsapi.mlb.com/api/v1/people/425844",
    "http://statsapi.mlb.com/api/v1/people/434564",
    ...
```

In 3 lines of code, we turned a list of numbers into a list of urls.

## Using the URLS

Let's add one line to download all those files:

```rust
let urls: Vec<String> = players.into_iter()
    .map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}", player))
    .map(|url| isahc::get(url).unwrap().text().unwrap())
    .collect();
```

After we map our player ids to urls, we take those URLs and download the data from those urls.

Change the debug line to this:

```rust
dbg!(&urls[0]);
```

This should give you a printout that looks like this:

```bash
Running `target\debug\fangraphs-learn-to-code.exe`
[src\main.rs:28] &urls[0] = "{\n  \"copyright\" : \"Copyright 2020 MLB Advanced Media, L.P.  Use of any content on this page acknowledges agreement to the terms posted here http://gdx.mlb.com/components/copyright.txt\",\n  \"people\" : [ {\n    \"id\" : 400124,\n    \"fullName\" : \"Zach Sorensen\",\n    \"link\" : \"/api/v1/people/400124\",\n    \"firstName\" : \"Zach\",\n    \"lastName\" : \"Sorensen\",\n    \"birthDate\" : \"1977-01-03\",\n    \"currentAge\" : 43,\n    \"birthCity\" : \"Salt Lake City\",\n    \"birthStateProvince\" : \"UT\",\n    \"birthCountry\" : \"USA\",\n    \"height\" : \"6\' 0\\\"\",\n    \"weight\" : 190,\n    \"active\" : false,\n    \"primaryPosition\" : {\n      \"code\" : \"4\",\n      \"name\" : \"Second Base\",\n      \"type\" : \"Infielder\",\n      \"abbreviation\" : \"2B\"\n    },\n    \"useName\" : \"Zach\",\n    \"middleName\" : \"Hart\",\n    \"boxscoreName\" : \"Sorensen\",\n    \"gender\" : \"M\",\n    \"isPlayer\" : true,\n    \"isVerified\" : true,\n    \"draftYear\" : 1998,\n    \"lastPlayedDate\" : \"2005-10-02\",\n    \"mlbDebutDate\" : \"2003-06-03\",\n    \"batSide\" : {\n      \"code\" : \"S\",\n      \"description\" : \"Switch\"\n    },\n    \"pitchHand\" : {\n      \"code\" : \"R\",\n      \"description\" : \"Right\"\n    },\n    \"nameFirstLast\" : \"Zach Sorensen\",\n    \"nameSlug\" : \"zach-sorensen-400124\",\n    \"firstLastName\" : \"Zach Sorensen\",\n    \"lastFirstName\" : \"Sorensen, Zach\",\n    \"lastInitName\" : \"Sorensen, Z\",\n    \"initLastName\" : \"Z Sorensen\",\n    \"fullFMLName\" : \"Zach Hart Sorensen\",\n    \"fullLFMName\" : \"Sorensen, Zach Hart\",\n    \"strikeZoneTop\" : 3.371,\n    \"strikeZoneBottom\" : 1.535\n  } ]\n}"
```

This is the raw text that we pulled when we downloaded the data.

## Convert The Text to A Person

We want all of the text we download to be magically converted to a Person. Let's create a function that does that for us:

```rust
fn to_person (text: &str) -> Person {
    let person_temp: Players = serde_json::from_str(text).unwrap();
    person_temp.people[0].clone().into()
}
```

This function takes the text we feed it (`text: &str`) and gives us back a `Person` struct (`-> Person`). We follow the same method we did at the end of chapter 6, but now have "abstracted" that logic into a function called `to_person`. We can now use that function in a `.map()`!. It's not important to understand the function completely, it's important to understand how to use the function in an iterator.

You can put this function before or after the iterator, it doesn't matter.

## Updating our Iterator

We now want our vector to be a list of Persons. So let's change our code a little and add one line:

```rust
let persons: Vec<Person> = players.into_iter()
    .map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}", player))
    .map(|url| isahc::get(url).unwrap().text().unwrap())
    .map(|text| to_person(&text) )
    .collect();

dbg!(&persons[0]);
```

We changed the name to `persons` and the list to `Vec<Person>` instead of `Vec<string>`. We also added this line:

```rust
.map(|text| to_person(&text) )
```

This mapped the text to a Person. If you look at the `to_person` function you see that it has a `()` and an `->` in its *signature*. The brackets tell it what it takes as input and the `->` specifies what it returns as output. In other words, it maps the input to the output.

Do a cargo run and you should get a big fat error:

```bash
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error("missing field `mlbDebutDate`", line: 48, column: 3)', src\main.rs:24:36
```

## Remember Options?

If you go back to Chapter 5, you'll remember that we had this same issue with `birth_state_province`. Specifically, any field which might be missing, we need to wrap in an `Option`. Let's go ahead and update both our `PersonTemp` (used to capture the data) and our `Person` to reflect this. We'll keep the `bat_side` and `pitch_hand` fields alone for now, as converting from them is slightly more complicated.

```rust
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
```

Cargo Run should give you a nice printout of Zach Sorenson's bio.

## Putting It All Together

The heart of our code is a simple iterator that does things step by step.

```rust
let persons: Vec<Person> = players.into_iter()
    // Take the players list, and turn it into an iterator
    .map(|player| format!("http://statsapi.mlb.com/api/v1/people/{}", player))
    // Take every player in the the list, and convert it into text that looks like a url
    .map(|url| isahc::get(url).unwrap().text().unwrap())
    // Download the url
    .map(|text| to_person(&text) )
    // Take the downloaded url text and convert it into a Person
    .collect();
    // Collect all the Persons into the Vec<Person> into the persons variable we called at the top.
```

Abstraction, in a programming context, means putting all the complicated stuff behind the scenes. This could mean that another library is handling it (such as Isahc for downloading data), or we've created our own function to do it (our `to_person` function). The end result is a 5 line iterator which turns our original list of player ids into a list of `Person`s.

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

    dbg!(&persons[0]);


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


}
```

`cargo run` will output in your terminal:

```bash
 Finished dev [unoptimized + debuginfo] target(s) in 2.83s
     Running `target\debug\fangraphs-learn-to-code.exe`
[src\main.rs:34] &persons[0] = Person {
    id: 400124,
    full_name: "Zach Sorensen",
    height: Some(
        "6\' 0\"",
    ),
    weight: Some(
        190,
    ),
    birth_date: Some(
        "1977-01-03",
    ),
    mlb_debut_date: Some(
        "2003-06-03",
    ),
    birth_city: Some(
        "Salt Lake City",
    ),
    birth_state_province: Some(
        "UT",
    ),
    birth_country: Some(
        "USA",
    ),
    bat_side_code: S,
    bat_side_description: Switch,
    pitch_hand_code: R,
    pitch_hand_description: Right,
}
```
