// Copyright (C) 2018 Stephane Raux. Distributed under the MIT license.

macro_rules! element_constructor {
    ($name:ident) => {
        #[doc="Returns an HTML element of the given type"]
        pub fn $name<'a, W>(out: &'a mut ::xml::EventWriter<W>)
            -> $crate::Element<'a, W>
        where
            W: ::std::io::Write + 'a,
        {
            $crate::Element::new(out, stringify!($name))
        }
    };
}
