// 2019-02-04


// Un struct pour contenir abcisse et ordonnée
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// pour définir une méthode dans le struct Point<T>
// on écrit impl<T> pour que rust sache que Point<T> a un type générique
// et non un type concret (qu'est-ce qu'un type concret ?)
impl<T> Point<T> {

    // cette méthode appelée x() renvoie l'abcisse du point
    // la méthode prend comme argument une référence à soi-même : &self
    // et renvoie une référence à une valeur de type T
    fn x(&self) -> &T {
        &self.x
    }
}

// Si on veut définir une méthode qui a besoin de nombres flottant,
// on crée une autre implémentation sur le struct Point<f32>
// pourquoi ? parce que les fonctions utilisées ne fonctionnent qu'avec des flottants
impl Point<f32> {

    // cette fonction calcule la distance à l'origine (avec pythagore)
    fn distance_origine(&self) -> f32 {

        // racine de (x² + y²)
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
// ATTENTION cette méthode n'acceptera que des instances de Point dont les
// coordonnées sont des nombres flottants !

fn main() {
    let p = Point { x: 5, y: 10};
    println!("l'abcisse du point {:?} est {}", p, p.x());

    let punkt = Point { x: 15.2, y: 45.47};
    println!("La distance à l'origine du point {:?} est {}", punkt, punkt.distance_origine());
}
