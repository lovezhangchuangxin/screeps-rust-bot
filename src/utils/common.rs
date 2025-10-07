//! 本模块包含常用的工具函数

use screeps::game;

/// 检查游戏时间是否达到该间隔，用于每隔一定时间执行一次的场景
pub fn is_interval(time: u32) -> bool {
    return game::time() % time == 0;
}
