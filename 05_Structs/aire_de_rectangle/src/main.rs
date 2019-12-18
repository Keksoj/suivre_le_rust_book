// 2019-01-02
// On crée un programme qui calcule l'aire d'un rectangle. Bien sûr, on peut
// créer une fonction aire() avec deux paramètres, longueur et largeur. 
// On peut définir le rectangle avec un tuple (n-uplet)
// Mais le plus cool, c'est avec un struct

// Debug est un TRAIT. Permet d'afficher un struct. Explication dans le book 5.3
#[derive(Debug)]

// Définition du struct. Une majuscule à Rectangle, par convention
struct Rectangle {
    longueur: u32,
    largeur: u32,
}

fn main() {
    // définit une instance du struct Rectangle
    let instance_rectangle = Rectangle {
        longueur: 30,
        largeur: 50,
    };

    // :? signifie à println! qu'on veut un output au format Debug
    println!("L'air du {:#?} est de {} pixels carrés.",
            instance_rectangle, aire(&instance_rectangle)
    );
}

// Voici la fonction aire(). Voyons son argument
// 'rectangle' est le nom de la variable qu'elle traitera
// le type &Rectangle : emprunt à une instance d'un struct Rectangle
// On ne veut pas prendre l'ownership de instance_rectangle
fn aire(rectangle: &Rectangle) -> u32 {

    
    rectangle.longueur * rectangle.largeur
}
