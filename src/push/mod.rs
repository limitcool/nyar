mod pushplus;
pub use pushplus::*;
pub trait Push {
    async fn send_message(
        &self,
        message: String,
        title: String,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
