// 2019-01-01
// On va créer un pointeur vide, un DANGLING POINTER, une référence vide.
// Le compilateur va repérer l'erreur. Son rôle est de s'assurer que la donnée 
// ne sorte pas du scope tant que les pointeurs, eux, sont dans le scope.

fn main() {

    // On appelle la fonction dangle(), anglais pour "pendre de façon lâche"
    // La fonction dangle() doit créer la variable reference_a_rien
    let reference_a_rien = dangle();
    println!("{}", reference_a_rien);
}

// la fonction dangle() renvoie une référence à une chaîne de caractère
fn dangle() -> &String {
    
    // s est une nouvelle chaîne de caractères
    let s = String::from("Hello");

    // renvoyer une référene à s
    &s

}   // fin du scope, s est dropé, sa mémoire est nettoyée. 
    // La référence &s est donc vide !

// C'est une histoire de LIFETIME (chap 10). 
// Message d'erreur du compilateur :
// this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
// La solution est de retourner directement la chaîne de caractère : 

// fn dangle() -> String {
//     let s = String::from("Hello");
//     s
// }
