// 2019-07-07
// Attention, nuance entre MyBox<T> et Box<T>.

use std::ops::Deref;

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // ce qui équivaut à écrire :
    assert_eq!(5, *(y.deref()));
}

// le type Box<T> est défini comme un struct de n-uplet à un élément
struct MyBox<T>(T);
// Cependant, ce type ne peut pas être déréférencé si simplement avec * !

// On implémente pour MyBox
impl<T> MyBox<T> {

    // une fonction qui met en boîte
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Pour déréférencer MyBox, il faut implémenter le trait Deref
impl<T> Deref for MyBox<T> {
    type Target = T;


    // la méthode deref emprunte self et renvoie une référence à la donnée
    // contenue dans MyBox
    fn deref(&self) -> &T {
        &self.0
    }
}
