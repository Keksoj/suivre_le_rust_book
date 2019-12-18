// 2019-01-01
// On veut à nouveau calculer la longueur d'une chaine de caractères.
// Seulement, on va chercher à ne pas prendre l'ownership de la variable.
// Pour agrémenter le game, j'ai rajouté une invite à l'utilisateur.


// importer la bibliothèque entrée/sortie
use std::io;


fn main() {
    
    println!("Tapez une chaîne de caractères !");

    // Définir la variable s1 comme une chaîne de caractères
    let mut s1 = String::new();

    // récupérer l'entrée tapée par l'utilisateur dans la variable s1
    io::stdin()
        .read_line(&mut s1)
        .expect("Pas pu lire l'entrée");

    // Ici il faut encore nettoyer la variable pour enlever le retour à la ligne.
    
    // Appel de la fonction calcule_longueur()
    let len = calcule_longueur(&s1);

    println!("La longueur de la chaîne '{}' est de {} caractères.", s1, len);
}


// Contrairement au programme ownership_length, on ne renvoie pas la variable.
// La variable n'est qu'empruntée.
// L'input est défini entre parenthèse. Il s'appelle s, c'est une chaîne.
// L'ampersand fait de s une RÉFÉRENCE qui pointera vers s1
// L'output est un entier positif. usize signifie u64 pour une archi 64 bits
fn calcule_longueur(s: &String) -> usize {

    // la fonction len() calcule et retourne la longueur de s
    s.len()
}           // On sort du scope, s est dropé, mais pas s1
