#[cfg(feature="networkmanager")]
mod networkmanager;

#[cfg(feature="networkmanager")]
pub use networkmanager::watch_network;