#[macro_export]
// import c function that writes to a buffer and returns the buffer value as string
macro_rules! get_item_from_c_buf {
    ($func:ident) => {{
        extern {
            fn $func(buffer: *mut u8, size: u64) -> u8;
        }

        let mut v: Vec<u8> = vec![0; 128];

        unsafe {
            $func(v.as_mut_ptr(), 128);
        }

        String::from_utf8(v).unwrap()
    }}
}
