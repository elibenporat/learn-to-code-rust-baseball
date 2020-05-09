# Learn To Code With Rust and Baseball - Chapter 6 : Shaping Data

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization. We're going to dig much deeper here.

Chapter 5 dug into `struct`s and `enum`s, and introduced the `Option` type.

In this chapter, we'll explore how to shape our data. This is a very powerful feature of rust. If you prefer to skip to the end, go to the [All The Code In This Chapter](#all-the-code-from-this-chapter) section.

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

The ```SideDescription``` adds an "Either" which is what they use to describe switch pitchers, such as Pat Vendite.

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

I've left out the similar Error for `BatSideDesciption`. The compiler is telling us that on Line 45, we said that **code** field should look for a `BatSideCode`, but it can't find it. It helpfully tells us that there is an `enum` that looks quite similar.

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

You'll also need to change the **bat_side** field from `bat_side: BatSide` to `bat_side: Side`.

The helpful compiler messages literally hold your hand if you ever need to make a lot of changes in your code. This is an *amazing* feature of Rust.

Do a cargo run and make sure everything works.

## Update Our Person Struct

We can now add **pitch_hand** to our `Person` by adding one line:

```rust
pitch_hand: Side,
```

Notice that it has the same structure as the **bat_side** field.

## Caputuring Data vs "Shaped"/"Final" Data

Our `Person` `struct` has a field called **bat_side** that has nested items, which is another way of saying that it is a field that has two fields inside of it. Later, when we output the data to CSV format, we'll need our `struct` to be "flat". Which means we'll want to take something that looks like this:

```rust
struct PersonCapture {
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

In other words, we want to pull out the `BatSideCode` and `BatSideDescription` from the **bat_side** field and create a flat structure. To do this, we'll need two structs - one for capturing/de-serializing the data, as well as one for our final output. We'll call them `PersonCapture` and `Person` respectively.

The `Person` struct we've been building is actually the capturing struct, rename that to `PersonCapture`.

## From PersonCapture Into Person


## All The Code From This Chapter
