#![allow(non_snake_case)]

use std::{env, sync::Once};

#[allow(dead_code)]
static INIT: Once = Once::new();

///
/// 
#[allow(dead_code)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
///
/// 
#[allow(dead_code)]
pub enum Backtrace {
    Full,
    Short,
}

///
/// Call DebugSession::init() to initialize logging
#[allow(dead_code)]
pub struct DebugSession {}
///
/// 
impl DebugSession {
    ///
    /// Initialize debug session on first call, all next will be ignored
    #[allow(dead_code)]
    pub fn init(logLevel: LogLevel, backtrace: Backtrace) {
        INIT.call_once(|| {
            let logStyle = "always";
            let logLevel = match logLevel {
                LogLevel::Off => "off",
                LogLevel::Error => "error",
                LogLevel::Warn => "warn",
                LogLevel::Info => "info",
                LogLevel::Debug => "debug",
                LogLevel::Trace => "trace",
                // _ => "debug",
            };
            let backtrace = match backtrace {
                Backtrace::Full => "full",
                Backtrace::Short => "short",
            };
            env::set_var("RUST_LOG", logLevel);  // off / error / warn / info / debug / trace
            assert_eq!(env::var("RUST_LOG"), Ok(logLevel.to_string()), "Set env RUST_LOG={} failed", logLevel);
            env::set_var("RUST_BACKTRACE", backtrace);
            assert_eq!(env::var("RUST_BACKTRACE"), Ok(backtrace.to_string()), "Set env RUST_BACKTRACE={} failed", backtrace);
            env::set_var("RUST_LOG_STYLE", logStyle);     // auto / always / never
            assert_eq!(env::var("RUST_LOG_STYLE"), Ok(logStyle.to_string()), "Set env RUST_LOG_STYLE={} failed", logStyle);
            match env_logger::builder().is_test(true).try_init() {
            // match builder.is_test(true).try_init() {
                Ok(_) => {
                    println!("DebugSession.init | Ok");
                    println!("DebugSession.init | RUST_LOG = {:?}", env::var("RUST_LOG"));
                    println!("DebugSession.init | RUST_BACKTRACE = {:?}", env::var("RUST_BACKTRACE"));
                    println!("DebugSession.init | RUST_LOG_STYLE = {:?}", env::var("RUST_LOG_STYLE"));
                    println!("");
                },
                Err(err) => {
                    println!("DebugSession.init | error: {:?}", err)
                },
            };
        })
    }
}
