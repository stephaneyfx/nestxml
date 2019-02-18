// Copyright (C) 2018 Stephane Raux. Distributed under the MIT license.

//! HTML helpers.

use std::io::Write;
use xml::EventWriter;
use xml::writer::Error;

element_constructor!(a);
element_constructor!(b);
element_constructor!(body);
element_constructor!(br);
element_constructor!(button);
element_constructor!(canvas);
element_constructor!(caption);
element_constructor!(code);
element_constructor!(div);
element_constructor!(em);
element_constructor!(form);
element_constructor!(h1);
element_constructor!(h2);
element_constructor!(h3);
element_constructor!(h4);
element_constructor!(h5);
element_constructor!(h6);
element_constructor!(head);
element_constructor!(html);
element_constructor!(i);
element_constructor!(iframe);
element_constructor!(img);
element_constructor!(input);
element_constructor!(label);
element_constructor!(li);
element_constructor!(link);
element_constructor!(meta);
element_constructor!(nav);
element_constructor!(noscript);
element_constructor!(ol);
element_constructor!(p);
element_constructor!(pre);
element_constructor!(q);
element_constructor!(script);
element_constructor!(span);
element_constructor!(strong);
element_constructor!(style);
element_constructor!(sub);
element_constructor!(sup);
element_constructor!(svg);
element_constructor!(table);
element_constructor!(td);
element_constructor!(textarea);
element_constructor!(th);
element_constructor!(title);
element_constructor!(tr);
element_constructor!(ul);

/// Writes the html DOCTYPE declaration.
pub fn write_doctype<W: Write>(w: &mut EventWriter<W>) -> Result<(), Error> {
    w.inner_mut().write_all(b"<!DOCTYPE html>\n")?;
    Ok(())
}
