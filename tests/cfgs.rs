#![cfg_attr(RUSTC_USE_FEATURE, feature(lint_reasons))]
#![cfg_attr(RUSTC_USE_FEATURE, feature(raw_ref_op))]

use pin_init::{pin_data, pin_init, PinInit};

#[pin_data]
pub struct Struct {
    #[cfg(kernel)]
    field_d: Field,
    #[cfg(not(kernel))]
    field_e: Field,
}

impl Struct {
    pub fn new() -> impl PinInit<Self> {
        pin_init!(Self {
            #[cfg(kernel)]
            field_d: Field {},
            #[cfg(not(kernel))]
            field_e: Field {},
        })
    }
}

struct Field {}
