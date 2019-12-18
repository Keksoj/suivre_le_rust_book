// 2019-03-12

// Rédigeons une fonction qui renvoie la plus grande de deux tranches de chaîne
// de caractère. (the longer of two string slices).

use std::fmt::Display;


fn main() {

    // Déclarer les deux chaînes
    let string1 = String::from("abcd");  // une String
    let string2 = "xyz";                 // une "string literal"

    // comme paramètres : des slices, donc des &str, des références
    // on ne veut pas prendre l'ownership
    let result = longest(string1.as_str(), string2);
    println!("La plus longue chaîne est {}", result);

    let result2 = longest_with_annoucement(string1.as_str(), string2, "annonce");
    println!("{}", result2);
}

// La fonction avec les annotations d'espérance de vie (lifetime annotations)
// Comme pour les types de données génériques, on utilise des chevrons <> placés
// entre le nom de la fonction et la liste des paramètres.
// On veut donner la même lifetime aux deux paramètres et à la valeur retour
// La lifetime s'appelle <'a>, on l'ajoute partout
fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len () {
        x
    } else {
        y
    }
}
// Du vocabulaire : les input lifetimes, et les output lifetimes

// Maintenant, une fonction qui contient la même chose plus un trait bound
fn longest_with_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len () {
        x
    } else {
        y
    }
}
