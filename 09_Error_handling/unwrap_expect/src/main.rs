// 2019-01-29

// Le type Result<T, E> a des méthodes qui facilitent la tâche et permettent
// de ne pas avoir à utiliser match tout le temps. unwrap en est une.
// unwrap() récupère la valeur contenue dans Ok() et la renvoie
// En cas de Err(), unwrap appelle panic! pour nous

use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();

    // on peut aussi utiliser expect() et définir le message d'erreur
    // même chose : soit ça marche et expect renvoie le fichier
    // soit ça ne marche pas et expect renvoie le message d'erreur
    let m = File::open("hi.txt").expect("pas pu ouvrir le fichier hi.txt");
}
