fn main() {

    // les ENUMS
    // Un enum est un type qui peut prendre plusieurs valeurs différentes.
    // Chaque valeur d'un enum est appelée une variante.
    // Les enums sont souvent utilisés pour représenter des types de données qui peuvent être de différentes formes.
    // Par exemple, un enum peut représenter un type de données qui peut être soit un entier, soit une chaîne de caractères, soit un booléen.
    // En Rust, les enums sont définis avec le mot-clé `enum`, suivi du nom de l'enum et des variantes entre accolades.
    
    enum UneEnum {
        Variant,
        VariantStruct { a: i32, b: i32 },
        VariantTuple(String),
    }

    enum MonMatos {
        Ordinateur,
        Telephone,
        Tablette,
    }
    let e = UneEnum::VariantStruct { a: 1, b: 2 };
    let e2 = UneEnum::VariantTuple("Hello".to_string());
    let e3 = UneEnum::Variant;
    let e4 = MonMatos::Ordinateur;
    let e5 = MonMatos::Telephone;
    let e6 = MonMatos::Tablette;
    match e {
        UneEnum::Variant => println!("C'est une variante simple !"),
        UneEnum::VariantStruct { a, b } => println!("C'est une variante struct avec a = {} et b = {}", a, b),
        UneEnum::VariantTuple(s) => println!("C'est une variante tuple avec la chaîne : {}", s),
    }
    match e2 {
        UneEnum::Variant => println!("C'est une variante simple !"),
        UneEnum::VariantStruct { a, b } => println!("C'est une variante struct avec a = {} et b = {}", a, b),
        UneEnum::VariantTuple(s) => println!("C'est une variante tuple avec la chaîne : {}", s),
    }
    match e3 {
        UneEnum::Variant => println!("C'est une variante simple !"),
        UneEnum::VariantStruct { a, b } => println!("C'est une variante struct avec a = {} et b = {}", a, b),
        UneEnum::VariantTuple(s) => println!("C'est une variante tuple avec la chaîne : {}", s),
    }
    match e4 {
        MonMatos::Ordinateur => println!("C'est un ordinateur !"),
        MonMatos::Telephone => println!("C'est un téléphone !"),
        MonMatos::Tablette => println!("C'est une tablette !"),
    }
    match e5 {
        MonMatos::Ordinateur => println!("C'est un ordinateur !"),
        MonMatos::Telephone => println!("C'est un téléphone !"),
        MonMatos::Tablette => println!("C'est une tablette !"),
    }
    match e6 {
        MonMatos::Ordinateur => println!("C'est un ordinateur !"),
        MonMatos::Telephone => println!("C'est un téléphone !"),
        MonMatos::Tablette => println!("C'est une tablette !"),
    }
}