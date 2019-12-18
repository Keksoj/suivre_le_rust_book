// 2019-01-06
// Le but de ce programme est de comparer les rectangles. Peuvent-ils s'inscrire
// l'un dans l'autre ?
// On va utiliser une méthode, can_hold()

use std::io;

#[derive(Debug)]

// Définition du struct.
struct Rectangle {
    largeur: u32,
    longueur: u32,
}

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

// Dans un seul bloc d'implémentation, on peut définir plusieurs méthodes
// La méthode aire() est reprise des autres programmes
impl Rectangle {
    fn aire(&self) -> u32 {
        self.longueur * self.largeur
    }

    // définit la méthode can_hold(). Voyons les paramètres
    // &self signifie : "emprunte une instance du struct Rectangle" (voir impl)
    // "emprunte-en une autre" est dit ainsi :     'other: &Rectangle'
    fn can_hold(&self, other: &Rectangle) -> bool {
        // la comparaison proprement dite.
        self.largeur > other.largeur && self.longueur > other.longueur
    }
}

