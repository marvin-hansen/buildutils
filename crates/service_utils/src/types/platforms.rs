// MacOS, ARM_64; MacOS can only link dynamically; thus no linking specifier
const MAC_OS_ARM64: &str = "Mach-O 64-bit executable arm64";

// MacOS, X86_64; MacOS can only link dynamically; thus no linking specifier
const MAC_OS_X86_64: &str = "Mach-O 64-bit executable x86-64";

// Linux, X86_64, STATICALLY linked
const LINUX_X86_64_STATIC: &str =
    "ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), static-pie linked";

// Linux, ARM_64, STATICALLY linked
const LINUX_ARM_64_STATIC: &str =
    "ELF 64-bit LSB executable, ARM aarch64, version 1 (SYSV), statically linked";

// Linux, X86_64, DYNAMICALLY linked
const LINUX_X86_64_DYNAMIC: &str = "ELF 64-bit LSB shared object, x86-64";

// Linux, ARM_64, DYNAMICALLY linked
const LINUX_ARM_64_DYNAMIC: &str = "ELF 64-bit LSB shared object, ARM aarch64";

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Platform {
    MacOsArm64,
    MacOsX8664,
    LinuxX8664Static,
    LinuxArm64Static,
    LinuxX8664Dynamic,
    LinuxArm64Dynamic,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::MacOsArm64 => MAC_OS_ARM64,
            Platform::MacOsX8664 => MAC_OS_X86_64,
            Platform::LinuxX8664Static => LINUX_X86_64_STATIC,
            Platform::LinuxArm64Static => LINUX_ARM_64_STATIC,
            Platform::LinuxX8664Dynamic => LINUX_X86_64_DYNAMIC,
            Platform::LinuxArm64Dynamic => LINUX_ARM_64_DYNAMIC,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
