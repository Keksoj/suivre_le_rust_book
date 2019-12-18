// 2018-12-31
// Cet exemple présente comment prendre puis de rendre l'ownership d'une variable
// en la retournant dans un n-uplet.

fn main() {

    // créer une variable s1, qui aura l'ownership
    let s1 = String::from("Hello");

    // La fonction calcule_longueur() déplace la valeur vers s2
    let (s2, len) = calcule_longueur(s1);

    // s1 est maintenant invalide, c'est s2 qu'on affiche
    println!("La longueur de \"{}\" est {}", s2, len);
}

// On définit la fonction calcule_longueur()
// input  : une chaîne de caractère qu'on baptise 's'
// output : deux variables. Une chaîne de caractères, et un entier.
fn calcule_longueur(s: String) -> (String, usize) {

    // on déclare la variable 'length'
    // on passe l'input, la chaîne de caractère 's', dans la fonction len
    let length = s.len();

    // on retourne deux valeurs dans un n-uplet: l'input et l'output
    (s, length)
}

