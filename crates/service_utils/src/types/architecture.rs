// X86_64,
const X86_64: &str = "x86-64";

// ARM_64
const ARM_64: &str = "aarch64";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Architecture {
    X8664,
    ARM64,
}

impl Architecture {
    fn as_str(&self) -> &'static str {
        match self {
            Architecture::X8664 => X86_64,
            Architecture::ARM64 => ARM_64,
        }
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
