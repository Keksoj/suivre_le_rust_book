// Ici, des

// Une fonction test
#[cfg(test)]
mod tests {

    // Il faut amener TOUT le contenu de lib.rs dans le scope du module 'tests'
    use super::*;

    // Enfonçons des portes ouvertes
    #[test]
    fn it_works() {
        // assert_eq! compare deux valeurs (valeur1, valeur2)
        assert_eq!(2 + 2, 4);
    }


    // La fonction can_hold renvoie une valeur booléenne, ce qui est parfait pour
    // utiliser la macro assert!
    // On va créer des instances de Rectangle, et affirmer qu'elles peuvent se
    // contenir
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { largeur: 8, longueur: 7};
        let smaller = Rectangle { largeur: 5, longueur: 1};

        // Entre les parenthèses d'assert! on doit avoir une valeur bolééene
        assert!(larger.can_hold(&smaller));
    }

    // Faisons l'inverse
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { largeur: 8, longueur: 7};
        let smaller = Rectangle { largeur: 5, longueur: 1};

        // Noter que les arguments sont précédés d'un !
        // Il vient nier le résultat (negate) avant de le passer à assert!
        assert!(!smaller.can_hold(&larger));
    }
}

// Définition du struct Rectangle
#[derive(Debug)]
pub struct Rectangle {
    pub largeur: u32,
    pub longueur: u32,
}

impl Rectangle {
    // définit la méthode can_hold(). Voyons les paramètres
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.largeur > other.largeur && self.longueur > other.longueur
    }
}
