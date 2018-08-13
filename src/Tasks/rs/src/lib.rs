#![feature(async_await)]
#![feature(futures_api)]

#[cfg(not(meson))]
extern crate lmcp;
#[cfg(meson)]
extern crate lmcp_rs as lmcp;

pub mod line_search;
