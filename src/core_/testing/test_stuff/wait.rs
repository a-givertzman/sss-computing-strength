#![allow(non_snake_case)]

use std::{thread::JoinHandle, any::Any};

use log::{info, error};

///
/// Performs JoinHandle.join() wrapped in some debuging
pub trait WaitTread {
    fn wait(self) -> Result<(), Box<dyn Any + Send>>;
}


impl WaitTread for JoinHandle<()> {
    ///
    /// Performs JoinHandle.join() wrapped in some debuging
    fn wait(self) -> Result<(), Box<dyn Any + Send>> {
        let thdId = format!("{:?}-{:?}", self.thread().id(), self.thread().name());
        info!("Waiting for thread: {:?}...", thdId);
        let r = self.join();
        match &r {
            Ok(_) => {
                info!("Waiting for thread: '{}' - finished", thdId);
            },
            Err(err) => {
                error!("Waiting for thread '{}' error: {:?}", thdId, err);                
            },
        }
        r
    }
}
