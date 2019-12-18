// 2019-01-08 

// On continue l'exercice commencé dans match_syntaxe



// L'enum pour définir les différentes pièces existantes
// Cette fois-ci on rajoute une précision. Les quarterse, entre 1999 et 2008, 
// étaient différents selon l'état américain.
enum Coin {
    Penny,             // nota bene : ce sont des VARIANTES qu'on liste ici
    Nickel,
    Dime,
    Quarter(UsState),  // la variante Quarter porte une valeur UsState
}

// un enum pour les états américains
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    //--etc--
}

// Une fonction valeur_en_cents() qui contient le match
// la fonction renvoie un entier, ce sera la valeur de la pièce en cents.
fn valeur_en_cent(coin: Coin) -> u32 {
    match coin {                
        Coin::Penny => 1,       
        Coin::Nickel => 5,      
        Coin::Dime => 10,       
        Coin::Quarter(state) => {
            println!("Bravo, un quarter du {:?}!", state);
            25
        },
    }
}

fn main() {

    // appel de la fonction valeur_en_cent
    // le paramètre est la variante Quarter de l'enum Coin
    // Quarter porte la valeur Alaska, une variatne de l'enum UsState
    valeur_en_cent(Coin::Quarter(UsState::Alaska));
}
