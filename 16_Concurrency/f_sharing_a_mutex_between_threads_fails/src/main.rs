// 2019-07-12

// Cet exemple ne compile pas, c'est pour bien comprendre comment ça marche.

use std::sync::Mutex;
use std::thread;

fn main() {
    // La variable counter contient un i32 dans un Mutex<T>
    let counter = Mutex::new(0);
    let mut handles = vec![];

    // Créons 10 threads avec 10 fois la même closure qui déplace le counter
    // dans le thread, acquiers un verrou (lock) sur le Mutex<T>, et ajoute 1 à
    // la valeur contenue par le Mutex
    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        // On rassemble toutes les handles (les identifiants de thread) dans un
        // vecteur appelé handles
        handles.push(handle);
    }

    // Rejoindre toutes les handles, c'est-à-dire : attendre leur exécution à
    // toutes
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

// Le résultat évidemment, c'est qu'à déplacer (move) la variable counter, on ne
// peut la réutiliser dans la deuxième itération de la boucle for.
