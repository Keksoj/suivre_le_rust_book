// 2019-07-08

// Un programme avec
    // un thread qui génère des valeurs et les envoie dans une channel
    // un thread qui les reçoit et les affiche

// mpsc signigie Multiple Producer, Single Consumer
// Une channel peut avoir plusieurs extrémités qui produisent des valeurs
// mais UNE SEULE extrémité qui les consomme.
use std::sync::mpsc;
use std::thread;

fn main() {
    // La fonction mpsc::channel() renvoie un n-uplet
        // premier élément: l'extrémité qui émet (transmitter)
        // deuxième élément: l'extrémité qui reçoit (receiver)
    let (tx, rx) = mpsc::channel();

    // ENVOYER
    // Déplaçons l'extrémité émettrice tx dans un nouveau thread
    thread::spawn(move || {
        let val = String::from("hi");
        // la méthode send() prend la valeur et renvoie Result<T, E>
        // l'erreur est utile s'il n'y a personne pour recevoir, mais ici on
        // utilise unwrap() par commodité.
        tx.send(val).unwrap();

        // send() a pris l'ownership. Une fois la valeur envoyée, on ne peut plus
        // l'utiliser. Ceci est interdit:
        // println!("{}", val);
    });

    // RECEVOIR
    // L'extrémité receptrise, rx, a deux méthodes utiles.
    // recv() blocque l'exécution du thread principal dans l'attente d'une valeur
    // De même, recv() renvoie Result<T, E> (erreur en cas de channel fermée)
    // try_recv() n'attends pas, renvoie Ok(valeur) ou Err s'il n'y a rien à
    // recevoir.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

}
