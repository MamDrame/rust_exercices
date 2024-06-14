use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Park {
    pub name: String,
    pub park_type: ParkType,
    pub address: String,
    pub cap: String,
    pub state: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParkType {
    Garden,
    Forest,
    Playground,
}

impl fmt::Display for Park {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}, {}, {} - {}",
            self.park_type,
            if self.name.is_empty() {
                "No name"
            } else {
                &self.name
            },
            if self.address.is_empty() {
                "No address"
            } else {
                &self.address
            },
            if self.cap.is_empty() {
                "No cap"
            } else {
                &self.cap
            },
            if self.state.is_empty() {
                "No state"
            } else {
                &self.state
            }
        )
    }
}

impl fmt::Display for ParkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match *self {
            ParkType::Garden => "garden",
            ParkType::Forest => "forest",
            ParkType::Playground => "playground",
        };
        write!(f, "{}", type_str)
    }
}
