use env_logger::Logger;
use log::info;

pub fn init_logger() {
    let _log2 = log2::open("log.txt")
    .level(Logger::from_default_env().filter().as_str())
    .size(100*1024*1024)
    .rotate(20)
    .tee(true)
    .module(true)
    .start();
    info!("logger starting up");
}