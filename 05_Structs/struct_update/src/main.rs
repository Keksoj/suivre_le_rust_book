// 2019-01-02
// On va fabriquer un nouveau struct en modifiant quelques variables d'un autre.


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
    let sophie = User {
        email: String::from("sophie.lambert@skynet.com"),
        username: String::from("Sophie"),
        age: 25,
        active: true,
    };

    // Copie l'instance 'sophie' dans l'instance 'pierre' en la changeant.
    let pierre = User {
        email: String::from("adresse.de@pierre.com"),
        username: String::from("Pierre"),
        age: sophie.age,
        active: sophie.active,
    };

    // syntaxe plus simple avec l'instance 'fabrice'
    let fabrice = User {
        email: String::from("fabrice@denice.fr"),
        username: String::from("Fabrice"),
        ..pierre
    };

    // affiche le tout
    println!("Voici le struct de Sophie :\n{:?}", sophie);
    println!("Voici le struct de Pierre :\n{:?}", pierre);
    println!("Voici le struct de Fabrice :\n{:?}", fabrice);
}
