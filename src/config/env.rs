pub enum Env {
    Local
}

impl Env {
    pub fn as_str(&self) -> &'static str {
        match self {
            Env::Local => "localhost"
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "localhost" => Ok(Self::Local),
            other => Err(format!(
                "{} is not a supported environment. The only supported environment is localhost.",
                other
            )),
        }
    }
}
