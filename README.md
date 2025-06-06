# Rust Training Repository

Welcome to my personal Rust training repository!  
This project gathers a collection of self-contained examples and experiments to explore various Rust concepts.

## Why this repo?

I come from a background in [bioinformatics / system scripting / X], and I'm learning Rust to explore safe, performant, and modern system-level programming.  
Rather than following a single tutorial, I mix structured learning with trial-and-error coding. This repo reflects that journey: sometimes messy, sometimes elegant — but always intentional.


## Repository Structure

Each numbered directory corresponds to a Rust concept or theme (e.g. variables, expressions, loops, enums, etc.).  
Inside each folder, you'll typically find a `main.rs` file illustrating the key ideas.

Example:
```
1.variables/
└── main.rs

2.expressions/
└── main.rs

3.boucles/
└── main.rs

...
```

This modular structure makes it easy to focus on specific topics independently.

## Running an Example

You can run any example using `cargo run` inside the corresponding folder.  
Make sure each folder is initialized as a separate binary crate (with its own `Cargo.toml`) or adjust accordingly if using a workspace.

```bash
cd 3.boucles
cargo run
```

## Learning Resources

Most of the content is inspired by or adapted from:
- [Rust expliqué par Guillaume Gomez (FR)](https://blog.guillaume-gomez.fr/Rust/)
- [The Rust Programming Language Book ("The Book")](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings: Small exercises to get you used to reading and writing Rust code](https://github.com/rust-lang/rustlings)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

This repository evolves as I progress in my learning, so feel free to explore, copy, or reuse anything useful!

---

Happy Rusting !
