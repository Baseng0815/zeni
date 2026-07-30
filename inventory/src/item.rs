//! Item definitions: the *kind* of thing that can be stocked.
//!
//! An item is the catalogue entry ("wheat flour, 1 kg bag"), distinct from the
//! [`crate::stock`] lots that record actual on-hand instances of it. Items are
//! long-lived and shared; lots come and go.
//!
//! Planned surface: an `Item` with an id, name, default `Unit`, optional
//! barcode, the [`crate::location`] it normally lives in, and a desired minimum
//! stock level that [`crate::shopping`] compares against.
