// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    return match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
    };
}
pub fn info(message: &str) -> String {
    return "[INFO]: ".to_string() + message;
}
pub fn debug(message: &str) -> String {
    return "[DEBUG]: ".to_string() + message;
}
pub fn warn(message: &str) -> String {
    return "[WARNING]: ".to_string() + message;
}
pub fn error(message: &str) -> String {
    return "[ERROR]: ".to_string() + message;
}
