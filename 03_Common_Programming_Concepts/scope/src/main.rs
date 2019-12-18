// le scope est le domaine où un item est valide.

fn main() {                   // la variable s n'est pas valide car pas déclarée
    let s = "hello world";    // s est valide à partir d'ici
    println!("{}", s);        // on fait des trucs avec s
}                             // s est OUT OF SCOPE

