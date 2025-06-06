// EXPRESSIONS

fn interpret_read_count(count: i32) -> i32 {
    // `if`, `else if`, `else` are expressions in Rust.
    // They return a value - not just control flow, they got return flow.
    if count < 0 {
        println!("{} reads? Did you just sequence air?", count);
        -1
    } else if count == 0 {
        println!("{} reads: classic 'no signal, but still published'", count);
        0
    } else {
        println!("{} reads: looks legit, probably Illumina", count);
        1
    }
}

fn main() {
    // In Rust, everything is an expression - even blocks!
    // That means you can assign stuff like `let x = if true { 1 } else { 2 };`
    // It's a bit weird at first, but once you get it, you'll judge other languages hard.

    interpret_read_count(-1);
    interpret_read_count(0);
    interpret_read_count(1);

    let seq_platform = "ONT";

    // `match` is Rust's upgraded `switch`, but with safety, power, and zero fallthrough nonsense.
    // It's extremely useful when matching against enums or fixed value sets.
    // Also works well for fake sarcasm over sequencing platforms.
    let platform_type = match seq_platform {
        "Illumina" => "short and sweet",
        "PacBio" => "long and luxurious",
        "ONT" => "noisy but gets the job done",
        _ => "mysterious third-party Chinese box",
    };

    println!("Sequencer profile: {}", platform_type);

    // `if` as an expression - yeah, no ternary `? :` like in C.
    // Just use `if`, `else`, and assign the result. Simple, elegant, and a bit smug.
    let read_quality = if 1 == 2 { "lol nope" } else { "looks fine" };
    println!("Quality check: {}", read_quality);

    // Another `match` example: categorizing read counts just for fun
    let read_count = 5;
    let read_label = match read_count {
        0 => "no data, no cry",
        1 => "one lonely read",
        2 => "a couple of reads",
        _ => "enough to pretend it's a dataset",
    };

    println!("Sample summary: {}", read_label);
}
