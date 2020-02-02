use crate::{Level, Xp};

#[derive(Debug)]
pub struct CombatLevel(i32);

#[derive(Debug)]
pub struct Account {
    display_name: String,
    skills: Skills,
}

impl Account {
    pub fn combat_level(self) -> CombatLevel {
        let base_level = {
            let defence: Level = self.skills.defence.into();
            let hitpoints: Level = self.skills.hitpoints.into();
            let prayer: Level = self.skills.prayer.into();

            (defence.0 as f32 + hitpoints.0 as f32 + (prayer.0 as f32 / 2.0).floor()) / 4.0
        };

        let melee_level = {
            let attack: Level = self.skills.attack.into();
            let strength: Level = self.skills.strength.into();

            (attack.0 + strength.0) as f32 * 0.325
        };

        let range_level = {
            let ranged: Level = self.skills.ranged.into();

            (3.0 * ranged.0 as f32 / 2.0).floor() * 0.325
        };

        let mage_level = {
            let magic: Level = self.skills.magic.into();

            (3.0 * magic.0 as f32 / 2.0) * 0.325
        };

        let combat_level = base_level + melee_level.max(range_level.max(mage_level));

        CombatLevel(combat_level.floor() as i32)
    }
}

#[derive(Debug)]
pub struct Skills {
    pub attack: Xp,
    pub defence: Xp,
    pub strength: Xp,
    pub hitpoints: Xp,
    pub ranged: Xp,
    pub prayer: Xp,
    pub magic: Xp,
    pub cooking: Xp,
    pub woodcutting: Xp,
    pub fletching: Xp,
    pub fishing: Xp,
    pub firemaking: Xp,
    pub crafting: Xp,
    pub smithing: Xp,
    pub mining: Xp,
    pub herblore: Xp,
    pub agility: Xp,
    pub thieving: Xp,
    pub slayer: Xp,
    pub farming: Xp,
    pub runecraft: Xp,
    pub hunter: Xp,
    pub construction: Xp,
}
