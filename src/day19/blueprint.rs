#[derive(Debug, Clone)]
pub struct LoopInfo {
    time: i32,
    robot_list: Vec<Robot>,
    resource_list: Vec<(i32, CostType)>,
}

#[derive(Debug)]
pub struct Blueprint {
    pub id: i32,
    pub robots: Vec<Robot>,
}

impl Blueprint {
    pub fn new(id: i32, robots: Vec<Robot>) -> Self {
        Blueprint { id, robots }
    }
}

#[derive(Debug, Clone)]
pub enum RobotType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl RobotType {
    pub fn form_str(s: &str) -> Self {
        match s {
            "ore" => RobotType::Ore,
            "clay" => RobotType::Clay,
            "obsidian" => RobotType::Obsidian,
            "geode" => RobotType::Geode,
            _ => panic!("invalid cost type"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub type_name: RobotType,
    pub cost: Vec<(i32, CostType)>,
}

impl Robot {
    pub fn new(name: &str, cost: Vec<(i32, CostType)>) -> Self {
        let type_name = RobotType::form_str(name);
        Robot { type_name, cost }
    }
}

#[derive(Debug, Clone)]
pub enum CostType {
    Ore,
    Clay,
    Obsidian,
}

impl CostType {
    pub fn form_str(s: &str) -> Self {
        match s {
            "ore" => CostType::Ore,
            "clay" => CostType::Clay,
            "obsidian" => CostType::Obsidian,
            _ => panic!("invalid cost type"),
        }
    }
}
