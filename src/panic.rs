// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of various bits and pieces of the `panic!` macro and
//! associated runtime pieces.

use core::fmt::{self, Display, Write};
use core::any::Any;

use collections::String;
use collections::boxed::Box;

// The compiler wants this to be here. Otherwise it won't be happy. And we like happy compilers.
#[lang = "eh_personality"]
extern fn eh_personality() {}

// It also wants these functions because we're not linking libgcc anymore
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() {}
#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr1() {}

/// Entry point of panic from the libcore crate.
#[lang = "panic_fmt"]
extern fn panic_fmt(msg: fmt::Arguments, file: &'static str, line: u32) -> ! {
    begin_panic_fmt(&msg, &(file, line))
}

/// The entry point for panicking with a formatted message.
///
/// This is designed to reduce the amount of code required at the call
/// site as much as possible (so that `panic!()` has as low an impact
/// on (e.g.) the inlining of other functions as possible), by moving
/// the actual formatting into this shared place.
#[inline(never)]
#[cold]
pub fn begin_panic_fmt(msg: &fmt::Arguments, file_line: &(&'static str, u32)) -> ! {
    let mut s = String::new();
    let _ = s.write_fmt(*msg);
    begin_panic(s, file_line);
}

/// This is where the main panic logic happens.
#[inline(never)]
#[cold]
pub fn begin_panic<M: Any + Send + Display>(msg: M, file_line: &(&'static str, u32)) -> ! {
    use gfx::Screen;
    use console::Console;

    let msg = Box::new(msg);
    let (file, line) = *file_line;

    let mut error_top = Console::init(Screen::Top);
    let mut error_bottom = Console::init(Screen::Bottom);

    write!(error_top, "--------------------------------------------------").unwrap();
    writeln!(error_top, "PANIC in {} at line {}:", file, line).unwrap();
    writeln!(error_top, "    {}", msg).unwrap();
    write!(error_top, "\x1b[29;00H--------------------------------------------------").unwrap();

    write!(error_bottom, "").unwrap();     

    loop {}
}
