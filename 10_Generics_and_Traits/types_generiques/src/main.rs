// 2019-02-03

use std::cmp::PartialOrd;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    println!("Le plus grand nombre est {}", largest(&number_list));

    let autre_liste = vec!['y', 'm', 'c', 'a'];
    println!("Le plus grand caractère est {}", largest(&autre_liste));

}

// une fonction GÉNÉRIQUE pour trouver le plus grand élément d'une liste
// argument : un TYPE GÉNÉRIQUE  auquel on donne un nom plutôt court
// le type générique est défini ainsi : <T>
// &[T] signifie : une référence à un vecteur de... Trucs
// la fonction renvoie une valeur de type... Truc
// On LIE (bind) le type Truc au trait PartialOrd. C'est un TRAIT BOUND
// Il faut rajouter le trait Copy, puisque cette fonction a un type générique,
// et que tous les types ne se copient pas automatiquement.
// Tous les types n'implémentent pas Copy, alors on pourrait aussi utiliser Clone
// mais ça consommerait plus de mémoire sur le tas
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {

    // crée la variable largest et lui attribue la valeur du premier élément de la liste
    let mut largest = list[0];

    // parcours la liste, compare 'largest' à chaque élément et garde le plus grand
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    // renvoie la valeur 'largest'
    largest
}
