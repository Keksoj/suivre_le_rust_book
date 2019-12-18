// 2019-02-04

// On veut définir un struct Point, qui contienne une abcisse et une ordonnée
// x et y sont de deux types génériques, T et U, qu'on définira
// l'un peut être entier et l'autre flottant, par exemple

struct Point<T, U> {
    x: T,
    y: U,
}



fn main() {
    let toutentier = Point { x: 4, y: 24};
    let toutflottant = Point { x: 4.45, y: 5.48};
    let mixte = Point { x: 4, y: 5.48};
}
