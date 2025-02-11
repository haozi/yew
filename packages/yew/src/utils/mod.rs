//! This module contains useful utilities to get information about the current document.

use std::marker::PhantomData;

use anyhow::{anyhow, Error};
use web_sys::{Document, Window};

use yew::html::ChildrenRenderer;

mod not_equal_assign;
pub use not_equal_assign::*;

/// Returns the current window. This function will panic if there is no available window.
pub fn window() -> Window {
    web_sys::window().expect("no window available")
}

/// Returns the current document.
pub fn document() -> Document {
    window().document().unwrap()
}

/// Returns the `host` for the current document. Useful for connecting to the server which serves
/// the app.
pub fn host() -> Result<String, Error> {
    let location = document()
        .location()
        .ok_or_else(|| anyhow!("can't get location"))?;

    let host = location.host().map_err(|e| {
        anyhow!(e
            .as_string()
            .unwrap_or_else(|| String::from("error not recoverable")),)
    })?;

    Ok(host)
}

/// Returns the `origin` of the current window.
pub fn origin() -> Result<String, Error> {
    let location = window().location();

    let origin = location.origin().map_err(|e| {
        anyhow!(e
            .as_string()
            .unwrap_or_else(|| String::from("error not recoverable")),)
    })?;

    Ok(origin)
}

/// Map IntoIterator<Item=Into<T>> to Iterator<Item=T>
pub fn into_node_iter<IT, T, R>(it: IT) -> impl Iterator<Item = R>
where
    IT: IntoIterator<Item = T>,
    T: Into<R>,
{
    it.into_iter().map(|n| n.into())
}

/// A special type necessary for flattening components returned from nested html macros.
#[derive(Debug)]
pub struct NodeSeq<IN, OUT>(Vec<OUT>, PhantomData<IN>);

impl<IN: Into<OUT>, OUT> From<IN> for NodeSeq<IN, OUT> {
    fn from(val: IN) -> Self {
        Self(vec![val.into()], PhantomData::default())
    }
}

impl<IN: Into<OUT>, OUT> From<Vec<IN>> for NodeSeq<IN, OUT> {
    fn from(val: Vec<IN>) -> Self {
        Self(
            val.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        )
    }
}

impl<IN: Into<OUT>, OUT> From<ChildrenRenderer<IN>> for NodeSeq<IN, OUT> {
    fn from(val: ChildrenRenderer<IN>) -> Self {
        Self(
            val.into_iter().map(|x| x.into()).collect(),
            PhantomData::default(),
        )
    }
}

impl<IN, OUT> IntoIterator for NodeSeq<IN, OUT> {
    type Item = OUT;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Hack to force type mismatch compile errors in yew-macro.
//
// TODO: replace with `compile_error!`, when `type_name_of_val` is stabilised (https://github.com/rust-lang/rust/issues/66359).
#[doc(hidden)]
pub fn __ensure_type<T>(_: T) {}

/// Print the [web_sys::Node]'s contents as a string for debugging purposes
pub fn print_node(n: &web_sys::Node) -> String {
    use wasm_bindgen::JsCast;

    match n.dyn_ref::<web_sys::Element>() {
        Some(el) => el.outer_html(),
        None => n.text_content().unwrap_or_default(),
    }
}
