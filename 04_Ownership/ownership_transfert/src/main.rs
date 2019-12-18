// Des fonctions peuvent transférer l'ownership, le prendre, le rendre


fn main() {

    // Une fonction peut donner l'ownership
    let s1 = donne_ownership(); // la fonction déplace sa valeur retour dans s1.
                                // s1 est valide dans le scope actuel

    // Créons une variable dans le scope. Elle sera déplacée par prend_et_rend()
    let s2 = String::from("Autre chaine de caractères.");

    // Une fonction peut prendre et rendre l'ownership
    let s3 = prend_et_rend(s2);     // s2 est déplacée vers prend_et_rend()
                                    // la valeur est retournée à s3
                                    // s2 est invalide maintenant

    // Afficher les résultats
    println!("s1: {}, s3 : {}", s1, s3);        

}   // on sort du scope, toutes les variables sont lâchées (dropped)




fn donne_ownership() -> String {    // cette fonction déplacera sa valeur
                                    // de retour vers la fonction qui l'appelle.

    let une_chaine = String::from("Hello"); // la variable une_chaine rentre le
                                            // scope. 

    une_chaine  // renvoie la variable à la fonction qui a appelé donne_ownership()
}


fn prend_et_rend(autre_chaine: String) -> String { // définir input et output
                                                   // autre_chaine rentre en scope
    autre_chaine // la variable est retournée à la fonction appelante
}

