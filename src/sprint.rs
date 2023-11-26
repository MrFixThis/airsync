macro_rules! sprint {
    ($serial:expr, $($arg:tt)*) => {{
        ::ufmt::uwrite!(&mut $serial, $($arg)*).void_unwrap()
    }};
}

macro_rules! sprintln {
    ($serial:expr, $($arg:tt)*) => {{
        ::ufmt::uwriteln!(&mut $serial, $($arg)*).void_unwrap()
    }};
}
