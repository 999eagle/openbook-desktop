use fern::colors::{Color, ColoredLevelConfig};

pub fn setup_logging(verbosity: u8, log_to_file: bool) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::White)
        .trace(Color::BrightBlack);
    let mut logger = fern::Dispatch::new();
    logger = match verbosity {
        0 => logger.level(log::LevelFilter::Warn),
        1 => logger.level(log::LevelFilter::Info),
        2 => logger.level(log::LevelFilter::Debug),
        _ => logger.level(log::LevelFilter::Trace),
    };

    let stdout_logger = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}][{}] {}\x1B[0m",
                format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str()),
                chrono::Local::now().format("%H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout());

    if log_to_file {
        logger = logger.chain(
            fern::Dispatch::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}][{}][{}] {}",
                        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .chain(fern::log_file("sol.log")?),
        );
    }
    logger.chain(stdout_logger).apply()?;
    Ok(())
}
