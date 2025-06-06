fn main() {
    // ENUMS - aka Rust's answer to "what even is this thing?"

    // An enum is a type that can represent multiple variants - like a sequencing run:
    // could be "Success", "Fail", or "Suspiciously Too Good To Be True".

    // You define them using the `enum` keyword, followed by the variants.
    // Variants can be:
    // - unit-like (just a name)
    // - tuple-like (with unnamed fields)
    // - struct-like (with named fields)

    enum BioinfoStatus {
        Success,
        CrashAndBurn(String),
        WeirdButWorks {
            warning_count: u32,
            memory_used: u64,
        },
    }

    enum SampleOrigin {
        Human,
        Bacteria,
        Alien(String),
    }

    // Let's create some variants
    let run1 = BioinfoStatus::Success;
    let run2 = BioinfoStatus::CrashAndBurn("Segfault in alignment step".to_string());
    let run3 = BioinfoStatus::WeirdButWorks {
        warning_count: 42,
        memory_used: 1024,
    };

    let sample1 = SampleOrigin::Human;
    let sample2 = SampleOrigin::Bacteria;
    let sample3 =
        SampleOrigin::Alien("Planet BINLUP (BioInformaticians No Longer Use Python)".into());

    // Matching on enum values is the real MVP here
    // we had ref msg in the CrashAndBurn variant to avoid ownership issues
    // and to allow us to use the message later without moving it
    // (because Rust is all about ownership and borrowing, remember?)

    match run1 {
        BioinfoStatus::Success => println!("Run completed without (visible) issues."),
        BioinfoStatus::CrashAndBurn(ref msg) => println!("Pipeline exploded: {}", msg),
        BioinfoStatus::WeirdButWorks {
            warning_count,
            memory_used,
        } => {
            println!(
                "Completed with {} warnings and used {}MB RAM",
                warning_count, memory_used
            )
        }
    }

    match run2 {
        BioinfoStatus::Success => println!("Run completed without (visible) issues."),
        BioinfoStatus::CrashAndBurn(ref msg) => println!("Pipeline exploded: {}", msg),
        BioinfoStatus::WeirdButWorks {
            warning_count,
            memory_used,
        } => {
            println!(
                "Completed with {} warnings and used {}MB RAM",
                warning_count, memory_used
            )
        }
    }

    match run3 {
        BioinfoStatus::Success => println!("Run completed without (visible) issues."),
        BioinfoStatus::CrashAndBurn(ref msg) => println!("Pipeline exploded: {}", msg),
        BioinfoStatus::WeirdButWorks {
            warning_count,
            memory_used,
        } => {
            println!(
                "Completed with {} warnings and used {}MB RAM",
                warning_count, memory_used
            )
        }
    }

    // Enums are perfect for clearly modelling possible states or categories
    match sample1 {
        SampleOrigin::Human => println!("Classic Homo sapiens sample."),
        SampleOrigin::Bacteria => println!("Ah, an Escherichia coli entry."),
        SampleOrigin::Alien(planet) => println!("Intergalactic contamination from {}", planet),
    }

    match sample2 {
        SampleOrigin::Human => println!("Classic Homo sapiens sample."),
        SampleOrigin::Bacteria => println!("Ah, an Escherichia coli entry."),
        SampleOrigin::Alien(planet) => println!("Intergalactic contamination from {}", planet),
    }

    match sample3 {
        SampleOrigin::Human => println!("Classic Homo sapiens sample."),
        SampleOrigin::Bacteria => println!("Ah, an Escherichia coli entry."),
        SampleOrigin::Alien(planet) => println!("Intergalactic contamination from {}", planet),
    }

    // You can even destructure enums inside `if let` or `while let`
    // again, we add ref 
    if let BioinfoStatus::CrashAndBurn(ref reason) = run2 {
        println!("Handling crash: {}", reason);
    }

    // Bonus: enums can implement methods via `impl`
    impl BioinfoStatus {
        fn is_ok(&self) -> bool {
            matches!(
                self,
                BioinfoStatus::Success | BioinfoStatus::WeirdButWorks { .. }
            )
        }
    }

    println!("Run 1 status OK? {}", run1.is_ok());
    println!("Run 2 status OK? {}", run2.is_ok());
    println!("Run 3 status OK? {}", run3.is_ok());
}
