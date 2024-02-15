use debugging::session::debug_session::{Backtrace, DebugSession, LogLevel};
use log::debug;
use testing::entities::test_value::Value;

mod data;
mod tests;

fn main() {
    DebugSession::init(LogLevel::Debug, Backtrace::Short);
    debug!("Test the debugging...");
    debug!("Test the testing...");
    let value = Value::Bool(false);
    debug!("\t bool value: {:?}", value);
    let value = Value::Int(444);
    debug!("\t int value: {:?}", value);
    let value = Value::Float(55.55);
    debug!("\t float value: {:?}", value);
    let value = Value::String("66.77".to_string());
    debug!("\t string value: {:?}", value);    
}