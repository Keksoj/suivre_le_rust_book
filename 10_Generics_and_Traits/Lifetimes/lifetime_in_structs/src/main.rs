// 2019-03-12
// Jusqu'à présent, les structs contenaient des types dont ils avaient
// l'ownership. Ils peuvent aussi contenir des références, mais il faut annoter
// des lifetimes pour chaque référence.

// Définir le struct, avec la lifetime 'a
// il contient une variable 'part', une référence à une string slice
// Il FAUT préciser la lifetime de cette référence
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {

    // Créons une chaîne de caractères, ici un début de roman
    let novel = String::from("Call me Ishmael. Some years ago...");

    // Extrayons-en la première phrase.
    let first_sentence = novel.split('.')
        .next()
        .expect("Pas trouvé de '.'");

    // Affichons-là pour être sûr
    println!("{}", first_sentence);

    // Créer une instance de ImportantExcerpt avec comme variable part
    // une référence à la première phrase
    let i = ImportantExcerpt { part: first_sentence };
    println!("La première phrase est : {:#?}", i);
}
