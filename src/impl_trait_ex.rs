pub struct Player {
    pub first_name: String,
    pub last_name: String,
}

pub trait LastName {
    fn last_name(&self) -> String;
}

pub trait FullName {
    fn full_name(&self) -> String;
}

impl FullName for Player {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

pub trait PlayNumber {
    fn getDefaultPlayerNumber(&self) -> i32 {
        22
    }
    fn getPlayerShirtNumberByName(&self) -> i32;
}

impl Player {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

impl LastName for Player {
    fn last_name(&self) -> String {
        format!("{}", self.last_name)
    }
}

impl PlayNumber for Player {
    fn getPlayerShirtNumberByName(&self) -> i32 {
        let shirt_number = match self.last_name.as_str() {
            "Federer" => 10,
            "Nadal" => 7,
            _ => 22,
        };
        shirt_number
    }
}
