// 2019-07-14
// Le but du but : le serveur web multi-thread.

// La bibliothèque standard propose un module pour écouter une connection TCP
use std::net::TcpListener;
use std::net::TcpStream;

// le prélude d'io donne accès à des traits qui permettent de lire et d'écrire
// le flux (stream)
use std::io::prelude::*;

// Pour lire le fichier hello.html
use std::fs;

fn main() {
    // La méthode bind du struct std::net::TcpListener crée un nouvel écouteur
    // L'argument est une adresse IP (socket address), dont on précise le port
    // (si le port n'est pas précisé, c'est à l'OS de décider)
    // On appelle ça "binding to a port"
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // la méthode incoming() renvoie un itérateur qui donne une séquence de flux
    // Les flux sont de type TcpStream.
    // Un flux (stream) est une connexion ouverte entre client et serveur
    // Une connexion (connection) c'est le processus requête-réponse-fermeture
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // On passe chaque flux à la fonction handle_connection
        handle_connection(stream);
    }
}

// cette fonction lit le flux, en l'occurence une requête HTTP,
// et y écrit une réponse HTTP
fn handle_connection(mut stream: TcpStream) {
    // Ce buffer de 512 octets est sur la pile (stack)
    let mut buffer = [0; 512];

    // lire le flux se fait en deux étapes:
    // La méthode read() lit les octets du flux (type TcpStream) et les met
    // dans le buffer.
    stream.read(&mut buffer).unwrap();
    // Afficher la requête HTTP
    // On convertit les octets du buffer dans une chaine de caractères avec la
    // méthode String::from_utf8_lossy(), qui prend une référénce à un tableau
    // d'octets &[u8] comme argument, et produit une chaîne de caractères.
    // "lossy" car ce qui n'est pas UTF-8 sera remplacé par �
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    // Envoyer une réponse HTTP
    // Voir le readme pour la composition de la réponse
    let response = "HTTP/1.1 200 OK\r\n\r\n"; // type: &str
    // On introduit notre page HTML
    let contents = fs::read_to_string("hello.html").unwrap();

    // Je concatène les deux avec format! (différent du tuto ici)
    let complete_response = format!("{}{}", response, contents);
    // La méthode write() écrit le contenu d'un buffer dans le flux
    stream.write(complete_response.as_bytes()).unwrap();
    // flush() attend que tout soit écrit avant de laisser le programme tourner
    stream.flush().unwrap();

}
// Voir le README
// Tout ça c'est bien mais on veut customiser la réponse du serveur
