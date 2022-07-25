pub enum Env {
    Local,
    Production,
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Local => "localhost",
            Env::Production => "production",
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "localhost" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `localhost` or `production`.",
                other
            )),
        }
    }
}
