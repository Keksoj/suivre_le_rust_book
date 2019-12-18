// 2019-01-01
// Comme leur nom l'indique, les slices sont des tranches de donnée. 
// Elles RÉFÉRENCENT une séquence contiguë d'éléments d'une collection.
// Dans slice_besoin, on a vu pourquoi elles sont nécessaire.


fn main() {
    
    // définir une chaine de caractères s
    let s = String::from("Hello world!");

    // voici à quoi ressemblent des tranches
    // C'est comme prendre une référence à s, mais avec [0..5] en plus.
    // c'est une référence à une portion de la chaîne de caractère
    let hello = &s[0..5];

    // On commence à compter à 0
    // la limite supérieure est exclusif, sauf à noter ..=
    // l'index du point d'exclamation est donc 11
    let world = &s[6..=10];

    println!("premier mot : {}, et deuxième : {}", hello, world);

    // une autre chaîne pour le plaisir
    let c = String::from("Ce cheval est beau comme un camion");
    
    // Quand on part du début (0), on n'est pas obligé de l'indiquer.
    let ce = &c[..2];

    // De même pour la fin
    let fin = &c[28..];

    println!("premier mot : {}, et dernier : {}", ce, fin);







}
