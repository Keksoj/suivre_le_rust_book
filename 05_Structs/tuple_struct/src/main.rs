// 2019-01-02

// Les tuples structs sont comme des n-uplets qu'on peut nommer.
// Ce code ne fait rien mais c'est pas grave.
// Les tuples structs s'utilisent à peu près comme des tuples ordinaires.

fn main() {

    // Définit des tuples structs
    struct Couleur(i32, i32, i32);
    struct Point(i32, i32);

    // Crée des instances
    let black = Couleur(0, 0, 0);
    let origine = Point(0, 0);
}
