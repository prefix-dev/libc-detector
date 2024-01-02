//! This crates provides utilities for detecting the version of libc on the current system.

#![deny(missing_docs)]

use memfd_exec::{MemFdExecutable, Stdio};
use std::{path::PathBuf, process::Command};

/// Returns a list of glibc detectors applicable for the current architecture.
///
/// A glibc detector is a binary that can be executed to determine the version of glibc.
fn glibc_detectors() -> Vec<(&'static str, &'static [u8])> {
    let mut detectors = Vec::new();

    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    {
        detectors.push((
            "x86_64",
            include_bytes!("./linux-glibc-detectors/glibc-detector-x86_64").as_slice(),
        ));
        detectors.push((
            "i686",
            include_bytes!("./linux-glibc-detectors/glibc-detector-i686").as_slice(),
        ));
    }

    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    {
        detectors.push((
            "aarch64",
            include_bytes!("./linux-glibc-detectors/glibc-detector-aarch64").as_slice(),
        ));
        detectors.push((
            "armv7l",
            include_bytes!("./linux-glibc-detectors/glibc-detector-armv7l").as_slice(),
        ));
    }

    #[cfg(target_arch = "powerpc64")]
    {
        detectors.push((
            "ppc64le",
            include_bytes!("./linux-glibc-detectors/glibc-detector-ppc64le").as_slice(),
        ));
    }

    #[cfg(target_arch = "s390x")]
    {
        detectors.push((
            "s390x",
            include_bytes!("./linux-glibc-detectors/glibc-detector-s390x").as_slice(),
        ));
    }

    detectors
}

/// Detect the current version of `glibc` using a binary detector.
pub fn glibc_version() -> Option<(u32, u32)> {
    for (arch, detector) in glibc_detectors() {
        let output = MemFdExecutable::new("glibc-detector", detector)
            .stdout(Stdio::piped())
            .output();

        let stdout = match &output {
            Ok(output) => {
                if output.status.code() != Some(0) {
                    tracing::debug!("glibc detector for {arch} exited with {:?}", output.status);
                    continue;
                }
                String::from_utf8_lossy(&output.stdout)
            }
            Err(err) => {
                tracing::debug!("glibc detector for {arch} failed: {err}");
                continue;
            }
        };

        let Some((major, minor)) = parse_major_minor_version(&stdout) else {
            tracing::debug!("failed to parse glibc version '{stdout}'");
            continue;
        };

        return Some((major, minor));
    }

    None
}

/// Detect the current version of `musl` `libc` by inspecting the `/lib/ld-musl-*.so.1` loaders.
pub fn musl_libc_version() -> Option<(u32, u32)> {
    for arch in ["x86_64", "aarch64", "i386", "armhf", "powerpc64le", "s390x"] {
        let loader = PathBuf::from(format!("/lib/ld-musl-{arch}.so.1"));
        if !loader.exists() {
            continue;
        }

        match Command::new(loader).output() {
            Err(e) => {
                tracing::debug!("failed to execute musl loader for {arch}: {e}");
                continue;
            }
            Ok(output) => {
                // Don't check output.status, because it's expected to return non-zero.
                let output_text = String::from_utf8_lossy(&output.stderr);

                // The output is in the form of "Version {major}.{minor}"
                let Some((major, minor)) = output_text
                    .lines()
                    .find_map(|l| l.strip_prefix("Version "))
                    .and_then(parse_major_minor_version)
                else {
                    tracing::debug!("failed to parse musl version from '{output_text}'");
                    continue;
                };

                return Some((major, minor));
            }
        }
    }

    None
}

/// Parses a version string into a major and minor version.
fn parse_major_minor_version(version: &str) -> Option<(u32, u32)> {
    let mut segment_iter = version.trim().split('.');
    let major = segment_iter.next()?.parse().ok()?;
    let minor = segment_iter.next()?.parse().ok()?;
    Some((major, minor))
}

/// The family of libc implementation.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum LibCFamily {
    /// GNU libc
    GLibC,

    /// musl libc
    Musl,
}

/// Represents a detected version of libc.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LibCVersion {
    /// The family of the libc implementation.
    pub family: LibCFamily,

    /// The major and minor version of the library.
    pub version: (u32, u32),
}

/// Detects the current version and family of libc on the system.
pub fn libc_version() -> Option<LibCVersion> {
    if let Some(version) = glibc_version() {
        return Some(LibCVersion {
            family: LibCFamily::GLibC,
            version,
        });
    }

    if let Some(version) = musl_libc_version() {
        return Some(LibCVersion {
            family: LibCFamily::Musl,
            version,
        });
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_glibc_version() {
        eprintln!("glibc version: {:?}", glibc_version());
    }

    #[test]
    fn test_musl_version() {
        eprintln!("musl version: {:?}", musl_libc_version());
    }

    #[test]
    fn test_libc_version() {
        let version = libc_version();
        match version {
            Some(version) => eprintln!("libc version: {version:?}"),
            None => panic!("no libc version detected"),
        }
    }
}
