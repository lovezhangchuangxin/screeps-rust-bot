use wasm_bindgen::JsValue;

/// 角色枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// 矿工
    Harvester = 1,
    /// 建造者
    Builder = 2,
    /// 升级者
    Upgrader = 3,
}

impl From<JsValue> for Role {
    fn from(value: JsValue) -> Self {
        match value.as_string().unwrap().as_str() {
            "harvester" => Role::Harvester,
            "builder" => Role::Builder,
            "upgrader" => Role::Upgrader,
            _ => panic!("Invalid role"),
        }
    }
}

impl Role {
    /// 获取角色名称
    pub fn name(&self) -> &'static str {
        match self {
            Role::Harvester => "harvester",
            Role::Builder => "builder",
            Role::Upgrader => "upgrader",
        }
    }
}
