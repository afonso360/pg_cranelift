use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn cranelift_add(left: c_int, right: c_int) -> c_int {
    println!("In Rust: left = {}, right = {}", left, right);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = cranelift_add(2, 2);
        assert_eq!(result, 4);
    }
}
