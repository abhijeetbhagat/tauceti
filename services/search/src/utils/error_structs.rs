#[derive(Debug)]
pub enum TaucetiError {
    AppError,
    EventListenerError,
    MessageBrokerError,
    QueryParseError,
    SearchError,
    NotFoundInDictError,
    CacheConnectionError,
    CacheError,
    CacheDataNotFoundError,
    CacheWriteError,
    ServiceStartError,
}
