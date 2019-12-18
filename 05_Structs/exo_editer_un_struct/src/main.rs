// 2019-01-02
// Un struct est un groupe de variable. Ici, des données utilisateur.
// Ici, on va construire un struct avec une fonction.
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
    // Demande le nom d'utilisateur puis l'adresse mail
    // Crée les variables modifiables 'nom' et 'adresse'
    // Utilise la fonction coupe() pour enlever le retour à la ligne
    println!("Quel est votre nom ?");
    let mut nom = String::new();
    io::stdin()
        .read_line(&mut nom)
        .expect("Pas pu lire l'entrée");
    let prenom = coupe(&nom);

    println!("Quel est votre adresse mail ?");
    let mut adresse = String::new();
    io::stdin()
        .read_line(&mut adresse)
        .expect("Pas pu lire l'entrée");
    let adresse2 = coupe(&adresse);

    // appelle la fonction build_user avec 'nom' et 'adresse' comme arguments
    // crée le struct 'nouveau'
    // Attention, la fonction build_user() ne doit pas prendre l'ownership des
    // variables 'adresse' et 'nom' !
    let nouveau = build_user(adresse2, prenom);

    // Affiche deux choses : le nom et le struct 'nouveau'
    // au lieu de 'nom', on aurait pu mettre nouveau.username
    println!("\nVoici votre struct, cher {} ! \n{:#?}", prenom, nouveau);   
}

// Définit la fonction build_user(), fonction qui construit un struct. 
// Seules deux variables sont à donner en argument, email et username
// Les variables 'active' et 'age' sont remplies d'office
fn build_user(email: &str, username: &str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        active: true,
        age: 20,
    }
}

// Cette fonction enlève le retour à la ligne à la fin d'une chaîne de caractères.
fn coupe(s: &str) -> &str {
    s.trim()
}
