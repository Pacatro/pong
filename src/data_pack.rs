pub struct DataPack {
    user: String,
    password: String
}

impl DataPack {
    pub fn new(user: &str, password: &str) -> DataPack {
        DataPack { user: user.to_string(), password: password.to_string() }
    }

    pub fn to_string(&self) -> String {
        format!("{};{}", self.user, self.password)
    }
}
