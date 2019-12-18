// 2019-04-26

// Le relecteur parcourra first.rs d'abord, donc le code a été récupéré pour servir ici.
use std::mem;

// rendons le code générique avec des <T> partout où il faut
pub struct List<T> {
    head: Link<T>,
}

// On remplace l'enum Link { Empty, More(Box<Node>), } par ceci :
type Link<T> = Option<Box<Node<T>>>;
// Link est un ALIAS pour le type Option<Box<Node>> SUPER PRATIQUE
// Partant de là, on remplace.   Link::Empty => None   Link::More() => Some()

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// un struct pour fabriquer un itérateur, voir plus bas
// (List<T>) est un n-uplet (tuple)
pub struct IntoIter<T>(List<T>);


// ce bloc impl contient des fonctions, qu'on ferait mieux d'appeler méthodes.
impl<T> List<T> {

    // Créer une nouvelle list, facile, la fonction ne change pas.
    pub fn new() -> Self {
        List { head: None}
    }

    // Ajouter une élément à la liste
    pub fn push(&mut self, elem: T) {

        // Box::new() alloue une partie du heap au struct Node
        let new_node = Box::new(Node {
            // ce struct a comme premier item l'élement à rajouter. Évident.
            elem: elem,
            // la ligne utilisée dans first.rs, que voici :
            // next: mem::replace(&mut self.head, None),
            // peut être écrite avec la méthode take(), car Link est une Option<>
            next: self.head.take()
        });
        // ... et on remet le lien vers new_node dans l'instance de List !
        // on a triché avec la mémoire.
        self.head = Some(new_node);
    }

    // la fonction pop(). On effectuait la manœuvre avec match, ainsi :
        // match self.head.take() {
        //     None => None,
        //     Some(node) => {
        //         self.head = node.next;
        //         Some(node.elem)
        //     }
    // La manœuvre est si courante qu'on utilise map pour ça.
    // map prend une Option<T> et renvoie une Option<U>
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {  // take() prend le premier élément et
                                       // map() scrute cet élément de la liste,
            self.head = node.next;     // le remplace par l'élément suivant,
            node.elem                  // renvoie le premier élément
        })
    }

    // la fonction (ou plutôt méthode) peek()
    pub fn peek(&self) -> Option<&T> {
        // on ne veux pas utiliser take() pour sortir la valeur !
        // on utilise as_ref(), une méthode d'Option
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    // la même chose en modifiable, mais je sais pas à quoi ça sert
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

    // là j'ai du mal à suivre ce qu'on fait
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // accès aux champs d'un struct n-uplet numériquement
        self.0.pop()
    }
}




// le trait Drop pour nettoyer la mémoire
pub trait Drop {
    fn drop(&mut self);
}

// J'ai pas tout compris à ce qui suit... mais c'est pas grave
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);

        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }


}
