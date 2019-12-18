// 2019-01-27

// HashMap<K, V> conserve des clés de type K pour des valeurs de type V
// L'idée est de récupérer facilement une valeur V grâce à la clé K
// Pas besoin d'index.

use std::collections::HashMap;

fn main() {

    // on va suivre les scores d'une bataille dans Halo
    // toutes les clés sont du même type : String
    // toutes les valeurs sont du même type : des entiers i32 (détection auto ?)
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // on peut aussi définir un vecteur pour les équipes, un pour les scores
    let equipes = vec![String::from("Bleus"), String::from("Rouges")];
    let scores_de_debut = vec![10, 50];
    // on les rassemble ainsi : 
    // on parcourt chaque vecteur avec iter()
    // la méthode zip() crée un vecteur de plusieurs n-uplets à chaque itération
    // chaque n-uplet est de type (String, i32) : (équipe, score)
    // la méthode collect() transforme ce vecteur de n-uplets en HashMap
    let resultats: HashMap<_, _> = equipes.iter().zip(scores_de_debut.iter()).collect();
    println!("voici la hashmap des résultats : {:?}", resultats);

    // une hashmap prend l'ownership des chaines de caractères qu'elle contient ! 
    
    // Accéder à une valeur
    // Prenons le nom de notre équipe
    let bleus = String::from("Bleus");
    // créons une variable score
    // cherchons avec le nom dans notre hashmap 'resultats' avec get()
    let score = resultats.get(&bleus);
    println!("le score des {} est : {:?}", bleus, score);

    // le résultat est emballé (wrapped) dans Some, ce qui affiche Some(10)
    // en effet, la fonction get() renvoit une Option<&V>. S'il n'y a pas de 
    // valeur, get() renverra None. 

    // On peut parcourir chaque paire <K, V> avec une boucle
    for (cle, valeur) in &resultats {
        println!("l'équipe {} a le score : {}", cle, valeur);
    }
  
    // écraser une valeur
    // sur la première hashmap crée, celle avec les noms en anglais
    scores.insert(String::from("Blue"), 25);
    println!("On a écrasé la valeur associée à 'Bleus' : {:?}", scores);

    // Comment n'insérer une valeur que si la clé n'en a pas encore
    scores.entry(String::from("Yellow")).or_insert(75);
    // entry prend la clé ("Yellow") comme paramètre et renvoie un enum appelé
    // Entry. Cet enum représente une valeur qui existe... ou pas. 
    // Si la valeur existe, or_insert() renvoie la valeur existante
    // Si la valeur n'existe pas, or_insert() renvoie son paramètre comme 
    // nouvelle valeur.
    // essayons de changer le score des bleus...
    scores.entry(String::from("Blue")).or_insert(158);
    println!("On a ajouté le score des jaunes et 
        essayé de tricher avec les bleus : {:?}",
        scores);

    // mettre à jour une valeur : UN COMPTE-MOTS ! 
    // on va compter le nombre d'occurences du mot "world" dans ce texte : 
    // (attention, ce texte est un LITERAL, stocké sur la pile)
    let texte = "Le fracas ! 
        Qu'est-ce que le fracas !
        Et qui te dira
        Ce qu'est le fracas !";
    
    let mut map = HashMap::new();           // crée une hashmap vide
    for mot in texte.split_whitespace() {   // étudie chaque mot entre espace

        // pas tout compris ici
        let count = map.entry(mot).or_insert(0);   
        *count += 1;
    };
    println!("le comptage des mots : {:#?}", map);

    // Rust utilise par défaut la fonction de hachage SipHash, 
    // conçue pour hacher de petites choses très vite.
    
}










