macro_rules! bin {
    ($bin:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let bin = $bin;
        #[cfg(target_os = "windows")]
        let bin = concat!($bin, ".exe");
        bin
    }};
}

macro_rules! bat {
    ($bat:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let bat = $bat;
        #[cfg(target_os = "windows")]
        let bat = concat!($bat, ".bat");
        bat
    }};
}

macro_rules! cmd {
    ($cmd:expr) => {{
        #[cfg(not(target_os = "windows"))]
        let cmd = $cmd;
        #[cfg(target_os = "windows")]
        let cmd = concat!($cmd, ".cmd");
        cmd
    }};
}

pub mod apk;
pub mod cargo;
pub mod dylibs;
pub mod error;
pub mod manifest;
pub mod ndk;
pub mod readelf;
pub mod target;
