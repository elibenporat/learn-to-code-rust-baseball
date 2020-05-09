# Learn To Code With Rust and Baseball - Chapter 6 : Shaping Data

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization. We're going to dig much deeper here.

Chapter 5 dug into `struct`s and `enum`s, and introduced the `Option` type.

Today, we'll add more `Option`s to our `struct`ure and then flatten the data out. If you prefer to skip to the end, go to the [All The Code In This Chapter](#all-the-code-from-this-chapter) section.

## Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## Being Explicit About Things That COULD Go Wrong

Rust takes `Error` handling **very** seriously.

## All The Code From This Chapter
