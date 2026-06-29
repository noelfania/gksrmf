pub const APP_NAME: &str = "key2gksrmf";
pub const APP_MUTEX_NAME: &str = "Local\\key2gksrmf_single_instance";
pub const APP_WINDOW_CLASS: &str = "key2gksrmf_wnd";
pub const APP_OWNER_WINDOW_CLASS: &str = "key2gksrmf_owner_wnd";

#[cfg(target_arch = "x86_64")]
const BITNESS_LABEL: &str = "64bit";
#[cfg(target_arch = "x86")]
const BITNESS_LABEL: &str = "32bit";
#[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
const BITNESS_LABEL: &str = "unknown";

pub fn window_title() -> String {
    format!("{APP_NAME} {BITNESS_LABEL} {}", env!("CARGO_PKG_VERSION"))
}

pub fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}
