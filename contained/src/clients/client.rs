/*
    Appellation: client <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use tokio::sync::mpsc;


pub struct Client {
    request: mpsc::Sender<String>,
}
