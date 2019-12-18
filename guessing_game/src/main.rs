// Code rédigé en suivant les instructions de https://doc.rust-lang.org/book
// L'un des premiers tutos, donc très détaillé en vocabulaire.

// importer la bibliothèque entrée/sortie
use std::io;

// importer le crate "rand" pour random
// Il faut ajouter rand="0.3.14" aux dépendances dans Cargo.toml
// Rng (random number generator) est un TRAIT qui sera utilisé par des MÉTHODES
use rand::Rng;

// importer un truc de plus, pour faire les comparaisons
use std::cmp::Ordering;

mod lib;
use crate::lib::Guess;


// fonction principale, sans argument
fn main() {

    //println n'est pas une fonction ici, mais une macro, à cause du '!'
    println!("Bonjour et bienvenue dans 'Devine' !");
    println!("Tape un nombre entre 0 et 100!");

    // Générer un nombre random entre 1 et 100
    // rand est le CRATE, le package importé
    // thread_rng est une FONCTION, associée par ::
    // gen_range est une MÉTHODE. Inclusif en bas (1), exclusif en haut (101)
    // pour plus d'info, taper "cargo doc --open"
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // boucle
    loop {

        // définir la variable de l'entrée avec let
        // mut : variable non immuable (mut = mutable)
        // guess est le nom de la variable
        // new est une fonction associée ::
        // qui amène une chaîne de caractère, String, qui est vide ()
        let mut guess = String::new();

        // lire l'entrée tapée par l'utilisateur et en faire guess
        // stdin, fonction associée :: à la bibliothèque io
        // .read_line est une méthode
        // &mut guess est l'argument, la variable non immuable guess
        // l'ampersand indique que l'argument est une référence (histoire de mémoire)
        io::stdin().read_line(&mut guess)

            // encore une méthode, donc nouvelle ligne et indentation
            // la méthode expect est nécessaire sinon on a un warning à la compilation
            .expect("Failed to read line");

        // passer la variable guess à un nombre entier
        // u32 : unsigned 32-bit number
        // trim est une MÉTHODE, enlève les espaces et le retour à la ligne (\n)
        // parse est une MÉTHODE
        // match vérifie que parse parvient à convertir l'entrée en nombre
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("On veut un NOMBRE entier naturel.");
                continue;
            }
        };

        // Appelons les fonctions déclarées dans lib.rs
        // Créer une instance du struct Guess
        // la valeur est vérifiée au passage
        let instance: Guess = Guess::new(guess);

        // Maintenant que la valeur est vérifiée, la réattribuer
        let guess = instance.value();


        // afficher le résultat tapé par l'utilisateur
        // les accolades {} servent à indiquer l'endroit où afficher la variable
        println!("Tu as tapé le nombre : {}", guess);

        // match est une EXPRESSION
        // guess est notre VARIABLE
        // cmp est une MÉTHODE
        // l'expression match va de bas en haut, s'arrête dès qu'un critère est rempli
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Greater => println!("Trop grand !"),
            Ordering::Equal => {
                println!("Bravo !");
                break;
            }
        }
    }
}
