// 2019-07-08

// move permet d'utiliser la donnée d'un thread dans un autre, ownership comprise
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // En rajoutant le mot-clé move, on oblige la closure à prendre l'ownership
    // de v.
    let handle = thread::spawn(move || {
        println!("Voici le vecteur v: {:?}", v);
    });

    // Maintenant, main() ne peut plus utiliser v. Cette ligne est interdite:
    // println!("Voici le vecteur v: {:?}", v);

    // Si l'on ne joint pas le thread créé, rien ne se passera !
    handle.join().unwrap();
}
