//! 本模块提供清除过期 memory 的方法

use std::str::FromStr;

use js_sys::{JsString, Object, Reflect};
use screeps::{RoomName, game};
use wasm_bindgen::JsCast;
use web_sys::console;

use crate::{memory::MEMORY, utils};

/// 清除过期 memory
pub fn clear_memory() {
    clear_creep_memory();
    clear_room_memory();
}

/// 清除过期的 creep memory
pub fn clear_creep_memory() {
    MEMORY.with(|memory| {
        if let Ok(creep_memorys) = Reflect::get(&memory, &JsString::from("creeps")) {
            console::log_1(&JsString::from("清理 creep memory"));
            let creep_memorys: Object = creep_memorys.unchecked_into();
            Object::keys(&creep_memorys).for_each(&mut |key, _, __| {
                let creep_name = String::from(key.dyn_ref::<JsString>().unwrap());
                let creep = game::creeps().get(creep_name.clone());
                if creep.is_none() {
                    console::log_1(&JsString::from(format!(
                        "清理 creep memory: {}",
                        &creep_name
                    )));
                    // 删除过期 creep memory
                    Reflect::delete_property(&creep_memorys, &key).unwrap();
                }
            });
        } else {
            // 创建 creep memorys
            Reflect::set(&memory, &JsString::from("creeps"), &Object::new()).unwrap();
        }
    })
}

/// 清除过期的房间记忆
pub fn clear_room_memory() {
    if !utils::is_interval(531) {
        return;
    }

    MEMORY.with(|memory| {
        if let Ok(room_memorys) = Reflect::get(memory, &JsString::from("rooms")) {
            let room_memorys: Object = room_memorys.unchecked_into();
            Object::keys(&room_memorys).for_each(&mut |key, _, __| {
                let room_name = String::from(key.dyn_ref::<JsString>().unwrap());
                if game::rooms()
                    .get(RoomName::from_str(&room_name).unwrap())
                    .is_none()
                {
                    // 删除过期 room memory
                    Reflect::delete_property(&room_memorys, &key).unwrap();
                }
            })
        }
    })
}
