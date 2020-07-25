use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::skills::Skill;
use crate::Level;

lazy_static! {
    pub static ref QUEST_LIST: HashMap<QuestId, Quest> = all_quests()
        .into_iter()
        .map(|quest| (quest.id(), quest))
        .collect();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct QuestId(&'static str);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> QuestKind {
        self.kind
    }
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

pub const COOKS_ASSISTANT: QuestId = QuestId("cooks_assistant");

pub const DEMON_SLAYER: QuestId = QuestId("demon_slayer");

pub const THE_RESTLESS_GHOST: QuestId = QuestId("the_restless_ghost");

pub const ROMEO_AND_JULIET: QuestId = QuestId("romeo_and_juliet");

pub const SHEEP_SHEARER: QuestId = QuestId("sheep_shearer");

pub const SHIELD_OF_ARRAV: QuestId = QuestId("shield_of_arrav");

pub const ERNEST_THE_CHICKEN: QuestId = QuestId("ernest_the_chicken");

pub const VAMPIRE_SLAYER: QuestId = QuestId("vampire_slayer");

pub const IMP_CATCHER: QuestId = QuestId("imp_catcher");

pub const PRINCE_ALI_RESCUE: QuestId = QuestId("prince_ali_rescue");

pub const DORICS_QUEST: QuestId = QuestId("dorics_quest");

pub const BLACK_KNIGHTS_FORTRESS: QuestId = QuestId("black_knights_fortress");

pub const WITCHS_POTION: QuestId = QuestId("witchs_potion");

pub const THE_KNIGHTS_SWORD: QuestId = QuestId("the_knights_sword");

pub const GOBLIN_DIPLOMACY: QuestId = QuestId("goblin_diplomacy");

pub const PIRATES_TREASURE: QuestId = QuestId("pirates_treasure");

pub const DRAGON_SLAYER: QuestId = QuestId("dragon_slayer");

pub const DRUIDIC_RITUAL: QuestId = QuestId("druidic_ritual");

pub const LOST_CITY: QuestId = QuestId("lost_city");

pub const WITCHS_HOUSE: QuestId = QuestId("witchs_house");

pub const MERLINS_CRYSTAL: QuestId = QuestId("merlins_crystal");

pub const HEROES_QUEST: QuestId = QuestId("heroes_quest");

pub const SCORPION_CATCHER: QuestId = QuestId("scorpion_catcher");

pub const FAMILY_CREST: QuestId = QuestId("family_crest");

pub const TRIBAL_TOTEM: QuestId = QuestId("tribal_totem");

pub const FISHING_CONTEST: QuestId = QuestId("fishing_contest");

pub const MONKS_FRIEND: QuestId = QuestId("monks_friend");

pub const TEMPLE_OF_IKOV: QuestId = QuestId("temple_of_ikov");

pub const CLOCK_TOWER: QuestId = QuestId("clock_tower");

pub const HOLY_GRAIL: QuestId = QuestId("holy_grail");

pub const TREE_GNOME_VILLAGE: QuestId = QuestId("tree_gnome_village");

pub const FIGHT_ARENA: QuestId = QuestId("fight_arena");

pub const HAZEEL_CULT: QuestId = QuestId("hazeel_cult");

pub const SHEEP_HERDER: QuestId = QuestId("sheep_herder");

pub const PLAGUE_CITY: QuestId = QuestId("plague_city");

pub const SEA_SLUG: QuestId = QuestId("sea_slug");

pub const WATERFALL_QUEST: QuestId = QuestId("waterfall_quest");

pub const BIOHAZARD: QuestId = QuestId("biohazard");

pub const JUNGLE_POTION: QuestId = QuestId("jungle_potion");

pub const THE_GRAND_TREE: QuestId = QuestId("the_grand_tree");

pub const SHILO_VILLAGE: QuestId = QuestId("shilo_village");

pub const UNDERGROUND_PASS: QuestId = QuestId("underground_pass");

pub const OBSERVATORY_QUEST: QuestId = QuestId("observatory_quest");

pub const THE_TOURIST_TRAP: QuestId = QuestId("the_tourist_trap");

pub const WATCHTOWER: QuestId = QuestId("watchtower");

pub const DWARF_CANNON: QuestId = QuestId("dwarf_cannon");

pub const MURDER_MYSTERY: QuestId = QuestId("murder_mystery");

pub const THE_DIG_SITE: QuestId = QuestId("the_dig_site");

pub const GERTRUDES_CAT: QuestId = QuestId("gertrudes_cat");

pub const LEGENDS_QUEST: QuestId = QuestId("legends_quest");

pub const RUNE_MYSTERIES: QuestId = QuestId("rune_mysteries");

pub const BIG_CHOMPY_BIRD_HUNTING: QuestId = QuestId("big_chompy_bird_hunting");

pub const ELEMENTAL_WORKSHOP_I: QuestId = QuestId("elemental_workshop_i");

pub const PRIEST_IN_PERIL: QuestId = QuestId("priest_in_peril");

pub const NATURE_SPIRIT: QuestId = QuestId("nature_spirit");

pub const DEATH_PLATEAU: QuestId = QuestId("death_plateau");

pub const TROLL_STRONGHOLD: QuestId = QuestId("troll_stronghold");

pub const TAI_BWO_WANNAI_TRIO: QuestId = QuestId("tai_bwo_wannai_trio");

pub const REGICIDE: QuestId = QuestId("regicide");

pub const EADGARS_RUSE: QuestId = QuestId("eadgars_ruse");

pub const SHADES_OF_MORTTON: QuestId = QuestId("shades_of_mortton");

pub const THE_FREMENNIK_TRIALS: QuestId = QuestId("the_fremennik_trials");

pub const HORROR_FROM_THE_DEEP: QuestId = QuestId("horror_from_the_deep");

pub const THRONE_OF_MISCELLANIA: QuestId = QuestId("throne_of_miscellania");

pub const MONKEY_MADNESS_I: QuestId = QuestId("monkey_madness_i");

pub const HAUNTED_MINE: QuestId = QuestId("haunted_mine");

pub const TROLL_ROMANCE: QuestId = QuestId("troll_romance");

pub const IN_SEARCH_OF_THE_MYREQUE: QuestId = QuestId("in_search_of_the_myreque");

pub const CREATURE_OF_FENKENSTRAIN: QuestId = QuestId("creature_of_fenkenstrain");

pub const ROVING_ELVES: QuestId = QuestId("roving_elves");

pub const GHOSTS_AHOY: QuestId = QuestId("ghosts_ahoy");

pub const ONE_SMALL_FAVOUR: QuestId = QuestId("one_small_favour");

pub const MOUNTAIN_DAUGHTER: QuestId = QuestId("mountain_daughter");

pub const BETWEEN_A_ROCK: QuestId = QuestId("between_a_rock");

pub const THE_FEUD: QuestId = QuestId("the_feud");

pub const THE_GOLEM: QuestId = QuestId("the_golem");

pub const DESERT_TREASURE: QuestId = QuestId("desert_treasure");

pub const ICTHLARINS_LITTLE_HELPER: QuestId = QuestId("icthlarins_little_helper");

pub const TEARS_OF_GUTHIX: QuestId = QuestId("tears_of_guthix");

pub const ZOGRE_FLESH_EATERS: QuestId = QuestId("zogre_flesh_eaters");

pub const THE_LOST_TRIBE: QuestId = QuestId("the_lost_tribe");

pub const THE_GIANT_DWARF: QuestId = QuestId("the_giant_dwarf");

pub const RECRUITMENT_DRIVE: QuestId = QuestId("recruitment_drive");

pub const MOURNINGS_END_PART_I: QuestId = QuestId("mournings_end_part_i");

pub const FORGETTABLE_TALE_OF_A_DRUNKEN_DWARF: QuestId =
    QuestId("forgettable_tale_of_a_drunken_dwarf");

pub const GARDEN_OF_TRANQUILLITY: QuestId = QuestId("garden_of_tranquillity");

pub const A_TAIL_OF_TWO_CATS: QuestId = QuestId("a_tail_of_two_cats");

pub const WANTED: QuestId = QuestId("wanted");

pub const MOURNINGS_END_PART_II: QuestId = QuestId("mournings_end_part_ii");

pub const RUM_DEAL: QuestId = QuestId("rum_deal");

pub const SHADOW_OF_THE_STORM: QuestId = QuestId("shadow_of_the_storm");

pub const MAKING_HISTORY: QuestId = QuestId("making_history");

pub const RATCATCHERS: QuestId = QuestId("ratcatchers");

pub const SPIRITS_OF_THE_ELID: QuestId = QuestId("spirits_of_the_elid");

pub const DEVIOUS_MINDS: QuestId = QuestId("devious_minds");

pub const THE_HAND_IN_THE_SAND: QuestId = QuestId("the_hand_in_the_sand");

pub const ENAKHRAS_LAMENT: QuestId = QuestId("enakhras_lament");

pub const CABIN_FEVER: QuestId = QuestId("cabin_fever");

pub const FAIRY_TALE_I_GROWING_PAINS: QuestId = QuestId("fairy_tale_i_growing_pains");

pub const RECIPE_FOR_DISASTER: QuestId = QuestId("recipe_for_disaster");

pub const IN_AID_OF_THE_MYREQUE: QuestId = QuestId("in_aid_of_the_myreque");

pub const A_SOULS_BANE: QuestId = QuestId("a_souls_bane");

pub const RAG_AND_BONE_MAN: QuestId = QuestId("rag_and_bone_man");

pub const RAG_AND_BONE_MAN_II: QuestId = QuestId("rag_and_bone_man_ii");

pub const SWAN_SONG: QuestId = QuestId("swan_song");

pub const ROYAL_TROUBLE: QuestId = QuestId("royal_trouble");

pub const DEATH_TO_THE_DORGESHUUN: QuestId = QuestId("death_to_the_dorgeshuun");

pub const FAIRY_TALE_II_CURE_A_QUEEN: QuestId = QuestId("fairy_tale_ii_cure_a_queen");

pub const LUNAR_DIPLOMACY: QuestId = QuestId("lunar_diplomacy");

pub const THE_EYES_OF_GLOUPHRIE: QuestId = QuestId("the_eyes_of_glouphrie");

pub const DARKNESS_OF_HALLOWVALE: QuestId = QuestId("darkness_of_hallowvale");

pub const THE_SLUG_MENACE: QuestId = QuestId("the_slug_menace");

pub const ELEMENTAL_WORKSHOP_II: QuestId = QuestId("elemental_workshop_ii");

pub const MY_ARMS_BIG_ADVENTURE: QuestId = QuestId("my_arms_big_adventure");

pub const ENLIGHTENED_JOURNEY: QuestId = QuestId("enlightened_journey");

pub const EAGLES_PEAK: QuestId = QuestId("eagles_peak");

pub const ANIMAL_MAGNETISM: QuestId = QuestId("animal_magnetism");

pub const CONTACT: QuestId = QuestId("contact");

pub const COLD_WAR: QuestId = QuestId("cold_war");

pub const THE_FREMENNIK_ISLES: QuestId = QuestId("the_fremennik_isles");

pub const TOWER_OF_LIFE: QuestId = QuestId("tower_of_life");

pub const THE_GREAT_BRAIN_ROBBERY: QuestId = QuestId("the_great_brain_robbery");

pub const WHAT_LIES_BELOW: QuestId = QuestId("what_lies_below");

pub const OLAFS_QUEST: QuestId = QuestId("olafs_quest");

pub const ANOTHER_SLICE_OF_HAM: QuestId = QuestId("another_slice_of_ham");

pub const DREAM_MENTOR: QuestId = QuestId("dream_mentor");

pub const GRIM_TALES: QuestId = QuestId("grim_tales");

pub const KINGS_RANSOM: QuestId = QuestId("kings_ransom");

pub const MONKEY_MADNESS_II: QuestId = QuestId("monkey_madness_ii");

pub const MISTHALIN_MYSTERY: QuestId = QuestId("misthalin_mystery");

pub const CLIENT_OF_KOUREND: QuestId = QuestId("client_of_kourend");

pub const BONE_VOYAGE: QuestId = QuestId("bone_voyage");

pub const THE_QUEEN_OF_THIEVES: QuestId = QuestId("the_queen_of_thieves");

pub const THE_DEPTHS_OF_DESPAIR: QuestId = QuestId("the_depths_of_despair");

pub const THE_CORSAIR_CURSE: QuestId = QuestId("the_corsair_curse");

pub const DRAGON_SLAYER_II: QuestId = QuestId("dragon_slayer_ii");

pub const TALE_OF_THE_RIGHTEOUS: QuestId = QuestId("tale_of_the_righteous");

pub const A_TASTE_OF_HOPE: QuestId = QuestId("a_taste_of_hope");

pub const MAKING_FRIENDS_WITH_MY_ARM: QuestId = QuestId("making_friends_with_my_arm");

pub const THE_FORSAKEN_TOWER: QuestId = QuestId("the_forsaken_tower");

pub const THE_ASCENT_OF_ARCEUUS: QuestId = QuestId("the_ascent_of_arceuus");

pub const X_MARKS_THE_SPOT: QuestId = QuestId("x_marks_the_spot");

pub const SONG_OF_THE_ELVES: QuestId = QuestId("song_of_the_elves");

pub const THE_FREMENNIK_EXILES: QuestId = QuestId("the_fremennik_exiles");

// Miniquests

pub const ALFRED_GRIMHANDS_BARCRAWL: QuestId = QuestId("alfred_grimhands_barcrawl");

pub const ARCHITECTURAL_ALLIANCE: QuestId = QuestId("architectural_alliance");

pub const BEAR_YOUR_SOUL: QuestId = QuestId("bear_your_soul");

pub const CURSE_OF_THE_EMPTY_LORD: QuestId = QuestId("curse_of_the_empty_lord");

pub const ENCHANTED_KEY: QuestId = QuestId("enchanted_key");

pub const ENTER_THE_ABYSS: QuestId = QuestId("enter_the_abyss");

pub const FAMILY_PEST: QuestId = QuestId("family_pest");

pub const THE_GENERALS_SHADOW: QuestId = QuestId("the_generals_shadow");

pub const IN_SEARCH_OF_KNOWLEDGE: QuestId = QuestId("in_search_of_knowledge");

pub const LAIR_OF_TARN_RAZORLOR: QuestId = QuestId("lair_of_tarn_razorlor");

pub const THE_MAGE_ARENA: QuestId = QuestId("the_mage_arena");

pub const THE_MAGE_ARENA_II: QuestId = QuestId("the_mage_arena_ii");

pub const SKIPPY_AND_THE_MOGRES: QuestId = QuestId("skippy_and_the_mogres");

fn all_quests() -> Vec<Quest> {
    use Skill::*;

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
                SkillRequirement::builder(Level(10), Mining).build(),
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
                    SkillRequirement::builder(Level(8), Crafting)
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
                    SkillRequirement::builder(Level(31), Crafting)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(36), Woodcutting).build()),
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
                    SkillRequirement::builder(Level(53), Cooking)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(53), Fishing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Herblore)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Mining)
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
                    SkillRequirement::builder(Level(31), Prayer)
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
                    SkillRequirement::builder(Level(40), Mining)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Smithing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(59), Magic)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Crafting)
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
                SkillRequirement::builder(Level(21), Thieving).build(),
            )],
        },
        Quest {
            id: FISHING_CONTEST,
            name: "Fishing Contest".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(10), Fishing).build(),
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
                    SkillRequirement::builder(Level(42), Thieving)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Ranged).build()),
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Attack).build()),
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
                SkillRequirement::builder(Level(30), Firemaking).build(),
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
                SkillRequirement::builder(Level(25), Agility)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Crafting).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(32), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(4), Smithing)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Ranged).build()),
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Fletching).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Smithing).build()),
            ],
        },
        Quest {
            id: WATCHTOWER,
            name: "Watchtower".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Magic).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Thieving).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(14), Herblore).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Mining)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Herblore).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Thieving).build()),
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
                    SkillRequirement::builder(Level(50), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Crafting).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Herblore)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(56), Magic).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(52), Mining)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(42), Prayer).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Smithing).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Strength)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Thieving).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Woodcutting)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(5), Fletching).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Cooking)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Ranged).build()),
            ],
        },
        Quest {
            id: ELEMENTAL_WORKSHOP_I,
            name: "Elemental Workshop I".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Mining).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Smithing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Crafting).build()),
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
                    SkillRequirement::builder(Level(18), Crafting)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Agility).build()),
            ],
        },
        Quest {
            id: TAI_BWO_WANNAI_TRIO,
            name: "Tai Bwo Wannai Trio".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Cooking).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(5), Fishing).build()),
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
                    SkillRequirement::builder(Level(56), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Crafting).build()),
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
                    SkillRequirement::builder(Level(31), Herblore)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Herblore).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(5), Firemaking).build()),
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
                    SkillRequirement::builder(Level(35), Agility)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(35), Crafting).build()),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
            ],
        },
        Quest {
            id: TROLL_ROMANCE,
            name: "Troll Romance".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(28), Agility).build()),
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
                    SkillRequirement::builder(Level(25), Agility)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Thieving).build()),
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
                    SkillRequirement::builder(Level(56), Agility)
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
                    SkillRequirement::builder(Level(25), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(20), Cooking)
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
                    SkillRequirement::builder(Level(36), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Crafting).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(18), Herblore)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Smithing)
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
                SkillRequirement::builder(Level(20), Agility)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Defence).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Mining)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Smithing)
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
                SkillRequirement::builder(Level(30), Thieving).build(),
            )],
        },
        Quest {
            id: THE_GOLEM,
            name: "The Golem".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Crafting).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(25), Thieving)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(53), Thieving).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Firemaking).build()),
                // TODO: Figure out how to represent that the Slayer requirement
                // is optional if you have a Gas mask from Plague City.
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Slayer).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Magic).build()),
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
            name: "Icthlarin's Little Helper".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Quest(GERTRUDES_CAT)],
        },
        Quest {
            id: TEARS_OF_GUTHIX,
            name: "Tears of Guthix".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(43),
                QuestRequirement::Skill(SkillRequirement::builder(Level(49), Firemaking).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Mining).build()),
            ],
        },
        Quest {
            id: ZOGRE_FLESH_EATERS,
            name: "Zogre Flesh Eaters".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(BIG_CHOMPY_BIRD_HUNTING),
                QuestRequirement::Quest(JUNGLE_POTION),
                QuestRequirement::Skill(SkillRequirement::builder(Level(4), Smithing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(8), Herblore).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Ranged).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Fletching)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(12), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(13), Thieving).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(17), Mining).build()),
            ],
        },
        Quest {
            id: THE_GIANT_DWARF,
            name: "The Giant Dwarf".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(12), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(16), Firemaking).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(33), Magic).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(14), Thieving).build()),
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Ranged).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Thieving)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(22), Cooking).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(17), Farming).build()),
                QuestRequirement::Quest(THE_GIANT_DWARF),
                QuestRequirement::Quest(FISHING_CONTEST),
            ],
        },
        Quest {
            id: GARDEN_OF_TRANQUILLITY,
            name: "Garden of Tranquillity".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Farming).build()),
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
                    SkillRequirement::builder(Level(42), Crafting)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Fishing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Farming)
                        .boostable()
                        .build(),
                ),
                // TODO: Figure out how to represent 47 Prayer points requirement.
                QuestRequirement::Skill(SkillRequirement::builder(Level(42), Slayer).build()),
            ],
        },
        Quest {
            id: SHADOW_OF_THE_STORM,
            name: "Shadow of the Storm".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Crafting)
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
                    SkillRequirement::builder(Level(33), Magic)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(37), Ranged)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(37), Mining).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(37), Thieving)
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
                    SkillRequirement::builder(Level(65), Smithing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Runecraft).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Fletching).build()),
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(17), Thieving).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(49), Crafting).build()),
            ],
        },
        Quest {
            id: ENAKHRAS_LAMENT,
            name: "Enakhra's Lament".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(45), Firemaking).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(43), Prayer).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(39), Magic).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Mining)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(42), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Smithing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Ranged).build()),
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Mining).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(7), Magic).build()),
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Slayer).build()),
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
                    SkillRequirement::builder(Level(66), Magic)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(62), Cooking)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(62), Fishing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Smithing)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(42), Firemaking).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Crafting).build()),
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
                    SkillRequirement::builder(Level(40), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Slayer)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(23), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(23), Thieving).build()),
            ],
        },
        Quest {
            id: FAIRY_TALE_II_CURE_A_QUEEN,
            name: "Fairy Tale II - Cure a Queen".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(FAIRY_TALE_I_GROWING_PAINS),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Thieving).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(49), Farming)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(57), Herblore)
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(61), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Defence).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(49), Firemaking).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(65), Magic).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Mining).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(55), Woodcutting).build()),
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
                QuestRequirement::Skill(SkillRequirement::builder(Level(5), Construction).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(46), Magic).build()),
                // TODO: Figure out how to represent requirements to get a maple log.
                QuestRequirement::Quest(THE_GRAND_TREE),
            ],
        },
        Quest {
            id: DARKNESS_OF_HALLOWVALE,
            name: "Darkness of Hallowvale".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(IN_AID_OF_THE_MYREQUE),
                QuestRequirement::Skill(SkillRequirement::builder(Level(5), Construction).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Mining).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(22), Thieving).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(26), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(32), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(33), Magic).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Strength).build()),
            ],
        },
        Quest {
            id: THE_SLUG_MENACE,
            name: "The Slug Menace".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(WANTED),
                QuestRequirement::Quest(SEA_SLUG),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Runecraft).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Slayer).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Thieving).build()),
            ],
        },
        Quest {
            id: ELEMENTAL_WORKSHOP_II,
            name: "Elemental Workshop II".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Magic).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Smithing).build()),
                QuestRequirement::Quest(ELEMENTAL_WORKSHOP_I),
            ],
        },
        Quest {
            id: MY_ARMS_BIG_ADVENTURE,
            name: "My Arm's Big Adventure".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Woodcutting).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(29), Farming)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(EADGARS_RUSE),
                QuestRequirement::Quest(THE_FEUD),
                QuestRequirement::Quest(JUNGLE_POTION),
                // TODO: Figure out how to represent 60% Tai Bwo Wannai Cleanup favor.
            ],
        },
        Quest {
            id: ENLIGHTENED_JOURNEY,
            name: "Enlightened Journey".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(20),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Firemaking).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(30), Farming)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(SkillRequirement::builder(Level(36), Crafting).build()),
            ],
        },
        Quest {
            id: EAGLES_PEAK,
            name: "Eagles' Peak".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(27), Hunter)
                    .boostable()
                    .build(),
            )],
        },
        Quest {
            id: ANIMAL_MAGNETISM,
            name: "Animal Magnetism".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(THE_RESTLESS_GHOST),
                QuestRequirement::Quest(ERNEST_THE_CHICKEN),
                QuestRequirement::Quest(PRIEST_IN_PERIL),
                QuestRequirement::Skill(SkillRequirement::builder(Level(18), Slayer).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(19), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Ranged).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(35), Woodcutting).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(31), Prayer)
                        .ironman_only()
                        .build(),
                ),
            ],
        },
        Quest {
            id: CONTACT,
            name: "Contact!".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(PRINCE_ALI_RESCUE),
                QuestRequirement::Quest(ICTHLARINS_LITTLE_HELPER),
            ],
        },
        Quest {
            id: COLD_WAR,
            name: "Cold War".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Hunter).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(34), Construction).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Thieving).build()),
                // TODO: Figure out how to represent the requirement of having
                // access to a crafting table 3.
            ],
        },
        Quest {
            id: THE_FREMENNIK_ISLES,
            name: "The Fremennik Isles".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(THE_FREMENNIK_TRIALS),
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Construction).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(40), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(56), Woodcutting)
                        .ironman_only()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(46), Crafting)
                        .ironman_only()
                        .build(),
                ),
            ],
        },
        Quest {
            id: TOWER_OF_LIFE,
            name: "Tower of Life".into(),
            kind: QuestKind::Members,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(10), Construction).build(),
            )],
        },
        Quest {
            id: THE_GREAT_BRAIN_ROBBERY,
            name: "The Great Brain Robbery".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(16), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Construction).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Prayer).build()),
                QuestRequirement::Quest(CREATURE_OF_FENKENSTRAIN),
                QuestRequirement::Quest(CABIN_FEVER),
                // TODO: Add Recipe for Disaster: Pirate Pete subquest requirement.
            ],
        },
        Quest {
            id: WHAT_LIES_BELOW,
            name: "What Lies Below".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(RUNE_MYSTERIES),
                QuestRequirement::Skill(SkillRequirement::builder(Level(35), Runecraft).build()),
                // TODO: Figure out how to indicate that the Mining requirement is
                // only applicable if not using the Abyss.
                QuestRequirement::Skill(SkillRequirement::builder(Level(42), Mining).build()),
            ],
        },
        Quest {
            id: OLAFS_QUEST,
            name: "Olaf's Quest".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(THE_FREMENNIK_TRIALS),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Firemaking).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(50), Woodcutting)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: ANOTHER_SLICE_OF_HAM,
            name: "Another Slice of H.A.M.".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Attack).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(25), Prayer).build()),
                QuestRequirement::Quest(DEATH_TO_THE_DORGESHUUN),
                QuestRequirement::Quest(THE_GIANT_DWARF),
                QuestRequirement::Quest(THE_DIG_SITE),
                QuestRequirement::Quest(DRUIDIC_RITUAL),
            ],
        },
        Quest {
            id: DREAM_MENTOR,
            name: "Dream Mentor".into(),
            kind: QuestKind::Members,
            requirements: vec![
                // TODO: Figure out how to represent 85 Combat requirement.
                QuestRequirement::Quest(LUNAR_DIPLOMACY),
                QuestRequirement::Quest(EADGARS_RUSE),
            ],
        },
        Quest {
            id: GRIM_TALES,
            name: "Grim Tales".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(WITCHS_HOUSE),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(45), Farming)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(52), Herblore)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(58), Thieving)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(59), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(71), Woodcutting)
                        .boostable()
                        .build(),
                ),
            ],
        },
        Quest {
            id: KINGS_RANSOM,
            name: "King's Ransom".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(45), Magic).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(65), Defence).build()),
                QuestRequirement::Quest(BLACK_KNIGHTS_FORTRESS),
                QuestRequirement::Quest(HOLY_GRAIL),
                QuestRequirement::Quest(MURDER_MYSTERY),
                QuestRequirement::Quest(ONE_SMALL_FAVOUR),
            ],
        },
        Quest {
            id: MONKEY_MADNESS_II,
            name: "Monkey Madness II".into(),
            kind: QuestKind::Members,
            requirements: vec![
                // TODO: Figure out how to indicate that the Gnome Stronghold
                // Route must be unlocked, requiring 60 Firemaking.
                QuestRequirement::Quest(ENLIGHTENED_JOURNEY),
                QuestRequirement::Quest(THE_EYES_OF_GLOUPHRIE),
                // TODO: Add requirement for Recipe for Disaster: King Awowogei
                // subquest.
                QuestRequirement::Quest(TROLL_STRONGHOLD),
                QuestRequirement::Quest(WATCHTOWER),
                QuestRequirement::Skill(SkillRequirement::builder(Level(69), Slayer).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Hunter).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(55), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(55), Thieving).build()),
            ],
        },
        Quest {
            id: MISTHALIN_MYSTERY,
            name: "Misthalin Mystery".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: CLIENT_OF_KOUREND,
            name: "Client of Kourend".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: BONE_VOYAGE,
            name: "Bone Voyage".into(),
            kind: QuestKind::Members,
            requirements: vec![
                // TODO: Figure out how to represent 100 Kudos requirement.
                QuestRequirement::Quest(THE_DIG_SITE),
            ],
        },
        Quest {
            id: THE_QUEEN_OF_THIEVES,
            name: "The Queen of Thieves".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(20), Thieving).build()),
                QuestRequirement::Quest(CLIENT_OF_KOUREND),
                // TODO: Figure out how to represent 20% Port Piscarilius favor requirement.
            ],
        },
        Quest {
            id: THE_DEPTHS_OF_DESPAIR,
            name: "The Depths of Despair".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(CLIENT_OF_KOUREND),
                QuestRequirement::Skill(SkillRequirement::builder(Level(18), Agility).build()),
                // TODO: Figure out how to represent 20% Hosidius favor requirement.
            ],
        },
        Quest {
            id: THE_CORSAIR_CURSE,
            name: "The Corsair Curse".into(),
            kind: QuestKind::Members,
            requirements: vec![],
        },
        Quest {
            id: DRAGON_SLAYER_II,
            name: "Dragon Slayer II".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::QuestPoints(200),
                QuestRequirement::Quest(LEGENDS_QUEST),
                QuestRequirement::Quest(DREAM_MENTOR),
                QuestRequirement::Quest(A_TAIL_OF_TWO_CATS),
                QuestRequirement::Quest(ANIMAL_MAGNETISM),
                QuestRequirement::Quest(GHOSTS_AHOY),
                QuestRequirement::Quest(BONE_VOYAGE),
                QuestRequirement::Quest(CLIENT_OF_KOUREND),
                QuestRequirement::Skill(SkillRequirement::builder(Level(75), Magic).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Smithing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(68), Mining).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(62), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Thieving).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Construction).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(50), Hitpoints).build()),
                // TODO: Figure out how to represent the requirement of starting
                // the Firemaking part of Barbarian Training.
            ],
        },
        Quest {
            id: TALE_OF_THE_RIGHTEOUS,
            name: "Tale of the Righteous".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(16), Strength).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(10), Mining).build()),
                QuestRequirement::Quest(CLIENT_OF_KOUREND),
                // TODO: Figure out how to represent 20% Shayzien favor requirement.
            ],
        },
        Quest {
            id: A_TASTE_OF_HOPE,
            name: "A Taste of Hope".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(DARKNESS_OF_HALLOWVALE),
                QuestRequirement::Skill(SkillRequirement::builder(Level(48), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(45), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Attack).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Herblore).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(38), Slayer).build()),
            ],
        },
        Quest {
            id: MAKING_FRIENDS_WITH_MY_ARM,
            name: "Making Friends with My Arm".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(66), Firemaking).build()),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(72), Mining)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(35), Construction)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Skill(
                    SkillRequirement::builder(Level(68), Agility)
                        .boostable()
                        .build(),
                ),
                QuestRequirement::Quest(MY_ARMS_BIG_ADVENTURE),
                QuestRequirement::Quest(SWAN_SONG),
                QuestRequirement::Quest(COLD_WAR),
                QuestRequirement::Quest(ROMEO_AND_JULIET),
            ],
        },
        Quest {
            id: THE_FORSAKEN_TOWER,
            name: "The Forsaken Tower".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(CLIENT_OF_KOUREND),
                // TODO: Figure out how to represent 20% Lovakengj favor requirement.
            ],
        },
        Quest {
            id: THE_ASCENT_OF_ARCEUUS,
            name: "The Ascent of Arceuus".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(12), Hunter).build()),
                QuestRequirement::Quest(CLIENT_OF_KOUREND),
                // TODO: Figure out how to represent 20% Arceuus favor requirement.
            ],
        },
        Quest {
            id: X_MARKS_THE_SPOT,
            name: "X Marks the Spot".into(),
            kind: QuestKind::Free,
            requirements: vec![],
        },
        Quest {
            id: SONG_OF_THE_ELVES,
            name: "Song of the Elves".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Quest(MOURNINGS_END_PART_II),
                QuestRequirement::Quest(MAKING_HISTORY),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Agility).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Construction).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Farming).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Herblore).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Hunter).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Mining).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Smithing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(70), Woodcutting).build()),
            ],
        },
        Quest {
            id: THE_FREMENNIK_EXILES,
            name: "The Fremennik Exiles".into(),
            kind: QuestKind::Members,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(65), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Slayer).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Smithing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(60), Fishing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(55), Runecraft).build()),
                QuestRequirement::Quest(THE_FREMENNIK_ISLES),
                QuestRequirement::Quest(LUNAR_DIPLOMACY),
                QuestRequirement::Quest(MOUNTAIN_DAUGHTER),
                QuestRequirement::Quest(HEROES_QUEST),
            ],
        },
        // Miniquests
        Quest {
            id: ALFRED_GRIMHANDS_BARCRAWL,
            name: "Alfred Grimhand's Barcrawl".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![],
        },
        Quest {
            id: ARCHITECTURAL_ALLIANCE,
            name: "Architectural Alliance".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(42), Mining).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(45), Smithing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Fishing).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(30), Crafting).build()),
                QuestRequirement::Skill(SkillRequirement::builder(Level(15), Hunter).build()),
            ],
        },
        Quest {
            id: BEAR_YOUR_SOUL,
            name: "Bear Your Soul".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![],
        },
        Quest {
            id: CURSE_OF_THE_EMPTY_LORD,
            name: "Curse of the Empty Lord".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![
                // TODO: Figure out how to represent started Desert Treasure requirement.
            ],
        },
        Quest {
            id: ENCHANTED_KEY,
            name: "Enchanted Key".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![QuestRequirement::Quest(MAKING_HISTORY)],
        },
        Quest {
            id: ENTER_THE_ABYSS,
            name: "Enter the Abyss".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![QuestRequirement::Quest(RUNE_MYSTERIES)],
        },
        Quest {
            id: FAMILY_PEST,
            name: "Family Pest".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![QuestRequirement::Quest(FAMILY_CREST)],
        },
        Quest {
            id: THE_GENERALS_SHADOW,
            name: "The General's Shadow".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![
                QuestRequirement::Quest(FIGHT_ARENA),
                QuestRequirement::Quest(CURSE_OF_THE_EMPTY_LORD),
            ],
        },
        Quest {
            id: IN_SEARCH_OF_KNOWLEDGE,
            name: "In Search of Knowledge".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![],
        },
        Quest {
            id: LAIR_OF_TARN_RAZORLOR,
            name: "Lair of Tarn Razorlor".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![
                QuestRequirement::Quest(HAUNTED_MINE),
                QuestRequirement::Skill(SkillRequirement::builder(Level(40), Slayer).build()),
            ],
        },
        Quest {
            id: THE_MAGE_ARENA,
            name: "The Mage Arena".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![QuestRequirement::Skill(
                SkillRequirement::builder(Level(60), Magic).build(),
            )],
        },
        Quest {
            id: THE_MAGE_ARENA_II,
            name: "The Mage Arena II".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![
                QuestRequirement::Skill(SkillRequirement::builder(Level(75), Magic).build()),
                // TODO: Figure out how to represent the requirement to have unlocked all god spells.
            ],
        },
        Quest {
            id: SKIPPY_AND_THE_MOGRES,
            name: "Skippy and the Mogres".into(),
            kind: QuestKind::Miniquest,
            requirements: vec![],
        },
    ]
}
