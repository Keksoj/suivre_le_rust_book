// 2019-03-24

// Créons des fonctions que nous testerons individuellement (test private functions)
// C'est un sujet discuté chez les programeurs, tout le monde ne fait pas ça

// Une fonction d'addition. Noter qu'elle est privée.
fn additionneur(a: i32, b: i32) -> i32 {
    a + b
}

// Une fonction +2 qui reprend la fonction d'addition. Celle-ci est publique.
pub fn plus_deux(a: i32) -> i32 {
    additionneur(a, 2)
}



#[cfg(test)]
mod tests {

    use super::*;

    // On teste la fonction plus_deux (et, passant, la fonction d'addition)
    #[test]
    fn interne() {
        assert_eq!(4, plus_deux(2));
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn autre() {
        panic!("Fait faillir ce test");
    }
}
