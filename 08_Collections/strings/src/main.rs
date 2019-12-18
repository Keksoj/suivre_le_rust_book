// 2019-01-21
// Les chaînes de caractères sont une collection d'octets. String est pourvu par
// la bibliothèque standard, est possédée (owned), modifiable, agrandissable.


fn main() {
    // Nouvelle chaîne de caractère, vide. Avec la fonction String::new()
    let c = String::new();

    // Souvent, on part d'une chaîne initiale. Deux étapes.
    // Déclarer un LITERAL (stocké sur la pile - stack)
    // en faire une chaîne de caractère (stockée dans le tas - heap)
    // pour la deuxième étape, on utilise la méthode to_sring()
    let data = "contenu initial";
    let d = data.to_string();

    // on peut aussi prendre un raccourci
    let e = "contenu de départ".to_string();

    // énième option, on peut utiliser la fonction String::from()
    // f est modifiable parce qu'on va l'agrandir après
    let mut f = String::from("un nouveau départ");

    // Agrandir avec la MÉTHODE push_str(), qui prend un tranche de chaîne de
    // caractères. (It takes a string slice).
    // Donc, push_str() NE PRENDS PAS L'OWNERSHIP
    f.push_str(" pour une nouvelle vie");
    println!("{}", f);

    // On peut concaténer !
    let g = String::from("Hello, ");
    let h = String::from("world!");
    let i = g + &h; // on prend l'ownership de g, qu'on ne peut plus
    println!("{}", i); // utiliser. push_str() utilise la méthode add()
                       // qui prend comme argument une String et une &str
                       // h est une &String et pas une &str, mais le compi-
                       // -lateur s'en débrouille.
                       // bref, h reste valide, g est invalidé.

    // on peut concaténer avec la MACRO format! qui ne prend PAS l'ownership
    let j = String::from("riri");
    let k = String::from("fifi");
    let l = String::from("loulou");
    let trio = format!("{}-{}-{}", j, k, l);
    println!("{}", trio);

    // Indexer une string n'est pas idéal, en effet, selon les langues, il faut
    // soit un octet, soit deux, pour encoder un caractère. Et encore, pour cer-
    // taines langues on encode séparément une lettre et son diacritique !

    // Si on dit bonjour en russe :
    let zdrav = "Здравствуйте";

    // et on qu'on extrait les quatres premiers octets : 0, 1, 2 et 3 (4 exclu)
    let m = &zdrav[0..4];

    // Quand on les affiches, println! renverra "Зд" ! 4 octets pour 2 lettres
    println!("{}", m);

    // On a une MÉTHODE appelée chars() qui décortique les strings
    let namaste = String::from("नमस्ते");

    println!("On va décortiquer le mot {} (namaste)", namaste);
    for c in namaste.chars() {
        println!("{}", c);
    }
    println!(
        "Les quatrièmes et sixièmes caractères sont des diacritiques.\n
        Voyons avec le bonjour russe."
    );
    for c in zdrav.chars() {
        println!("{}", c);
    }

    // on peut afficher les octets avec bytes(), une MÉTHODE.
    for b in namaste.bytes() {
        println!("octet de namaste : {}", b);
    }

    // scrutons la mémoire
    let mut story = String::from("Il était une fois, dans une galaxie lointaine...");

    // on a ces méthodes
    let ptr = story.as_ptr();   // raw pointer vers le premier octet de la chaîne
    let len = story.len();
    let capacity = story.capacity();

    println!("{}", story);
    println!("Analysons cette chaîne de caractère.
        ptr = raw pointer : {:?}, 
        longueur : {},
        capacité : {}",
        ptr, len, capacity
    );
    
    // essayons de tronquer l'histoire
    &story.truncate(5);
    println!("Les premiers caractères de l'histoire : {}", story);
}
