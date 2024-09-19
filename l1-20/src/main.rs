// Код, который записывает лог в файл
struct FileLogger;

impl FileLogger {
    fn log(&self, message: &str) {
        println!("Logging to file: {}", message);
    }
}

// Код, который записывает лог в базу данных
struct DatabaseLogger;

impl DatabaseLogger {
    fn write(&self, message: &str) {
        println!("Writing to database: {}", message);
    }
}

// Адаптер для FileLogger, чтобы использовать его как ILogger
struct FileLoggerAdapter {
    file_logger: FileLogger,
}

impl FileLoggerAdapter {
    fn new(file_logger: FileLogger) -> FileLoggerAdapter {
        FileLoggerAdapter { file_logger }
    }
}

impl ILogger for FileLoggerAdapter {
    fn log(&self, message: &str) {
        self.file_logger.log(message);
    }
}

// Адаптер для DatabaseLogger, чтобы использовать его как ILogger
struct DatabaseLoggerAdapter {
    database_logger: DatabaseLogger,
}

impl DatabaseLoggerAdapter {
    fn new(database_logger: DatabaseLogger) -> DatabaseLoggerAdapter {
        DatabaseLoggerAdapter { database_logger }
    }
}

impl ILogger for DatabaseLoggerAdapter {
    fn log(&self, message: &str) {
        self.database_logger.write(message);
    }
}

// Интерфейс для логирования
trait ILogger {
    fn log(&self, message: &str);
}

fn main() {
    let file_logger = FileLoggerAdapter::new(FileLogger);
    let database_logger = DatabaseLoggerAdapter::new(DatabaseLogger);

    // Используем адаптеры для записи логов
    let loggers: Vec<Box<dyn ILogger>> = vec![
        Box::new(file_logger),
        Box::new(database_logger),
    ];

    for logger in loggers {
        logger.log("This is a log message");
    }
}
