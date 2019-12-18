// 2019-04-10

// Le smart pointer le plus habituel est la Box<T>, qui garde de la donnée sur
// le tas plutôt que sur la pile. C'est tout. On s'en sert :
    // quand on ne connaît pas la taille d'un type à la compilation
    // quand on a beaucoup de donnée dont on veut transférer l'ownership mais
        // sans que ce soit copié.
    // quand on veut posséder une valeur, mais le type a moins d'importance que
        // le trait implémenté par le type

use List::{Cons, Nil};

fn main() {

    // La syntaxe
    // voici une box qui garde une valeur i32 sur le tas (heap)
    // Le type est inféré par rustc, ici c'est le struct std::boxed::Box<i32>
    let b = Box::new(5);
    println!("b = {}", b);
    // le tas sera nettoyé, et le pointeur supprimé, quand on sortira du scope
    // Évidemment, on ne va pas se contenter d'une seule valeur

    // De la récursivité, en veux-tu en voilà. Ce sont les CONS LISTS !
    let liste = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil)
                ))
            ))
        );

    println!("{:#?}", liste);
}

// Les élements d'une cons list sont :
    // soit une cons list
    // soit une valeur nulle, le Nil

// Normalement, une telle liste récursive a une taille infinie, c'est ingérable
// On utilise Box<List> pour limiter la taille de List
// Box<List> ne contient que des pointeurs, de taille connue
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
