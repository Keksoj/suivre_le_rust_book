// 2019-01-06
// Un enum, ou énumération, contient plusieurs variantes pour une seule valeur.
// la valeur sera l'une, au l'autre, ou l'autre.

// définir un enum pour le type d'adresse IP. Soit version 4, soit version 6
// Entre accolades, on définit des VARIANTES.
// IpAddrKind est maintenant un TYPE DE DONNÉE
enum IpAddrKind {
    V4,
    V6,
}

// Pour des instances d'adresse IP, on définit un struct
// Noter qu'on utiliser IpAddrKind comme type de donnnée !
// Deux champs : kind et adresse, l'un de de type IpAddrKind et l'autre chaîne
// de caractères
struct AdresseIP {
    kind: IpAddrKind,
    adresse: String,
}

fn main() {

    // On peut définir des instances d'un enum avec la syntaxe suivante :
    //         Enum::variante
    // Mais pour le moment je vois pas à quoi ça sert...
    let four = IpAddrKind::V4;

    // Plus cool, on peut définir des instances du struct AdresseIP
    let maison = AdresseIP {
        kind: IpAddrKind::V4,
        adresse: String::from("127.0.0.1"),
    };
    let loopback = AdresseIP {
        kind: IpAddrKind::V6,
        adresse: String::from("::1"),
    };

    println!(
        "Ce programme expose la syntaxe à utiliser pour définir des enums."
    );
}



