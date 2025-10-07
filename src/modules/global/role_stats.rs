use std::{cell::RefCell, collections::HashMap};

use js_sys::{JsString, Object, Reflect};
use screeps::Creep;
use wasm_bindgen::JsCast;

use crate::roles::Role;

thread_local! {
    /// 不同房间的角色统计
    pub static ROLE_STATS: RefCell<HashMap<String, RoleStats>> = RefCell::new(HashMap::new());
}

/// 角色统计类型
pub type RoleStats = HashMap<Role, u32>;

/// 获取指定房间的角色统计
pub fn get_role_stats_by_room(room: &str) -> Option<RoleStats> {
    ROLE_STATS.with(|role_stats| role_stats.borrow().get(room).cloned())
}

/// 获取指定房间指定角色的统计
pub fn get_role_stat_by_room(room: &str, role: Role) -> u32 {
    ROLE_STATS.with(|role_stats| {
        let stats = role_stats.borrow();
        let room_stats = stats.get(room);
        if room_stats.is_none() {
            return 0;
        }
        let room_stats = room_stats.unwrap();
        let stat = room_stats.get(&role);
        if stat.is_none() {
            return 0;
        }
        return *stat.unwrap();
    })
}

/// 清空房间角色统计
pub fn clear_role_stats() {
    ROLE_STATS.with(|role_stats| {
        role_stats.borrow_mut().clear();
    })
}

/// 添加角色统计
pub fn add_role_stat(room: &str, role: Role) {
    ROLE_STATS.with(|role_stats| {
        let mut stats = role_stats.borrow_mut();
        let room_stats = stats.entry(room.to_string()).or_insert(RoleStats::new());
        let stat = room_stats.entry(role).or_insert(0);
        *stat += 1;
    })
}

/// 获取 creep 的角色
pub fn get_creep_role(creep: &Creep) -> Role {
    let creep_memory = creep.memory().dyn_into::<Object>().unwrap();
    Reflect::get(&creep_memory, &JsString::from("role"))
        .unwrap()
        .into()
}
