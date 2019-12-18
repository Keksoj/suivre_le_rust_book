// 2019-07-11
// Le fameux cours sur les Mutex !

use std::sync::Mutex;


fn main() {
    let m = Mutex::new(5);

    {
        // la méthode lock() permet d'acquérir les droits de changement sur les
        // données. Cette méthode renvoie la valeur contenue par le Mutex,
        // on la traite comme une référence mutable.
        // Lock() renvoie un smart pointer appellé MutexGuard, emballé dans un
        // LockResult qu'on déballe avec unwrap()
        // ce MutexGuard a la bonne idée d'implémenter Deref et Drop. Le lock sera
        // relâché automatiquement dès qu'on sortira du scope.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
