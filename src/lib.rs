use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
};

use js_sys::JsString;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::core::{Ayaka, AyakaOptions, LifeCycle};

pub mod core;
pub mod memory;
pub mod modules;
pub mod roles;
pub mod utils;

thread_local! {
    static INIT: Cell<bool> = Cell::new(false);
    static AYAKA: RefCell<Ayaka> = RefCell::new(Ayaka::new(AyakaOptions::default()));
}

#[wasm_bindgen(js_name = "loop")]
pub fn game_loop() {
    memory::deserialize_memory();

    INIT.with(|init| {
        if !init.get() {
            // 注册生命周期函数
            AYAKA.with(|ayaka| {
                ayaka.borrow_mut().on(HashMap::from([(
                    LifeCycle::Mounted,
                    Box::new(|| {
                        console::log_1(&JsString::from("Ayaka 挂载中"));
                    }) as Box<dyn Fn()>,
                )]));

                ayaka
                    .borrow_mut()
                    .on(modules::global::get_global_lifecycles());
            });

            console::log_1(&JsString::from("Ayaka 启动成功"));
            init.set(true);
        } else {
            AYAKA.with(|ayaka| {
                ayaka.borrow_mut().run();
            });
        }
    });

    memory::serialize_memory();
}
