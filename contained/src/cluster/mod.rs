/*
    Appellation: cluster <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: A cluster describes a user-owned set of nodes, which are used to orchestrate various workloads and implement a personal cloud.
        The cluster implements a subnet to synchronize activities between the systems and to provide a secure, private, and reliable network.
        Doing so also allows the clusters to support the implementation of a distributed file system, which can be used to store and share data.
*/
//! # Clusters
//! 
//! Clusters describe the physical networking layer of the system. Each cluster is composed of a set of user-owned nodes
//! which are used to orchestrate various workloads and implement a personal cloud. Additionally, each cluster is 
//! used to abstraclty describe a type of virtual node used to empower the mainnet.
pub use self::stack::*;

mod stack;
