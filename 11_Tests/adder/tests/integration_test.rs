// 2019-03-24

// Ici, des tests d'intégration, qui étudient le comportement du pogramme
// de l'extérieur, pour ainsi dire.
// Dans le dossier tests/, chaque fichier sera compilé séparément, comme un crate
// à part.
// Les fonctions à tester doivent être dans src/lib.rs, PAS dans src/main.rs

// On peut tester ce fichier en faisant spécifiquement
// cargo test --test integration_test

use adder;

#[test]
fn fait_plus_deux() {
    assert_eq!(4, adder::plus_deux(2))
}
