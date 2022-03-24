#[cfg(windows)]
extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.set_language(0x041F); // Turkish
    res.compile().unwrap();
}

#[cfg(not(target_os = "windows"))]
fn main() {
}