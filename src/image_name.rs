use std::{fmt::Display, ops::Deref};

/// Wrapper for strings used in image names.
///
/// It implements [Deref] for [String], thus every method that can be called on [String] can also
/// be called on ImageName.
#[derive(Debug, Clone)]
pub struct ImageName(pub String);

impl Deref for ImageName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ImageName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
