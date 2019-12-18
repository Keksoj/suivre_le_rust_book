// 2019-01-01
// On veut modifier une variable sans en prendre l'ownership.

fn main() {

    // On crée une chaine de caractère s qui est MODIFIABLE (mutable)
    let mut s = String::from("Hello");

    // On appelle la fonction change()
    // On crée une RÉFÉRENCE MODIFIABLE (mutable reference) avec &mut s
    change(&mut s);

    // On peut emprunter une variable modifiable une fois...
    let s1 = &mut s;
    println!("{}", s1);

    // ... mais pas deux ! Ce code-ci planterait :
        // let s2 = &mut s;
        // println!("{}", s1);
    // avec l'erreur suivante : 
    // cannot borrow `s` as mutable more than once at a time
}

// Définissons change()
// L'input est défini comme une chaine de caractère modifiable
// On accepte ici la référence modifiable créée plus haut
// Pas besoin de définir l'output qui est de même type
fn change(une_chaine: &mut String) {

    // la fonction push_str() ajoute du texte à la variable modifiable.
    une_chaine.push_str(", world!");
}
