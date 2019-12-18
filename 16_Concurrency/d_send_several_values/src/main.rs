// 2019-07-08

// On reprend le code de message_passing_with_channels
// mais cette fois on veut envoyer plusieurs valeurs
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // La fonction mpsc::channel() renvoie un n-uplet
        // premier élément: l'extrémité qui émet (transmitter)
        // deuxième élément: l'extrémité qui reçoit (receiver)
    let (tx, rx) = mpsc::channel();

    // ENVOYER AVEC UN PREMIER ÉMETTEUR
    // On clone l'émetteur ainsi:
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        // Créons plusieurs valeurs
        let valeurs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("thread"),
        ];
        for valeur in valeurs {
            tx1.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // ENVOYER AVEC UN DEUXIÈME ÉMETTEUR
    // Pas besoin de le cloner, celui-là
    thread::spawn(move || {
        // Créons plusieurs valeurs
        let valeurs = vec![
            String::from("Some"),
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you!"),
        ];
        for valeur in valeurs {
            tx.send(valeur).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // RECEVOIR
    for received in rx {
        println!("Got: {}", received);

    }

}
