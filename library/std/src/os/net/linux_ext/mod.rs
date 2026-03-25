//! Linux, Android and Cygwin-specific networking functionality.

#![doc(cfg(any(any(target_os = "linux", target_os = "runixos"), target_os = "android", target_os = "cygwin")))]

#[stable(feature = "unix_socket_abstract", since = "1.70.0")]
pub(crate) mod addr;

#[unstable(feature = "unix_socket_ancillary_data", issue = "76915")]
pub(crate) mod socket;

#[stable(feature = "tcp_quickack", since = "1.89.0")]
pub(crate) mod tcp;

#[cfg(test)]
mod tests;
