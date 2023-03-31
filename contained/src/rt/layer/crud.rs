/*
    Appellation: crud <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{BsonOid, Timestamp};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use strum::{Display, EnumString, EnumVariantNames};


#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum CRUD {
    Create,
    #[default]
    Read,
    Update,
    Delete
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum CRUDStatus {
    Failure,
    #[default]
    Success,
}

#[derive(
    Clone,
    Debug,
    Deserialize,
    Display,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
    SmartDefault,
)]
#[repr(i64)]
#[strum(serialize_all = "snake_case")]
pub enum CRUDFrame<T: Serialize = String> {
    #[default]
    Request {
        addr: String,
        crud: CRUD,
        payload: Option<T>,
    },
    Response {
        crud: CRUD,
        payload: Option<T>,
        status: CRUDStatus,
    },
}

pub struct Message<T> {
    id: String,
    method: CRUD,
    payload: Option<T>,
    ts: i64
}

impl<T> Message<T> {
    pub fn new(method: CRUD, payload: Option<T>) -> Self {
        let id = BsonOid::new().to_hex();
        let ts = Timestamp::default().into();
        Self {
            id,
            method,
            payload,
            ts
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn method(&self) -> CRUD {
        self.method
    }
    pub fn payload(&self) -> Option<&T> {
        self.payload.as_ref()
    }
    pub fn ts(&self) -> i64 {
        self.ts
    }
}