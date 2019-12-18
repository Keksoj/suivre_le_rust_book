// 2019-01-15


// Appel du module 'plante' qu'on trouve à src/plante.rs
mod plante;

fn main() {
    let mut v = plante::Legume::new("Oignon"); // ceci est un chemin vers new()
                                               // fabrique une instance de Legume
                                
    println!("On a fabriqué une instance de légume : {:?}", v);
               
    v.nom = String::from("Oignon jaune");   //
    println!("Et on a changé son contenu : {:?}", v);

    // et on ne peux pas compiler ce code
    //println!("Et l'ID c'est {}", v.id);
}

// On peut afficher id avec {:?}, parce que #[derive(Debug) implémente automati-
// quement fmt::Debug pour le struct Legume. Debug est une implémentation de 
// fonctions associées, et a accés aux données privées. {:?} INSPECTE la variable
// avec v.fmt (qui vient de Debug)

