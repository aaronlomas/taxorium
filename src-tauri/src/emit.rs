use tauri::{AppHandle, Emitter};

#[derive(serde::Serialize, Clone)]
struct LogPayload {
    level: &'static str,
    message: String,
    source: &'static str,
}

/// Emit an INFO-level log event to the frontend TaxoLog.
pub fn info(app: &AppHandle, source: &'static str, message: impl Into<String>) {
    let _ = app.emit(
        "taxo://log",
        LogPayload {
            level: "info",
            message: message.into(),
            source,
        },
    );
}

/// Emit a WARN-level log event to the frontend TaxoLog.
pub fn warn(app: &AppHandle, source: &'static str, message: impl Into<String>) {
    let _ = app.emit(
        "taxo://log",
        LogPayload {
            level: "warn",
            message: message.into(),
            source,
        },
    );
}

/// Emit an ERROR-level log event to the frontend TaxoLog.
pub fn error(app: &AppHandle, source: &'static str, message: impl Into<String>) {
    let _ = app.emit(
        "taxo://log",
        LogPayload {
            level: "error",
            message: message.into(),
            source,
        },
    );
}
