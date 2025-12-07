use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

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

        for (ix, entry) in text.lines().map(HiscoreEntry::from_str).enumerate() {
            let entry = entry?;

            let entry_kind = match HiscoreEntryKind::from_index(ix as u32) {
                Ok(kind) => kind,
                Err(err) => {
                    match err {
                        KindFromIndexError::Ignore => {}
                        KindFromIndexError::OutOfRange(ix) => {
                            return Err(ParseHiscoreError::IndexOutOfRange(ix));
                        }
                    }

                    continue;
                }
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

#[derive(Debug)]
pub enum KindFromIndexError {
    Ignore,
    OutOfRange(u32),
}

impl HiscoreEntryKind {
    fn from_index(index: u32) -> Result<HiscoreEntryKind, KindFromIndexError> {
        match index {
            0 => Ok(Self::Overall),
            1 => Ok(Self::Skill(Skill::Attack)),
            2 => Ok(Self::Skill(Skill::Defence)),
            3 => Ok(Self::Skill(Skill::Strength)),
            4 => Ok(Self::Skill(Skill::Hitpoints)),
            5 => Ok(Self::Skill(Skill::Ranged)),
            6 => Ok(Self::Skill(Skill::Prayer)),
            7 => Ok(Self::Skill(Skill::Magic)),
            8 => Ok(Self::Skill(Skill::Cooking)),
            9 => Ok(Self::Skill(Skill::Woodcutting)),
            10 => Ok(Self::Skill(Skill::Fletching)),
            11 => Ok(Self::Skill(Skill::Fishing)),
            12 => Ok(Self::Skill(Skill::Firemaking)),
            13 => Ok(Self::Skill(Skill::Crafting)),
            14 => Ok(Self::Skill(Skill::Smithing)),
            15 => Ok(Self::Skill(Skill::Mining)),
            16 => Ok(Self::Skill(Skill::Herblore)),
            17 => Ok(Self::Skill(Skill::Agility)),
            18 => Ok(Self::Skill(Skill::Thieving)),
            19 => Ok(Self::Skill(Skill::Slayer)),
            20 => Ok(Self::Skill(Skill::Farming)),
            21 => Ok(Self::Skill(Skill::Runecraft)),
            22 => Ok(Self::Skill(Skill::Hunter)),
            23 => Ok(Self::Skill(Skill::Construction)),
            24 => Ok(Self::Skill(Skill::Sailing)),
            25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 => Err(KindFromIndexError::Ignore),
            33 => Ok(Self::ClueScrollsAll),
            34 => Ok(Self::ClueScrolls(ClueTier::Beginner)),
            35 => Ok(Self::ClueScrolls(ClueTier::Easy)),
            36 => Ok(Self::ClueScrolls(ClueTier::Medium)),
            37 => Ok(Self::ClueScrolls(ClueTier::Hard)),
            38 => Ok(Self::ClueScrolls(ClueTier::Elite)),
            39 => Ok(Self::ClueScrolls(ClueTier::Master)),
            40 | 41 | 42 => Err(KindFromIndexError::Ignore),
            43 => Ok(Self::RiftsClosed),
            44 => Ok(Self::ColosseumGlory),
            45 => Ok(Self::CollectionsLogged),
            46 => Ok(Self::Boss(Boss::AbyssalSire)),
            47 => Ok(Self::Boss(Boss::AlchemicalHydra)),
            48 => Ok(Self::Boss(Boss::Amoxliatl)),
            49 => Ok(Self::Boss(Boss::Araxxor)),
            50 => Ok(Self::Boss(Boss::Artio)),
            51 => Ok(Self::Boss(Boss::BarrowsChests)),
            52 => Ok(Self::Boss(Boss::Byrophyta)),
            53 => Ok(Self::Boss(Boss::Callisto)),
            54 => Ok(Self::Boss(Boss::Calvarion)),
            55 => Ok(Self::Boss(Boss::Cerberus)),
            56 => Ok(Self::Boss(Boss::ChambersOfXeric)),
            57 => Ok(Self::Boss(Boss::ChambersOfXericChallengeMode)),
            58 => Ok(Self::Boss(Boss::ChaosElemental)),
            59 => Ok(Self::Boss(Boss::ChaosFanatic)),
            60 => Ok(Self::Boss(Boss::CommanderZilyana)),
            61 => Ok(Self::Boss(Boss::CorporealBeast)),
            62 => Ok(Self::Boss(Boss::CrazyArchaeologist)),
            63 => Ok(Self::Boss(Boss::DagannothPrime)),
            64 => Ok(Self::Boss(Boss::DagannothRex)),
            65 => Ok(Self::Boss(Boss::DagannothSupreme)),
            66 => Ok(Self::Boss(Boss::DerangedArchaeologist)),
            67 => Ok(Self::Boss(Boss::DoomOfMokhaiotl)),
            68 => Ok(Self::Boss(Boss::DukeSucellus)),
            69 => Ok(Self::Boss(Boss::GeneralGraardor)),
            70 => Ok(Self::Boss(Boss::GiantMole)),
            71 => Ok(Self::Boss(Boss::GrotesqueGuardians)),
            72 => Ok(Self::Boss(Boss::Hespori)),
            73 => Ok(Self::Boss(Boss::KalphiteQueen)),
            74 => Ok(Self::Boss(Boss::KingBlackDragon)),
            75 => Ok(Self::Boss(Boss::Kraken)),
            76 => Ok(Self::Boss(Boss::Kreearra)),
            77 => Ok(Self::Boss(Boss::KrilTsutsaroth)),
            78 => Ok(Self::Boss(Boss::LunarChests)),
            79 => Ok(Self::Boss(Boss::Mimic)),
            80 => Ok(Self::Boss(Boss::Nex)),
            81 => Ok(Self::Boss(Boss::Nightmare)),
            82 => Ok(Self::Boss(Boss::PhosanisNightmare)),
            83 => Ok(Self::Boss(Boss::Obor)),
            84 => Ok(Self::Boss(Boss::PhantomMuspah)),
            85 => Ok(Self::Boss(Boss::Sarachnis)),
            86 => Ok(Self::Boss(Boss::Scorpia)),
            87 => Ok(Self::Boss(Boss::Scurrius)),
            88 => Ok(Self::Boss(Boss::ShellbaneGryphon)),
            89 => Ok(Self::Boss(Boss::Skotizo)),
            90 => Ok(Self::Boss(Boss::SolHeredit)),
            91 => Ok(Self::Boss(Boss::Spindel)),
            92 => Ok(Self::Boss(Boss::Tempoross)),
            93 => Ok(Self::Boss(Boss::TheGauntlet)),
            94 => Ok(Self::Boss(Boss::TheCorruptedGauntlet)),
            95 => Ok(Self::Boss(Boss::TheHueycoatl)),
            96 => Ok(Self::Boss(Boss::TheLeviathan)),
            97 => Ok(Self::Boss(Boss::TheRoyalTitans)),
            98 => Ok(Self::Boss(Boss::TheWhisperer)),
            99 => Ok(Self::Boss(Boss::TheatreOfBlood)),
            100 => Ok(Self::Boss(Boss::TheatreOfBloodHardMode)),
            101 => Ok(Self::Boss(Boss::ThermonuclearSmokeDevil)),
            102 => Ok(Self::Boss(Boss::TombsOfAmascut)),
            103 => Ok(Self::Boss(Boss::TombsOfAmascutExpertMode)),
            104 => Ok(Self::Boss(Boss::TzKalZuk)),
            105 => Ok(Self::Boss(Boss::TzTokJad)),
            106 => Ok(Self::Boss(Boss::Vardorvis)),
            107 => Ok(Self::Boss(Boss::Venenatis)),
            108 => Ok(Self::Boss(Boss::Vetion)),
            109 => Ok(Self::Boss(Boss::Vorkath)),
            110 => Ok(Self::Boss(Boss::Wintertodt)),
            111 => Ok(Self::Boss(Boss::Yama)),
            112 => Ok(Self::Boss(Boss::Zalcano)),
            113 => Ok(Self::Boss(Boss::Zulrah)),
            index => Err(KindFromIndexError::OutOfRange(index)),
        }
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

#[derive(Debug, Clone, Copy)]
pub struct SkillHiscoreEntry {
    pub rank: i32,
    pub level: i32,
    pub xp: i32,
}

#[derive(Debug, Clone, Copy)]
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
