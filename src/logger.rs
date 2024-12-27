#![allow(warnings)]

use {
    anyhow::Result,
    chrono,
    tracing::{
        self,
        debug,
        debug_span,
        info,
        info_span,
        level_filters::LevelFilter,
        log,
        trace,
        warn_span,
        Instrument,
    },
    tracing_subscriber::{
        fmt,
        prelude::*,
    },
};

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => {{
        $crate::tracing::warn!(
            file=file!(),
            line=line!(),
            column=column!(),
            $fmt,
        );
    }};

    ($fmt:expr, $($args:tt)*) => {
        {
            $crate::tracing::warn!(
                file=file!(),
                line=line!(),
                column=column!(),
                $fmt,
                $($args)*
            );
        }
    };
}

#[macro_export]
macro_rules! panic {
    ($fmt:expr) => {{
        $crate::tracing::error!(
            file=file!(),
            line=line!(),
            column=column!(),
            $fmt,
        );
    }};
    ($fmt:expr, $($args:tt)*) => {
        {
            $crate::tracing::error!(
                file=file!(),
                line=line!(),
                column=column!(),
                $fmt,
                $($args)*
            );
        }
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => {{
        $crate::tracing::error!(
            file=file!(),
            line=line!(),
            column=column!(),
            $fmt,
        );
    }};
    ($fmt:expr, $($args:tt)*) => {
        {
            $crate::tracing::error!(
                file=file!(),
                line=line!(),
                column=column!(),
                $fmt,
                $($args)*
            );
        }
    };
}

pub fn init(
    file: Option<String>,
    verbose: bool,
) -> Result<Option<tracing_appender::non_blocking::WorkerGuard>> {
    let subscriber = tracing_subscriber::registry().with(
        tracing_subscriber::EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );
    let ret = if let Some(file) = file {
        let path = std::path::Path::new(file.as_str());
        let filename = path.file_name().unwrap();
        let dir = file.replace(filename.to_str().unwrap(), "");
        let file_appender = tracing_appender::rolling::daily(dir, filename);
        let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
        let subscriber = subscriber.with(fmt::Layer::new().with_writer(file_writer));
        if !verbose {
            tracing::subscriber::set_global_default(subscriber)?;
        } else {
            let subscriber = subscriber.with(fmt::Layer::new().with_writer(std::io::stdout));
            tracing::subscriber::set_global_default(subscriber)?;
        }
        Some(_guard)
    } else {
        let subscriber = subscriber.with(fmt::Layer::default().with_writer(std::io::stdout));
        tracing::subscriber::set_global_default(subscriber)?;
        None
    };
    Ok(ret)
}
