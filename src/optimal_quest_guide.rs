use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::Level;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct QuestId(i32);

#[derive(Debug)]
pub enum QuestKind {
    Free,
    Members,
    Miniquest,
}

#[derive(Debug)]
pub struct Quest {
    id: QuestId,
    name: String,
    kind: QuestKind,
    requirements: Vec<QuestRequirement>,
}

impl Quest {
    pub fn id(&self) -> QuestId {
        self.id
    }
}

#[derive(Debug)]
pub enum Skill {
    Attack,
    Defence,
    Strength,
    Hitpoints,
    Ranged,
    Prayer,
    Magic,
    Cooking,
    Woodcutting,
    Fletching,
    Fishing,
    Firemaking,
    Crafting,
    Smithing,
    Mining,
    Herblore,
    Agility,
    Thieving,
    Slayer,
    Farming,
    Runecraft,
    Hunter,
    Construction,
}

#[derive(Debug)]
pub struct SkillRequirement {
    skill: Skill,
    level: Level,
    boostable: bool,
    ironman_only: bool,
}

impl SkillRequirement {
    fn builder(level: Level, skill: Skill) -> SkillRequirementBuilder {
        SkillRequirementBuilder::new(level, skill)
    }
}

struct SkillRequirementBuilder(SkillRequirement);

impl SkillRequirementBuilder {
    fn new(level: Level, skill: Skill) -> Self {
        Self(SkillRequirement {
            skill,
            level,
            boostable: false,
            ironman_only: false,
        })
    }

    fn boostable(mut self) -> Self {
        self.0.boostable = true;
        self
    }

    fn ironman_only(mut self) -> Self {
        self.0.ironman_only = true;
        self
    }

    fn build(self) -> SkillRequirement {
        self.0
    }
}

#[derive(Debug)]
pub enum QuestRequirement {
    Skill(SkillRequirement),
    Quest(QuestId),
    QuestPoints(u32),
}

pub const COOKS_ASSISTANT: QuestId = QuestId(1);

pub const DEMON_SLAYER: QuestId = QuestId(2);

pub const THE_RESTLESS_GHOST: QuestId = QuestId(3);

pub const ROMEO_AND_JULIET: QuestId = QuestId(4);

pub const SHEEP_SHEARER: QuestId = QuestId(5);

pub const SHIELD_OF_ARRAV: QuestId = QuestId(6);

pub const ERNEST_THE_CHICKEN: QuestId = QuestId(7);

pub const VAMPIRE_SLAYER: QuestId = QuestId(8);

pub const IMP_CATCHER: QuestId = QuestId(9);

pub const PRINCE_ALI_RESCUE: QuestId = QuestId(10);

pub const DORICS_QUEST: QuestId = QuestId(11);

pub const BLACK_KNIGHTS_FORTRESS: QuestId = QuestId(12);

pub const WITCHS_POTION: QuestId = QuestId(13);

pub const THE_KNIGHTS_SWORD: QuestId = QuestId(14);

pub const GOBLIN_DIPLOMACY: QuestId = QuestId(15);

pub const PIRATES_TREASURE: QuestId = QuestId(16);

pub const DRAGON_SLAYER: QuestId = QuestId(17);

pub const DRUIDIC_RITUAL: QuestId = QuestId(18);

pub const LOST_CITY: QuestId = QuestId(19);

pub const WITCHS_HOUSE: QuestId = QuestId(20);

pub const MERLINS_CRYSTAL: QuestId = QuestId(21);

pub const HEROES_QUEST: QuestId = QuestId(22);

pub const SCORPION_CATCHER: QuestId = QuestId(23);

pub const FAMILY_CREST: QuestId = QuestId(24);

pub const TRIBAL_TOTEM: QuestId = QuestId(25);

pub const FISHING_CONTEST: QuestId = QuestId(26);

pub const MONKS_FRIEND: QuestId = QuestId(27);

pub const TEMPLE_OF_IKOV: QuestId = QuestId(28);

pub const CLOCK_TOWER: QuestId = QuestId(29);

pub const HOLY_GRAIL: QuestId = QuestId(30);

pub const TREE_GNOME_VILLAGE: QuestId = QuestId(31);

pub const FIGHT_ARENA: QuestId = QuestId(32);

pub const HAZEEL_CULT: QuestId = QuestId(33);

pub const SHEEP_HERDER: QuestId = QuestId(34);

pub const PLAGUE_CITY: QuestId = QuestId(35);

pub const SEA_SLUG: QuestId = QuestId(36);

pub const WATERFALL_QUEST: QuestId = QuestId(37);

pub const BIOHAZARD: QuestId = QuestId(38);

pub const JUNGLE_POTION: QuestId = QuestId(39);

pub const THE_GRAND_TREE: QuestId = QuestId(40);

pub const SHILO_VILLAGE: QuestId = QuestId(41);

pub const UNDERGROUND_PASS: QuestId = QuestId(42);

pub const OBSERVATORY_QUEST: QuestId = QuestId(43);

pub const THE_TOURIST_TRAP: QuestId = QuestId(44);

pub const WATCHTOWER: QuestId = QuestId(45);

pub const DWARF_CANNON: QuestId = QuestId(46);

pub const MURDER_MYSTERY: QuestId = QuestId(47);

pub const THE_DIG_SITE: QuestId = QuestId(48);

pub const GERTRUDES_CAT: QuestId = QuestId(49);

pub const LEGENDS_QUEST: QuestId = QuestId(50);

pub const RUNE_MYSTERIES: QuestId = QuestId(51);

pub const BIG_CHOMPY_BIRD_HUNTING: QuestId = QuestId(52);

pub const ELEMENTAL_WORKSHOP_I: QuestId = QuestId(53);

pub const PRIEST_IN_PERIL: QuestId = QuestId(54);

pub const NATURE_SPIRIT: QuestId = QuestId(55);

pub const DEATH_PLATEAU: QuestId = QuestId(56);

pub const TROLL_STRONGHOLD: QuestId = QuestId(57);

pub const TAI_BWO_WANNAI_TRIO: QuestId = QuestId(58);

pub const REGICIDE: QuestId = QuestId(59);

pub const EADGARS_RUSE: QuestId = QuestId(60);

pub const SHADES_OF_MORTTON: QuestId = QuestId(61);

pub const THE_FREMENNIK_TRIALS: QuestId = QuestId(62);

pub const HORROR_FROM_THE_DEEP: QuestId = QuestId(63);

pub const THRONE_OF_MISCELLANIA: QuestId = QuestId(64);

pub const MONKEY_MADNESS_I: QuestId = QuestId(65);

pub const HAUNTED_MINE: QuestId = QuestId(66);

pub const TROLL_ROMANCE: QuestId = QuestId(67);

pub const IN_SEARCH_OF_THE_MYREQUE: QuestId = QuestId(68);

pub const CREATURE_OF_FENKENSTRAIN: QuestId = QuestId(69);

pub const ROVING_ELVES: QuestId = QuestId(70);

pub const GHOSTS_AHOY: QuestId = QuestId(71);

pub const ONE_SMALL_FAVOUR: QuestId = QuestId(72);

pub const MOUNTAIN_DAUGHTER: QuestId = QuestId(73);

pub const BETWEEN_A_ROCK: QuestId = QuestId(74);

pub const THE_FEUD: QuestId = QuestId(75);

pub const THE_GOLEM: QuestId = QuestId(76);

pub const DESERT_TREASURE: QuestId = QuestId(77);

pub const ICTHLARINS_LITTLE_HELPER: QuestId = QuestId(78);

pub const TEARS_OF_GUTHIX: QuestId = QuestId(79);

pub const ZOGRE_FLESH_EATERS: QuestId = QuestId(80);

pub const THE_LOST_TRIBE: QuestId = QuestId(81);

pub const THE_GIANT_DWARF: QuestId = QuestId(82);

pub const RECRUITMENT_DRIVE: QuestId = QuestId(83);

pub const MOURNINGS_END_PART_I: QuestId = QuestId(84);

pub const FORGETTABLE_TALE_OF_A_DRUNKEN_DWARF: QuestId = QuestId(85);

pub const GARDEN_OF_TRANQUILLITY: QuestId = QuestId(86);

pub const A_TAIL_OF_TWO_CATS: QuestId = QuestId(87);

pub const WANTED: QuestId = QuestId(88);

pub const MOURNINGS_END_PART_II: QuestId = QuestId(89);

pub const RUM_DEAL: QuestId = QuestId(90);

pub const SHADOW_OF_THE_STORM: QuestId = QuestId(91);

pub const MAKING_HISTORY: QuestId = QuestId(92);

pub const RATCATCHERS: QuestId = QuestId(93);

pub const SPIRITS_OF_THE_ELID: QuestId = QuestId(94);

pub const DEVIOUS_MINDS: QuestId = QuestId(95);

pub const THE_HAND_IN_THE_SAND: QuestId = QuestId(96);

pub const ENAKHRAS_LAMENT: QuestId = QuestId(97);

pub const CABIN_FEVER: QuestId = QuestId(98);

pub const FAIRY_TALE_I_GROWING_PAINS: QuestId = QuestId(99);

pub const RECIPE_FOR_DISASTER: QuestId = QuestId(100);

pub const IN_AID_OF_THE_MYREQUE: QuestId = QuestId(101);

pub const A_SOULS_BANE: QuestId = QuestId(102);

pub const RAG_AND_BONE_MAN: QuestId = QuestId(103);

pub const RAG_AND_BONE_MAN_II: QuestId = QuestId(104);

pub const SWAN_SONG: QuestId = QuestId(105);

pub const ROYAL_TROUBLE: QuestId = QuestId(106);

pub const DEATH_TO_THE_DORGESHUUN: QuestId = QuestId(107);

pub const FAIRY_TALE_II_CURE_A_QUEEN: QuestId = QuestId(108);

pub const LUNAR_DIPLOMACY: QuestId = QuestId(109);

pub const THE_EYES_OF_GLOUPHRIE: QuestId = QuestId(110);

pub const DARKNESS_OF_HALLOWVALE: QuestId = QuestId(111);

pub const THE_SLUG_MENACE: QuestId = QuestId(112);

pub const ELEMENTAL_WORKSHOP_II: QuestId = QuestId(113);

pub const MY_ARMS_BIG_ADVENTURE: QuestId = QuestId(114);

pub const ENLIGHTENED_JOURNEY: QuestId = QuestId(115);

pub const EAGLES_PEAK: QuestId = QuestId(116);

pub const ANIMAL_MAGNETISM: QuestId = QuestId(117);

pub const CONTACT: QuestId = QuestId(118);

pub const COLD_WAR: QuestId = QuestId(119);

pub const THE_FREMENNIK_ISLES: QuestId = QuestId(120);

pub const TOWER_OF_LIFE: QuestId = QuestId(121);

pub const THE_GREAT_BRAIN_ROBBERY: QuestId = QuestId(122);

pub const WHAT_LIES_BELOW: QuestId = QuestId(123);

pub const OLAFS_QUEST: QuestId = QuestId(124);

pub const ANOTHER_SLICE_OF_HAM: QuestId = QuestId(125);

pub const DREAM_MENTOR: QuestId = QuestId(126);

pub const GRIM_TALES: QuestId = QuestId(127);

pub const KINGS_RANSOM: QuestId = QuestId(128);

pub const MONKEY_MADNESS_II: QuestId = QuestId(129);

pub const MISTHALIN_MYSTERY: QuestId = QuestId(130);

pub const CLIENT_OF_KOUREND: QuestId = QuestId(131);

pub const BONE_VOYAGE: QuestId = QuestId(132);

pub const THE_QUEEN_OF_THIEVES: QuestId = QuestId(133);

pub const THE_DEPTHS_OF_DESPAIR: QuestId = QuestId(134);

pub const THE_CORSAIR_CURSE: QuestId = QuestId(135);

pub const DRAGON_SLAYER_II: QuestId = QuestId(136);

pub const TALE_OF_THE_RIGHTEOUS: QuestId = QuestId(137);

pub const A_TASTE_OF_HOPE: QuestId = QuestId(138);

pub const MAKING_FRIENDS_WITH_MY_ARM: QuestId = QuestId(139);

pub const THE_FORSAKEN_TOWER: QuestId = QuestId(140);

pub const THE_ASCENT_OF_ARCEUUS: QuestId = QuestId(141);

pub const X_MARKS_THE_SPOT: QuestId = QuestId(142);

pub const SONG_OF_THE_ELVES: QuestId = QuestId(143);

pub const THE_FREMENNIK_EXILES: QuestId = QuestId(144);

// Miniquests

// TODO: We should probably assign the miniquests IDs in their release order.

pub const ALFRED_GRIMHANDS_BARCRAWL: QuestId = QuestId(-1);

pub const ARCHITECTURAL_ALLIANCE: QuestId = QuestId(-2);

pub const BEAR_YOUR_SOUL: QuestId = QuestId(-3);

pub const CURSE_OF_THE_EMPTY_LORD: QuestId = QuestId(-4);

pub const ENCHANTED_KEY: QuestId = QuestId(-5);

pub const ENTER_THE_ABYSS: QuestId = QuestId(-6);

pub const FAMILY_PEST: QuestId = QuestId(-7);

pub const THE_GENERALS_SHADOW: QuestId = QuestId(-8);

pub const IN_SEARCH_OF_KNOWLEDGE: QuestId = QuestId(-9);

pub const LAIR_OF_TARN_RAZORLOR: QuestId = QuestId(-10);

pub const THE_MAGE_ARENA: QuestId = QuestId(-11);

pub const THE_MAGE_ARENA_II: QuestId = QuestId(-12);

pub const SKIPPY_AND_THE_MOGRES: QuestId = QuestId(-13);

fn all_quests() -> Vec<Quest> {
    vec![
        Quest {
            id: COOKS_ASSISTANT,
            name: "Cook's Assistant".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: DEMON_SLAYER,
            name: "Demon Slayer".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: THE_RESTLESS_GHOST,
            name: "The Restless Ghost".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: ROMEO_AND_JULIET,
            name: "Romeo & Juliet".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: SHEEP_SHEARER,
            name: "Sheep Shearer".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: SHIELD_OF_ARRAV,
            name: "Shield of Arrav".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: ERNEST_THE_CHICKEN,
            name: "Ernest the Chicken".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: VAMPIRE_SLAYER,
            name: "Vampire Slayer".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: IMP_CATCHER,
            name: "Imp Catcher".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: PRINCE_ALI_RESCUE,
            name: "Prince Ali Rescue".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: DORICS_QUEST,
            name: "Doric's Quest".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: BLACK_KNIGHTS_FORTRESS,
            name: "Black Knights' Fortress".into(),
            kind: QuestKind::Free,
            requirements: vec![QuestRequirement::QuestPoints(12)],
        },
        Quest {
            id: WITCHS_POTION,
            name: "Witch's Potion".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: THE_KNIGHTS_SWORD,
            name: "The Knight's Sword".into(),
            kind: QuestKind::Free,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(10), Skill::Mining).build(),
            )],
        },
        Quest {
            id: GOBLIN_DIPLOMACY,
            name: "Goblin Diplomacy".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: PIRATES_TREASURE,
            name: "Pirate's Treasure".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: DRAGON_SLAYER,
            name: "Dragon Slayer".into(),
            kind: QuestKind::Free,
            requirements: vec![
                QuestRequirement::QuestPoints(32),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(8), Skill::Crafting)
                        .ironman_only()
                        .build(),
                ),
            ],
        },
        Quest {
            id: DRUIDIC_RITUAL,
            name: "Druidic Ritual".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: LOST_CITY,
            name: "Lost City".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(31), Skill::Crafting)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(36), Skill::Woodcutting).build(),
                ),
            ],
        },
        Quest {
            id: WITCHS_HOUSE,
            name: "Witch's House".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: MERLINS_CRYSTAL,
            name: "Merlin's Crystal".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: HEROES_QUEST,
            name: "Heroes' Quest".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(55),
                QuestRequirement::Quest(SHIELD_OF_ARRAV),
                QuestRequirement::Quest(LOST_CITY),
                QuestRequirement::Quest(MERLINS_CRYSTAL),
                QuestRequirement::Quest(DRAGON_SLAYER),
                QuestRequirement::Quest(DRUIDIC_RITUAL),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(53), Skill::Cooking)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(53), Skill::Fishing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Herblore)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Mining)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: SCORPION_CATCHER,
            name: "Scorpion Catcher".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(31), Skill::Prayer)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(ALFRED_GRIMHANDS_BARCRAWL),
            ],
        },
        Quest {
            id: FAMILY_CREST,
            name: "Family Crest".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Mining)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Smithing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(59), Skill::Magic)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Crafting)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: TRIBAL_TOTEM,
            name: "Tribal Totem".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(21), Skill::Thieving).build(),
            )],
        },
        Quest {
            id: FISHING_CONTEST,
            name: "Fishing Contest".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(10), Skill::Fishing).build(),
            )],
        },
        Quest {
            id: MONKS_FRIEND,
            name: "Monk's Friend".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: TEMPLE_OF_IKOV,
            name: "Temple of Ikov".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(42), Skill::Thieving)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Ranged).build(),
                ),
            ],
        },
        Quest {
            id: CLOCK_TOWER,
            name: "Clock Tower".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: HOLY_GRAIL,
            name: "Holy Grail".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(MERLINS_CRYSTAL),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Attack).build(),
                ),
            ],
        },
        Quest {
            id: TREE_GNOME_VILLAGE,
            name: "Tree Gnome Village".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: FIGHT_ARENA,
            name: "Fight Arena".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: HAZEEL_CULT,
            name: "Hazeel Cult".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: SHEEP_HERDER,
            name: "Sheep Herder".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: PLAGUE_CITY,
            name: "Plague City".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: SEA_SLUG,
            name: "Sea Slug".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(30), Skill::Firemaking).build(),
            )],
        },
        Quest {
            id: WATERFALL_QUEST,
            name: "Waterfall Quest".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: BIOHAZARD,
            name: "Biohazard".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Quest(PLAGUE_CITY)],
        },
        Quest {
            id: JUNGLE_POTION,
            name: "Jungle Potion".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Quest(DRUIDIC_RITUAL)],
        },
        Quest {
            id: THE_GRAND_TREE,
            name: "The Grand Tree".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(25), Skill::Agility)
                    .boostable()
                    .build(),
            )],
        },
        Quest {
            id: SHILO_VILLAGE,
            name: "Shilo Village".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(JUNGLE_POTION),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(32), Skill::Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(4), Skill::Smithing)
                        .ironman_only()
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: UNDERGROUND_PASS,
            name: "Underground Pass".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Ranged).build(),
                ),
                QuestRequirement::Quest(BIOHAZARD),
            ],
        },
        Quest {
            id: OBSERVATORY_QUEST,
            name: "Observatory Quest".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: THE_TOURIST_TRAP,
            name: "The Tourist Trap".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(10), Skill::Fletching).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Smithing).build(),
                ),
            ],
        },
        Quest {
            id: WATCHTOWER,
            name: "Watchtower".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Skill::Magic).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(15), Skill::Thieving).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Agility).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(14), Skill::Herblore).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Mining)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: DWARF_CANNON,
            name: "Dwarf Cannon".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: MURDER_MYSTERY,
            name: "Murder Mystery".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: THE_DIG_SITE,
            name: "The Dig Site".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(DRUIDIC_RITUAL),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(10), Skill::Agility).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(10), Skill::Herblore).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Thieving).build(),
                ),
            ],
        },
        Quest {
            id: GERTRUDES_CAT,
            name: "Gertrude's Cat".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: LEGENDS_QUEST,
            name: "Legends' Quest".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(107),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Skill::Herblore)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(56), Skill::Magic).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(52), Skill::Mining)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(42), Skill::Prayer).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Smithing).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Strength)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Thieving).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Woodcutting)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(FAMILY_CREST),
                QuestRequirement::Quest(HEROES_QUEST),
                QuestRequirement::Quest(SHILO_VILLAGE),
                QuestRequirement::Quest(UNDERGROUND_PASS),
                QuestRequirement::Quest(WATERFALL_QUEST),
            ],
        },
        Quest {
            id: RUNE_MYSTERIES,
            name: "Rune Mysteries".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: BIG_CHOMPY_BIRD_HUNTING,
            name: "Big Chompy Bird Hunting".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(5), Skill::Fletching).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Cooking)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Ranged).build(),
                ),
            ],
        },
        Quest {
            id: ELEMENTAL_WORKSHOP_I,
            name: "Elemental Workshop I".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Mining).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Smithing).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Crafting).build(),
                ),
            ],
        },
        Quest {
            id: PRIEST_IN_PERIL,
            name: "Priest in Peril".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: NATURE_SPIRIT,
            name: "Nature Spirit".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Quest(THE_RESTLESS_GHOST),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(18), Skill::Crafting)
                        .ironman_only()
                        .build(),
                ),
            ],
        },
        Quest {
            id: DEATH_PLATEAU,
            name: "Death Plateau".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: TROLL_STRONGHOLD,
            name: "Troll Stronghold".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(DEATH_PLATEAU),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(15), Skill::Agility).build(),
                ),
            ],
        },
        Quest {
            id: TAI_BWO_WANNAI_TRIO,
            name: "Tai Bwo Wannai Trio".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(15), Skill::Agility).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Cooking).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(5), Skill::Fishing).build(),
                ),
                QuestRequirement::Quest(JUNGLE_POTION),
            ],
        },
        Quest {
            id: REGICIDE,
            name: "Regicide".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(UNDERGROUND_PASS),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(56), Skill::Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(10), Skill::Crafting).build(),
                ),
            ],
        },
        Quest {
            id: EADGARS_RUSE,
            name: "Eadgar's Ruse".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(DRUIDIC_RITUAL),
                QuestRequirement::Quest(TROLL_STRONGHOLD),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(31), Skill::Herblore)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: SHADES_OF_MORTTON,
            name: "Shades of Mort'ton".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(15), Skill::Herblore).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(5), Skill::Firemaking).build(),
                ),
            ],
        },
        Quest {
            id: THE_FREMENNIK_TRIALS,
            name: "The Fremennik Trials".into(),
            kind: QuestKind::Members,
            requirements: vec![
                // TODO: Figure out how to represent skill requirements for
                // crafting the lyre.
            ],
        },
        Quest {
            id: HORROR_FROM_THE_DEEP,
            name: "Horror from the Deep".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(35), Skill::Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(ALFRED_GRIMHANDS_BARCRAWL),
            ],
        },
        Quest {
            id: THRONE_OF_MISCELLANIA,
            name: "Throne of Miscellania".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(HEROES_QUEST),
                QuestRequirement::Quest(THE_FREMENNIK_TRIALS),
                // TODO: Figure out how to represent skill requirements for
                // gaining favor.
            ],
        },
        Quest {
            id: MONKEY_MADNESS_I,
            name: "Monkey Madness".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(THE_GRAND_TREE),
                QuestRequirement::Quest(TREE_GNOME_VILLAGE),
            ],
        },
        Quest {
            id: HAUNTED_MINE,
            name: "Haunted Mine".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(15), Skill::Agility).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(35), Skill::Crafting).build(),
                ),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
            ],
        },
        Quest {
            id: TROLL_ROMANCE,
            name: "Troll Romance".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(28), Skill::Agility).build(),
                ),
                QuestRequirement::Quest(TROLL_STRONGHOLD),
                // TODO: Figure out how to represent skill requirements for
                // obtaining a maple or yew log for ironmen.
            ],
        },
        Quest {
            id: IN_SEARCH_OF_THE_MYREQUE,
            name: "In Search of the Myreque".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(NATURE_SPIRIT),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Agility)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: CREATURE_OF_FENKENSTRAIN,
            name: "Creature of Fenkenstrain".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Quest(THE_RESTLESS_GHOST),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Thieving).build(),
                ),
            ],
        },
        Quest {
            id: ROVING_ELVES,
            name: "Roving Elves".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(REGICIDE),
                QuestRequirement::Quest(WATERFALL_QUEST),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(56), Skill::Agility)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: GHOSTS_AHOY,
            name: "Ghosts Ahoy".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(THE_RESTLESS_GHOST),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Cooking)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: ONE_SMALL_FAVOUR,
            name: "One Small Favour".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(36), Skill::Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(18), Skill::Herblore)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Smithing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(RUNE_MYSTERIES),
                QuestRequirement::Quest(SHILO_VILLAGE),
            ],
        },
        Quest {
            id: MOUNTAIN_DAUGHTER,
            name: "Mountain Daughter".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(20), Skill::Agility)
                    .boostable()
                    .build(),
            )],
        },
        Quest {
            id: BETWEEN_A_ROCK,
            name: "Between a Rock...".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(DWARF_CANNON),
                QuestRequirement::Quest(FISHING_CONTEST),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Defence).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Mining)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Smithing)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: THE_FEUD,
            name: "The Feud".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(30), Skill::Thieving).build(),
            )],
        },
        Quest {
            id: THE_GOLEM,
            name: "The Golem".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Thieving)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: DESERT_TREASURE,
            name: "Desert Treasure".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(53), Skill::Thieving).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Firemaking).build(),
                ),
                // TODO: Figure out how to represent that the Slayer requirement
                // is optional if you have a Gas mask from Plague City.
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(10), Skill::Slayer).build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Skill::Magic).build()),
                QuestRequirement::Quest(THE_DIG_SITE),
                QuestRequirement::Quest(TEMPLE_OF_IKOV),
                QuestRequirement::Quest(THE_TOURIST_TRAP),
                QuestRequirement::Quest(TROLL_STRONGHOLD),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Quest(WATERFALL_QUEST),
            ],
        },
        Quest {
            id: ICTHLARINS_LITTLE_HELPER,
            name: "Ichlarin's Little Helper".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Quest(GERTRUDES_CAT)],
        },
        Quest {
            id: TEARS_OF_GUTHIX,
            name: "Tears of Guthix".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(43),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(49), Skill::Firemaking).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Skill::Mining).build(),
                ),
            ],
        },
        Quest {
            id: ZOGRE_FLESH_EATERS,
            name: "Zogre Flesh Eaters".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(BIG_CHOMPY_BIRD_HUNTING),
                QuestRequirement::Quest(JUNGLE_POTION),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(4), Skill::Smithing).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(8), Skill::Herblore).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Ranged).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Fletching)
                        .ironman_only()
                        .build(),
                ),
            ],
        },
        Quest {
            id: THE_LOST_TRIBE,
            name: "The Lost Tribe".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(GOBLIN_DIPLOMACY),
                QuestRequirement::Quest(RUNE_MYSTERIES),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(12), Skill::Agility).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(13), Skill::Thieving).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(17), Skill::Mining).build(),
                ),
            ],
        },
        Quest {
            id: THE_GIANT_DWARF,
            name: "The Giant Dwarf".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(12), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(16), Skill::Firemaking).build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(33), Skill::Magic).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(14), Skill::Thieving).build(),
                ),
            ],
        },
        Quest {
            id: RECRUITMENT_DRIVE,
            name: "Recruitment Drive".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(12),
                QuestRequirement::Quest(BLACK_KNIGHTS_FORTRESS),
                QuestRequirement::Quest(DRUIDIC_RITUAL),
            ],
        },
        Quest {
            id: MOURNINGS_END_PART_I,
            name: "Mourning's End Part I".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(60), Skill::Ranged).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Thieving)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(ROVING_ELVES),
                QuestRequirement::Quest(BIG_CHOMPY_BIRD_HUNTING),
                QuestRequirement::Quest(SHEEP_HERDER),
            ],
        },
        Quest {
            id: FORGETTABLE_TALE_OF_A_DRUNKEN_DWARF,
            name: "Forgettable Tale of a Drunken Dwarf".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(22), Skill::Cooking).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(17), Skill::Farming).build(),
                ),
                QuestRequirement::Quest(THE_GIANT_DWARF),
                QuestRequirement::Quest(FISHING_CONTEST),
            ],
        },
        Quest {
            id: GARDEN_OF_TRANQUILLITY,
            name: "Garden of Tranquillity".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Farming).build(),
                ),
                QuestRequirement::Quest(CREATURE_OF_FENKENSTRAIN),
            ],
        },
        Quest {
            id: A_TAIL_OF_TWO_CATS,
            name: "A Tail of Two Cats".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Quest(ICTHLARINS_LITTLE_HELPER)],
        },
        Quest {
            id: WANTED,
            name: "Wanted!".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(32),
                QuestRequirement::Quest(RECRUITMENT_DRIVE),
                QuestRequirement::Quest(THE_LOST_TRIBE),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Quest(ENTER_THE_ABYSS),
            ],
        },
        Quest {
            id: MOURNINGS_END_PART_II,
            name: "Mourning's End Part II".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Quest(MOURNINGS_END_PART_I)],
        },
        Quest {
            id: RUM_DEAL,
            name: "Rum Deal".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(ZOGRE_FLESH_EATERS),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(42), Skill::Crafting)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Fishing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Farming)
                        .boostable()
                        .build(),
                ),
                // TODO: Figure out how to represent 47 Prayer points requirement.
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(42), Skill::Slayer).build(),
                ),
            ],
        },
        Quest {
            id: SHADOW_OF_THE_STORM,
            name: "Shadow of the Storm".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Skill::Crafting)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(THE_GOLEM),
                QuestRequirement::Quest(DEMON_SLAYER),
            ],
        },
        Quest {
            id: MAKING_HISTORY,
            name: "Making History".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Quest(THE_RESTLESS_GHOST),
            ],
        },
        Quest {
            id: RATCATCHERS,
            name: "Ratcatchers".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(ICTHLARINS_LITTLE_HELPER),
                QuestRequirement::Quest(GERTRUDES_CAT),
                // TODO: Figure out how to represent access to Keldagrim requirement.
            ],
        },
        Quest {
            id: SPIRITS_OF_THE_ELID,
            name: "Spirits of the Elid".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(33), Skill::Magic)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(37), Skill::Ranged)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(37), Skill::Mining).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(37), Skill::Thieving)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: DEVIOUS_MINDS,
            name: "Devious Minds".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(65), Skill::Smithing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Runecraft).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Fletching).build(),
                ),
                QuestRequirement::Quest(WANTED),
                QuestRequirement::Quest(TROLL_STRONGHOLD),
                QuestRequirement::Quest(DORICS_QUEST),
                QuestRequirement::Quest(ENTER_THE_ABYSS),
            ],
        },
        Quest {
            id: THE_HAND_IN_THE_SAND,
            name: "The Hand in the Sand".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(17), Skill::Thieving).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(49), Skill::Crafting).build(),
                ),
            ],
        },
        Quest {
            id: ENAKHRAS_LAMENT,
            name: "Enakhra's Lament".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Skill::Firemaking).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(43), Skill::Prayer).build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(39), Skill::Magic).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Skill::Mining)
                        .ironman_only()
                        .build(),
                ),
            ],
        },
        Quest {
            id: CABIN_FEVER,
            name: "Cabin Fever".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(PIRATES_TREASURE),
                QuestRequirement::Quest(RUM_DEAL),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(42), Skill::Agility).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Skill::Smithing).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Ranged).build(),
                ),
            ],
        },
        Quest {
            id: FAIRY_TALE_I_GROWING_PAINS,
            name: "Fairy Tail I - Growing Pains".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(LOST_CITY),
                QuestRequirement::Quest(NATURE_SPIRIT),
            ],
        },
        // TODO: Add Recipe for Disaster. We just need to figure out how to
        // represent its subquests.
        Quest {
            id: IN_AID_OF_THE_MYREQUE,
            name: "In Aid of the Myreque".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(IN_SEARCH_OF_THE_MYREQUE),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(15), Skill::Mining).build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(7), Skill::Magic).build()),
            ],
        },
        Quest {
            id: A_SOULS_BANE,
            name: "A Soul's Bane".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: RAG_AND_BONE_MAN,
            name: "Rag and Bone Man".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: RAG_AND_BONE_MAN_II,
            name: "Rag and Bone Man II".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Slayer).build(),
                ),
                QuestRequirement::Quest(RAG_AND_BONE_MAN),
                // TODO: Figure out how to represent other partial quest requirements.
            ],
        },
        Quest {
            id: SWAN_SONG,
            name: "Swan Song".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(100),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(66), Skill::Magic)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(62), Skill::Cooking)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(62), Skill::Fishing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Skill::Smithing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(42), Skill::Firemaking).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Crafting).build(),
                ),
                QuestRequirement::Quest(ONE_SMALL_FAVOUR),
                QuestRequirement::Quest(GARDEN_OF_TRANQUILLITY),
            ],
        },
        Quest {
            id: ROYAL_TROUBLE,
            name: "Royal Trouble".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Slayer)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(THRONE_OF_MISCELLANIA),
            ],
        },
        Quest {
            id: DEATH_TO_THE_DORGESHUUN,
            name: "Death to the Dorgeshuun".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(THE_LOST_TRIBE),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(23), Skill::Agility).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(23), Skill::Thieving).build(),
                ),
            ],
        },
        Quest {
            id: FAIRY_TALE_II_CURE_A_QUEEN,
            name: "Fairy Tale II - Cure a Queen".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(FAIRY_TALE_I_GROWING_PAINS),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Thieving).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(49), Skill::Farming)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(57), Skill::Herblore)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: LUNAR_DIPLOMACY,
            name: "Lunar Diplomacy".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(61), Skill::Crafting).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Skill::Defence).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(49), Skill::Firemaking).build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(65), Skill::Magic).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(60), Skill::Mining).build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(55), Skill::Woodcutting).build(),
                ),
                QuestRequirement::Quest(THE_FREMENNIK_TRIALS),
                QuestRequirement::Quest(LOST_CITY),
                QuestRequirement::Quest(RUNE_MYSTERIES),
                QuestRequirement::Quest(SHILO_VILLAGE),
            ],
        },
        Quest {
            id: THE_EYES_OF_GLOUPHRIE,
            name: "The Eyes of Glouphrie".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(5), Skill::Construction).build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(46), Skill::Magic).build()),
                // TODO: Figure out how to represent requirements to get a maple log.
                QuestRequirement::Quest(THE_GRAND_TREE),
            ],
        },
    ]
}

lazy_static! {
    pub static ref QUEST_LIST: HashMap<QuestId, Quest> = all_quests()
        .into_iter()
        .map(|quest| (quest.id(), quest))
        .collect();
}
