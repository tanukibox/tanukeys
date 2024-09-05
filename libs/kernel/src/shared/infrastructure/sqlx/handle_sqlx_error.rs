/*fn handle_sqlx_err(res: Result<Option<PgRow>, sqlx::Error>) {
    match res.err().unwrap() {
        sqlx::Error::Configuration(err) => {
            logger::error!("{}: {}", "Configuration", err);
        }
        sqlx::Error::Database(err) => {
            let msg = err.message();
            let code = err.code().unwrap();
            logger::error!("{}: {}", "Database", err);
        }
        sqlx::Error::Io(err) => {
            logger::error!("{}: {}", "Io", err);
        }
        sqlx::Error::Tls(err) => {
            logger::error!("{}: {}", "Tls", err);
        }
        sqlx::Error::Protocol(err) => {
            logger::error!("{}: {}", "Protocol", err);
        }
        sqlx::Error::ColumnNotFound(err) => {
            logger::error!("{}: {}", "ColumnNotFound", err);
        }
        sqlx::Error::Encode(err) => {
            logger::error!("{}: {}", "Encode", err);
        }
        sqlx::Error::Decode(err) => {
            logger::error!("{}: {}", "Decode", err);
        }
        sqlx::Error::AnyDriverError(err) => {
            logger::error!("{}: {}", "AnyDriverError", err);
        }
        sqlx::Error::Migrate(err) => {
            logger::error!("{}: {}", "Migrate", err);
        }
        sqlx::Error::RowNotFound => {}
        sqlx::Error::PoolTimedOut => {}
        sqlx::Error::PoolClosed => {}
        sqlx::Error::WorkerCrashed => {}
        sqlx::Error::ColumnDecode { .. } => {}
        sqlx::Error::TypeNotFound { .. } => {}
        sqlx::Error::ColumnIndexOutOfBounds { .. } => {}
        err => {
            logger::error!("{}: {}", "Unknown", err);
        }
    }
}*/