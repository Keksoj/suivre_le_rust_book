// Un exemple d'ownership
// Jusqu'ici, les données étaient gardées dans le stack, peu complexes.
// Les données textes comme "Hello world!" étaient des LITERALS, non modifiables.
// Une fois utilisées, elles étaient enlevées du stack (popped off the stack).
// Comment mettre de la données dans le heap ? Voyons avec le type String.
// String contient du texte d'une longueur inconnue à la compilation.


fn main() {

    // Créer une String à partir d'un STRING LITERAL (entre guillemets)
    // on utilise la fonction FROM
    // pour les doubles points, voir le chapitre 5
    // Dès le moment où l'on fait ça, la variable s POSSÈDE un espace mémoire,
    // et ce, tant qu'elle est en vue (in scope)
    let mut s = String::from("Helloooooooo");

    // cette variable est modifiable (mutable), ajoutons-lui du texte
    // push_str() est une méthode (à vérifier)
    s.push_str("\n , world!\nOn peut mettre des retours à la ligne avec \\n ! \
                \nCette variable String a été modifiée grâce à son placement \
                dans le heap.");

    // voyons voir ce que ça donne
    println!("{}", s);

}   // Nous sortons du scope, s est ivalide, la mémoire allouée à s est libérée
    // Une fonction se charge de ce travail automatiquement, la fonction DROP


