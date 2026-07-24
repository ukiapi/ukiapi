pub fn method_to_lowercase() {
    let method = "GET";
    let method_low = method.to_lowercase();
}

pub fn method_as_bytes_eq() {
    let method = "GET";
    let is_get = method.as_bytes().eq_ignore_ascii_case(b"get");
}

fn main() {
    method_to_lowercase();
    method_as_bytes_eq();
}
