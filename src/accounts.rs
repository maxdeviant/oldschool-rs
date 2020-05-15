use crate::diaries::DiaryId;
use crate::quests::QuestId;
use crate::skills::Skill;
use crate::{Level, Xp};

#[derive(Debug, Clone)]
pub struct CombatLevel(i32);

#[derive(Debug, Clone)]
pub struct Account {
    pub display_name: String,
    pub skills: Skills,
    pub completed_quests: Vec<QuestId>,
    pub completed_diaries: Vec<DiaryId>,
}

impl Account {
    pub fn get_level(&self, skill: &Skill) -> Level {
        let xp = match skill {
            Skill::Attack => self.skills.attack,
            Skill::Defence => self.skills.defence,
            Skill::Strength => self.skills.strength,
            Skill::Hitpoints => self.skills.hitpoints,
            Skill::Ranged => self.skills.ranged,
            Skill::Prayer => self.skills.prayer,
            Skill::Magic => self.skills.magic,
            Skill::Cooking => self.skills.cooking,
            Skill::Woodcutting => self.skills.woodcutting,
            Skill::Fletching => self.skills.fletching,
            Skill::Fishing => self.skills.fishing,
            Skill::Firemaking => self.skills.firemaking,
            Skill::Crafting => self.skills.crafting,
            Skill::Smithing => self.skills.smithing,
            Skill::Mining => self.skills.mining,
            Skill::Herblore => self.skills.herblore,
            Skill::Agility => self.skills.agility,
            Skill::Thieving => self.skills.thieving,
            Skill::Slayer => self.skills.slayer,
            Skill::Farming => self.skills.farming,
            Skill::Runecraft => self.skills.runecraft,
            Skill::Hunter => self.skills.hunter,
            Skill::Construction => self.skills.construction,
        };

        xp.into()
    }

    pub fn total_level(&self) -> Level {
        self.skills.total_level()
    }

    pub fn combat_level(&self) -> CombatLevel {
        self.skills.combat_level()
    }
}

#[derive(Debug, Clone)]
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

impl Skills {
    pub fn total_level(&self) -> Level {
        vec![
            self.attack,
            self.defence,
            self.strength,
            self.hitpoints,
            self.ranged,
            self.prayer,
            self.magic,
            self.cooking,
            self.woodcutting,
            self.fletching,
            self.fishing,
            self.firemaking,
            self.crafting,
            self.smithing,
            self.mining,
            self.herblore,
            self.agility,
            self.thieving,
            self.slayer,
            self.farming,
            self.runecraft,
            self.hunter,
            self.construction,
        ]
        .into_iter()
        .map(Level::from)
        .sum()
    }

    pub fn combat_level(&self) -> CombatLevel {
        let base_level = {
            let defence: Level = self.defence.into();
            let hitpoints: Level = self.hitpoints.into();
            let prayer: Level = self.prayer.into();

            (defence.0 as f32 + hitpoints.0 as f32 + (prayer.0 as f32 / 2.0).floor()) / 4.0
        };

        let melee_level = {
            let attack: Level = self.attack.into();
            let strength: Level = self.strength.into();

            (attack.0 + strength.0) as f32 * 0.325
        };

        let range_level = {
            let ranged: Level = self.ranged.into();

            (3.0 * ranged.0 as f32 / 2.0).floor() * 0.325
        };

        let mage_level = {
            let magic: Level = self.magic.into();

            (3.0 * magic.0 as f32 / 2.0) * 0.325
        };

        let combat_level = base_level + melee_level.max(range_level.max(mage_level));

        CombatLevel(combat_level.floor() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::Skills;
    use crate::Level;

    #[test]
    fn test_new_account_total_level() {
        let new_account_skills = Skills {
            attack: Level(1).into(),
            defence: Level(1).into(),
            strength: Level(1).into(),
            hitpoints: Level(10).into(),
            ranged: Level(1).into(),
            prayer: Level(1).into(),
            magic: Level(1).into(),
            cooking: Level(1).into(),
            woodcutting: Level(1).into(),
            fletching: Level(1).into(),
            fishing: Level(1).into(),
            firemaking: Level(1).into(),
            crafting: Level(1).into(),
            smithing: Level(1).into(),
            mining: Level(1).into(),
            herblore: Level(1).into(),
            agility: Level(1).into(),
            thieving: Level(1).into(),
            slayer: Level(1).into(),
            farming: Level(1).into(),
            runecraft: Level(1).into(),
            hunter: Level(1).into(),
            construction: Level(1).into(),
        };

        assert_eq!(new_account_skills.total_level(), Level(32));
    }

    #[test]
    fn test_maxed_total_level() {
        let maxed_skills = Skills {
            attack: Level(99).into(),
            defence: Level(99).into(),
            strength: Level(99).into(),
            hitpoints: Level(99).into(),
            ranged: Level(99).into(),
            prayer: Level(99).into(),
            magic: Level(99).into(),
            cooking: Level(99).into(),
            woodcutting: Level(99).into(),
            fletching: Level(99).into(),
            fishing: Level(99).into(),
            firemaking: Level(99).into(),
            crafting: Level(99).into(),
            smithing: Level(99).into(),
            mining: Level(99).into(),
            herblore: Level(99).into(),
            agility: Level(99).into(),
            thieving: Level(99).into(),
            slayer: Level(99).into(),
            farming: Level(99).into(),
            runecraft: Level(99).into(),
            hunter: Level(99).into(),
            construction: Level(99).into(),
        };

        assert_eq!(maxed_skills.total_level(), Level(2277));
    }
}
