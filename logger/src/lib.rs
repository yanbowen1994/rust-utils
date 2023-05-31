use std::collections::BTreeMap;
use std::io::Write;

use flexi_logger::writers::LogWriter;
use flexi_logger::{
    Age, Cleanup, Criterion, DeferredNow, FlexiLoggerError, FormatFunction, LogSpecBuilder,
    LogTarget, ModuleFilter, Naming, ReconfigurationHandle, Record,
};

pub struct StdoutLogWriter {
    format: FormatFunction,
}

impl StdoutLogWriter {
    fn new(format: FormatFunction) -> Self {
        Self { format }
    }
}

impl LogWriter for StdoutLogWriter {
    #[inline]
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        (self.format)(&mut std::io::stdout(), now, record)?;
        writeln!(std::io::stdout())
    }

    #[inline]
    fn flush(&self) -> std::io::Result<()> {
        std::io::stdout().flush()
    }

    #[inline]
    fn max_log_level(&self) -> log::LevelFilter {
        log::LevelFilter::Trace
    }

    fn format(&mut self, format: FormatFunction) {
        self.format = format;
    }
}

fn color_logger_format(
    writer: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        writer,
        "[{}][{}][{}:{}] {}",
        now.now().format("%Y-%m-%dT%H:%M:%S%.3f"),
        style(level, level),
        record.module_path().unwrap_or("<unnamed>"),
        record
            .line()
            .map(|x| x.to_string())
            .unwrap_or("".to_owned()),
        record.args(),
    )
}

fn style<T>(level: log::Level, item: T) -> yansi::Paint<T> {
    match level {
        log::Level::Error => yansi::Paint::red(item),
        log::Level::Warn => yansi::Paint::yellow(item),
        log::Level::Info => yansi::Paint::green(item),
        log::Level::Debug => yansi::Paint::blue(item),
        log::Level::Trace => yansi::Paint::magenta(item),
    }
}

fn nocolor_logger_format(
    writer: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    write!(
        writer,
        "[{}][{}][{}:{}] {}",
        now.now().format("%Y-%m-%dT%H:%M:%S%.3f"),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record
            .line()
            .map(|x| x.to_string())
            .unwrap_or("".to_owned()),
        record.args(),
    )
}

pub fn init_logger(
    log_level: log::LevelFilter,
    log_dir: Option<String>,
    log_rotate_day: bool,
    log_keep_files: Option<usize>,
    log_color: bool,
) -> Result<ReconfigurationHandle, FlexiLoggerError> {
    init_logger_with_filters(
        log_level,
        log_dir,
        log_rotate_day,
        log_keep_files,
        log_color,
        BTreeMap::new(),
    )
}

pub fn init_logger_with_filters(
    log_level: log::LevelFilter,
    log_dir: Option<String>,
    log_rotate_day: bool,
    log_keep_files: Option<usize>,
    log_color: bool,
    log_filters: BTreeMap<String, log::LevelFilter>,
) -> Result<ReconfigurationHandle, FlexiLoggerError> {
    let mut f: Vec<ModuleFilter> = vec![];

    f.push(ModuleFilter {
        module_name: None,
        level_filter: log_level,
    });

    for (m, level) in log_filters {
        f.push(ModuleFilter {
            module_name: Some(m),
            level_filter: level,
        });
    }

    let log_spec = LogSpecBuilder::from_module_filters(&f).build();

    let logger = if log_color {
        flexi_logger::Logger::with(log_spec)
            .format_for_writer(color_logger_format)
            .format_for_files(nocolor_logger_format)
            .log_target(LogTarget::FileAndWriter(Box::new(StdoutLogWriter::new(
                color_logger_format,
            ))))
    } else {
        flexi_logger::Logger::with(log_spec)
            .format_for_writer(nocolor_logger_format)
            .format_for_files(nocolor_logger_format)
            .log_target(LogTarget::FileAndWriter(Box::new(StdoutLogWriter::new(
                nocolor_logger_format,
            ))))
    };

    let logger = logger.append();

    let logger = match (log_rotate_day, log_keep_files.is_some()) {
        (true, true) => logger.rotate(
            Criterion::AgeOrSize(Age::Day, 1 << 25),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(*log_keep_files.as_ref().unwrap()),
        ),
        (true, false) => logger.rotate(
            Criterion::AgeOrSize(Age::Day, 1 << 25),
            Naming::Timestamps,
            Cleanup::Never,
        ),
        (false, true) => logger.rotate(
            Criterion::Size(1 << 25),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(*log_keep_files.as_ref().unwrap()),
        ),
        (false, false) => {
            logger.rotate(Criterion::Size(1 << 25), Naming::Timestamps, Cleanup::Never)
        }
    };

    if let Some(log_dir) = log_dir {
        logger.directory(log_dir).start()
    } else {
        logger.start()
    }
}
