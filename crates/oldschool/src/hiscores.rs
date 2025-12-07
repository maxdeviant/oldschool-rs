use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

use strum::IntoEnumIterator;
use thiserror::Error;
use url::Url;

use crate::{Boss, ClueTier, Skill};

#[derive(Clone)]
pub struct Hiscore {
    entries: HashMap<HiscoreEntryKind, HiscoreEntry>,
}

impl Hiscore {
    /// Returns the hiscores URL for the player with the given name and account type.
    pub fn url(name: &str, account_type: AccountType) -> Url {
        let url = format!(
            "https://secure.runescape.com/m=hiscore_oldschool{}/index_lite.ws",
            account_type.hiscores_suffix()
        );
        let mut url = Url::parse(&url).unwrap();
        url.query_pairs_mut().append_pair("player", name);

        url
    }

    pub fn overall(&self) -> Option<&SkillHiscoreEntry> {
        self.entries.get(&HiscoreEntryKind::Overall)?.as_skill()
    }

    pub fn skill(&self, skill: Skill) -> Option<&SkillHiscoreEntry> {
        self.entries
            .get(&HiscoreEntryKind::Skill(skill))?
            .as_skill()
    }

    pub fn clue_scrolls(&self, tier: ClueTier) -> Option<&ScalarHiscoreEntry> {
        self.entries
            .get(&HiscoreEntryKind::ClueScrolls(tier))?
            .as_scalar()
    }

    pub fn rifts_closed(&self) -> Option<&ScalarHiscoreEntry> {
        self.entries
            .get(&HiscoreEntryKind::RiftsClosed)?
            .as_scalar()
    }

    pub fn collections_logged(&self) -> Option<&ScalarHiscoreEntry> {
        self.entries
            .get(&HiscoreEntryKind::CollectionsLogged)?
            .as_scalar()
    }

    pub fn boss(&self, boss: Boss) -> Option<&ScalarHiscoreEntry> {
        self.entries.get(&HiscoreEntryKind::Boss(boss))?.as_scalar()
    }
}

#[derive(Error, Debug)]
pub enum ParseHiscoreError {
    #[error(transparent)]
    InvalidEntry(#[from] ParseHiscoreEntryError),
    #[error("index out of range: {0}")]
    IndexOutOfRange(u32),
}

impl FromStr for Hiscore {
    type Err = ParseHiscoreError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut hiscore = Self {
            entries: HashMap::new(),
        };

        let entries = text
            .lines()
            .map(HiscoreEntry::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        for (entry_kind, entry) in HiscoreEntryKind::order().zip(entries) {
            let entry_kind = match entry_kind {
                Ok(kind) => kind,
                Err(IgnoreEntry) => continue,
            };

            hiscore.entries.insert(entry_kind, entry);
        }

        Ok(hiscore)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HiscoreEntryKind {
    Overall,
    Skill(Skill),
    ClueScrollsAll,
    ClueScrolls(ClueTier),
    RiftsClosed,
    ColosseumGlory,
    CollectionsLogged,
    Boss(Boss),
}

#[derive(Debug, Clone)]
struct IgnoreEntry;

impl HiscoreEntryKind {
    fn order() -> impl Iterator<Item = Result<HiscoreEntryKind, IgnoreEntry>> {
        fn one(kind: HiscoreEntryKind) -> std::iter::Once<Result<HiscoreEntryKind, IgnoreEntry>> {
            std::iter::once(Ok(kind))
        }

        fn many(
            kinds: impl IntoIterator<Item = HiscoreEntryKind>,
        ) -> impl Iterator<Item = Result<HiscoreEntryKind, IgnoreEntry>> {
            kinds.into_iter().map(Ok)
        }

        fn ignore(n: usize) -> std::iter::RepeatN<Result<HiscoreEntryKind, IgnoreEntry>> {
            std::iter::repeat_n(Err(IgnoreEntry), n)
        }

        one(Self::Overall)
            .chain(Skill::iter().map(|skill| Ok(Self::Skill(skill))))
            .chain(ignore(7))
            .chain(one(Self::ClueScrollsAll))
            .chain(ClueTier::iter().map(|tier| Ok(Self::ClueScrolls(tier))))
            .chain(ignore(3))
            .chain(many([
                Self::RiftsClosed,
                Self::ColosseumGlory,
                Self::CollectionsLogged,
            ]))
            .chain(Boss::iter().map(|boss| Ok(Self::Boss(boss))))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HiscoreEntry {
    Skill(SkillHiscoreEntry),
    Scalar(ScalarHiscoreEntry),
}

impl HiscoreEntry {
    pub fn as_skill(&self) -> Option<&SkillHiscoreEntry> {
        match self {
            Self::Skill(entry) => Some(entry),
            _ => None,
        }
    }

    pub fn as_scalar(&self) -> Option<&ScalarHiscoreEntry> {
        match self {
            Self::Scalar(entry) => Some(entry),
            _ => None,
        }
    }
}

#[derive(Error, Debug)]
pub enum ParseHiscoreEntryError {
    #[error("invalid hiscore entry: {text:?}")]
    InvalidEntry { text: String },
    #[error("failed to parse hiscore value: {0}")]
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for HiscoreEntry {
    type Err = ParseHiscoreEntryError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts = text.split(',').collect::<Vec<_>>();

        match parts.as_slice() {
            [rank, level, xp] => {
                let rank = rank.parse()?;
                let level = level.parse()?;
                let xp = xp.parse()?;

                Ok(Self::Skill(SkillHiscoreEntry { rank, level, xp }))
            }
            [rank, quantity] => {
                let rank = rank.parse()?;
                let amount = quantity.parse()?;

                Ok(Self::Scalar(ScalarHiscoreEntry { rank, amount }))
            }
            _ => Err(ParseHiscoreEntryError::InvalidEntry {
                text: text.to_string(),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SkillHiscoreEntry {
    pub rank: i32,
    pub level: i32,
    pub xp: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ScalarHiscoreEntry {
    pub rank: i32,
    pub amount: i32,
}

#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, strum::Display, strum::EnumString,
)]
#[strum(serialize_all = "snake_case")]
pub enum AccountType {
    Main,
    Ironman,
    UltimateIronman,
    HardcoreIronman,
    Leagues,
}

impl AccountType {
    fn hiscores_suffix(&self) -> &'static str {
        match self {
            Self::Main => "",
            Self::Ironman => "_ironman",
            Self::UltimateIronman => "_ultimate",
            Self::HardcoreIronman => "_hardcore",
            Self::Leagues => "_seasonal",
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_hiscore_parsing() {
        let hiscore = Hiscore::from_str(include_str!("../fixtures/hiscores_1.txt")).unwrap();

        assert_eq!(
            hiscore.overall(),
            Some(&SkillHiscoreEntry {
                rank: 5_154,
                level: 2376,
                xp: 424_043_990,
            })
        );

        assert_eq!(
            hiscore.skill(Skill::Sailing),
            Some(&SkillHiscoreEntry {
                rank: 768,
                level: 99,
                xp: 15_437_526,
            })
        );

        assert_eq!(
            hiscore.clue_scrolls(ClueTier::Hard),
            Some(&ScalarHiscoreEntry {
                rank: 447_040,
                amount: 50,
            })
        );

        assert_eq!(
            hiscore.rifts_closed(),
            Some(&ScalarHiscoreEntry {
                rank: 177_649,
                amount: 220
            })
        );

        assert_eq!(
            hiscore.collections_logged(),
            Some(&ScalarHiscoreEntry {
                rank: -1,
                amount: 388,
            })
        );

        assert_eq!(
            hiscore.boss(Boss::Zulrah),
            Some(&ScalarHiscoreEntry {
                rank: 430_027,
                amount: 57,
            })
        );
    }
}
