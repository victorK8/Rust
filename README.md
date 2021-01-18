# Rust


Repository for starting with Rust programming language.

## Basic commands

	Setup of *cargo* package-environment `source $HOME/.cargo/env`

    Build your project with `cargo build`

    Run your project with `cargo run`

    Test your project with `cargo test`

    Build documentation for your project with `cargo doc`

    Publish a library to crates.io with `cargo publish`

## Build a cargo project

	1. Create a foler `mkdir -p [PRJ-NAME]`

	2. Init the project structure `cargo init .` 

	3. Build package `cargo build`. By default, this command build project as debug mode. For building project as release mode use `cargo build --release`

	4. Run project `cargo run` or check directories and run binaries.

## Repository structure

	hello-world [Hello World example project]
	easy-game [Easy command line game project]
	shadowing [Example of what is shadowing in Rust]
	settings.sh [Bash script for setting up cargo and rust environment]
	README.md [Readme file]

## Variables

	1. Variables are inmutable. But, what is the difference between "let mut" variable and a constant? You can't modify the value of contants with "mut" instance.

	2. Shadowing. Re-use of previous declared variables (only using "let"). Program remembers the previous values.

## Some annotations about Rust language

	1. For creating a local varible `let foo = bar;`. In Rust, variables are inmutable. So, for declaring a mutable variable we need to use the next line `let mut foo = bar;`.

	2. The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.


## Links 

	1. https://doc.rust-lang.org/book/ [Libro oficial sobre Rust]
