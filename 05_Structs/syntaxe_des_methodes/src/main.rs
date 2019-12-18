// 2019-01-05
// Les MÉTHODES ressemblent aux fonctions. On les déclare avec fn, un nom,
// elles acceptent des PARAMÈTRES, retournent des VALEURS, contiennent du code,
// sont APPELÉES.
// Leur différence : elles sont définies dans un struct
// Le premier paramètre est toujours self. self est l'instance du struct sur
// lequel on appelle la méthode.

use std::io;
// Reprenons le programme aire_de_rectangle et le trait Debug pour l'affichage.
#[derive(Debug)] 

// Définition du struct.
struct Rectangle {
    longueur: u32,
    largeur: u32,
}

// Un bloc d'IMPLÉMENTATION commence par impl, il définit la méthode aire()
// On reprend la fonction aire(), mais dans le contexte du struct Rectangle
// L'avantage principal c'est que l'argument de aire() est plus clair (comparer)
// Le contenu de la fonction est également plus simple.
// Le plus souvent, on emprunte l'instance avec &self (pas de prise d'ownership)
impl Rectangle {
    fn aire(&self) -> u32 {
        self.longueur * self.largeur
    }
}

// Le reste du programme ne change qu'à peine
fn main() {

    // définit une instance du struct Rectangle
    let instance_rectangle = Rectangle {
        longueur: 30,
        largeur: 50,
    };

    // une autre instance
    let flag = Rectangle {
        longueur: 10,
        largeur: 20,
    };

    // la SYNTAXE DES MÉTHODES est utilisée pour appeler la fonction aire()
    // instance.methode()
    println!(
        "L'aire du {:#?} est de {} pixels carrés.",
        instance_rectangle,
        instance_rectangle.aire()
    );

    println!(
        "L'aire du {:#?} est de {} pixels carrés.",
        flag,
        flag.aire()
    );

    // Demander à l'utilisateur la taille du prochain rectangle
    println!("Quelle sera la longueur du votre rectangle ?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Pas pu lire l'entrée");
    let lon: u32 = input
        .trim()
        .parse()
        .expect("On veut un nombre !");

    println!("Quelle sera la largeur du votre rectangle ?");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Pas pu lire l'entrée");
    let lar: u32 = input2
        .trim()
        .parse()
        .expect("On veut un nombre !");

    // définir l'instance du rectangle défini par l'utilisateur
    let prochain = Rectangle {
        longueur: lon,
        largeur: lar,
    };

    println!(
        "L'aire de votre {:#?} est de {} pixels carrés.",
        prochain,
        prochain.aire()
    );


}






















