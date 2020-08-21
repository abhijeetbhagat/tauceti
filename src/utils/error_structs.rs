#[derive(Debug)]
pub enum TaucetiError {
    AppError,
    EventListenerError,
    MessageBrokerError,
    QueryParseError,
}
