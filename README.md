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

	api-rest [Api Rest via http example]
	server [Little example of http server]
	hello-world [Hello World example project]
	easy-game [Easy command line game project]
	shadowing [Example of what is shadowing in Rust]
	settings.sh [Bash script for setting up cargo and rust environment]
	README.md [Readme file]

## Common Programming Concepts
	
	VARIABLES / VARIABLE TYPES

		1. Variables are inmutable. But, what is the difference between "let mut" variable and a constant? You can't modify the value of contants with "mut" instance.

		2. Shadowing. Re-use of previous declared variables (only using "let"). Program remembers the previous values.

		3. For check data types check https://doc.rust-lang.org/rust-by-example/types.html

	FLOW CONTROL

		1. Check the next link. https://doc.rust-lang.org/rust-by-example/flow_control.html 

	ERROR HANDLING

		1. Check the next link. https://doc.rust-lang.org/rust-by-example/error.html



## Some annotations about Rust language

	1. For creating a local varible `let foo = bar;`. In Rust, variables are inmutable. So, for declaring a mutable variable we need to use the next line `let mut foo = bar;`.

	2. The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references. You don’t need to know a lot of those details to finish this program. For now, all you need to know is that like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

	3. Example of cast from string to u32 `let guess: u32 = "42".parse().expect("Not a number!");`

	4. Structures as variables. Example of declaration:

		struct User {
		    username: String,
		    email: String,
		    sign_in_count: u64,
		    active: bool,
		}

	Example of use:

		let user1 = User {
	        email: String::from("someone@example.com"),
	        username: String::from("someusername123"),
	        active: true,
	        sign_in_count: 1,
	    };


## Links 

	1. https://doc.rust-lang.org/book/ [Libro oficial sobre Rust]

	2. https://dev.to/gruberb/intro-to-web-programming-in-rust-for-nodejs-developers-lp [Web development on Rust]

	3. https://jondot.medium.com/12-killer-rust-libraries-you-should-know-c60bab07624f [Some cool libs for Rust]

	4. https://actix.rs/docs/getting-started/ [Framework for wweb development]