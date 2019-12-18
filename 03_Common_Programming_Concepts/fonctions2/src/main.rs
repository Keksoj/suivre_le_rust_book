// Function bodies are made up of a series of statements
// optionally ending in an expression.
// Le corps d'un fonction est faite d'une suite de DÉCLARATIONS (ex : let y = 6)
// qui peuvent finir par un EXPRESSION qui renvoie une valeur.

fn main() {
    // voici une DÉCLARATION. Ne renvoie pas de valeur (contrairement à C et Ruby)
    // se termine par un point-virgule
    let x = 5;

    // voici, suite à une déclaration, une EXPRESSION entre accolades
    // elle renvoie une valeur (ici 4)
    // elle ne se termine pas par un point-virgule ! Celui-ci finit la déclaration.
    let y = {
        let x = 3;
        x + 1
    };

    println!("La valeur de y : {}", y);

//Les fonctions qui renvoient des valeurs

    // On appelle la valeur de retour de la fonction cinq(), définie plus bas
    let w = cinq();
    println!("La valeur de w est {}", w);

    // Même chose, autre exemple
    let z = plus_un(7);
    println!("La valeur de z, 7+1, est {}", z);

}

// Définition de cinq() avec fn
// la flèche -> indique que la fonction RENVOIE une valeur.
// On ne nomme pas la valeur de retour, mais on déclare son type (i32 ici)
// 5 est une EXPRESSION qui renvoie la valeur 5
fn cinq() -> i32 {
    5
}

// Définition de plus_un()
// x + 1 est une expression qui renverra une valeur de retour
fn plus_un(x: i32) -> i32 {
    x + 1
}



