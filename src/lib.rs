//! Zenver provides a simple Version struct for representing version numbers.
//!
//! # Examples
//!
//! ```
//! use zenver::Version;
//!
//! let mut version = Version::new(1);
//! version.next();
//! println!("Next version: {}", version);
//! ```

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Represents a version number.
pub struct Version {
    number: u128,
}

impl Version {
    /// Creates a new Version instance with the specified number.
    ///
    /// # Example
    ///
    /// ```
    /// use zenver::Version;
    ///
    /// let version = Version::new(1);
    /// ```
    pub fn new(number: u128) -> Self {
        Self { number }
    }

    /// Increments the version number by one.
    ///
    /// # Example
    ///
    /// ```
    /// use zenver::Version;
    ///
    /// let mut version = Version::new(1);
    /// version.next();
    /// assert_eq!(version.to_string(), "2");
    /// ```
    pub fn next(&mut self) {
        self.number += 1;
    }
}

impl Display for Version {
    /// Formats the Version as a string.
    ///
    /// # Example
    ///
    /// ```
    /// use zenver::Version;
    ///
    /// let version = Version::new(123);
    /// assert_eq!(version.to_string(), "123");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_version() {
        let version = Version::new(1);
        assert_eq!(version.number, 1);
    }

    #[test]
    fn test_next_version() {
        let mut version = Version::new(1);
        version.next();
        assert_eq!(version.number, 2);
    }

    #[test]
    fn test_to_string() {
        let version = Version::new(123);
        assert_eq!(version.to_string(), "123");
    }
}
