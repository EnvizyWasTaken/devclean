// utils.rs
pub fn format_size(bytes: u64) -> String {
    let gb = bytes as f64 / 1_073_741_824.0;
    let mb = bytes as f64 / 1_048_576.0;

    if gb >= 1.0 {
        format!("{:.2} GB", gb)
    } else {
        format!("{:.2} MB", mb)
    }
}