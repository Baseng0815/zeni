use crate::money::{
    Currency,
    Money,
};
use std::fmt::{
    Display,
    Formatter,
};

impl Display for Currency {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Currency::EUR => write!(f, "€"),
        }
    }
}

impl Display for Money {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        self.currency.format_cents(self.amount, f)
    }
}

impl Currency {
    pub fn format_cents(
        &self,
        cents: i128,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Currency::EUR => {
                let sign = if cents < 0 { "-" } else { "" };
                let abs = cents.unsigned_abs();
                write!(f, "{sign}{},{:02} {self}", abs / 100, abs % 100)?;
            }
        }

        Ok(())
    }
}
