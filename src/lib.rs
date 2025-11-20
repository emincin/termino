pub mod info;

#[cfg(target_family = "unix")]
#[path = "native/unix/io.rs"]
pub mod io;

#[cfg(target_family = "windows")]
#[path = "native/windows/io.rs"]
pub mod io;
