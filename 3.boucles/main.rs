
fn boucle_et_print() {
    loop {
        println!("Toujours là !");
        let mut i = 0i32;

        loop {
            println!("sous-boucle !");
            i += 1;
            if i > 2 {
                // On quitte la fonction "boucle_et_print".
                return;
            }
        }
    }
}
fn main () {

    // Les boucles
    // En Rust, il existe trois types de boucles : `loop`, `while` et `for`.

    
    // La boucle `loop` est une boucle infinie qui s'exécute jusqu'à ce qu'elle soit interrompue par une instruction `break`.
    let mut i: i32 = 0;
    loop {
        println!("bonjour !");
        i += 1;
        if i > 10 {
            break; // On arrête la boucle.
        }
    }

    // La boucle `while` s'exécute tant qu'une condition est vraie.
    while i < 10 {
        println!("i vaut : {}", i);
        i += 1; // On incrémente i.
    }
    // On peut imbriquer des boucles et utiliser `break` pour sortir de la boucle la plus interne.
    while i < 20 {
        println!("i vaut : {}", i);
        i += 1; // On incrémente i.
        if i == 15 {
            println!("On quitte la boucle car i vaut 15 !");
            break; // On quitte la boucle.
        }
    }
    // On peut aussi imbriquer des boucles et utiliser `continue` pour passer à l'itération suivante de la boucle.
    while i < 30 {
        i += 1; // On incrémente i.
        if i % 2 == 0 {
            continue; // On passe à l'itération suivante si i est pair.
        }
        println!("i vaut : {}", i); // On affiche i s'il est impair.
    }
    // et enfin, dans une fonction, on peut imbriquer des boucles et utiliser `return` pour quitter la fonction.
    boucle_et_print();
    
    // La boucle `for` s'exécute sur une plage de valeurs ou sur les éléments d'une collection.
    for i in 0..10 {
        println!("i vaut : {}", i);
    }
    // On peut aussi utiliser `for` pour itérer sur les éléments d'un tableau ou d'un vecteur.
    let v = vec![1, 4, 5, 10, 6]; // On crée un vecteur qui contient ces valeurs.
    println!("Voici les valeurs du vecteur :");
    for value in &v { // Puis on itère sur les valeurs de ce vecteur.
        // ATTENTION : on utilise une référence pour ne pas consommer le vecteur.
        // Si on utilisait `for value in v`, le vecteur serait consommé et ne pourrait plus être utilisé.
        println!("{}", value);
    }

    // enumerate permet d'obtenir à la fois la position et la valeur de chaque élément.
    for (position, value) in v.iter().enumerate() { // On itère sur ses valeurs.
        println!("position = {} et value = \"{}\"", position, value);
    }

    
    // On peut aussi utiliser `for` pour itérer sur les caractères d'une chaîne de caractères.
    let s = "Bonjour le monde !";
    println!("Voici les caractères de la chaîne :");
    for c in s.chars() { // On itère sur les caractères de la chaîne.
        println!("{}", c);
    }
    // On peut aussi utiliser `for` pour itérer sur les octets d'une chaîne de caractères.
    println!("Voici les octets de la chaîne :");
    for b in s.bytes() { // On itère sur les octets de la chaîne.
        println!("{}", b);
    }
    // On peut aussi utiliser `for` pour itérer sur les lignes d'un fichier.
    // Pour cela, on utilise la bibliothèque `std::fs` pour lire le fichier.
    // let file = std::fs::File::open("fichier.txt").expect("Impossible d'ouvrir le fichier");
    // let reader = std::io::BufReader::new(file);
    // for line in reader.lines() {
    //     let line = line.expect("Impossible de lire la ligne");
    //     println!("{}", line);
    // }

    // Les boucles imbriquées
    // On peut imbriquer des boucles pour parcourir des structures de données plus complexes.
    // Par exemple, on peut imbriquer des boucles pour parcourir une matrice (tableau à deux dimensions).
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("Voici les éléments de la matrice :");
    for row in matrix.iter() { // On itère sur les lignes de la matrice.
        for &value in row.iter() { // On itère sur les valeurs de chaque ligne.
            println!("{}", value);
        }
    }
    // On peut aussi utiliser des étiquettes pour sortir de boucles imbriquées.
    // Les étiquettes permettent de nommer des boucles et de sortir de la boucle la plus externe.
    'global: for _ in 0..10 {
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                // on arrête la boucle qui s'appelle global
                if x > 3 { break 'global; }
    
                // on continue la boucle sur x
                if x % 2 == 0 { continue 'outer; }
    
                // on continue la boucle sur y
                if y % 2 == 0 { continue 'inner; }
    
                println!("x: {}, y: {}", x, y);
            }
        }
    }
    
}