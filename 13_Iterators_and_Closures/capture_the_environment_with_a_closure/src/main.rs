// 2019-04-05

// Les fermetures (closures) sont capable de capturer leur environnement et
// d'accéder à des variables du scope où elles sont définies

fn main() {
    let x = 4;

    // Définition de la fermeture.
    // dans z == x, x n'est pas une variable, la fermeture a le droit de l'utiliser
    let egal_a_x = |z| z == x;

    let y = 4;

    assert!(egal_a_x(y));

}
