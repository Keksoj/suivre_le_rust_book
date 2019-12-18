
// Fabriquons un struct pour les chaussures
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// Une fonction à deux arguments :
    // un vecteur de structs (ça existe)
    // Une taille de chaussure
// Et qui renvoie un vecteur de struct

// Cette fonction prend l'ownership du vecteur
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {

    // On passe le vecteur dans un itérateur avec into_iter()
    shoes.into_iter()

        // 'filter' amène l'itérateur dans un nouvel itérateur qui ne contiendra que les
        // éléments où la fermeture renverra TRUE.
        // la fermeture capture le paramètre shoe_size de l'environnement
        .filter(|s| s.size == shoe_size)

        // on récupère tout ça avec collect()
        .collect()
}


// Un test pour vérifier que la fonction... fonctionne
#[test]
fn filters_by_size() {
    let vecteur_de_chaussures = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(vecteur_de_chaussures, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    )
}
