// 2019-01-07
// (Voir le premier programme pour plus de clarté)



// On peut être plus cool en mettant de la donnée directement dans les
// variantes d'un enum. Ici, on définit que V4 et V6 auront des chaînes de
// caractères qui leur seront associées.
// Et on peut être ENCORE plus cool en définissant le type de chaque variante
enum AdresseIP {
    V4(u8, u8, u8, u8), // c'est un n-uplet pour ceux qui suivent pas
    V6(String),
}



fn main() {

    let home = AdresseIP::V4(127, 0, 0, 1);
    let loopback = AdresseIP::V6(String::from("::1"));

    // Message pour se référer à l'enum Message défini plus bas
    // ::Write(), fonction associée défini dans le bloc impl, plus bas
    let m = Message::Write(String::from("Ceci a été appelé dans une méthode"));

    println!("Intéressez-vous plutôt au code.");
}

// enfin, la bibliothèque standard std::net a tout prévu ! std::net::IpAddr
// je copie-colle sans tout comprendre. Ce que je retiens : les variantes d'un
// enum peuvent être n'importe quelle genre de donnée : chaînes de caractères,
// nombres, structs

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// AUTRE EXEMPLE D'ENUM avec des données différentes pour chaque variable
enum Message {
    Quit,                       // même pas de donnée
    Move { x:i32, y:i32 },      // inclut un struct anonyme
    Write(String),              // inclut une chaîne de caractère
    ChangeColor(i32, i32, i32), // un n-uplet de trois entiers de 32 bits
}

// Cette méthode permet de contenir les mêmes données que plein de structs
// différents qui seraient définis chacun de leur côté.
struct QuitMessage; // struct unitaire
struct MoveMessage { x:i32, y:i32 }
struct WriteMessage(String);             // struct n-uplet (avec un seul item)
struct ChangeColorMessage(i32, i32, i32);// struct n-uplet (avec un seul item)

// Encore un truc, on peut définir une méthode sur des enums, comme sur les structs.
impl Message {
    fn call(&self) {
        // contenu de la fonction
    }
}
