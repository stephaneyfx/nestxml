// Copyright (C) 2018 Stephane Raux. Distributed under the MIT license.

//! HTML helpers.

use std::io::Write;
use xml::EventWriter;
use xml::writer::Error;

element_constructor!(html);
element_constructor!(head);
element_constructor!(title);
element_constructor!(meta);
element_constructor!(style);
element_constructor!(body);
element_constructor!(table);
element_constructor!(tr);
element_constructor!(th);
element_constructor!(td);
element_constructor!(h1);
element_constructor!(h2);
element_constructor!(ul);
element_constructor!(ol);
element_constructor!(li);
element_constructor!(a);

/// Writes the html DOCTYPE declaration.
pub fn write_doctype<W: Write>(w: &mut EventWriter<W>) -> Result<(), Error> {
    w.inner_mut().write_all(b"<!DOCTYPE html>\n")?;
    Ok(())
}
