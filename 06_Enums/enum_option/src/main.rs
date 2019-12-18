// 2019-01-07

// J'avoue que le chapitre sur les enums n'est pas hyper pédagogique, je sais 
// pas bien à quoi ils vont servir encore. Mais voyons-voir un enum tout spécial.

// L'enum Option permet de coder un scénario fréquent, quand une valeur peut être
// quelque chose, ou alors rien. Ça aide le compilateur et évite des bugs.
// En rust, il n'y a pas de NULL FEATURE, c'est comme ça. Tony Hoare regretterait
// même d'avoir inventé le null. Moi je m'en fous, je sais pas ce que c'est.

// En rust, on a un enum qui exprime le truc. Défini dans std::option

enum Option<T> {        // <T> est un paramètre de type générique
    Some(T),            // Some(T) contient n'importe quelle donnée
    None,
}

fn main() {
        
    // On peut utiliser les valeurs de Option pour contenir des trucs
    // voici avec Some(T)
    let un_nombre = Some(5);
    let une_chaine = Some("une chaîne");

    // avec None, par contre, il faut préciser quel genre d'Option<T> on a.
    // le compilateur doit savoir quel type de donnée se présentara quand il y
    // en aura enfin.
    let nombre_absent: Option<i32> = None;
}

// Par contre, on peut pas utiliser la valeur T dans Some(T) aussi facilement.
// Consulter la doc à std::option::Option, des méthodes existent, à connaître.

// Grosso modo, on procèdera ainsi : 
// Si on a une valeur à Some(T), c'est tel bout de code qui s'éxécutera.
// si on a une valeur None, c'est tel autre bout de code qui s'éxécutera.
// voir l'expression match, qui fait justement ça
