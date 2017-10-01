#[macro_use]
extern crate lazy_static;
extern crate libc;

mod nes;
mod externs;

use std::cell::RefCell;
use std::ptr::null_mut;
use std::rc::Rc;
use nes::Nes;

fn main() {
}

#[no_mangle]
pub fn run(len: usize, ptr: *mut u8) {
    let buf: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(ptr, len) };
    let mut nes = Nes::new(buf);
    nes.reset();
    let mut main_loop = || {
        // externs::eval(&format!("console.log({:?});", a));
        // println!("{:?}", nes.cpu);
        nes.run();
        let js = ["const canvas = document.querySelector('canvas');",
                  "const ctx = canvas.getContext('2d');",
                  "const image = ctx.createImageData(256, 240);",
                  "for (let i = 0; i < 256 * 240; i += 1) {",
                  "const color = 0;",
                  "image.data[i * 4] = color;",
                  "image.data[i * 4 + 1] = color;",
                  "image.data[i * 4 + 2] = color;",
                  "image.data[i * 4 + 3] = 0xFF;",
                  "}",
                  "ctx.putImageData(image, 0, 0);"]
                .join("");
        externs::eval(&js);
    };
    externs::set_main_loop_callback(main_loop);
}
