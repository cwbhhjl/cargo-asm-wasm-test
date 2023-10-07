use wasm_bindgen::prelude::*;

mod zkwasm {
    extern "C" {
        pub fn wasm_input(is_public: u32) -> u64;
        pub fn wasm_output(v: u64);
    }

    pub fn private_input() -> u64 {
        unsafe {
            wasm_input(0)
        }
    }

    pub fn output(v: u64) {
        unsafe {
            wasm_output(v)
        }
    }
}

#[wasm_bindgen]
#[inline(never)]
pub fn sum(i: usize) -> usize {
    let mut s = 0;
    for n in 0..i {
        s += n + s;
    }

    s
}

#[wasm_bindgen]
pub fn zkmain() {
    let i = zkwasm::private_input() as usize;

    let s = sum(i);

    zkwasm::output(s as u64)
}
