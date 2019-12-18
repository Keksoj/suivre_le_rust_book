// if est une EXPRESSION

fn main() {
    let nombre = 8; 

    // 'nombre < 5' est une CONDITION. C'est impérativement une boléenne.
    if nombre % 5 == 0 {
        println!("Le nombre est divisible par 5.");
    } 

    // on peut ajouter d'autres conditions avec 'else if' et on finit par 'else'
    // solution à ne pas trop utiliser, ça encombre le code. Utiliser 'match'
    else if nombre % 4 == 0 {
        println!("Le nombre est divisible par 4.");
    } else if nombre % 3 == 0 {
        println!("Le nombre est divisible par 3.");
    } else if nombre % 2 == 0 {
        println!("Le nombre est divisible par 2.");
    } else {
        println!("Le nombre n'est divisible ni par 5, ni par 4, ni par 3, ni par 2.");
    }
}
