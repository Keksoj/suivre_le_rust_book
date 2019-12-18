// 2019-01-15

// Jouons à utiliser les modules.


// cette hiérarchie crée un arbre de module pour le crate. 
// J'ai du mal à comprendre. Dans cette hiérarchie, comme dans un système de 
// fichiers, on définit des CHEMINS.
// Pour appeler une fonction, on doit connaître son chemin.

mod sound;

fn main() {
    // chemin absolu
    crate::sound::instrument::clarinette();

    // chemin relatif, puisque 'sound' et 'main()' sont au même niveau
    sound::instrument::clarinette();
}
