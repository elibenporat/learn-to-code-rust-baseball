# Learn To Code With Rust and Baseball - Chapter 7 : Houston, We Have Problems

## Review

In Chapters 1 & 2 we discussed the motivation for this series and got our very first "Hello, baseball" application working. Each chapter builds upon the previous chapters. You'll need a working Rust installation, along with a good text editor. I recommend the excellent (and free) Visual Studio Code.

Chapter 3 covered the basics of downloading a file from the internet. We used the excellent [Isahc](https://crates.io/crates/isahc) crate for this and pulled Mike Trout's bio.

Chapter 4 covered the very basics of de-serialization. We're going to dig much deeper here.

Chapter 5 dug into `struct`s and `enum`s, and introduced the `Option` type.

Chapter 6 showed how to split up captured/de-serialized data and shape it to the format we want.

## Copyright Notice

The data we are going to leverage are all copyright MLB, subject to the [copyright notice](http://gdx.mlb.com/components/copyright.txt) by MLBAM. Neither the author, nor this series, are affiliated with Major League Baseball. We will be using the data in a non-commercial, non-bulk, manner, for educational purposes only.

## Being Explicit About Things That COULD Go Wrong

Rust takes `Error` handling **very** seriously. Potential errors are all around you; in most languages, you are free to "ignore" all of these. What that means in practice is that the error is being handled for you, but you have no control over how that error is handled. So far, we've handled all our potential errors by using `.unwrap()` which assumes everything is good, or will just crash (`panic` in Rust terminology) if it isn't.

As discussed in Chapter 5, anything that could be empty/missing, we wrap in an `Option`. So if we know that a field may or may not exist, instead of giving it a type `String`, we'll give it an `Option<String>` type, which you should read as `Option` of `String`.

Let's make our `Person` struct more complete, and wrap all the potentially empty fields in `Option`s. From practice, I can tell you that the only fields that **don't** need `Option`s are the **id** and **name** fields.
