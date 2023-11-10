use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Error return when conversion to [`BackupStrategy`] fails
    #[error("Provided value cannot be converted BackupStrategy enum")]
    InvalidBackupType,

    /// Error return when conversion to [`BackupCompression`] fails
    #[error("Provided value cannot be converted BackupCompression enum")]
    InvalidCompressionType,

    /// Error returned when a required directory does not exit
    #[error("No volume mounted at: {0}")]
    NoVolumeMounted(String),

    // ### Converting from other error types ###
    /// Pass-thru [`std::io::Error`].
    #[error("std::io Error: {0}")]
    IO(#[from] std::io::Error),

    /// Pass-thru `time::error::Error`
    #[error("time::error::Error: {0}")]
    Time(#[from] time::error::Error),

    /// Pass-thru `time::error::IndeterminateOffset`
    #[error("time::error::IndeterminateOffset Error: {0}")]
    TimeOffset(#[from] time::error::IndeterminateOffset),

    /// Pass-thru `time::error::Format`
    #[error("time::error::Format Error: {0}")]
    TimeFormat(#[from] time::error::Format),
}