// Inicio Fase 1 Euler
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::{TcpListener, TcpStream};
use std::env;
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    source_id: String,
    payload: String,
    timestamp: u64,
    #[serde(default)]
    is_concurrent: bool,
}

struct LamportClock {
    counter: AtomicU64,
}
// Fim Fase 1 Euler

// Inicio Fase 2 Euler
impl LamportClock {
    fn new(initial: u64) -> Self {
        Self {
            counter: AtomicU64::new(initial),
        }
    }

    fn increment(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::SeqCst) + 1
    }

    fn update(&self, received_timestamp: u64) -> u64 {
        let mut current = self.counter.load(Ordering::SeqCst);
        loop {
            // Regra 2: max(local, received) + 1
            let new_val = std::cmp::max(current, received_timestamp) + 1;
            match self.counter.compare_exchange_weak(
                current,
                new_val,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => return new_val,
                Err(actual) => current = actual,
            }
        }
    }

    fn get(&self) -> u64 {
        self.counter.load(Ordering::SeqCst)
    }
}
// Fim Fase 2 Euler

// Inicio Fase 1 Euler
struct BackendNode {
    id: String,
    clock: Arc<LamportClock>,
    history: Mutex<Vec<Message>>,
}

impl BackendNode {
    fn new(id: String) -> Self {
        Self {
            id,
            clock: Arc::new(LamportClock::new(0)),
            history: Mutex::new(Vec::new()),
        }
    }

// Inicio Fase 3 Euler
    fn process_message(&self, mut msg: Message) -> Message {
        let local_ts = self.clock.get();
        
        // Regra 2: Atualiza o relógio local
        let new_ts = self.clock.update(msg.timestamp);
        
        let mut history = self.history.lock().unwrap();
        
        // Detecção de Concorrência:
        // Heurística de Lamport para ordem parcial e detecção básica de concorrência.
        msg.is_concurrent = history.iter().any(|m| m.timestamp == msg.timestamp && m.source_id != msg.source_id);
        
        println!(
            "[Node {}] Received from {}: '{}' (msg_ts: {}, local_ts_before: {}, local_ts_after: {}) {}",
            self.id, msg.source_id, msg.payload, msg.timestamp, local_ts, new_ts,
            if msg.is_concurrent { "[CONCURRENT]" } else { "" }
        );

        history.push(msg.clone());
        msg
    }
}
// Fim Fase 3 Euler

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node_id = env::var("NODE_ID").unwrap_or_else(|_| "unknown".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "9000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let node = Arc::new(BackendNode::new(node_id.clone()));
// Fim Fase 1 Euler

// Inicio Fase 1 Daniel
    let listener = TcpListener::bind(&addr).await?;
    println!("Backend Node {} listening on {}", node_id, addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let node = Arc::clone(&node);

        tokio::spawn(async move {
            // Inicio Fase 1 Euler: Framing com LinesCodec
            let mut framed = Framed::new(socket, LinesCodec::new());
            // Fim Fase 1 Euler

            while let Some(result) = framed.next().await {
                match result {
                    Ok(line) => {
                        if let Ok(msg) = serde_json::from_str::<Message>(&line) {
                            // Fase 3 Euler: Processamento
                            let processed = node.process_message(msg);
                            
                            // Resposta serializada
                            if let Ok(response) = serde_json::to_string(&processed) {
                                let _ = framed.send(response).await;
                            }
                        } else if line.trim() == "increment" {
                            // Fase 2 Euler: Incremento local
                            let new_ts = node.clock.increment();
                            println!("[Node {}] Local event. New Clock: {}", node.id, new_ts);
                            let _ = framed.send(format!("Clock incremented to {}", new_ts)).await;
                        } else {
                            let _ = framed.send("Invalid JSON or command".to_string()).await;
                        }
                    }
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
// Fim Fase 1 Daniel