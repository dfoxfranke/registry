use core::fmt;
use std::{str::FromStr, time::SystemTime};

use crate::hash;
use crate::signing;
use crate::version::Version;

/// A package record is a collection of entries published together by the same author
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageRecord {
    /// The hash of the previous package record envelope
    pub prev: Option<hash::Hash>,
    /// The version of the registry protocol used
    pub version: u32,
    /// When this record was published
    pub timestamp: SystemTime,
    /// The entries being published in this record
    pub entries: Vec<PackageEntry>,
}

/// Each permission represents the ability to use the specified entry
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Permission {
    Release,
    Yank,
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Permission::Release => write!(f, "release"),
            Permission::Yank => write!(f, "yank"),
        }
    }
}

impl FromStr for Permission {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "release" => Ok(Permission::Release),
            "yank" => Ok(Permission::Yank),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageEntry {
    /// Initializes a package log.
    /// Must be the first entry of every log and not appear elsewhere.
    Init {
        /// The hash algorithm this log will use for linking
        hash_algorithm: hash::HashAlgorithm,
        /// The key of the original package maintainer
        key: signing::PublicKey,
    },
    /// Grant the specified key a permission.
    /// The author of this entry must have the permission.
    GrantFlat {
        key: signing::PublicKey,
        permission: Permission,
    },
    /// Remove a permission from a key.
    /// The author of this entry must have the permission.
    RevokeFlat {
        key_id: hash::Hash,
        permission: Permission,
    },
    /// Release a version of a package.
    /// The version must not have been released yet.
    Release {
        version: Version,
        content: hash::Hash,
    },
    /// Yank a version of a package.
    /// The version must have been released and not yanked.
    Yank {
        version: Version,
    },
}

impl PackageEntry {
    /// Check permission is required to submit this entry
    pub fn required_permission(&self) -> Option<Permission> {
        match self {
            PackageEntry::Init { .. } => None,
            PackageEntry::GrantFlat { .. } => None,
            PackageEntry::RevokeFlat { .. } => None,
            PackageEntry::Release { .. } => Some(Permission::Release),
            PackageEntry::Yank { .. } => Some(Permission::Yank),
        }
    }
}