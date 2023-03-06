/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub trait Handle<T> {
    type Output;

    fn handle(&mut self, msg: T) -> Self::Output;
}

#[async_trait::async_trait]
pub trait AsyncHandle<T: Send + Sync> {
    type Output: Send + Sync;

    async fn handle(&mut self, msg: T) -> Self::Output;
}
