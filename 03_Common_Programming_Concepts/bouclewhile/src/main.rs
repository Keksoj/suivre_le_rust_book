// On va tester une condition dans une boucle. 
// Tant que (while) la condition est vraie (true), la boucle tourne.

fn main() {

    // définir une variable modifiable
    let mut nombre = 8;

    // déclaration de la boucle
    // l'EXPRESSION 'nombre =! 0' renvoie une valeur booléenne, true ou false
    // la boucle s'arrêtera quand l'expression renverra false
    while nombre != 0 {
        
        // afficher le nombre
        println!("{}", nombre);

        // désincrémenter. C'est une déclaration.
        nombre = nombre - 1;
    }

    println!("On dégage !");

}
