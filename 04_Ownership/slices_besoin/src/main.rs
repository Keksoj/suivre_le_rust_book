// 2019-01-01
// On va voir ici pourquoi on a besoin des slices.

// On veut une fonction qui retourne le premier mot d'une chaine de caractères.
// La fonction scanne la chaine jusqu'à trouver un espace.
// S'il n'y a pas d'espace, la chaine entière est retournée.

fn main() {

    // Créons une chaîne de caractères s
    let s = String::from("Hello, world!");
    println!("Nous avons une chaîne de caractères : '{}', \
              et nous voulons en extraire le premier mot.", s);

    // Appel de la fonction premier_mot() et affichage du résultat
    // mot est de type usize, donc un entier positif (64 bits sur mon archi)
    let mot = premier_mot(&s);
    println!("Le premier mot a une longueur de {} (en comptant la virgule)", mot);

}


// La fonction premier_mot() va renvoyer l'index de la fin du premier mot
// l'input est une référence à chaîne de caractère (on ne veut pas l'ownership)
fn premier_mot(s: &String) -> usize {
    
    // convertir la chaîne de caractère en tableau d'octets (array of bytes)
    let octets = s.as_bytes();

    // Là je comprends pas très bien mais c'est pas grave.
    // la boucle for parcourt le tableau 'octets' à la recherche d'un espace
    // i est l'index de chaque octet, donc de chaque lettre
    // l'index du premier espace venu correspond à la longueur du premier mot
    // iter et enumerate sont des MÉTHODES
    for (i, &item) in octets.iter().enumerate() {
    
        // b' ' est la syntaxe pour les octets, ici un espace
        if item == b' ' {
            return i;
        }
    }
    
    // si on n'a pas trouvé d'espace, c'est qu'il n'y a qu'un mot dans la chaine.
    // autant retourner la longueur de la chaîne.
    s.len()
}

// Tout ça c'est très joli et ça compile bien, mais dans le scope de main(), 
// la variable 'mot' est dépendante de s. Si l'on nettoie s, mot perd sa validité !
// Et si l'on veut extraire un mot et qu'il faut deux index, début et fin de mot ?
// C'est pour ça qu'on a besoin des tranches, ou slices.




