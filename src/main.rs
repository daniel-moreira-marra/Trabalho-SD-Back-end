use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::net::{TcpListener, TcpStream};
use std::env;
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    source_id: String,
    payload: String,
    timestamp: u64,
    #[serde(default)]
    is_concurrent: bool,
    #[serde(default)]
    is_broadcast: bool,
    #[serde(default)]
    forwarder_id: Option<String>,
}

struct LamportClock {
    counter: AtomicU64,
}

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

    fn process_message(&self, mut msg: Message) -> Message {
        let local_ts_before = self.clock.get();

        if msg.is_broadcast {
            self.clock.update(msg.timestamp);
        } else {
            let new_ts = self.clock.increment();
            msg.timestamp = new_ts;
        }

        let local_ts_after = self.clock.get();
        let mut history = self.history.lock().unwrap();
        
        msg.is_concurrent = history.iter().any(|m| m.timestamp == msg.timestamp && m.source_id != msg.source_id);
        
        let forwarder_info = match &msg.forwarder_id {
            Some(f_id) => format!(" (via Node {})", f_id),
            None => "".to_string(),
        };
        
        println!(
            "[Node {}] Processou de {}{}: '{}' (msg_ts: {}, local_ts_before: {}, local_ts_after: {}) {}",
            self.id, msg.source_id, forwarder_info, msg.payload, msg.timestamp, local_ts_before, local_ts_after,
            if msg.is_concurrent { "[CONCURRENT]" } else { "" }
        );

        history.push(msg.clone());
        
        history.sort_by(|a, b| {
            if a.timestamp == b.timestamp {
                a.source_id.cmp(&b.source_id)
            } else {
                a.timestamp.cmp(&b.timestamp)
            }
        });
        
        msg
    }
}

async fn broadcast_to_peers(msg: Message, my_node_id: String) {
    let all_nodes = ["1", "2", "3"];
    
    for target_id in all_nodes {
        if target_id == my_node_id { continue; }
        
        let peer_hostname = format!("backend{}:9000", target_id);
        let mut broadcast_msg = msg.clone();
        broadcast_msg.is_broadcast = true;
        broadcast_msg.forwarder_id = Some(my_node_id.clone()); 
        
        let msg_str = serde_json::to_string(&broadcast_msg).unwrap() + "\n";
        
        tokio::spawn(async move {
            if let Ok(mut stream) = TcpStream::connect(&peer_hostname).await {
                let _ = stream.write_all(msg_str.as_bytes()).await;
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node_id = env::var("NODE_ID").unwrap_or_else(|_| "unknown".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "9000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let node = Arc::new(BackendNode::new(node_id.clone()));

    let listener = TcpListener::bind(&addr).await?;
    println!("Backend Node {} listening on {}", node_id, addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let node = Arc::clone(&node);
        let current_node_id = node_id.clone();

        tokio::spawn(async move {
            let mut framed = Framed::new(socket, LinesCodec::new());

            while let Some(result) = framed.next().await {
                match result {
                    Ok(line) => {
                        if let Ok(msg) = serde_json::from_str::<Message>(&line) {
                            let is_incoming_broadcast = msg.is_broadcast;
                            
                            let processed = node.process_message(msg);
                            
                            if !is_incoming_broadcast {
                                broadcast_to_peers(processed.clone(), current_node_id.clone()).await;
                            }
                            
                            if let Ok(response) = serde_json::to_string(&processed) {
                                let _ = framed.send(response).await;
                            }
                        } else {
                            let _ = framed.send("Invalid JSON\n".to_string()).await;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
}