use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    // La partie émettrice du canal est tenue par le struct Threadpool avec :
    sender: mpsc::Sender<Message>,
    // les threads sont contenus dans chaque Worker, qui crée un thread customisé
    // Chaque Worker s'accorche à la partie réceptrice du canal
    workers: Vec<Worker>,
}

// Le message qu'on envoie au workers
enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    // Un peu de documentation qui suit les bonnes pratiques
    /// Create a new ThreadPool
    ///
    /// # Panics
    ///
    /// The `new`  function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool {
        // Si size n'est pas plus grand que zéro c'est la panique !
        assert!(size > 0);

        // Crée un canal. L'émetteur ira dans le struct Threadpool,
        // le récepteur est un argument dans la création DES workers
        let (sender, receiver) = mpsc::channel();

        // Il faut impérativement mettre le récepteur dans un mutex et dans un Arc
        // pour pouvoir le CLONER et le DISTRIBUER à tous les workers
        let receiver = Arc::new(Mutex::new(receiver));

        // Crée un vecteur qui peut contenir exactement le nombre de thread
        let mut workers = Vec::with_capacity(size);

        // Crée les workers avec leur id et leur récepteur cloné
        for id in 0..size {
            // Le récepteur est cloné puis donné comme argument à la méthode
            // new() qui crée un worker qui exécute le job
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }

    // Nous voulons une interface qui ressemble à std::thread::spawn()
    // (voir la documentation)
    // Cette méthode sera appelée dans main() pour chaque flux. Elle prendra
    // comme argument la threadpool elle-même évidemment,
    // et la closure  || { handle_connection(stream); }
    pub fn execute<F>(&self, f: F)
    where
        // la closure doit implémenter ces traits :
        F: FnOnce() + Send + 'static,
    {
        // la closure est mise sur le tas avec Box<>, on l'appelle job
        let job = Box::new(f);
        // et on l'envoie dans le canal avec send()
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

// le TYPE ALIAS Job contiendra les closures que nous voulons envoyer le long du canal
// le trait FnBox est défini plus bas
// dyn signifie: dynamic dispatch to a trait object
type Job = Box<dyn FnBox + Send + 'static>;

// Worker est un wrapper pour thread::JoinHandle<()>, bref, il contient un thread
// Normalement, les threads sont créés par spawn(), et recoivent directement
// du travail à faire. Dans notre cas, nous voulons qu'ils attendent
// sagement. Nous devons donc les créer manuellement avec Worker.
// Noter que le struct et ses méthodes sont privées.
struct Worker {
    id: usize,
    // On place le thread dans Option<>, ça facilitera sa fermeture
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // On implémente move pour récupérer le récepteur
        // Avec le récepteur on récupère le job et on l'exécute
        let thread = thread::spawn(move || {
            // on boucle en permanence pour trouver des jobs
            loop {
                // On appelle la méthode lock() sur le récepteur pour aquérir le
                // mutex, puis unwrap() en cas de panique.
                // Une fois le lock obtenu, on appelle recv() pour récupérer le
                // message qui a circulé dans le canal
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        // Faire le boulot
                        job.call_box();
                    }
                    Message::Terminate => {
                        println!("Worker {} has to terminate.", id);
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// On veut déballer Box<T> pour accéder à la closure contenue dedans, or l'état
// actuel de Rust ne permet pas de le faire directement avec (*job)(). Il faut l'
// implémenter manuellement
// Créons un trait spécialement pour et appelons-le FnBox
trait FnBox {
    // La méthode call_box() correspond à la méthode call() des autres traits Fn*
    // sauf qu'elle doit prendre l'ownership de self en le sortant de la Box<>
    fn call_box(self: Box<Self>);
}

// On implémente ce trait FnBox pour toute type F qui implémente le trait FnOnce()
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        // cette syntaxe signifie : déréférencer self
        // (qui est une closure anonyme) et l'exécuter
        (*self)()
    }
}

// Implémenter le trait Drop pour ThreadPool
// On itère deux fois sur les workers : une fois pour leur envoyer le message
// Terminate, et une deuxième fois pour appeler join() sur chaque thread de
// chaque worker. Il faut respecter les deux étapes pour attendre que chaque
// thread soit bien fini.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminale message to all workers");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // avec take(), une méthode d'Option<T>, on sort thread du worker.
            // thread: Some(thread)
            // devient
            // thread: None
            if let Some(thread) = worker.thread.take() {
                //
                thread.join().unwrap();
            }
        }
    }
}
