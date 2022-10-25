use std::{collections::BTreeMap, net::SocketAddr};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub messages: BTreeMap<DateTime<Utc>, Message>,
    pub active_users: BTreeMap<String, SocketAddr>,
}

impl Room {
    pub fn from(name: String) -> Self {
        Room {
            name,
            messages: BTreeMap::new(),
            active_users: BTreeMap::new(),
        }
    }
}
