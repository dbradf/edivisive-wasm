mod util;

use edivisive::EDivisive;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub fn change_points(series: Vec<f64>) -> Vec<usize> {
    let edivisive = EDivisive::default();
    edivisive.get_change_points(&series)
}
