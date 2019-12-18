// 2019-04-05

// Toute l'idée est de parcourir les valeurs d'un vecteur

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4];

    println!("Voici le vecteur : {:?}", v1);

    // Créons un itérateur qui parcours v1, il est stocké dans la variable v1_iter
    // Cet itérateur est paresseux (lazy)
    let v1_iter = v1.iter();

    // L'itérateur est "consommé" par la méthode sum() parce qu'elle utilise la
    // méthode next(), qui passe d'un item à l'autre. Les méthodes qui utilisent
    // next() sont donc appelées des CONSUMING ADAPTORS.
    // Une fois emprunté, l'itérateur n'est plus utilisable...
    let somme: i32 = v1_iter.sum();
    println!("La somme de ses éléments : {}", somme);

    // Je m'en sors en le refabriquant avec la méthode iter()
    let compte: usize = v1.iter().count();
    println!("Le nombre d'éléments : {}", compte);

    // Je crée un nouvel itérateur sur le vecteur, juste pour m'amuser
    // On peut trouver le dernier élément avec last(), qui consomme l'itérateur.
    // last() renvoie une Option<Self::Item>, que je déballe avec unwrap()
    // Self::Item est un TYPE ASSOCIÉ, voir chap. 19
    let v1_iterateur = v1.iter();
    let dernier = v1_iterateur.last().unwrap();
    println!("Le dernier élément: {}", dernier);


}
