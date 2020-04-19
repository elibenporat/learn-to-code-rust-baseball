# Learn To Code With Rust and Baseball - Chapter 2 : Hello, Baseball

## Part 1 Review

In part 1, we touched upon types and explained our motivation for using Rust as our pedagogical tool of choice. We'll review and expand on concepts as we go, so don't worry if there are parts that don't make sense yet. We're going to attempt to strike a fine balance between depth and consicion.

## Hello Baseball

It is traditional, in the programming world, to create the most simple program possible in a given language or platform. This is usually done by creating an application that simply prints the words "Hello, World!". In our case, we'll do the same thing, however, we'll print "Hello, Baseball!" instead. This will be our entire goal today. At the end of this article, you should have a working Rust application that prints out those two words.

## Installing Rust

Rust is free, open source, software (FOSS). Most of the contibutions to Rust are from volunteers who use the language. In many ways, the spirit of FanGraphs mirrors that of the FOSS community. There is a common mission to make all tools and resources available to everyone, equally, regardless of their ability to pay. For FanGraphs, those who can, are encouraged to purchase a membership; in FOSS, the expectation is usually geared towards contributing back to the project in some way.

To install Tust visit [the rust website](https://www.rust-lang.org/) and click on the "Get Started" button. It will figure out which operating system your using and give you an option to download Rustup in either the 32 bit or 64 bit version. Choose the 64 bit version and install it on your system. Installation is straightforward and should run smoothly on all systems. If you run into issues, please leave a comment and I'll try to walk you through it.

## Installing Visual Studio Code

You can write Rust code in any text editor, even Notepad, should you so desire. However, this would not be a very pleasant experience. Instead, we're going to use the free Visual Studio Code that Microsoft maintains. You can download VS Code easily from the [VS Code website](https://code.visualstudio.com/). This installation should also be straightforward. Let me know if you run into any issues.

## Setting up Visual Studio Code

One of the many neat features with VS Code, is the fantastic plugin system. Once you've got VS Code installed, you should see 4 icons going down the top left of the screen. When you hover over the icons, and find the one that says "extensions". On Windows, you can also use the keyboard shortcut ctrl+shift+X.

Search for rust-analyzer published by matklad. Rust analyzer will assist in checking our code, giving type hints and a lot of other things that will be invaluable. It is under heavy development and improves weekly.

Search for Better TOML published by bungcip. TOML is used by Rust's package manager.

VS code isn't Rust specific. There are plugins for pretty much every language out there. For example, the Python plugin has been downloaded about 18.8M times.

That's all we need to do do get started. You can play around with themes to change the look and feel of the integrated developer environment (IDE). I'll leave such exploration to the reader.

## Creating Our Very First Application

In order to get the most out of this series, we recommend following each step and making sure you arrive at the same checkpoints. Every step will build on the earlier foundations.

### Step 1: Open a Terminal in VS Code

In the menu bar at the top, got the Terminal menu and select New Terminal. This should open up a terminal at the bottom of the screen. You can also use the command prompt. I've only ever used Windows, so if the interface is different on MacOs or Linux, I'm not really able to help there. I don't think it is materially different (other than perhaps keyboard shortcuts).

Your terminal should give you a prompt which is in your computer's default directory. For now, for the sake of simplicity, we'll simply put our Rust projects wherever the Terminal decides to default to. If you are familiar with the command line, feel free to make new folders and change directories.

We're going to write one simple line in the terminal:\

```cargo new fangraphs-learn-to-code```

Cargo is Rust's package manager and was installed when you installed Rust. It makes creating applications and libraries super easy. Typing the above command created a folder called "fangraphs-learn-to-code" with some key files.

Go to the File menu and click Open Folder. Find the folder that Cargo created for you and open it. Re-open the terminal again. The terminal should now be pointing to the "fangraphs-learn-to-code" folder that was created. You should see a list of files and folders. If you don't find the icons on the top left of the screen and find called "Exploter".

#### Cargo.toml

You should a file called Cargo.toml in the list. Click on that file. You should see something that looks like this:

```toml
[package]
name = "fangraphs-learn-to-code"
version = "0.1.0"
authors = ["Eli Ben-Porat <fangraphs-learn@ben-porat.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

We'll use this file when we add dependencies. Rust makes adding software that other people wrote really easy. Since Rust compiles everything from source code, and has very strong stability guarantees, pretty much every single library that you import will just work. You don't need to change anything here yet.

#### The src folder and the main.rs file

You should see a folder called "src", with a main.rs file in the folder. Click through to the main.rs file and you should see something like this:

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo has already given you everything you need to create your first baseball application. The ```!``` simply means that the ```println``` function isn't core Rust syntax, it's a macro from the standard library (we'll dive into this more later). For today, all you need to understand is that ```println!``` will print whatever text you put inside, then insert a newline.

Let's make one simple change. Replace the word "world!" with the word "baseball!".

### Creating and compiling our Hello, Baseball! Application

Go down to the terminal window. If you forgot to re-open it, go to the Terminal menu at the top and re-open it. All we're going to do is type the following line:\

```cargo run```

This will compile our program, creating a fangraphs-learn-to-code.exe and running it. It will also print the words "Hello, baseball!" in the command line.

## Conclusion

If you see the words "Hello, baseball!" in your terminal, you are now officialy a Rust programmer. As a volunteer member of the Rust Evangelism Strike Force, I welcome you.
