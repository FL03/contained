/*
    Appellation: node <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements the node for the clusters / subnets. A node is best understood as a fragmented tonnetz capable of being glued together with other eligble nodes.
        With this in mind, it is important to consider that the mainnet considers the interactions between engaging entities or clusters.
        The mainnet is a virtual overlay network designed for efficient and secure communication between clusters, permitting the exchange of data and information between users,
        and for persisting information, resources, and otherwirse. In order to support the mainnet, users typically allocate a set amount of resources or specify a certain device in their personal cloud
        for the network to leverage. These partitions are cryptographically secure and prevent the user from accessing the contents of the device once toggled.

        Subnets or clusters are made up of physical nodes, optimized for the execution of various workloads and services. Each device registered to the system is partitioned into a set of locally persisted
        triads,
*/

use crate::{clients::Client, rt::Runtime};

pub struct Node {
    pub client: Client,
    pub rt: Runtime,
}
