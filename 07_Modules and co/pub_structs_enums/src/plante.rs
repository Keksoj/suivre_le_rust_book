// 2019-01-17 Déplacé le module dans son fichier à lui

// un struct est rendu public par pub, mais pas ses champs

// pas besoin d'annoncer la couleur avec  mod plante { }
// En effet, on est l'a déjà fait avec    mod plante;    dans main.rs


#[derive(Debug)]    // pour afficher les structs

pub struct Legume {         // définit un struct public
    pub nom: String,        // le nom du légume est public
    id: i32,                // son ID ne l'est pas
}

impl Legume {                           // un bloc d'implémentation dans le
                                        // contexte du struct Legume. Définit
                                        // la fonction new(). Noter 'pub'
    pub fn new(nom: &str) -> Legume {   // prend un &str comme argument et 
        Legume {                        // construit une instance de Legume
            nom: String::from(nom),
            id: 1,
        }
    }
}

