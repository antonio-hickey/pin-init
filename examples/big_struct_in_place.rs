// SPDX-License-Identifier: Apache-2.0 OR MIT

#![cfg_attr(not(RUSTC_RAW_REF_OP_IS_STABLE), feature(raw_ref_op))]

use pin_init::*;

// Struct with size over 1GiB
#[derive(Debug)]
#[allow(dead_code)]
pub struct BigStruct {
    buf: [u8; 1024 * 1024 * 1024],
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    managed_buf: ManagedBuf,
}

#[derive(Debug)]
pub struct ManagedBuf {
    buf: [u8; 1024 * 1024],
}

impl ManagedBuf {
    pub fn new() -> impl Init<Self> {
        init!(ManagedBuf { buf <- init_zeroed() })
    }
}

fn main() {
    #[cfg(any(feature = "std", feature = "alloc"))]
    {
        // we want to initialize the struct in-place, otherwise we would get a stackoverflow
        let buf: Box<BigStruct> = Box::init(init!(BigStruct {
            buf <- init_zeroed(),
            a: 7,
            b: 186,
            c: 7789,
            d: 34,
            managed_buf <- ManagedBuf::new(),
        }))
        .unwrap();
        println!("{}", core::mem::size_of_val(&*buf));
    }
}
