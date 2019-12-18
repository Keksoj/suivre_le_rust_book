// 2019-07-12

// On veut réparer le problème vu dans sharing_a_mutex_between_threads_fails avec
// le smart pointer Rc<T> (reference counter) qui contiendra le Mutex<T>.
//

use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // La variable counter contient un i32 dans un Mutex<T> dans un Rc<T>
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    // Créons 10 threads avec 10 fois la même closure qui déplace le counter
    // dans le thread APRÈS CLONAGE
    // la valeur contenue par le Mutex
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
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

// Ça plante parce que le type std::rc::Rc<std::sync::Mutex<i32>> n'implémente
// pas le trait std::marker::Send qui permettrait de l'envoyer d'un thread
// à l'autre.
