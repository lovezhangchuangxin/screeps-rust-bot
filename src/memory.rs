use js_sys::{JSON, Object};
use screeps::raw_memory;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = Memory, thread_local_v2)]
    pub static MEMORY: Object;
}

/// 反序列化 memory
pub fn deserialize_memory() {
    let raw_memory = raw_memory::get().as_string().unwrap();
    let memory_object = JSON::parse(&raw_memory)
        .unwrap()
        .dyn_into::<Object>()
        .unwrap();
    MEMORY.with(|old_memory| {
        Object::assign(old_memory, &memory_object);
    })
}

/// 序列化 memory
pub fn serialize_memory() {
    MEMORY.with(|memory| {
        let memory_string = JSON::stringify(memory).unwrap();
        raw_memory::set(&memory_string);
    })
}
