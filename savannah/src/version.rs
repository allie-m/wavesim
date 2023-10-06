use std::fmt::Formatter;
use std::process::Command;

/// A major / minor / patch version. Each part can only take up 2 digits. Represented in the
/// form of xx.yy.zz.
pub struct Version(pub Os, pub u32);

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Os {
    None,
    macOS,
    iOS,
    /// Not yet supported.
    tvOS,
    /// Not yet supported.
    watchOS,
}

impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Os::None => "none",
                Os::macOS => "macOS",
                Os::iOS => "iOS",
                Os::tvOS => "tvOS",
                Os::watchOS => "watchOS",
            }
        )
    }
}

impl Version {
    pub fn get_major(&self) -> u8 {
        (self.1 / 10000) as u8
    }
    pub fn get_minor(&self) -> u8 {
        ((self.1 / 100) % 100) as u8
    }
    pub fn get_patch(&self) -> u8 {
        (self.1 % 100) as u8
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}.{}.{}",
            self.0,
            self.get_major(),
            self.get_minor(),
            self.get_patch(),
        )
    }
}

/// Returns the version of the apple os running this.
pub fn get_os_version() -> Version {
    let v: Vec<u32> = String::from_utf8(
        Command::new("sh")
            .args(&["-c", "sw_vers -productVersion"])
            .output()
            .expect("Could not execute this command")
            .stdout,
    )
        .unwrap()
        .split(".")
        .map(|v: &str| v.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    Version(
        {
            #[cfg(target_os = "macos")]
                {
                    Os::macOS
                }
            #[cfg(target_os = "ios")]
                {
                    Os::iOS
                }
        },
        v[0] * 10000 + v[1] * 100 + v[2],
    )
}
