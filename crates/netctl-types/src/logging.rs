use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub enum LogFormat {
    Pretty,
    Json,
}

pub enum LogLevel {
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

pub fn init_tracing(format: LogFormat, level: LogLevel) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level.as_str()));

    let registry = tracing_subscriber::registry().with(filter);

    match format {
        LogFormat::Pretty => {
            registry.with(fmt::layer().pretty()).init();
        }
        LogFormat::Json => {
            registry.with(fmt::layer().json()).init();
        }
    }
}
