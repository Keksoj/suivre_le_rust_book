// 2019-01-06
// Les fonctions associées sont définies dans un bloc d'implémentation mais ne 
// prennent pas self comme paramètre.
// Ici on va définir une fonction associée construira une instance de struct.
// square() fabriquera une instance de Rectangle qui sera un carré.

use std::io;

#[derive(Debug)]

// Définition du struct.
struct Rectangle {
    largeur: u32,
    longueur: u32,
}

fn main() {

    // demander à l'utilisateur le côté du carré, variable 'cote'
    println!("Quel côté aura votre carré ?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Pas pu lire l'entrée");
    let cote: u32 = input
        .trim()
        .parse()
        .expect("On veut un nombre !");

    println!(
        "Nous avons créé votre carré, le voici : {:#?}
        et son aire est de {} pixels carrés.",

        // voici la syntaxe pour appeler fonction associée
        // Struct::fonction(paramètre)
        // Cette chose est de facto une instance du struct Rectangle !
        Rectangle::square(cote),

        // syntaxe d'une méthode    
        // paramètre.méthode()
        // ici le paramètre est l'instance du struct retourné par la fonction
        // associée. Dingue !
        Rectangle::square(cote).aire()
    );

}

// Dans un seul bloc d'implémentation, on peut définir plusieurs méthodes
impl Rectangle {
    
    // la fonction associée square() construit une instance de Rectangle
    // le paramètre doit être un entier
    // la fonction renvoie une instance du Struct, utilisable directement !
    fn square(size: u32) -> Rectangle {
        Rectangle { largeur: size, longueur: size}
    }

    // MÉTHODE qui calcule l'aire
    fn aire(&self) -> u32 {
        self.longueur * self.largeur
    }
}

