use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio::time::{interval, Duration};
use tokio::io::{AsyncBufReadExt, BufReader, AsyncWriteExt};


// Structure suggérée pour l'état partagé :
type Store = Arc<Mutex<HashMap<String, Entry>>>;

/// Une entrée du store : valeur + expiration optionnelle.
#[derive(Clone)]
struct Entry {
    value: String,
    expires_at: Option<Instant>,
}

/// Structure pour parser les requêtes JSON des clients.
#[derive(Deserialize)]
struct Request {
    cmd: String,
    key: Option<String>,
    value: Option<String>,
    seconds: Option<u64>,
}


#[tokio::main]
async fn main() {
    // Initialiser tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // TODO: Implémenter le serveur MiniRedis sur 127.0.0.1:7878
    //
    // Étapes suggérées :
    // 1. Créer le store partagé (Arc<Mutex<HashMap<String, ...>>>)
    let store: Store = Arc::new(Mutex::new(HashMap::new()));

    // 2. Bind un TcpListener sur 127.0.0.1:7878
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    tracing::info!("Serveur MiniRedis lancé sur 127.0.0.1:7878");
    // 3. Accept loop : pour chaque connexion, spawn une tâche
    let cleanup_store = store.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let mut store = cleanup_store.lock().await;
            let now = Instant::now();
            store.retain(|_, entry| match entry.expires_at {
                Some(exp) => exp > now,
                None => true,
            });
        }
    });


    // 4. Dans chaque tâche : lire les requêtes JSON ligne par ligne,
    //    traiter la commande, envoyer la réponse JSON + '\n'
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        tracing::info!("Nouveau client: {}", addr);
        let store = store.clone();
        tokio::spawn(async move {
            handle_client(socket, store).await;
        });
    }
    // println!("MiniRedis - à implémenter !");
}

async fn handle_client(socket: TcpStream, store: Store) {
    let (read_half, mut write_half) = socket.into_split();
    let mut reader = BufReader::new(read_half);
    let mut line = String::new();   
}

