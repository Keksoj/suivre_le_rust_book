// 2019-01-08 

// match permet d'exécuter tel ou tel code en fonction de la correspondance de
// certain pattern. S'ils correspondent, s'ils matchent, on éxécute tel code. 
// Sinon, on éxécute tel autre. Les patterns conditions sont multiples.


// Imaginons une application de match qui trie des pièces dans un distributeur.
// La pièce roule sur une piste qui arbore plusieurs trous de différentes tailles.
// La pièce tombe dans le premier trou qui correspond.
// De même, la valeur passe dans chaque pattern d'un match, dès qu'il y a une
// correspondance entre valeur et pattern (dès qu'ils matchent), la valeur tombe
// dans le bloc de code à exécuter.

// Faisons le tri ! Prenons la première pièce venue et trouvons sa valeur en 
// cents. On va suivre le tuto et utiliser des pièces US. 
// Penny : 1 centime de dollar, zinc plaqué cuivre. Abraham Lincoln.
// Nickel : 5 cents. 25% nickel, 75 % cuivre. Thomas Jefferson.
// Dime : 10 cents. Cupro-nickel 92% - 8%. Franklin Roosevelt.
// Quarter : 25 cents. Cupro-nickel aussi. George Washington.
// Il y a une pièce de 50 cents avec JFK, et même un dollar en pièce, mais peu
// usité.

// Un enum pour définir les différentes pièces existantes
enum Coin {
    Penny,          // nota bene : ce sont des VARIANTES qu'on liste ici
    Nickel,
    Dime,
    Quarter,
}

// Une fonction valeur_en_cents() qui contient le match
// En paramètre, une variable 'coin' qui appartient à l'enum Coin
// la fonction renvoie un entier, ce sera la valeur de la pièce en cents.
fn valeur_en_cent(coin: Coin) -> u32 {

    // 
    match coin {                
                                // présentons le contenu de gauche à droite
                                // les lignes d'un match sont des BRAS (arms)
                                // PATTERN => CODE
        Coin::Penny => 1,       // Coin::Penny est le PATTERN
        Coin::Nickel => 5,      // => est un OPÉRATEUR
        Coin::Dime => 10,       // 10 est du CODE (même si celui-ci est simple)
        Coin::Quarter => {      // du code plus complexe, entre accolades
            println!("Bravo, un quarter !");
            25
        },
    }
}

fn main() {

}
