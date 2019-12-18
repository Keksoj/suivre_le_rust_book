// On va étudier l'intérêt de la boucle for en partant de la boucle while.

// CE QU'ON PEUT FAIRE
// Utiliser la boucle while pour passer un tableau en revue.
// fn main() {
    
    // // définition du tableau (array) et de l'index (on commence à zéro)
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
        // println!("La valeur n°{} du tableau est {}", index, a[index]);

        // // incrémentation
        // index = index + 1;
    // }
// }

// ERREUR POSSIBLE : se planter dans l'index

// CE QUI EST MIEUX
// Utiliser la boucle for pour scanner le tableau
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("valeur du tableau : {}", element);
    }

    // On peut aussi utiliser un RANGE, un truc comme ça : 1..4
    // Étonnant, le début du range est inclusif, le haut est exclusif
    // Ce code ne compte que jusqu'à 3 !
    for nombre in 1..4 {
        println!("J'apprends à compter jusqu'à...{}", nombre);
    }

    // La même boucle for, mais à l'envers pour faire un compte à rebourg !
    // rev est une MÉTHODE encore pas vue. rev pour reverse, sans doute
    // le range 1..11 est exclusif en haut, sa valeur la plus haute est 10
    for zahl in (1..11).rev() {
        println!("Décollage dans {} secondes...", zahl);
    }
    println!("Décollaaaaaaaaaage !");



}
