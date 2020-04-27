# Learn To Code With Rust and Baseball - Chapter 4 : De-serialization

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

## Important Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

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

We now have exactly two crates that we are depending on. Isahc to grab stuff from the network and Serde to convert it into a data format we can use.

## Declarative De-serialization

In order to use the data, we need to tell our program what the data should like. Serde will then handle all of the logic and magically convert the data into something we can use. We don't need to define everything we want - we only need to define the parts we need. We'll build up our structure piece by piece.
