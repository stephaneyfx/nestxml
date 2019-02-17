// Copyright (C) 2018 Stephane Raux. Distributed under the MIT license.

//! Tools to nest XML or HTML elements. Relies on the xml-rs crate.
//!
//! # Example
//!
//! ```
//! use xml::EmitterConfig;
//!
//! fn main() {
//!     let out = Vec::new();
//!     let mut out = EmitterConfig::new()
//!         .write_document_declaration(false)
//!         .create_writer(out);
//!     nestxml::element(&mut out, "contacts").write(|out| {
//!         nestxml::element(out, "first_name").text("John")?;
//!         nestxml::element(out, "last_name").text("Doe")
//!     }).unwrap();
//!     let out = out.into_inner();
//!     assert_eq!(&out[..], &b"<contacts><first_name>John</first_name>\
//!         <last_name>Doe</last_name></contacts>"[..]);
//! }
//! ```

#![deny(missing_docs)]
#![deny(warnings)]

#[macro_use]
mod macros;
pub mod html;

use std::io::Write;
use xml::EventWriter;
use xml::writer::{Error, XmlEvent};

/// XML element
pub struct Element<'a, W: 'a> {
    out: &'a mut EventWriter<W>,
    name: String,
    attributes: Vec<(String, String)>,
}

impl<'a, W: Write + 'a> Element<'a, W> {
    /// Returns a XML element with the given `name` that will be written to
    /// `out`.
    pub fn new<S>(out: &'a mut EventWriter<W>, name: S) -> Self
    where
        S: Into<String>,
    {
        Element {out, name: name.into(), attributes: Vec::new()}
    }

    /// Adds an attribute to the element.
    pub fn attr<N, V>(mut self, name: N, value: V) -> Self
    where
        N: Into<String>,
        V: Into<String>,
    {
        self.attributes.push((name.into(), value.into()));
        self
    }

    /// Writes this element and invokes `f` to fill in its children.
    pub fn write<F>(self, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut EventWriter<W>) -> Result<(), Error>,
    {
        self.write_res(f)
    }

    /// Writes this element and invokes `f` to fill in its children.
    pub fn write_res<F, T, E>(self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut EventWriter<W>) -> Result<T, E>,
        E: From<Error>,
    {
        let mut elem_start = XmlEvent::start_element(self.name.as_str());
        for (name, value) in &self.attributes {
            elem_start = elem_start.attr(name.as_str(), value.as_str());
        }
        self.out.write(elem_start)?;
        let res = f(self.out)?;
        self.out.write(XmlEvent::end_element())?;
        Ok(res)
    }

    /// Writes this element without children.
    pub fn empty(self) -> Result<(), Error> {
        self.write(|_| Ok(()))
    }

    /// Writes this element with the given text as its only child.
    pub fn text(self, s: &str) -> Result<(), Error> {
        self.write(|out| out.write(XmlEvent::characters(s)))
    }
}

/// Returns an `Element`.
pub fn element<'a, W, S>(out: &'a mut EventWriter<W>, name: S) -> Element<'a, W>
where
    W: Write + 'a,
    S: Into<String>,
{
    Element::new(out, name)
}
