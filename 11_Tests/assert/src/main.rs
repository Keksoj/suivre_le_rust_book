// 2019-03-21

// Le but de ce programme est de comparer les rectangles. Peuvent-ils s'inscrire
// l'un dans l'autre ? On va utiliser une méthode, can_hold()

// Testons ce programme dans lib.rs

// use std::io;
mod lib;
use crate::lib::Rectangle;





fn main() {
    // définition des instances rect1, rect2, rect3
    let rect1 = Rectangle {
        largeur: 30,
        longueur: 50,
    };
    let rect2 = Rectangle {
        largeur: 10,
        longueur: 40,
    };
    let rect3 = Rectangle {
        largeur: 60,
        longueur: 45,
    };

    println!(
        "{:?} peut-il contenir {:?}? {}",
        rect1,
        rect2,

        // la syntaxe c'est paramètre1.méthode(paramètre2)
        rect1.can_hold(&rect2)
    );
    println!(
        "{:?} peut-il contenir {:?}? {}",
        rect1,
        rect3,
        rect1.can_hold(&rect3)
    );
}
