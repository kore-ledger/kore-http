use std::{collections::HashMap, time::SystemTime};

/// States
// Structure that keeps track of the number of times a ip has used in 1 minute
#[derive(Clone)]
pub struct ConnectionsDuration {
    pub time: SystemTime,
    pub connections: u32,
}

// Structure that relates a ip's to his or her connection attempts
#[derive(Clone)]
pub struct IPSMaxConnectState {
    pub ips_connects: HashMap<String, ConnectionsDuration>,
}
