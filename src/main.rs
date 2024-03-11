use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tokio::task::JoinHandle;
use tokio::sync::mpsc;

mod pow;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let difficulty = 7;

    let mut nonce = 0;
    let batch_size = 1_000_000_000;

    loop {
        let (sender, mut receiver) = mpsc::channel(1);
        let found = Arc::new(AtomicBool::new(false));
        let mut end_nonce = nonce + batch_size;

        let task_0 = spawn_task(nonce, end_nonce, difficulty, sender.clone(), found.clone());
        nonce = end_nonce;
        end_nonce = nonce + batch_size;
        let task_1 = spawn_task(nonce, end_nonce, difficulty, sender.clone(), found.clone());

        tokio::select! {
            _ = task_0 => {
                if found.load(Ordering::SeqCst) {
                    break
                }
            },
            _ = task_1 => {
                if found.load(Ordering::SeqCst) {
                    break
                }
            },
            Some((nonce, hash)) = receiver.recv() => {
                let duration = start.elapsed();
                println!("Found hash: {} in nonce: {}", hash, nonce);
                println!("Took {} seconds", duration.as_secs());
                break
            }
        }
    }
}

fn spawn_task (start_nonce: usize, end_nonce: usize, difficulty: usize, sender: mpsc::Sender<(usize, String)>, found: Arc<AtomicBool>) -> JoinHandle<()> {
    tokio::spawn(async move {
        for nonce in start_nonce..end_nonce {
            if found.load(Ordering::SeqCst) {
                break;
            }

            let hash = pow::guess("Hello", nonce);

            if pow::meets_difficulty(&*hash, difficulty) {
                if sender.send((nonce, hash)).await.is_ok() {
                    found.store(true, Ordering::SeqCst);
                    break;
                }
            }
        }
    })
}
