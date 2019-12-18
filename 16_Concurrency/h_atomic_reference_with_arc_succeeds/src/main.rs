// 2019-07-12

// Voici enfin la solution à notre problème. Rc<T>, le reference counter, ne
// suffisait pas. Voici Arc<T>, pour Atomic Reference Counter.
// Les types atomiques fonctionnent comme les types primitifs (u8, i32, etc)
// mais sont safe à échanger entre les threads (au prix d'une perte de perfor-
// mance).

use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // La variable counter est un i32 dans un Mutex<T> dans un Arc<T>
    // À noter : counter est immutable, mais sera modifié tout de même, car
    // Mutex fournit la mutabilité en son intérieur.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Créons 10 threads avec 10 fois la même closure qui déplace le counter
    // dans le thread APRÈS CLONAGE.
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
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
