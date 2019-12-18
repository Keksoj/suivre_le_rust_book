// 2018-12-26

fn main() {

    // déclare la variable x comme modifiable (mutable) sinon le code ne compile pas.
    let mut x = 5;
    println!("La valeur de x est {}", x);
    x = 6;
    println!("La valeur de x est {}", x);

    // déclare la constante MAX_POINTS. Convention : majuscules et underscores
    const MAX_POINTS: u32 = 100_000;
    println!("La constante est {}", MAX_POINTS);

    // shadowing, c'est redéfinir une variable, écraser sa valeur
    // seul 'let' peut le faire, et la variable n'a pas à être 'mut'
    let y = 4;
    let y = y + 1;
    let y = y * 2;
    println!("La valeur de y est {}", y);

    // l'intérêt du shadowing est qu'on peut modifier le type de variable.
    // D'abord, une chaîne de caractère avec plusieurs espaces
    let spaces = "    ";
    println!("La variable spaces, la voilà :{}et c'est cool.", spaces);
    // ensuite, un nombre, le nombre d'espace (length) dans spaces
    let spaces = spaces.len();
    println!("On a redéfini, par le shadowing, la variable spaces, ce n'est plus \
                une chaîne de caractères, mais le nombre d'espaces : {}", spaces);
    println!("Aller voir le code pour mieux comprendre.");
}
