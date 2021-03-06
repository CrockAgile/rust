// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//compile-flags: -Z emit-end-regions -Zborrowck=mir -Z nll

#![allow(warnings)]

struct Wrap<'p> { p: &'p mut i32 }

impl<'p> Drop for Wrap<'p> {
    fn drop(&mut self) {
        *self.p += 1;
    }
}

fn main() {
    let mut x = 0;
    let wrap = Wrap { p: &mut x };
    std::mem::drop(wrap);
    x = 1; // OK, drop is inert
}
