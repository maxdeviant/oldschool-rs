use indexmap::IndexSet;
use smol_str::SmolStr;

use crate::{Level, Skill, Xp};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct QuestId(pub(crate) SmolStr);

impl QuestId {
    pub fn new(name: &str) -> Self {
        Self(SmolStr::new(name))
    }

    pub const fn new_static(name: &'static str) -> Self {
        Self(SmolStr::new_static(name))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug)]
pub struct Quest {
    pub id: QuestId,
    pub name: String,
    pub quest_points: u8,
    pub requirements: IndexSet<QuestRequirement>,
    pub rewards: Vec<QuestReward>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum QuestRequirement {
    Skill { skill: Skill, level: Level },
    Quest(QuestId),
    QuestPoints(u16),
}

#[derive(Debug)]
pub enum QuestReward {
    Xp { skill: Skill, xp: Xp },
}
