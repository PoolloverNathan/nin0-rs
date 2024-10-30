#![cfg_attr(feature = "packet", feature(never_type))]

#[cfg(feature = "roles")]
pub mod roles;

#[cfg(feature = "packet")]
pub mod packet;

#[cfg(feature = "socket")]
pub mod socket;

#[cfg(feature = "rest")]
pub mod rest;
