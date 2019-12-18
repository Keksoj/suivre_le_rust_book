// 2019-01-01
// On va enfin finir ce programme d'extraction du premier mot (voir slices_besoin)

// On veut une fonction qui retourne le premier mot d'une chaine de caractères.
// La fonction scanne la chaine jusqu'à trouver un espace.
// S'il n'y a pas d'espace, la chaine entière est retournée.

// importer la bibliothèque entrée/sortie
use std::io;

fn main() {
    // Proposons à l'utilisateur de taper une chaîne de caractère.
    println!("Tapez n'importe quoi et faites [Entrée].");

    // Définir la variable s comme une chaîne de caractères
    let mut s = String::new();

    // récupérer l'entrée tapée par l'utilisateur, la mettre dans la variable 's'
    io::stdin()
        .read_line(&mut s)
        .expect("Pas pu lire l'entrée");

    // Appel de la fonction premier_mot() et affichage du résultat
    // mot est une chaîne de caractère maintenant !
    let mot = premier_mot(&s);
    println!("Le premier mot de la chaîne est '{}'", mot);
}


// La fonction premier_mot() va renvoyer une tranche de l'input, son premier mot
// l'input est une référence à une tranche (ça accepte les chaînes de caracatères)
// l'output est aussi une tranche, une slice
fn premier_mot(s: &str) -> &str {
    
    // convertir la chaîne de caractère en tableau d'octets (array of bytes)
    let octets = s.as_bytes();

    // la boucle for parcourt le tableau 'octets' à la recherche d'un espace
    // ne pas trop tenir compte de la première ligne
    // i est l'index de chaque octet, donc de chaque lettre
    // l'index du premier espace venu correspond à la longueur du premier mot
    for (i, &item) in octets.iter().enumerate() {
    
        // b' ' est la syntaxe pour les octets, ici un espace
        // &s[0..i] est une tranche, une RÉFÉRENCE à un morceau de s
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    // si on n'a pas trouvé d'espace, c'est qu'il n'y a qu'un mot dans la chaine.
    // autant retourner la chaîne entière
    &s[..]
}

// L'intérêt dans tout ça ? Le compilateur trouvera les erreurs plus tôt si des
// variables interdépendantes partent en couille.
// Anecdote : les LITERALS, variables textes mis sur le stack, sont des tranches.
