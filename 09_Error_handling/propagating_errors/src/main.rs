// 2019-01-29
// 


use std::io;
use std::io::Read;
use std::fs::File;
use std::fs;

// on définit une fonction qui renvoie :
// soit une chaîne de caracètre, sous forme emballée : Ok(String)
// soit une erreur
fn read_username_from_file() -> Result<String, io::Error> {

    // tente d'ouvrir le fichier
    let f = File::open("hello.txt");

    // fait un match sur le Result<Ok, Err> renvoyé par open()
    let mut f = match f {
        Ok(file) => file,           // renvoie le contenu si Ok
        Err(e) => return Err(e),    // renvoie le message d'erreur si Err
                                    // ceci ne déballe pas le message d'erreur !
    };                              // on la renvoie comme telle
                                    // on appelle ça "propager une erreur"
                                    // à cause de return, on s'arrête là

    // SI TOUT VA BIEN,
    // On crée une nouvelle chaîne s qui sera la String finale
    let mut s = String::new();

    // on scrute f pour vérifier si c'est une chaîne de caractère
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),     // si c'est une chaîne, on attribue sa valeur à s
        Err(e) => Err(e),   // si f n'est pas une chaîne,
                            // read_to_string renvoie une erreur
    }
}

// On peut faire la même chose de façon plus simple, avec ?
// Tentons d'ouvrir guten.txt
fn lire_d_un_fichier() -> Result<String, io::Error> {
    let mut f = File::open("guten.txt")?;           // noter le  ?
    let mut s = String::new();
    f.read_to_string(&mut s)?;          // ? fait la même chose que match plus haut
    Ok(s)                               // et en plus il convertit le message d'erreur
}                                       // avec From.
// ? n'est utilisable qu'avec des fonctions qui renvoient Result<T, E>
// ? NE S'UTILISE PAS DANS MAIN() !!!

// et on peut carrément raccourcir en païpant comme des gros sales
fn autre_fonction() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("guten.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
// Il faut avouer que ça va plus vite. Et il y a ENCORE plus court,
// avec une function de std::fs
// std::fs::read_to_string() fait tout ça :
    // ouvrir le fichier
    // crée une nouvelle chaîne
    // lit le fichier et met son contenu dans la chaîne
    // renvoie l'erreur, bien sûr
fn enieme_fonction() -> Result<String, io::Error> {
    fs::read_to_string("helo.txt")
}

fn main() {
    println!("{:?}", read_username_from_file());
    println!("{:?}", lire_d_un_fichier());
    println!("{:?}", autre_fonction());
    println!("{:?}", enieme_fonction());
}
