# Rust Training Repository

Welcome to my personal Rust training repository!
This project gathers a collection of self-contained examples and experiments to explore various Rust concepts.

## Why this repo?

Coming from almost two decades in bioinformatics, scripting in Bash, Perl, AWK (love it !!) and Groovy (have to confess, that's mostly because of Nextflow), with a background in Java and even some C/C++ from the early days, I've always been juggling between pragmatism and performance.

Rust caught my eye not just because it's trendy, but because it promises safety, speed, and expressiveness without the footguns of C or the bloat of Java.

I don't trust Python for anything that needs real optimisation (and R even less, let's not talk about it), so it was about time I learned a *real* language™.
This repo is where I experiment, fail, and eventually make peace with the borrow checker.

**And yes, comments and variables use some glorious franglais™, because why choose between Molière and Shakespeare when you can enjoy les deux en même temps?**

## Repository Structure

Each numbered directory corresponds to a Rust concept or theme (e.g. variables, expressions, loops, enums, etc.).
Inside each folder, you'll typically find a `main.rs` file illustrating the key ideas.

Example:
```
1_variables/
└── src/
└── main.rs
```

This modular structure makes it easy to focus on specific topics independently.

## Running an Example

Each folder is configured as an independent binary crate and part of a workspace.
To run a specific topic:

```bash
cargo run -p variables
```

To build everything:

```bash
cargo build
```

## Learning Resources

Most of the content is inspired by or adapted from:
- [Rust expliqué par Guillaume Gomez (FR)](https://blog.guillaume-gomez.fr/Rust/)
- [The Rust Programming Language Book ("The Book")](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings: Small exercises to get you used to reading and writing Rust code](https://github.com/rust-lang/rustlings)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)

## My Rust Journey So Far: A Series of Unfortunate but Funny Events

This repository may look neat and structured now, but it was built on top of mistakes, compiler tantrums, and a healthy dose of trial and error. Below is a non-exhaustive summary of the battles fought and lessons learned:

### 1. Folder Names and the Numeric Curse

I initially named my crates with prefixes like `1_variables`, `2_expressions`, etc. Seemed clean and sorted. Cargo disagreed violently:
> "Invalid character `1` in package name. Crate names cannot start with a digit."

Lesson: Folder names can start with numbers. Crate names cannot. Thank you, Cargo, for teaching me alphabetical humility.

### 2. Where is your `src/`?

Like any self-respecting Bash/Python person, I dropped my `main.rs` files right inside the themed folders. Cargo responded with cold detachment:
> "no targets specified in the manifest"

Turns out Rust wants your `main.rs` to live inside a `src/` folder. Not optional. It's like the `src/` folder is a sacred temple and `main.rs` must enter barefoot.

### 3. The Revolver Incident

While trying to follow Rust 2021 best practices, I proudly added `workspace.revolver = "2"` to my root `Cargo.toml`.
Cargo, ever the helpful compiler, quietly warned:
> "unused manifest key: workspace.revolver"

Indeed. What I meant was `resolver = "2"`. Freudian slip? Maybe. But lesson learned: don't bring a revolver to a dependency resolution problem.

### 4. Move Semantics: the Trap

I wrote a series of `match` statements on enums containing `String` values, then reused those variables later. Rust reminded me:
> "use of moved value: value borrowed after move"

In other words, once you give away ownership, it's gone. You can't keep referencing it like a clingy developer.
The fix? Pattern match with `ref msg` instead of moving `msg` out. It took three errors and a coffee refill to figure it out.

### 5. The Borrow Checker Still Hates Me

I tried to return references to things that no longer existed. I tried to clone everything. I added `.to_string()` so often it became muscle memory.
Eventually I learned that the borrow checker is not your enemy. It just wants you to commit to memory safety... or rage-quit.

---

This repository evolves as I progress in my learning, so feel free to explore, copy, or reuse anything useful.

Happy Rusting!
