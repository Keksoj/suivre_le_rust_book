fn main() {
    // Suite du cours : les méthodes qui produisent d'autres itérateurs
    // Créons d'abord un nouveau vecteur
    let v2 : Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Voici le vecteur 2 : {:?}", v2);

    // La méthode map() prend comme argument une fermeture (closure) et crée un
    // un itérateur qui appelle cette fermeture sur chaque élément.
    v2.iter().map(|x| x + 1);
    // Ce truc est un struc de type
        // std::iter::Map<std::slice::Iter<'_, i32>, [closure@src/main.rs:39:28: 39:37]>
    // Si on l'affiche avec {:?}, on a ça :
        // Map { iter: Iter([1, 2, 3, 4, 5]) }
    // On appelle ce truc un ITERATOR ADAPTOR (un adapteur d'itérateur ?), il est
    // lazy, paresseux, et doit être consommé.

    // Accolons à ce truc la méthode collect(), on fabrique un vecteur avec
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
    println!(
        "Appliquons ce truc au vecteur 2 : v2.iter().map(|x| x + 1).collect()
C'est un iterator adaptor. map() prend une fermeture comme argument, collect()
rassemble tout ceci dans le vecteur 3, avec cette ligne de code :
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
Le vecteur 3 ressemble donc à : {:?}",
        v3);

    
}
