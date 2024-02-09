#![allow(non_snake_case)]

use std::{time::{Duration, Instant}, thread::{JoinHandle, self}, sync::{Arc, atomic::{AtomicBool, Ordering}}};

use log::error;

///
/// If maximum test turation will be exceeded, then panics
pub struct TestDuration {
    id: String,
    duration: Duration,
    exit: Arc<AtomicBool>,
}
///
/// 
impl TestDuration {
    ///
    /// 
    pub fn new(parent: impl Into<String>, duration: Duration) -> Self {
        Self {
            id: format!("{}/TestDuration", parent.into()),
            duration,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn run(&self) -> Result<JoinHandle<()>, std::io::Error> {
        let selfId = self.id.clone();
        let exit = self.exit.clone();
        let duration = self.duration.clone();
        thread::Builder::new().name(format!("{}.run", selfId)).spawn(move || {
            let timer = Instant::now();
            loop {
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                thread::sleep(Duration::from_millis(100));
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                if timer.elapsed() > duration {
                    error!("{}.run | Maximum test duration ({:?}) exceeded", selfId, duration);
                    std::process::exit(70);   // SOFTWARE: ExitCode = 70
                }

            }
        })
    }
    ///
    ///
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}