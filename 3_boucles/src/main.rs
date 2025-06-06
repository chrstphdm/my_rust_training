// LOOPS - aka the eternal cycle of read QC

fn boucle_et_print() {
    // Infinite loop inside infinite loop inside your patience limits.
    loop {
        println!("Still alive in the main loop...");
        let mut i = 0i32;

        loop {
            println!("Inner loop: like nested FASTQ parsing, but louder.");
            i += 1;
            if i > 2 {
                // Time to bail out of this nested mess
                return;
            }
        }
    }
}

fn main() {
    // Rust has three kinds of loops: `loop`, `while`, and `for`.
    // Each with its own flavor of pain and control.

    // `loop` is the rawest form - infinite until you scream `break`.
    let mut i: i32 = 0;
    loop {
        println!("Sequencing still going... i = {}", i);
        i += 1;
        if i > 10 {
            break; // Abort mission
        }
    }

    // `while` continues as long as a condition is true.
    // It's the closest to your average C-style loop, minus segfaults.
    while i < 10 {
        println!("Read counter at {}", i);
        i += 1;
    }

    // You can break out of a loop based on logic,
    // e.g., rage-quit your pipeline at the sight of a duplicated sample.
    while i < 20 {
        println!("QC step... i = {}", i);
        i += 1;
        if i == 15 {
            println!("We're done here. i == 15 was the last straw.");
            break;
        }
    }

    // `continue` skips the current iteration.
    // Like ignoring all even-numbered samples because of course you do.
    while i < 30 {
        i += 1;
        if i % 2 == 0 {
            continue; // Skip the boring ones
        }
        println!("Retained i = {}", i);
    }

    // You can `return` from inside any loop inside a function.
    // Good for emergency exits when you realize your code is looping through NCBI metadata.
    boucle_et_print();

    // `for` is the friendliest loop.
    // It iterates over ranges, vectors, strings... anything iterable.
    for i in 0..10 {
        println!("Analyzing sample #{}", i);
    }

    // Iterate over a vector of funky read lengths
    let read_lengths = vec![35, 50, 76, 150, 250];
    println!("Read lengths encountered:");
    for length in &read_lengths {
        // Use a reference to avoid consuming the vector
        println!("{}bp", length);
    }

    // Use enumerate to get both index and value - perfect for blaming bad results on index 3.
    for (idx, length) in read_lengths.iter().enumerate() {
        println!("Index {} → length = {}", idx, length);
    }

    // You can iterate over characters in a string
    let sequence = "ATGCGT";
    println!("Sequence chars:");
    for base in sequence.chars() {
        println!("{}", base);
    }

    // You can also iterate over bytes - useful if you're debugging encoding weirdness
    println!("Sequence bytes:");
    for byte in sequence.bytes() {
        println!("{}", byte);
    }

    // You *could* iterate over file lines... but this is a dry example.
    // let file = std::fs::File::open("samples.txt").expect("can't open file");
    // let reader = std::io::BufReader::new(file);
    // for line in reader.lines() {
    // let line = line.expect("can't read line");
    // println!("{}", line);
    // }

    // Nested loops = double/triple for loop hell
    // e.g., matrix parsing, alignment scoring grids, etc.
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("Matrix elements:");
    for row in matrix.iter() {
        for &val in row.iter() {
            println!("{}", val);
        }
    }

    // Labels allow you to escape from specific loops.
    // Good for jumping out of deep nested loops when you regret everything.
    'genome: for chr in 1..=22 {
        'region: for start in 0..1_000_000 {
            'base: for offset in 0..100 {
                if start > 10 {
                    println!("Abort genome scan at chr{}, start = {}", chr, start);
                    break 'genome;
                }
                if start % 2 == 0 {
                    continue 'region;
                }
                if offset % 2 == 0 {
                    continue 'base;
                }
                println!("chr{}:{}-{}", chr, start, offset);
            }
        }
    }
}
