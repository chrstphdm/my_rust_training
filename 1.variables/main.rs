fn main() {
    println!("Hello world!");

    // Les variables
    // En Rust, les variables sont immuables par défaut.
    let i = 0;

    println!("i vaut : {}", i);

    // Pour déclarer une variable mutable, on utilise le mot-clé `mut`.
    let mut j = 0;
    j += 1;
    println!("j vaut : {}", j);

    // Les constantes
    // En Rust, les constantes sont déclarées avec le mot-clé `const`.
    const MAX: i32 = 100;
    println!("MAX vaut : {}", MAX);
    // Les constantes sont immuables et doivent avoir un type explicite.
    // Elles sont généralement utilisées pour des valeurs qui ne changent pas pendant l'exécution du programme.

    // Les types
    // En Rust, chaque variable a un type qui détermine la nature des données qu'elle peut contenir.
    // Les types peuvent être inférés par le compilateur ou explicitement déclarés.
    let x = 42; // Le compilateur infère que x est de type i32.
    // Pour déclarer explicitement le type d'une variable, on utilise la syntaxe `let nom: type = valeur;`.
    // Par exemple, on peut déclarer une variable de type i32 (entier signé de 32 bits) comme suit :
    let x: i32 = 42;
    // ou simplement :
    let x = 42i32;
    // Le compilateur inférera le type i32 à partir de la valeur 42.
    // On peut aussi déclarer une variable de type f64 (nombre à virgule flottante de 64 bits) :
    let y: f64 = 3.14;
    // ou simplement :
    let y = 3.14f64;
    // Le compilateur inférera le type f64 à partir de la valeur 3.14.

    // Les types de base
    // Les types de base en Rust sont :
    // - i8, i16, i32, i64, i128 : entiers signés de 8, 16, 32, 64 et 128 bits.
    // - u8, u16, u32, u64, u128 : entiers non signés de 8, 16, 32, 64 et 128 bits.
    // - f32, f64 : nombres à virgule flottante de 32 et 64 bits.
    // - bool : type booléen, qui peut être vrai (true) ou faux (false).
    // - char : type caractère, qui représente un caractère Unicode.
    // - str : type chaîne de caractères, qui représente une séquence de caractères Unicode.
    // Déclaration d'une variable de type i32
    let k: i32 = 0;
    // ou :
    let k = 0i32;

    // 10 est un i32 alors max est un i32
    let max = 10i32;

}