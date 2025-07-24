use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::{Arc};
use tokio::sync::Mutex;
use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;




// Type alias pour simplifier
type SharedLogger = Arc<Mutex<()>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {


    //-------------------------------------------------------------------------------//
    //--------------------------- Dossier et fichier logs ---------------------------//
    //-------------------------------------------------------------------------------//

    // Création du dossier de logs dans le cas ou il n'existe pas
    let log_dir = "logs";
    if !Path::new(log_dir).exists() {
        std::fs::create_dir(log_dir)?;
    }

    // Chemin du fichier log
    let log_file_path = format!("{}/server.log", log_dir);



    // Listener TCP sur le port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Le serveur écoute sur 127.0.0.1:8080...");

    // Mutex partagé dans le cas ou plusieurs personnes écrivent en même temps
    let logger = Arc::new(Mutex::new(()));



    // À chaque connexion client
    // Le serveur accepte la connexion
    // Et lance une nouvelle tâche asynchrone indépendante avec le tokio::spawn
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Connexion de {}", addr);

        let logger = logger.clone();
        let log_file_path = log_file_path.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, log_file_path, logger).await {
                eprintln!("Erreur avec client {}: {}", addr, e);
            }
        });
    }
}

async fn handle_client(stream: TcpStream, file_path: String, logger: SharedLogger) -> Result<(), Box<dyn std::error::Error>> {
    let peer = stream.peer_addr()?;
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();



    //Partie écriture dans le fichier log
    while let Some(line) = lines.next_line().await? {
        let timestamp = Utc::now().to_rfc3339();
        let log_entry = format!("[{}] {}\n", timestamp, line);


        //La on met le mutex bloquant pour éviter les conflits d'écriture dans le cas
        //ou plusieurs requêtes arrivent en même temps
        let _lock = logger.lock().await;

        // Ouverture et écriture dans le fichier
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;

        file.write_all(log_entry.as_bytes())?;
        println!("Message reçu de {}: {}", peer, line);
    }

    Ok(())
}
