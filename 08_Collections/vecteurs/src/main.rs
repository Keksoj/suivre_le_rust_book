// 2019-01-17

//

fn main() {
    // Crée un nouveau vecteur vide avec la fonction Vec::new()
    // v est le nom du vecteur
    // Vec<> est le type du contenant
    // i32 est le type du contenu
    let v: Vec<i32> = Vec::new();

    // plus souvent, on crée un vecteur qui contient déjà quelques valeurs, et
    // leur type est inféré par Rust. 
    let mut w = vec![1, 2, 3];
    // vec! est une macro qui crée un vecteur avec du i32, automatiquement.
    // si on écris vec![1.0, 2.0, 3.0] alors on aura du f32, automatiquement.

    // ajoutons des éléments à w (il doit être modifiable !)
    // push() est une fonction
    w.push(4);

    // un vecteur s'affiche avec cette syntaxe :
    // {:?}   affiche une ligne                 [1, 2, 3, 4]
    // {:#}   affiche sur plusieurs lignes      [
    //                                              1,
    //                                              2,
    //                                              3,
    //                                              4,
    //                                          ]
    println!("Voici le vecteur qu'on a créé et allongé :\n{:#?}", w);

    // extraire un élément, appelons-le 'third'
    // type &i32, donc référence à un entier
    // &v[2] signifie : "référence à l'élément 2 du vecteur w"
    // ce sera le troisième élément car on commence à compter à 0
    let third: &i32 = &w[2];
    println!("Le troisième élément est {}", third);

    // on peut aussi écrire directement
    println!("Le quatrième élément est {}", &w[3]); // ampersand facultatif

    // On peut carrément utiliser match
    match w.get(4) {   // match s'éxécute en fonction de ce que renvoie w.get(2)
                       // get() renvoie du Some(valeur) !
        Some(third) => println!("Le troisième élément est {}", third),
        None => println!("Il n'y a pas de troisième élément."),
    }

    // parcourons d'un vecteur
    let u = vec![100, 32, 57];
    for i in &u {
        println!("les éléments du vecteur u à la suite : {}", i);
    }

    // parcourons un vecteur en ajoutant chaque fois une certaine valeur à 
    // chaque élément
    let mut t = vec![100, 32, 57];
    for i in &mut t {               
        *i += 50;               // * est un opérateur de déréférencement
    }
    println!("Le même vecteur avec +50 à chaque élément : {:?}", t);

    // Un vecteur ne peut contenir que des objets d'un même type... oui, mais si
    // le type c'est "variante d'un enum" et que ces variantes sont d'un type 
    // différent au sein de l'enum ? Et bien ÇA PASSE !
    // Définissons un enum qui résume une ligne de tableur, avec une colonne pour
    // un nombre, une pour un flottant, une pour une chaîne de caractère
    enum CelluledeTableur {
        Int(i32),
        Float(f64),
        Text(String),    
    }

    // maintenant on peut définir un vecteur ligne qui contiendra tout ça !
    let ligne = vec![
        CelluledeTableur::Int(3),
        CelluledeTableur::Text(String::from("blue")),
        CelluledeTableur::Float(10.12),
    ];
}



















