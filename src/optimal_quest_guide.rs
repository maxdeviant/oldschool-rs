use crate::Level;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct QuestId(u32);

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
pub enum QuestRequirement {
    Skill { skill: Skill, level: Level },
    Quest(QuestId),
}

pub mod quests {
    use super::QuestId;

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

    pub const WITCHES_HOUSE: QuestId = QuestId(20);

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
}
