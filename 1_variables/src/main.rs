fn main() {
    println!("Hello geeks!");

    // NOTE : when the variable is not used, the compiler will complain
    // so we add _ as prefix to avoid unused variable warning
    // but this is not a good practice
    // you can also use `#[allow(unused_variables)]` to ignore unused variable warnings
    
    // Variables are immutable by default.
    // Like a file you forgot to chmod.
    let genome_ref = "GRCh38";
    println!("Reference genome: {}", genome_ref);
    
    // If you want to change a variable, you have to say "mut" like you mean it.
    let mut mapped_reads = 0;
    mapped_reads += 1;
    println!("Mapped reads count: {}", mapped_reads);
    
    // Constants are hardcore immutable and must have an explicit type.
    // Perfect for bioinfo magic numbers nobody dares to question.
    const MIN_PHRED_SCORE: u8 = 30;
    println!("Minimum quality score: {}", MIN_PHRED_SCORE);
    
    // Types - because Rust is obsessed with control.
    // Sometimes it guesses the type, sometimes it wants full disclosure.
    
    // Implicit typing: Rust just knows.
    let _coverage = 42; // Probably i32, probably enough, probably not.
    
    // Explicit typing: for when you want to sound like you know what you're doing.
    let _depth: i32 = 50;
    let _read_length = 150u16; // Because we're not all doing long reads, okay?
    
    // Float types: useful for GC content, TPM, and other lies we tell ourselves.
    let _gc_content: f64 = 0.486;
    let _read_counttpm = 12.5f64;
    
    // Base types you'll love to hate:
    // - u32 for positions in the genome (0-based, obviously, until it isn't)
    // - f64 for p-values (when you're still pretending 0.049 is significant)
    // - bool for QC filters: true, false, or "depends who's asking"
    // - char for nucleotides, including the mysterious 'N'
    // - str for things like "SRR1234567" you paste into 10 scripts
    
    let _base: char = 'G';
    let _passed_qc: bool = true;
    let _sample_id = "SRR007"; // totally a real sample name, very creative
    
    // Reminder: Rust makes you think about your types.
    // Which is annoying until you realise half your Python bugs came from not doing that.
    }
    