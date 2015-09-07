#[cfg(any(target_os = "mac_os", target_os = "windows", target_os = "linux"))]
mod gtk;

#[cfg(any(target_os = "mac_os", target_os = "windows", target_os = "linux"))]
pub use gtk;
