// 2019-04-26

// Fonctions de mémoire basiques
use std::mem;

// défintion : "A List is either Empty or an Element followed by a List"
// pub enum List {
//     Empty,
//     Elem(i32, Box<List>),
// }

// Mais pour tout un tas de raison de pointeurs, il vaut mieux faire ça :

// pub enum List {
//     Empty,
//     ElemThenEmpty(i32),
//     ElemThenNotEmpty(i32, Box<List>),
// }

// Cette solution permet d'éviter d'avoir à alouer un endroit du tas (heap) en
// cas d'Empty. En effet, il y a une "null pointer optimization" qui fait que
// l'enum est remplacé par des zéros de bout en bout, sans pointeur à l'intérieur.

// Il FAUT séparer ces deux objectifs :
    // avoir un élément de la liste, un Node
    // allouer un endroit du tas à une autre liste
// Utilisons des structs !


// DÉCOMMENTER CE BLOC POUR COMPRENDRE
//
// // Chaque nœud contient à la fois un élément,
// struct Node {
//     elem: i32,
//     next: List,
// }
//
// // et un enum qui contient...
// pub enum List {
//     Empty,                  // ... soit rien
//     More(Box<Node>),        // ... soit une référence à un autre nœud
// }

// Comme prévu :
    // la fin de la liste n'est pas une référence inutile !
    // on bénéficie de la null pointer optimization
    // les allocations sont régulières

// Seul problème, Node est privé, et on ne veux pas le rendre public, alors :
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}


// Des fonctions !
impl List {

    // Créer une nouvelle list, facile
    pub fn new() -> Self {
        // Un struct List dont l'item head est la variante Empty de l'enum Link
        List { head: Link::Empty}
    }

    // Ajouter une élément à la liste
    pub fn push(&mut self, elem: i32) {

        // Box::new() alloue une partie du heap au struct Node
        let new_node = Box::new(Node {
            // ce struct a comme premier item l'élement à rajouter. Évident.
            elem: elem,
            // le deuxième item devrait être self.head, donc l'élement pré-
            // existant de la liste, qu'il soit un Empty ou un More(Box<Node>)
            // mais on ne peut pas sortir un truc de &mut !
            // on triche, on remplace self.head par Link::Empty...
            next: mem::replace(&mut self.head, Link::Empty),
        });
        // ... et on remet le lien vers new_node dans l'instance de List !
        // on a triché avec la mémoire.
        self.head = Link::More(new_node);
    }

    // la fonction pop() doit enlever le premier élément de la liste (s'il existe)
    // et le retourner. La liste, au passage, est tronquée.
    pub fn pop(&mut self) -> Option<i32> {
        let result; // result sera de type Option<>

        // y a-t-il seulement un premier élément ? voyons avec match
        // problème : on ne peux pas sortir de la donnée de node quand on n'en a
        // qu'une référence partagée. Le seul moyen de sortir des trucs de la
        // liste, c'est d'abord de la remplacer. Remplaçons donc.
        // replace(), ici, récupère self.head, la remplace par un truc.
        // On peut disposer de self.head comme on l'entend, héhé
        match mem::replace(&mut self.head, Link::Empty) {

            // Si la liste est vide, on renvoie None.
            Link::Empty => {
                result = None;
            }

            // Si la liste contient un élément...
            Link::More(node) => {
                result = Some(node.elem); // ... on met l'élement dans Some()
                self.head = node.next;    // ... on met l'élément suivant à la
            }                             // tête de la liste
            // Voilà ! On a sorti le premier élément !
        };
        result // renvoyer result, qui est de type Option<i32, None>
    }
}

// le trait Drop pour nettoyer la mémoire
pub trait Drop {
    fn drop(&mut self);
}
// Il faut le définir nous-même, parce que nettoyer la mémoire d'une liste
// récursive pourrait être usant pour la pile (blow the stack)
// On va implémenter Drop pour la beauté du geste, en réalité on n'a pas besoin
// de le faire en général

// J'ai pas tout compris à ce qui suit... mais c'est pas grave
impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}


// Des tests !
#[cfg(test)] // Cette ligne fait que le module test n'est compilé que si l'on
             // effectue des tests.
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // vérifier que le premier élément est vide
        assert_eq!(list.pop(), None);

        // remplir la liste
        list.push(1);
        list.push(2);
        list.push(3);

        // vérifier que les éléments sortent bien
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // remettre des éléments pour être sûr
        list.push(4);
        list.push(5);

        //vérifier qu'ils s'enlèvent bien
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // vérifier qu'elle se vide sans problème
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}
