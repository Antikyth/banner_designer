// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod banner;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
	fn alert(string: &str);
}

#[wasm_bindgen]
pub fn greet() {
	alert("Hello, this is your captain speaking.");
}
