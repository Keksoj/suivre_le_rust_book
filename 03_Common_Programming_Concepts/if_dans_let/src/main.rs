// On peut inclure une boucle if-else et ses expressions dans une déclaration let


// les déclarations sont dites des BRAS (arms)


fn main() {

    // déclarer une variable boléenne. 
    // nom de la variable : 'condition'
    // On peut tout à fait écrire : 'let condition = true;'
    let condition: bool = true;

    
    // déclarer la variable nombre en fonction de la variable 'condition'
    // if ne comprend que le booléen !
    // en langage humain : si 'true', nombre = 5, si 'false', nombre = 6
    // les deux bras doivent retourner des valeurs DE MÊME TYPE. 
    let nombre = if condition {
        5
    } else {
        6
    };

    println!("La valeur du nombre est {}", nombre);
}
