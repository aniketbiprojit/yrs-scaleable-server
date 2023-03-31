use std::collections::HashSet;

use actix::Addr;
use chashmap::CHashMap;

use crate::ws_actor::WSActor;

pub(crate) type BroadcastToAddresses = CHashMap<String, HashSet<Addr<WSActor>>>;
