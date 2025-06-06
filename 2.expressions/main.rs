// EXPRESSIONS

fn test_expression(x: i32) -> i32 {
    if x < 0 {
        println!("{} < 0", x);
        -1
    } else if x == 0 {
        println!("{} == 0", x);
        0
    } else {
        println!("{} > 0", x);
        1
    }
}

fn main() {

    // Les expressions
    // En Rust, tout est une expression, y compris les blocs de code.
    // Une expression est une unité de code qui renvoie une valeur.
    // Les expressions peuvent être utilisées pour affecter une valeur à une variable, pour effectuer des calculs, etc.
    test_expression(-1);
    test_expression(0);
    test_expression(1);

    let my_string = "hello";
    // Les expressions de correspondance (match)
    // En Rust, les expressions de correspondance (match) sont utilisées pour comparer une valeur à plusieurs motifs.
    // Elles sont similaires aux instructions switch dans d'autres langages de programmation.
    // Elles permettent de vérifier la valeur d'une variable et d'exécuter du code en fonction de cette valeur.
    // Elles sont très puissantes et peuvent être utilisées pour décomposer des types complexes, comme les enums ou les structs.
    
    let s = match my_string {
        "bonjour" => "français",
        "ciao" => "italien",
        "hello" => "anglais",
        "hola" => "espagnol",
        _ => "je ne connais pas cette langue..."
    };
    
    println!("{}", s);
    
    let val = if 1 == 2 {
        "2"
    } else {
        "1"
    };
    println!("{}", val);
    // On peut utiliser des expressions de correspondance pour vérifier la valeur d'une variable et exécuter du code en fonction de cette valeur.
    let x = 5;
    let y = match x {
        0 => "zero",
        1 => "un",
        2 => "deux",
        _ => "autre"
    };
    println!("{}", y);
}
