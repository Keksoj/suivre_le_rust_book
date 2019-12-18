//2019-07-08

use std::thread;
use std::time::Duration;

fn main() {
    // pour créer un nouveau thread, on appelle la méthode spawn() est on lui
    // passe une closure qui contient le code qu'on veut y exécuter.
    thread::spawn(|| {
        for i in 1..10 {
            println!("Thread non joint: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    // Ce nouveau thread sera arrêté dès que le thread principal, ci-dessous,
    // aura fini !

    // On peut résoudre ce problème en sauvegardant la valeur renvoyée par le
    // thread dans une variable de type JoinHandle
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Ce thread a été joint par handle.join(): {}", i);
            thread::sleep(Duration::from_millis(60));
        }
    });

    for i in 1..10 {
        println!("Thread principal: {}", i);
        thread::sleep(Duration::from_millis(50));
    }

    // on appelle la méthode join() sur la variable handle
    // Cette variable va attendre que son thread soit fini avant de finir main()
    handle.join().unwrap();
    // Si on avait mis cette ligne avant la boucle for si-dessus, celle-ci aurait
    // du attendre la fin du thread joint.
}
