// Les fonctions s'écrivent en "snake case" : petits caractères, underscores
// la fonction 'main' est le point d'entrée
// On peut déclarer de nouvelles fonctions avec 'fn'. 
// Écrire 'fn main()' définit la fonction main() avec les accolades.


fn main() {
    println!("Hello, world!");
    autre_fonction(5, 6);
}

// La définition de autre_fonction se fait hors de main(). Après ou avant.
// Entre les parenthèses : un ou des PARAMÈTRE(s) dont il faut déclarer le type

fn autre_fonction (x: i32, y: i32) {
    println!("La fonction main appelle 'autre_fonction' qui a un paramètre x: i32\
            , la fonction main lui PASSE la valeur 5, autre_fonction imprime: {}", x);
    println!("Un autre paramètre passé est y : {}", y);
}
