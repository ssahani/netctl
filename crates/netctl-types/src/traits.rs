use async_trait::async_trait;

#[async_trait]
pub trait NetworkDevice: Send + Sync {
    fn name(&self) -> &str;
    fn index(&self) -> u32;
}
