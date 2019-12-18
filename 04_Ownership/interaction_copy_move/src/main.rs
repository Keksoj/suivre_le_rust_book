// L'intérêt de cette partie est d'illustrer l'ownership
// On alloue des parties du heap à des variables. Les variables les POSSÈDENT. (own)

fn main() {

    // Ça c'est facile :
    let x = 5;
    let y = x;
    println!("On a déclaré x = 5, puis y = x, et on peut afficher y : {}", y);
    // Ces données sont rangées sur le stack.
    // On dit que x est COPIÉ (copied)
    // on peut copier les entiers, les flottants, les boléens, les char
    // on peut copier les n-uplets s'ils contiennent des trucs copiables
    // on peut copier avec le TRAIT qui s'appelle Copy
    // on peut empêcher la copie avec le trait Drop

    // Mais peut-on faire ça ?
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);

    // Oui mais c'est un peu compliqué. Voyons.
    // La variable s1 est composée d'un pointeur, d'une longueur, d'une capacité.
    // Pointeur, longueur est capacité sont mis sur le stack
    // Le pointeur indique où est rangée la variable dans le heap
    // La longueur ici c'est 5 octet.
    // La capacité c'est 5 (dans ce contexte on peut l'ignorer)
    // Il y a donc un coin du heap qui contient hello

    // Quand on fait let s2 = s1, on recopie la donnée String sur le stack
    // On recopie le pointeur, la longueur, la capacité.
    // Mais on ne touche pas au heap, qui contient toujours hello au même endroit.    

    // PROBLÈME : Quand on sortira du scope, il faudra libérer deux fois le heap !
    // On appelle ça le DOUBLE FREE ERROR. Il faut l'éviter.
    // Rust va simplement invalider s1. Il DÉPLACE s1 dans s2.
    // s1 is MOVED into s2.

    println!("On a déclaré\n\
                let s1 = String::from(\"Hello\");\n\
          puis\nlet s2 = s1;\nce qui a déplacé s1 dans s2 (move).\n\
          voici s2: {}", s2);

} // maintenant s2 est hors de vue (out of scope), le heap est libéré.





