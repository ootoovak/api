// Copyright 2015-2016 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

#[cfg(all(feature = "local-run", feature = "remote-run"))]
fn main() {
    panic!("Mutually exclusive features `local-run` and `remote-run`. You must only enable one.");
}

#[cfg(all(not(feature = "local-run"), not(feature = "remote-run")))]
fn main() {
    panic!("Missing feature `local-run` or `remote-run`. You must enable one.");
}

#[cfg(all(feature = "local-run", not(feature = "remote-run")))]
fn main() {
}

#[cfg(all(feature = "remote-run", not(feature = "local-run")))]
fn main() {
}
