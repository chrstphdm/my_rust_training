# Rust Training Repository

Welcome to my personal Rust training repository!  
This project gathers a collection of self-contained examples and experiments to explore various Rust concepts.

## 🧠 Why this repo?

Coming from almost two decades in bioinformatics, scripting in Bash, Perl, AWK (love it !!) and Groovy (have to confess, that's mostly because of Nextflow), with a background in Java and even some C/C++ from the early days - I've always been juggling between pragmatism and performance.

Rust caught my eye not just because it's trendy, but because it promises safety, speed, and expressiveness without the footguns of C or the bloat of Java.

I don't trust Python for anything that needs real optimisation (and R even less - let's not talk about it 🫠), so it was about time I learned a *real* language™.
This repo is where I experiment, fail, and eventually make peace with the borrow checker.

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
