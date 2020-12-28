use super::{ConnectOptions, Runtime};

/// A unique connection (session) with a specific database.
///
/// For detailed information, refer to the asynchronous version of
/// this: [`Connection`][crate::Connection].
///
pub trait Connection<Rt>: 'static + Send
where
    Rt: Runtime,
{
    type Options: ConnectOptions<Rt, Connection = Self>;

    /// Establish a new database connection.
    ///
    /// For detailed information, refer to the asynchronous version of
    /// this: [`connect()`][crate::Connection::connect].
    ///
    fn connect(url: &str) -> crate::Result<Self>
    where
        Self: Sized,
    {
        url.parse::<Self::Options>()?.connect()
    }

    /// Explicitly close this database connection.
    ///
    /// For detailed information, refer to the asynchronous version of
    /// this: [`close()`][crate::Connection::close].
    ///
    fn close(self) -> crate::Result<()>;

    /// Checks if a connection to the database is still valid.
    ///
    /// For detailed information, refer to the asynchronous version of
    /// this: [`ping()`][crate::Connection::ping].
    ///
    fn ping(&mut self) -> crate::Result<()>;
}
