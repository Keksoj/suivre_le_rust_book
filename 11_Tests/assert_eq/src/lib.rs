// 2019-03-21

// Définissons la fonction plus_deux
pub fn plus_deux (a: i32) -> i32 {
    a + 2
}

// Testons-la
#[cfg(test)]
mod tests {
    use super::*;

    // assert_eq! s'assure que deux valeurs sont égales
    #[test]
    fn ca_fait_plus_deux() {
        assert_eq!(4, plus_deux(2));
    }

    // À l'inverse, assert_ne! s'assure que deux valeurs ne sont PAS égales.
    #[test]
    fn ca_fait_pas_plus_deux() {
        assert_ne!(5, plus_deux(2));
    }
}
