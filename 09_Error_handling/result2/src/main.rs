// 2019-01-28

// On refait la même qu'à result mais avec plusieurs erreurs possibles

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // tentons d'ouvrir un fichier qui n'existe pas
    let f = File::open("Hello.txt");
    // open() renvoie un enum qui s'appelle Result et ressemble à ça :
    // enum Result<T, E> {
    //      Ok(T),            renvoie le fichier
    //      Err(E),           renvoie une erreur
    // }

    // On va réagir en fonction des variantes de Result, avec match.
    let f = match f {
        // si open() a renvoyé du std::fs::File,
        // c'est ce qu'on voulait, on garde ça.
        // on sort la valeur 'file' de la variante Ok(),
        // et on l'assigne à la variable f
        Ok(file) => file,

        // si open() a renvoyé une erreur,
        // c'est un type d'erreur défini dans io::Error. Un struct.
        // on réagit en fonction des erreurs disponibles avec error.kind()
        // kind() est une méthode qui renvoie un enum : ErrorKind
        // ErrorKind a une petite vingtaine de variantes
        // liste des variantes : std/io/enum.ErrorKind.html
        Err(error) => match error.kind() {

            // Une erreur possible et probable, c'est Notfound
            // On en profite pour créer le fichier
            // On gère les erreurs de la création de fichier, tant qu'à faire
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("On a bien tenté de le créer mais raté :{}", e),
            },

            // En cas d'autre erreur
            other_error => panic!("Il y a eu une autre erreur ! {:?}", other_error)
        }
    };

    // on a fini avec la gestion d'erreur pour ouvrir ce fichier ! Pfiou !
    // pour éviter tant de match, on a des closures, voir chapitre 13
}
