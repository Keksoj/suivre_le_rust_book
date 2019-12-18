// 2019-01-09

// On va présenter l'intérêt de if let

fn main() {

    // Voyons le cas de figure à remplacer.
    // On fait passer match pour éxécuter du code si une_valeur_u8 égale 3
    // Définir la variable 3u8 veut dire "entier positif à 8 bits, valeur 3"
    // La définir avec Some() permet de... et bah en fait ça change rien.
    // Ce code fonctionne très bien sans écrire Some().
    let une_valeur_u8 = Some(3u8);

    // faire passer la variable dans match
    match une_valeur_u8 {
        Some(3) => println!("Trois !"),
        _ => (),
    }

    // Ce code avec match, il marche très bien, mais c'est un peu overkill
    // if let permet d'éxécuter du code en fonction d'une seule condition

    if let Some(3) = une_valeur_u8 {   // mettre les termes dans cet ordre
        println!("Trois encore !");    // un seul signe égal !
    } else {
        println!("C'était pas trois"); // on peut utiliser else
    }
}
