use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, Mutex};
use tracing::field::Field;
use tracing::Event;
use tracing_subscriber::layer::Context;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

const MAX_LOG_ENTRIES: usize = 200;

pub(crate) type LogBuffer = Arc<Mutex<Vec<LogEntry>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub message: String,
}

pub(crate) fn create_log_buffer() -> LogBuffer {
    Arc::new(Mutex::new(Vec::new()))
}

// -- Custom tracing layer that captures frontend-marked events --

struct FrontendLogLayer {
    buffer: LogBuffer,
}

impl<S> Layer<S> for FrontendLogLayer
where
    S: tracing::Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut visitor = FrontendVisitor::default();
        event.record(&mut visitor);
        if visitor.is_frontend {
            let mut logs = self.buffer.lock().unwrap();
            logs.push(LogEntry {
                timestamp: Local::now().format("%H:%M:%S").to_string(),
                message: visitor.message,
            });
            if logs.len() > MAX_LOG_ENTRIES {
                logs.remove(0);
            }
        }
    }
}

#[derive(Default)]
struct FrontendVisitor {
    is_frontend: bool,
    message: String,
}

impl tracing::field::Visit for FrontendVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        if field.name() == "frontend" {
            self.is_frontend = value;
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{:?}", value);
        }
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        }
    }
}

pub(crate) fn init_tracing(buffer: LogBuffer) {
    let frontend_layer = FrontendLogLayer { buffer };
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .with(frontend_layer)
        .init();
}
