//! Ayaka 框架，提供生命周期的管理

use std::collections::HashMap;

use screeps::{AccountPowerCreep, Creep, Room, game};

/// 生命周期函数字典，供其他模块使用暴露出对应的生命周期函数
pub type LifeCycles = HashMap<LifeCycle, Box<dyn Fn() -> ()>>;
/// 生命周期缓存，包含所有模块注册的生命周期函数
pub type LifeCyclesCache = HashMap<LifeCycle, Vec<Box<dyn Fn() -> ()>>>;

/// Ayaka 框架类型
pub struct Ayaka {
    /// 是否已经挂载
    pub is_mounted: bool,
    /// 生命周期函数缓存
    pub lifecycle: LifeCyclesCache,
    /// 框架配置对象
    pub options: AyakaOptions,
}

impl Ayaka {
    pub fn new(options: AyakaOptions) -> Self {
        Self {
            options,
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        // 执行生命周期
        // 先执行 Mounted 生命周期
        if !self.is_mounted {
            if self.options.min_cpu <= (game::cpu::bucket() as u32) {
                self.call(LifeCycle::Mounted);
                self.is_mounted = true;
            } else {
                return;
            }
        }

        // 执行 TickStart 生命周期
        self.call(LifeCycle::TickStart);

        // 执行各个运行器
        if let Some(global_runner) = &self.options.global_runner {
            global_runner();
        }
        if let Some(room_runner) = &self.options.room_runner {
            game::rooms().values().for_each(|room| room_runner(&room));
        }
        if let Some(creep_runner) = &self.options.creep_runner {
            game::creeps()
                .values()
                .for_each(|creep| creep_runner(&creep));
        }
        if let Some(power_creep_runner) = &self.options.power_creep_runner {
            game::power_creeps()
                .values()
                .for_each(|power_creep| power_creep_runner(&power_creep));
        }

        // 执行 TickEnd 生命周期
        self.call(LifeCycle::TickEnd);
    }

    /// 注册生命周期
    pub fn on(&mut self, lifecycles: LifeCycles) {
        for (key, func) in lifecycles {
            self.lifecycle.entry(key).or_default().push(func);
        }
    }

    /// 执行指定阶段的生命周期函数
    fn call(&self, lifecycle: LifeCycle) {
        if let Some(funcs) = self.lifecycle.get(&lifecycle) {
            funcs.iter().for_each(|f| f());
        }
    }
}

impl Default for Ayaka {
    fn default() -> Self {
        Self {
            is_mounted: false,
            lifecycle: HashMap::new(),
            options: AyakaOptions::default(),
        }
    }
}

/// Ayaka 框架配置对象
pub struct AyakaOptions {
    /// 挂载时需要的最低 cpu
    min_cpu: u32,
    /// 全局运行器
    global_runner: Option<Box<dyn Fn() -> ()>>,
    /// 房间运行器
    room_runner: Option<Box<dyn Fn(&Room) -> ()>>,
    /// creep 运行器
    creep_runner: Option<Box<dyn Fn(&Creep) -> ()>>,
    /// power_creep 运行器
    power_creep_runner: Option<Box<dyn Fn(&AccountPowerCreep) -> ()>>,
}

impl Default for AyakaOptions {
    fn default() -> Self {
        Self {
            // 默认最少 50 cpu 才执行挂载
            min_cpu: 50,
            global_runner: None,
            room_runner: None,
            creep_runner: None,
            power_creep_runner: None,
        }
    }
}

/// 生命周期类型
#[derive(Hash, Eq, PartialEq)]
pub enum LifeCycle {
    /// 初始化
    Mounted,
    /// 运行
    TickStart,
    /// 停止
    TickEnd,
}
