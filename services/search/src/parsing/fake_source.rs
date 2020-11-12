use super::reader::DocReader;
use async_std::path::PathBuf;
use async_trait::async_trait;

/// Represents a fake source.
///
/// Just returns fake data when asked for.
pub struct FakeSource {}

impl FakeSource {
    pub fn new() -> FakeSource {
        FakeSource {}
    }
}

#[async_trait]
impl DocReader for FakeSource {
    async fn parse(&mut self) -> Result<String, std::io::Error> {
        Ok("c++ python java c#".into())
    }
}
