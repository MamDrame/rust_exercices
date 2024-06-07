//Sometimes it is more desirable to catch the failure of some parts of a program instead of just calling panic.
//
// For this exercise you will have to create a tool that manages users' access levels.
//
// You will have to create an AccessLevel enum which could be Guest, Normal, Admin.
//
// You will also have to create a User struct which has:
//
// Fields:
// name: String
// acessLevel: enum
// Associated functions:
// new: which initializes the struct.
// send_name: which takes only self as argument and returns an Option<&str> with None if the user is a Guest or the name if the AccessLevel has any of the other options.
// Other functions:
// check_user_name: which takes a User, calls send_name and returns a tuple with true and the username if send_name returns the name or false and "Error: User is guest" if not.
pub enum AccessLevel {
    Guest,
    Normal,
    Admin
}

pub struct User {
    name:String,
    access_level: AccessLevel
}

impl User {
    pub fn new(name: String, level: AccessLevel) -> User {
        User {
            name,
            access_level: level,
        }
    }
    pub fn send_name(&self) -> Option<&str> {
        match self.access_level {
            AccessLevel::Guest => None,
            _ => Some(&self.name),
        }
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        Some(name) => (true, name),
        None => (false, "ERROR: User is guest"),
    }
}