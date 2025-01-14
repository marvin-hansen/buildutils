use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EnvVar {
    env: String,
    value: String,
}

impl EnvVar {
    pub fn new(env: String, value: String) -> Self {
        Self { env, value }
    }
}

impl EnvVar {
    pub fn values(&self) -> (String, String) {
        (self.env.to_string(), self.value.to_string())
    }
}

impl Display for EnvVar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnvVar {{ Env: {} Value: {} }}", &self.env, &self.value)
    }
}
