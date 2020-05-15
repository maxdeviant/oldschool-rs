use crate::accounts::Account;
use crate::diaries::*;
use crate::quests::{self, QuestId};
use crate::skills::Skill;
use crate::Level;

pub struct OptimalQuestGuide<'a> {
    account: &'a Account,
}

impl<'a> OptimalQuestGuide<'a> {
    pub fn new(account: &'a Account) -> Self {
        Self { account }
    }

    pub fn next_step(&self) -> Option<OptimalQuestGuideStep> {
        optimal_quest_guide()
            .into_iter()
            .skip_while(|step| match step {
                OptimalQuestGuideStep::Quest(quest_id) => {
                    self.account.completed_quests.contains(&quest_id)
                }
                OptimalQuestGuideStep::Diary(diary_id) => {
                    self.account.completed_diaries.contains(&diary_id)
                }
                OptimalQuestGuideStep::Train { skill, to, .. } => {
                    &self.account.get_level(skill) >= to
                }
            })
            .next()
    }
}

#[derive(Debug)]
pub enum OptimalQuestGuideStep {
    Quest(QuestId),
    Diary(DiaryId),
    Train {
        skill: Skill,
        from: Level,
        to: Level,
    },
}

pub fn optimal_quest_guide() -> Vec<OptimalQuestGuideStep> {
    use quests::*;
    use OptimalQuestGuideStep::*;
    use Skill::*;
    vec![
        Quest(COOKS_ASSISTANT),
        Quest(X_MARKS_THE_SPOT),
        Quest(THE_RESTLESS_GHOST),
        Quest(RUNE_MYSTERIES),
        Quest(IMP_CATCHER),
        Quest(WITCHS_POTION),
        Quest(CLIENT_OF_KOUREND),
        Quest(DWARF_CANNON),
        Quest(WATERFALL_QUEST),
        Quest(TREE_GNOME_VILLAGE),
        Quest(MONKS_FRIEND),
        Quest(HAZEEL_CULT),
        Quest(MURDER_MYSTERY),
        Quest(MERLINS_CRYSTAL),
        Quest(HOLY_GRAIL),
        Quest(DRUIDIC_RITUAL),
        Quest(WITCHS_HOUSE),
        Quest(BLACK_KNIGHTS_FORTRESS),
        Quest(RECRUITMENT_DRIVE),
        Quest(OBSERVATORY_QUEST),
        Quest(PRIEST_IN_PERIL),
        Quest(RAG_AND_BONE_MAN),
        Quest(NATURE_SPIRIT),
        Quest(ALFRED_GRIMHANDS_BARCRAWL),
        Quest(SCORPION_CATCHER),
        Quest(PLAGUE_CITY),
        Quest(BIOHAZARD),
        Quest(FIGHT_ARENA),
        Quest(GERTRUDES_CAT),
        // TODO: Natural History Quiz
        Quest(JUNGLE_POTION),
        Quest(VAMPIRE_SLAYER),
        Quest(DEATH_PLATEAU),
        Quest(GOBLIN_DIPLOMACY),
        Quest(THE_QUEEN_OF_THIEVES),
        Train {
            skill: Agility,
            from: Level(14),
            to: Level(18),
        },
        Quest(THE_DEPTHS_OF_DESPAIR),
        Quest(MOUNTAIN_DAUGHTER),
        Quest(ICTHLARINS_LITTLE_HELPER),
        Quest(THE_GRAND_TREE),
        Quest(TRIBAL_TOTEM),
        Quest(THE_DIG_SITE),
        Quest(THE_GOLEM),
        Quest(THE_KNIGHTS_SWORD),
        Quest(ELEMENTAL_WORKSHOP_I),
        // TODO: Recipe for Disaster: Another Cook's Quest
        // TODO: Recipe for Disaster: Goblin Generals
        Quest(DEMON_SLAYER),
        Quest(SHADOW_OF_THE_STORM),
        Quest(ELEMENTAL_WORKSHOP_II),
        Train {
            skill: Woodcutting,
            from: Level(22),
            to: Level(36),
        },
        Quest(LOST_CITY),
        Quest(FAIRY_TALE_I_GROWING_PAINS),
        Quest(SHIELD_OF_ARRAV),
        Quest(CREATURE_OF_FENKENSTRAIN),
        Quest(A_SOULS_BANE),
        Quest(THE_LOST_TRIBE),
        Quest(DEATH_TO_THE_DORGESHUUN),
        Train {
            skill: Firemaking,
            from: Level(1),
            to: Level(16),
        },
        Train {
            skill: Magic,
            from: Level(30),
            to: Level(33),
        },
        Quest(THE_GIANT_DWARF),
        Quest(ANOTHER_SLICE_OF_HAM),
        Quest(MAKING_HISTORY),
        Quest(IN_SEARCH_OF_THE_MYREQUE),
        Quest(SHADES_OF_MORTTON),
        Quest(IN_AID_OF_THE_MYREQUE),
        // TODO: Acquire 100 kudos.
        Quest(BONE_VOYAGE),
        Quest(ENTER_THE_ABYSS),
        Quest(WANTED),
        Quest(THE_FEUD),
        Quest(TROLL_STRONGHOLD),
        Quest(TROLL_ROMANCE),
        Quest(DRAGON_SLAYER),
        Quest(HORROR_FROM_THE_DEEP),
        Train {
            skill: Ranged,
            from: Level(23),
            to: Level(30),
        },
        Quest(ERNEST_THE_CHICKEN),
        Quest(ANIMAL_MAGNETISM),
        Quest(SHILO_VILLAGE),
        Quest(DORICS_QUEST),
        Train {
            skill: Ranged,
            from: Level(30),
            to: Level(37),
        },
        Quest(SPIRITS_OF_THE_ELID),
        Train {
            skill: Construction,
            from: Level(1),
            to: Level(5),
        },
        Quest(DARKNESS_OF_HALLOWVALE),
        Quest(TOWER_OF_LIFE),
        Train {
            skill: Fishing,
            from: Level(1),
            to: Level(10),
        },
        Quest(FISHING_CONTEST),
        // TODO: Recipe for Disaster: Dwarf
        Train {
            skill: Cooking,
            from: Level(19),
            to: Level(20),
        },
        Quest(GHOSTS_AHOY),
        Train {
            skill: Cooking,
            from: Level(20),
            to: Level(22),
        },
        Quest(FORGETTABLE_TALE_OF_A_DRUNKEN_DWARF),
        Quest(GARDEN_OF_TRANQUILLITY),
        Quest(ENLIGHTENED_JOURNEY),
        // TODO: Recipe for Disaster: Evil Dave
        Quest(BIG_CHOMPY_BIRD_HUNTING),
        Quest(ZOGRE_FLESH_EATERS),
        // TODO: Recipe for Disaster: Pirate Pete
        Quest(TAI_BWO_WANNAI_TRIO),
        Quest(THE_TOURIST_TRAP),
        Train {
            skill: Herblore,
            from: Level(25),
            to: Level(31),
        },
        Quest(EADGARS_RUSE),
        Quest(MY_ARMS_BIG_ADVENTURE),
        Quest(THE_FREMENNIK_TRIALS),
        Train {
            skill: Construction,
            from: Level(17),
            to: Level(20),
        },
        Quest(THE_FREMENNIK_ISLES),
        Train {
            skill: Cooking,
            from: Level(36),
            to: Level(41),
        },
        // TODO: Recipe for Disaster: The Lumbridge Guide
        // TODO: Recipe for Disaster: Skrach Uglogwee
        Quest(HAUNTED_MINE),
        Train {
            skill: Mining,
            from: Level(37),
            to: Level(40),
        },
        Quest(WATCHTOWER),
        Quest(PRINCE_ALI_RESCUE),
        Quest(CONTACT),
        Train {
            skill: Magic,
            from: Level(42),
            to: Level(46),
        },
        Quest(THE_EYES_OF_GLOUPHRIE),
        Train {
            skill: Firemaking,
            from: Level(25),
            to: Level(30),
        },
        Quest(SEA_SLUG),
        Train {
            skill: Firemaking,
            from: Level(30),
            to: Level(40),
        },
        Train {
            skill: Woodcutting,
            from: Level(42),
            to: Level(50),
        },
        Quest(OLAFS_QUEST),
        Train {
            skill: Firemaking,
            from: Level(40),
            to: Level(50),
        },
        Quest(TEARS_OF_GUTHIX),
        Quest(RATCATCHERS),
        Train {
            skill: Ranged,
            from: Level(38),
            to: Level(40),
        },
        Quest(TEMPLE_OF_IKOV),
        Quest(ONE_SMALL_FAVOUR),
        Quest(A_TAIL_OF_TWO_CATS),
        Train {
            skill: Smithing,
            from: Level(45),
            to: Level(50),
        },
        Quest(BETWEEN_A_ROCK),
        Train {
            skill: Slayer,
            from: Level(25),
            to: Level(30),
        },
        Train {
            skill: Runecraft,
            from: Level(23),
            to: Level(30),
        },
        Quest(THE_SLUG_MENACE),
        Quest(MONKEY_MADNESS_I),
        Train {
            skill: Construction,
            from: Level(26),
            to: Level(34),
        },
        Train {
            skill: Hunter,
            from: Level(9),
            to: Level(10),
        },
        Quest(COLD_WAR),
        Train {
            skill: Hunter,
            from: Level(10),
            to: Level(12),
        },
        Quest(THE_ASCENT_OF_ARCEUUS),
        Train {
            skill: Hunter,
            from: Level(16),
            to: Level(27),
        },
        Quest(EAGLES_PEAK),
        Quest(UNDERGROUND_PASS),
        Train {
            skill: Slayer,
            from: Level(30),
            to: Level(42),
        },
        Quest(SKIPPY_AND_THE_MOGRES),
        Quest(RAG_AND_BONE_MAN_II),
        Quest(LAIR_OF_TARN_RAZORLOR),
        Train {
            skill: Fishing,
            from: Level(33),
            to: Level(50),
        },
        Train {
            skill: Prayer,
            from: Level(39),
            to: Level(47),
        },
        Train {
            skill: Farming,
            from: Level(35),
            to: Level(40),
        },
        Quest(RUM_DEAL),
        Quest(SHEEP_SHEARER),
        Quest(MISTHALIN_MYSTERY),
        Train {
            skill: Crafting,
            from: Level(43),
            to: Level(45),
        },
        Train {
            skill: Agility,
            from: Level(41),
            to: Level(42),
        },
        Quest(PIRATES_TREASURE),
        Quest(CABIN_FEVER),
        Train {
            skill: Prayer,
            from: Level(47),
            to: Level(50),
        },
        Quest(THE_GREAT_BRAIN_ROBBERY),
        Train {
            skill: Crafting,
            from: Level(46),
            to: Level(50),
        },
        Quest(THE_HAND_IN_THE_SAND),
        Train {
            skill: Mining,
            from: Level(41),
            to: Level(45),
        },
        Quest(ENAKHRAS_LAMENT),
        Train {
            skill: Mining,
            from: Level(46),
            to: Level(50),
        },
        Train {
            skill: Cooking,
            from: Level(41),
            to: Level(53),
        },
        Train {
            skill: Fishing,
            from: Level(50),
            to: Level(53),
        },
        Quest(HEROES_QUEST),
        Quest(THRONE_OF_MISCELLANIA),
        Quest(ROYAL_TROUBLE),
        Train {
            skill: Magic,
            from: Level(48),
            to: Level(50),
        },
        Train {
            skill: Thieving,
            from: Level(44),
            to: Level(53),
        },
        Quest(DESERT_TREASURE),
        Quest(CURSE_OF_THE_EMPTY_LORD),
        Quest(THE_GENERALS_SHADOW),
        Train {
            skill: Magic,
            from: Level(51),
            to: Level(59),
        },
        Train {
            skill: Mining,
            from: Level(50),
            to: Level(52),
        },
        Train {
            skill: Herblore,
            from: Level(39),
            to: Level(45),
        },
        Train {
            skill: Agility,
            from: Level(44),
            to: Level(50),
        },
        Quest(A_TASTE_OF_HOPE),
        Quest(FAMILY_CREST),
        Quest(LEGENDS_QUEST),
        // TODO: Recipe for Disaster: Sir Amik Varze.
        Diary(EASY_ARDOUGNE_DIARY),
        Diary(EASY_DESERT_DIARY),
        Diary(EASY_FALADOR_DIARY),
        Diary(EASY_FREMENNIK_DIARY),
        Diary(EASY_KANDARIN_DIARY),
        Diary(EASY_KARAMJA_DIARY),
        Diary(EASY_KOUREND_AND_KEBOS_DIARY),
        Diary(EASY_LUMBRIDGE_AND_DRAYNOR_DIARY),
        Diary(EASY_MORYTANIA_DIARY),
        Diary(EASY_VARROCK_DIARY),
        Diary(EASY_WESTERN_PROVINCES_DIARY),
        Diary(EASY_WILDERNESS_DIARY),
        Diary(MEDIUM_ARDOUGNE_DIARY),
        Diary(MEDIUM_FALADOR_DIARY),
        Diary(MEDIUM_VARROCK_DIARY),
        Train {
            skill: Herblore,
            from: Level(45),
            to: Level(48),
        },
        Train {
            skill: Fletching,
            from: Level(30),
            to: Level(50),
        },
        Diary(MEDIUM_KANDARIN_DIARY),
        Train {
            skill: Herblore,
            from: Level(48),
            to: Level(53),
        },
        Train {
            skill: Farming,
            from: Level(41),
            to: Level(46),
        },
        Quest(FAIRY_TALE_II_CURE_A_QUEEN),
        Train {
            skill: Cooking,
            from: Level(53),
            to: Level(65),
        },
        // TODO: Recipe for Disaster: King Awowogei.
        Train {
            skill: Agility,
            from: Level(50),
            to: Level(56),
        },
        Quest(REGICIDE),
        Quest(SHEEP_HERDER),
        Quest(ROVING_ELVES),
        Train {
            skill: Ranged,
            from: Level(42),
            to: Level(60),
        },
        Quest(MOURNINGS_END_PART_I),
        Quest(MOURNINGS_END_PART_II),
        Train {
            skill: Magic,
            from: Level(59),
            to: Level(65),
        },
        Train {
            skill: Crafting,
            from: Level(50),
            to: Level(61),
        },
        Train {
            skill: Mining,
            from: Level(52),
            to: Level(60),
        },
        Train {
            skill: Woodcutting,
            from: Level(50),
            to: Level(55),
        },
        Quest(LUNAR_DIPLOMACY),
        Quest(WHAT_LIES_BELOW),
        Train {
            skill: Defence,
            from: Level(49),
            to: Level(65),
        },
        Quest(KINGS_RANSOM),
        // TODO: Knight's Wave Training Grounds
        Train {
            skill: Magic,
            from: Level(65),
            to: Level(66),
        },
        Train {
            skill: Fishing,
            from: Level(53),
            to: Level(57),
        },
        Quest(SWAN_SONG),
        // TODO: Recipe for Disaster: The Final Battle
        Train {
            skill: Agility,
            from: Level(57),
            to: Level(59),
        },
        Train {
            skill: Woodcutting,
            from: Level(55),
            to: Level(68),
        },
        Train {
            skill: Thieving,
            from: Level(54),
            to: Level(58),
        },
        Quest(GRIM_TALES),
        // TODO: Train Combat to 85.
        Quest(DREAM_MENTOR),
        Quest(TALE_OF_THE_RIGHTEOUS),
        Quest(THE_FORSAKEN_TOWER),
        Train {
            skill: Hunter,
            from: Level(29),
            to: Level(53),
        },
        Diary(MEDIUM_WILDERNESS_DIARY),
        Diary(MEDIUM_KARAMJA_DIARY),
        Diary(MEDIUM_KOUREND_AND_KEBOS_DIARY),
        Diary(MEDIUM_LUMBRIDGE_AND_DRAYNOR_DIARY),
        Diary(MEDIUM_MORYTANIA_DIARY),
        Train {
            skill: Smithing,
            from: Level(51),
            to: Level(65),
        },
        Train {
            skill: Runecraft,
            from: Level(39),
            to: Level(50),
        },
        Quest(DEVIOUS_MINDS),
        Train {
            skill: Crafting,
            from: Level(61),
            to: Level(65),
        },
        Train {
            skill: Slayer,
            from: Level(44),
            to: Level(60),
        },
        Train {
            skill: Runecraft,
            from: Level(50),
            to: Level(55),
        },
        Train {
            skill: Fishing,
            from: Level(57),
            to: Level(60),
        },
        Quest(THE_FREMENNIK_EXILES),
        Train {
            skill: Hunter,
            from: Level(53),
            to: Level(60),
        },
        Train {
            skill: Agility,
            from: Level(59),
            to: Level(68),
        },
        Train {
            skill: Firemaking,
            from: Level(50),
            to: Level(66),
        },
        Train {
            skill: Slayer,
            from: Level(60),
            to: Level(69),
        },
        Train {
            skill: Crafting,
            from: Level(65),
            to: Level(70),
        },
        Train {
            skill: Mining,
            from: Level(60),
            to: Level(71),
        },
        Quest(ROMEO_AND_JULIET),
        Quest(MAKING_FRIENDS_WITH_MY_ARM),
        Quest(MONKEY_MADNESS_II),
        Train {
            skill: Construction,
            from: Level(36),
            to: Level(37),
        },
        Diary(MEDIUM_FREMENNIK_DIARY),
        Train {
            skill: Construction,
            from: Level(37),
            to: Level(50),
        },
        Train {
            skill: Magic,
            from: Level(66),
            to: Level(75),
        },
        Train {
            skill: Smithing,
            from: Level(65),
            to: Level(70),
        },
        Train {
            skill: Thieving,
            from: Level(58),
            to: Level(60),
        },
        Quest(DRAGON_SLAYER_II),
        Train {
            skill: Agility,
            from: Level(68),
            to: Level(70),
        },
        Train {
            skill: Construction,
            from: Level(50),
            to: Level(70),
        },
        Train {
            skill: Farming,
            from: Level(46),
            to: Level(70),
        },
        Train {
            skill: Herblore,
            from: Level(53),
            to: Level(70),
        },
        Train {
            skill: Hunter,
            from: Level(60),
            to: Level(70),
        },
        Train {
            skill: Woodcutting,
            from: Level(68),
            to: Level(70),
        },
        Quest(SONG_OF_THE_ELVES),
        Quest(THE_CORSAIR_CURSE),
        Quest(CLOCK_TOWER),
    ]
}
