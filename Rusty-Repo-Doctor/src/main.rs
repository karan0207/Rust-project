use git2::Repository;
use cargo_metadata::{MetadataCommand, Dependency};
use std::path::Path;
use serde::Serialize;

#[derive(Serialize)]
struct DependencyInfo {
    name: String,
    version: String,
    is_outdated: bool,
}
