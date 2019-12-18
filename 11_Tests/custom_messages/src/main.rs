// 2019-03-22

// Disons bonjour à l'utilisateur

use std::io;
mod lib;
use crate::lib::salutation;

fn main() {

    println!("Bonjour, comment tu t'appelles ?");
    // Récupérer le nom de l'utilisateur
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Pas pu lire l'entrée");

    // convertir 'input' en nombre entier, ce sera la variable limite
    let name: String = input.trim().parse()
        .expect("On veut un nom !");

    // la fonction salutation() renvoie une String, il faut donc l'afficher avec
    // println!
    println!("{}", salutation(&name));
    println!("Ce qui est intéressant c'est dans lib.rs et 'cargo test'");

}
