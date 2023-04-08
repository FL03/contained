/*
    Appellation: cluster <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A cluster describes a user-owned set of nodes, which are used to orchestrate various workloads and implement a personal cloud.
        The cluster implements a subnet to synchronize activities between the systems and to provide a secure, private, and reliable network.
        Doing so also allows the clusters to support the implementation of a distributed file system, which can be used to store and share data.
*/
pub use self::stack::*;

mod stack;


