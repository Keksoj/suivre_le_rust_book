// 2019-03-24 Début du tutoriel

// Ce programme suit le tutoriel, jusqu'au moment où j'ai dédicé d'utiliser clap
// pour voir si j'y arrivais. C'est un peu brouillon parce que j'ai pas enlevé
// tout le code qui vient du tuto... mais ça marche.

// Le but : recréer grep, un outil en ligne de commande qui prendra deux arguments
// Première argument : le terme recherché
// Deuxième argument : le fichier où l'on cherchera ce terme

// importation correcte
use minigrep;
use minigrep::Config;

extern crate clap;
use clap::{Arg, App,};

// use std::env; // pour interagir avec l'environnement
use std::process; // pour renvoyer un exit status

fn main() {

    let matches = App::new("Minigrep")
        .author("Écrit par moi")
        .version("1.0")
        .about("Rechercher un terme dans un fichier")
        .arg(
            Arg::with_name("case-insensitive")
            .long("case-insensitive")
            .short("i")
            .help("Ignore la casse")
        )
        .arg(
            Arg::with_name("terme")
                .help("Le terme à rechercher")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("file")
                .help("Le nom du fichier à fouiller")
                .required(true)
                .index(2)
        )
        .get_matches();


    let config = Config::nouveau(matches).unwrap_or_else(|err| {
        eprintln!("On a du mal à parser les arguments: {}", err);
        process::exit(1); // un statut de sortie (exit status) de 1 : erreur
    });


    // La fonction run() va s'occuper de la suite. Elle est dans lib.rs
    // soit elle renvoie (), comme la fonction main. Elle fait des choses.
    // soit elle renvoie une erreur, et dans ce cas...
    if let Err(e) = minigrep::run(config) {

        // ...on balance l'erreur vers le flux d'erreur standard, avec eprintln!
        eprintln!("Erreur de l'application: {}", e);
        process::exit(1); // on renvoie un exit status de 1 (erreur)
    }
}
