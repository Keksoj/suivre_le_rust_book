// 2019-01-09

// On veut parcourir les variantes de Option<T> avec match. 
// Écrivons une fonction qui prend une Option<i32> et incrémente la valeur contenue
// s'il y en a une. S'il n'y en a pas, la fonction renverra juste la valeur None.

// La fonction accepte une variante d'Option<i32> comme paramètre. Un entier.
// Elle renvoie la même chose, Option<i32>
fn incremente(x: Option<i32>) -> Option<i32> {

    // match va parcourir les variantes d'Option<>
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let cinq = Some(5); 

    let six = incremente(cinq); // parcours les options de match, tombe sur 
                                // Some(i) et décide d'incrémenter.

    let none = incremente(None);

    println!("Difficile d'imprimer Option<T>, venez voir le code.");
}

// C'est très utilisé en rust, combiner match et des enums. 
