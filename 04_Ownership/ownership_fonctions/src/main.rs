// Une fonction peut s'approprier la possession d'une variable
// 

fn main() {
    let s = String::from("Hello");  // s possède une partie de la mémoire
    imprime_une_chaine(s);          // la fonction s'approprie l'ownership de s...
                                    // fonction terminée, s est en dehors du scope
                                    // s est invalide

    let x: i32 = 4;                 // x rentre dans le scope
    imprime_un_nombre(x);           // x est copié dans la fonction, pas déplacé
    println!("{}", x);              // x est toujours valide
}

// Définir la fonction imprime_une_chaine()
// (une_chaine: String) fait qu'elle n'accepte que des chaînes de caractères
// imprime_une_chaine() va récupérer l'ownership de s
// s sera DÉPLACÉ dans la fonction
// une fois la fonction terminée, la variable s sera invalide
fn imprime_une_chaine(une_chaine: String) {
    println!("{}", une_chaine);
}

// La fonction imprime_un_nombre accepte un entier comme input
// Or, les entiers sont gardés sur le stack, il n'y a pas d'ownership
// La fonction fait une COPIE
// Une fois la fonction terminée, la variable ne sera pas invalide
fn imprime_un_nombre(un_entier: i32) {
    println!("{}", un_entier);
}
