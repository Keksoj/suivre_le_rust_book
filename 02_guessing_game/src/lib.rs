// 2019-03-24

// Deux choses ici. D'un côté, vérifier l'entrée utilisateur avec des fonctions
// associées. De l'autre, gérer la panique.

// Premièrement
// Nous voulons nous assurer que l'entrée utilisateur est un entier naturel
// entre 0 et 100. On peut mettre une boucle if qui ressemble à ceci :

// if guess <1 || guess > 100 {
//             println!("Le nombre doit être entre 0 et 100 !");
//             continue;
//         }

// Mais ça serait ennuyeux de le faire à chaque fois, surtout si le programme
// a fréquemment besoin de cette valeur.
// Créons un nouveau type, le type Guess, et créons une fonction de validation.

// Le struct Guess contient un entier naturel (i32)
pub struct Guess {
    value: i32,
}

impl Guess {

    // La fonction new() prend un entier naturel comme argument et crée une
    // instance du struct Guess. Elle vérifie que l'entier est compris entre
    // 0 et 100, sinon elle panique.
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("La valeur doit être positive, mais on a {}", value);
        } else if value > 100 {
            panic!("La valeur doit être inférieure à 100, mais on a {}", value);
        }

        Guess {
            value
        }
    }

    // IMPORTANT
    // la variable 'value' est privée. Seules les fonctions new() et value()
    // peuvent y accéder. Ainsi, le reste du code ne pourra la régler directement
    // elle devra être créée et vérifiée par new() et récupérée par value()

    // la fonction value() prend une instance de Guess comme argument (&self)
    // et renvoie ce qu'elle contient : le nombre entier naturel
    pub fn value(&self) -> i32 {
        self.value
    }
}


// On veut tester que le code panique effectivement si la valeur est en dehors
// de l'intervalle [0, 100].

#[cfg(test)]
mod tests {
    use super::*;

    // On rédige un test avec [#should_panic], qui vérifie que le code panique
    // effectivement, dans une certaine condition (ici une valeur de 200)
    // On peut rajouter un paramètre expected, à quoi sera comparé le blabla
    // renvoyé par panic! plus haut
    #[test]
    #[should_panic(expected= "La valeur doit être comprise entre 0 et 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
