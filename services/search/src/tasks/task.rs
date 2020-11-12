pub trait TaucetiTask {
    fn execute() -> Result<(), std::io::Error>;
}
