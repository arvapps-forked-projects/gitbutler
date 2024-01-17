//! High-level operations that are implementation-agnostic.
//!
//! These operations are similar to Git's non-plumbing commands,
//! in that they compose both high- and low-level operations
//! into more complex operations, without caring about the
//! underlying implementation.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::{ConfigScope, Repository};

/// Returns whether or not the repository has GitButler's
/// utmost discretion enabled.
pub async fn has_utmost_discretion<R: Repository>(repo: &R) -> Result<bool, R::Error> {
    let config = repo
        .config_get("gitbutler.utmostDiscretion", ConfigScope::Auto)
        .await?;
    Ok(config == Some("true".to_string()))
}

/// Sets whether or not the repository has GitButler's utmost discretion.
pub async fn set_utmost_discretion<R: Repository>(repo: &R, value: bool) -> Result<(), R::Error> {
    repo.config_set(
        "gitbutler.utmostDiscretion",
        if value { "true" } else { "false" },
        ConfigScope::Local,
    )
    .await
}