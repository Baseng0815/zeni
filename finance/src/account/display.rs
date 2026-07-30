use crate::account::Account;
use std::fmt::Display;

impl Display for Account {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.description, self.balance)
    }
}
