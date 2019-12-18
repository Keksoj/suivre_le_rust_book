// 2019-01-28

// l'enum Result permet de gérer des erreurs récupérables.

use std::fs::File;

fn main() {
    // tentons d'ouvrir un fichier qui n'existe pas
    let f = File::open("Hello.txt");
    // open() renvoie un enum qui s'appelle Result et ressemble à ça :
    // enum Result<T, E> {
    //      Ok(T),
    //      Err(E),
    // }

    // On va réagir en fonction des variantes de Result, avec match.
    // Result a été amené dans le scope, pas besoin d'écrire Result::Ok
    // ou Result::err
    let f = match f {
        // si open() a renvoyé du std::fs::File,
        // c'est ce qu'on voulait, on garde ça.
        // on sort la valeur 'file' de la variante Ok(),
        // et on l'assigne à la variable f
        Ok(file) => file,

        // si open() a renvoyé une erreur, on fait deux choses :
        // on panique avec un message customisé
        // on affiche l'erreur renvoyée par open()
        Err(error) => {
            panic!(
                "Il y a un souci au moment d'ouvrir le fichier.\n{:?}",
                error
            );
        }
    };
}
