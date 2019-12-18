// 2019-01-02
// Un struct est un groupe de variable. Ici, des données utilisateur.
// On va CONSTRUIRE un struct, et le modifier un peu.
use std::io;

// Debug est un TRAIT. Solution tirée de stack overflow pour afficher le struct.
#[derive(Debug)]

// On définit le struct User EN DEHORS de la fonction main()
struct User {
    username: String,
    email: String,
    age: u8,
    active: bool,
}


fn main() {

    // Crée une INSTANCE de User, instance qui s'appelle sophie
    // les variables sont définies dans le désordre et ce n'est pas grave !
    let sophie = User {
        age: 25,
        username: String::from("Sophie"),
        active: true,
        email: String::from("sophie.lambert@skynet.com"),
    };

    // Affiche l'instance sophie
    // Remarquons qu'à l'affichage, les variables sont remises dans l'ordre.
    println!("Voici le struct de Sophie :\n{:?}", sophie);

    // Crée et affiche une autre instance, 'jean'. Modifiable cette fois-ci.
    let mut jean = User {
        age: 30,
        username: String::from("Jean"),
        active: true,
        email: String::from("jean.ai.marre@metatron.com"),
    };
    println!("Voici le struct de Jean :\n{:?}", jean);

    // Modifie l'instance 'jean', l'affiche
    jean.email = String::from("autre.email.de.jean@metatron.com");
    println!("Voici le struct de Jean une fois modifié :\n{:?}", jean);   
}

