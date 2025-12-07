use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

use indexmap::IndexSet;
use thiserror::Error;

use crate::{Level, Quest, QuestId, QuestRequirement, QuestReward, Skill, Xp};

impl QuestId {
    // Free-to-play quests

    /// <https://oldschool.runescape.wiki/w/Below_Ice_Mountain>
    pub const BELOW_ICE_MOUNTAIN: Self = Self::new_static("below_ice_mountain");

    /// <https://oldschool.runescape.wiki/w/Black_Knights%27_Fortress>
    pub const BLACK_KNIGHTS_FORTRESS: Self = Self::new_static("black_knights_fortress");

    /// <https://oldschool.runescape.wiki/w/Cook%27s_Assistant>
    pub const COOKS_ASSISTANT: Self = Self::new_static("cooks_assistant");

    /// <https://oldschool.runescape.wiki/w/The_Corsair_Curse>
    pub const THE_CORSAIR_CURSE: Self = Self::new_static("the_corsair_curse");

    /// <https://oldschool.runescape.wiki/w/Demon_Slayer>
    pub const DEMON_SLAYER: Self = Self::new_static("demon_slayer");

    /// <https://oldschool.runescape.wiki/w/Doric%27s_Quest>
    pub const DORICS_QUEST: Self = Self::new_static("dorics_quest");

    /// <https://oldschool.runescape.wiki/w/Dragon_Slayer_I>
    pub const DRAGON_SLAYER_I: Self = Self::new_static("dragon_slayer_i");

    /// <https://oldschool.runescape.wiki/w/Ernest_the_Chicken>
    pub const ERNEST_THE_CHICKEN: Self = Self::new_static("ernest_the_chicken");

    /// <https://oldschool.runescape.wiki/w/Goblin_Diplomacy>
    pub const GOBLIN_DIPLOMACY: Self = Self::new_static("goblin_diplomacy");

    /// <https://oldschool.runescape.wiki/w/Imp_Catcher>
    pub const IMP_CATCHER: Self = Self::new_static("imp_catcher");

    /// <https://oldschool.runescape.wiki/w/The_Knight%27s_Sword>
    pub const THE_KNIGHTS_SWORD: Self = Self::new_static("the_knights_sword");

    /// <https://oldschool.runescape.wiki/w/Misthalin_Mystery>
    pub const MISTHALIN_MYSTERY: Self = Self::new_static("misthalin_mystery");

    /// <https://oldschool.runescape.wiki/w/Pirate%27s_Treasure>
    pub const PIRATES_TREASURE: Self = Self::new_static("pirates_treasure");

    /// <https://oldschool.runescape.wiki/w/Prince_Ali_Rescue>
    pub const PRINCE_ALI_RESCUE: Self = Self::new_static("prince_ali_rescue");

    /// <https://oldschool.runescape.wiki/w/The_Restless_Ghost>
    pub const THE_RESTLESS_GHOST: Self = Self::new_static("the_restless_ghost");

    /// <https://oldschool.runescape.wiki/w/Romeo_%26_Juliet>
    pub const ROMEO_AND_JULIET: Self = Self::new_static("romeo_and_juliet");

    /// <https://oldschool.runescape.wiki/w/Rune_Mysteries>
    pub const RUNE_MYSTERIES: Self = Self::new_static("rune_mysteries");

    /// <https://oldschool.runescape.wiki/w/Sheep_Shearer>
    pub const SHEEP_SHEARER: Self = Self::new_static("sheep_shearer");

    /// <https://oldschool.runescape.wiki/w/Shield_of_Arrav>
    pub const SHIELD_OF_ARRAV: Self = Self::new_static("shield_of_arrav");

    /// <https://oldschool.runescape.wiki/w/Vampyre_Slayer>
    pub const VAMPYRE_SLAYER: Self = Self::new_static("vampyre_slayer");

    /// <https://oldschool.runescape.wiki/w/Witch%27s_Potion>
    pub const WITCHS_POTION: Self = Self::new_static("witchs_potion");

    /// <https://oldschool.runescape.wiki/w/X_Marks_the_Spot>
    pub const X_MARKS_THE_SPOT: Self = Self::new_static("x_marks_the_spot");

    // Members quests

    /// <https://oldschool.runescape.wiki/w/Animal_Magnetism>
    pub const ANIMAL_MAGNETISM: Self = Self::new_static("animal_magnetism");

    /// <https://oldschool.runescape.wiki/w/Another_Slice_of_H.A.M.>
    pub const ANOTHER_SLICE_OF_HAM: Self = Self::new_static("another_slice_of_ham");

    /// <https://oldschool.runescape.wiki/w/The_Ascent_of_Arceuus>
    pub const THE_ASCENT_OF_ARCEUUS: Self = Self::new_static("the_ascent_of_arceuus");

    /// <https://oldschool.runescape.wiki/w/At_First_Light>
    pub const AT_FIRST_LIGHT: Self = Self::new_static("at_first_light");

    /// <https://oldschool.runescape.wiki/w/Beneath_Cursed_Sands>
    pub const BENEATH_CURSED_SANDS: Self = Self::new_static("beneath_cursed_sands");

    /// <https://oldschool.runescape.wiki/w/Between_a_Rock...>
    pub const BETWEEN_A_ROCK: Self = Self::new_static("between_a_rock");

    /// <https://oldschool.runescape.wiki/w/Big_Chompy_Bird_Hunting>
    pub const BIG_CHOMPY_BIRD_HUNTING: Self = Self::new_static("big_chompy_bird_hunting");

    /// <https://oldschool.runescape.wiki/w/Biohazard>
    pub const BIOHAZARD: Self = Self::new_static("biohazard");

    /// <https://oldschool.runescape.wiki/w/Bone_Voyage>
    pub const BONE_VOYAGE: Self = Self::new_static("bone_voyage");

    /// <https://oldschool.runescape.wiki/w/Cabin_Fever>
    pub const CABIN_FEVER: Self = Self::new_static("cabin_fever");

    /// <https://oldschool.runescape.wiki/w/Children_of_the_Sun>
    pub const CHILDREN_OF_THE_SUN: Self = Self::new_static("children_of_the_sun");

    /// <https://oldschool.runescape.wiki/w/Client_of_Kourend>
    pub const CLIENT_OF_KOUREND: Self = Self::new_static("client_of_kourend");

    /// <https://oldschool.runescape.wiki/w/Clock_Tower>
    pub const CLOCK_TOWER: Self = Self::new_static("clock_tower");

    /// <https://oldschool.runescape.wiki/w/Cold_War>
    pub const COLD_WAR: Self = Self::new_static("cold_war");

    /// <https://oldschool.runescape.wiki/w/Contact!>
    pub const CONTACT: Self = Self::new_static("contact");

    /// <https://oldschool.runescape.wiki/w/Creature_of_Fenkenstrain>
    pub const CREATURE_OF_FENKENSTRAIN: Self = Self::new_static("creature_of_fenkenstrain");

    /// <https://oldschool.runescape.wiki/w/Current_Affairs>
    pub const CURRENT_AFFAIRS: Self = Self::new_static("current_affairs");

    /// <https://oldschool.runescape.wiki/w/The_Curse_of_Arrav>
    pub const THE_CURSE_OF_ARRAV: Self = Self::new_static("the_curse_of_arrav");

    /// <https://oldschool.runescape.wiki/w/Darkness_of_Hallowvale>
    pub const DARKNESS_OF_HALLOWVALE: Self = Self::new_static("darkness_of_hallowvale");

    /// <https://oldschool.runescape.wiki/w/Death_on_the_Isle>
    pub const DEATH_ON_THE_ISLE: Self = Self::new_static("death_on_the_isle");

    /// <https://oldschool.runescape.wiki/w/Death_Plateau>
    pub const DEATH_PLATEAU: Self = Self::new_static("death_plateau");

    /// <https://oldschool.runescape.wiki/w/Death_to_the_Dorgeshuun>
    pub const DEATH_TO_THE_DORGESHUUN: Self = Self::new_static("death_to_the_dorgeshuun");

    /// <https://oldschool.runescape.wiki/w/Defender_of_Varrock>
    pub const DEFENDER_OF_VARROCK: Self = Self::new_static("defender_of_varrock");

    /// <https://oldschool.runescape.wiki/w/The_Depths_of_Despair>
    pub const THE_DEPTHS_OF_DESPAIR: Self = Self::new_static("the_depths_of_despair");

    /// <https://oldschool.runescape.wiki/w/Desert_Treasure_I>
    pub const DESERT_TREASURE_I: Self = Self::new_static("desert_treasure_i");

    /// <https://oldschool.runescape.wiki/w/Desert_Treasure_II_-_The_Fallen_Empire>
    pub const DESERT_TREASURE_II: Self = Self::new_static("desert_treasure_ii");

    /// <https://oldschool.runescape.wiki/w/Devious_Minds>
    pub const DEVIOUS_MINDS: Self = Self::new_static("devious_minds");

    /// <https://oldschool.runescape.wiki/w/The_Dig_Site>
    pub const THE_DIG_SITE: Self = Self::new_static("the_dig_site");

    /// <https://oldschool.runescape.wiki/w/Dragon_Slayer_II>
    pub const DRAGON_SLAYER_II: Self = Self::new_static("dragon_slayer_ii");

    /// <https://oldschool.runescape.wiki/w/Dream_Mentor>
    pub const DREAM_MENTOR: Self = Self::new_static("dream_mentor");

    /// <https://oldschool.runescape.wiki/w/Druidic_Ritual>
    pub const DRUIDIC_RITUAL: Self = Self::new_static("druidic_ritual");

    /// <https://oldschool.runescape.wiki/w/Dwarf_Cannon>
    pub const DWARF_CANNON: Self = Self::new_static("dwarf_cannon");

    /// <https://oldschool.runescape.wiki/w/Eadgar%27s_Ruse>
    pub const EADGARS_RUSE: Self = Self::new_static("eadgars_ruse");

    /// <https://oldschool.runescape.wiki/w/Eagles%27_Peak>
    pub const EAGLES_PEAK: Self = Self::new_static("eagles_peak");

    /// <https://oldschool.runescape.wiki/w/Elemental_Workshop_I>
    pub const ELEMENTAL_WORKSHOP_I: Self = Self::new_static("elemental_workshop_i");

    /// <https://oldschool.runescape.wiki/w/Elemental_Workshop_II>
    pub const ELEMENTAL_WORKSHOP_II: Self = Self::new_static("elemental_workshop_ii");

    /// <https://oldschool.runescape.wiki/w/Enakhra%27s_Lament>
    pub const ENAKHRAS_LAMENT: Self = Self::new_static("enakhras_lament");

    /// <https://oldschool.runescape.wiki/w/Enlightened_Journey>
    pub const ENLIGHTENED_JOURNEY: Self = Self::new_static("enlightened_journey");

    /// <https://oldschool.runescape.wiki/w/Ethically_Acquired_Antiquities>
    pub const ETHICALLY_ACQUIRED_ANTIQUITIES: Self =
        Self::new_static("ethically_acquired_antiquities");

    /// <https://oldschool.runescape.wiki/w/The_Eyes_of_Glouphrie>
    pub const THE_EYES_OF_GLOUPHRIE: Self = Self::new_static("the_eyes_of_glouphrie");

    /// <https://oldschool.runescape.wiki/w/Fairytale_I_-_Growing_Pains>
    pub const FAIRYTALE_I: Self = Self::new_static("fairytale_i");

    /// <https://oldschool.runescape.wiki/w/Fairytale_II_-_Cure_a_Queen>
    pub const FAIRYTALE_II: Self = Self::new_static("fairytale_ii");

    /// <https://oldschool.runescape.wiki/w/Family_Crest>
    pub const FAMILY_CREST: Self = Self::new_static("family_crest");

    /// <https://oldschool.runescape.wiki/w/The_Feud>
    pub const THE_FEUD: Self = Self::new_static("the_feud");

    /// <https://oldschool.runescape.wiki/w/Fight_Arena>
    pub const FIGHT_ARENA: Self = Self::new_static("fight_arena");

    /// <https://oldschool.runescape.wiki/w/The_Final_Dawn>
    pub const THE_FINAL_DAWN: Self = Self::new_static("the_final_dawn");

    /// <https://oldschool.runescape.wiki/w/Fishing_Contest>
    pub const FISHING_CONTEST: Self = Self::new_static("fishing_contest");

    /// <https://oldschool.runescape.wiki/w/Forgettable_Tale_of_a_Drunken_Dwarf>
    pub const FORGETTABLE_TALE: Self = Self::new_static("forgettable_tale");

    /// <https://oldschool.runescape.wiki/w/The_Forsaken_Tower>
    pub const THE_FORSAKEN_TOWER: Self = Self::new_static("the_forsaken_tower");

    /// <https://oldschool.runescape.wiki/w/The_Fremennik_Exiles>
    pub const THE_FREMENNIK_EXILES: Self = Self::new_static("the_fremennik_exiles");

    /// <https://oldschool.runescape.wiki/w/The_Fremennik_Isles>
    pub const THE_FREMENNIK_ISLES: Self = Self::new_static("the_fremennik_isles");

    /// <https://oldschool.runescape.wiki/w/The_Fremennik_Trials>
    pub const THE_FREMENNIK_TRIALS: Self = Self::new_static("the_fremennik_trials");

    /// <https://oldschool.runescape.wiki/w/The_Garden_of_Death>
    pub const THE_GARDEN_OF_DEATH: Self = Self::new_static("the_garden_of_death");

    /// <https://oldschool.runescape.wiki/w/Garden_of_Tranquillity>
    pub const GARDEN_OF_TRANQUILLITY: Self = Self::new_static("garden_of_tranquillity");

    /// <https://oldschool.runescape.wiki/w/Gertrude%27s_Cat>
    pub const GERTRUDES_CAT: Self = Self::new_static("gertrudes_cat");

    /// <https://oldschool.runescape.wiki/w/Getting_Ahead>
    pub const GETTING_AHEAD: Self = Self::new_static("getting_ahead");

    /// <https://oldschool.runescape.wiki/w/Ghosts_Ahoy>
    pub const GHOSTS_AHOY: Self = Self::new_static("ghosts_ahoy");

    /// <https://oldschool.runescape.wiki/w/The_Giant_Dwarf>
    pub const THE_GIANT_DWARF: Self = Self::new_static("the_giant_dwarf");

    /// <https://oldschool.runescape.wiki/w/The_Golem>
    pub const THE_GOLEM: Self = Self::new_static("the_golem");

    /// <https://oldschool.runescape.wiki/w/The_Grand_Tree>
    pub const THE_GRAND_TREE: Self = Self::new_static("the_grand_tree");

    /// <https://oldschool.runescape.wiki/w/The_Great_Brain_Robbery>
    pub const THE_GREAT_BRAIN_ROBBERY: Self = Self::new_static("the_great_brain_robbery");

    /// <https://oldschool.runescape.wiki/w/Grim_Tales>
    pub const GRIM_TALES: Self = Self::new_static("grim_tales");

    /// <https://oldschool.runescape.wiki/w/The_Hand_in_the_Sand>
    pub const THE_HAND_IN_THE_SAND: Self = Self::new_static("the_hand_in_the_sand");

    /// <https://oldschool.runescape.wiki/w/Haunted_Mine>
    pub const HAUNTED_MINE: Self = Self::new_static("haunted_mine");

    /// <https://oldschool.runescape.wiki/w/Hazeel_Cult>
    pub const HAZEEL_CULT: Self = Self::new_static("hazeel_cult");

    /// <https://oldschool.runescape.wiki/w/The_Heart_of_Darkness>
    pub const THE_HEART_OF_DARKNESS: Self = Self::new_static("the_heart_of_darkness");

    /// <https://oldschool.runescape.wiki/w/Heroes%27_Quest>
    pub const HEROES_QUEST: Self = Self::new_static("heroes_quest");

    /// <https://oldschool.runescape.wiki/w/Holy_Grail>
    pub const HOLY_GRAIL: Self = Self::new_static("holy_grail");

    /// <https://oldschool.runescape.wiki/w/Horror_from_the_Deep>
    pub const HORROR_FROM_THE_DEEP: Self = Self::new_static("horror_from_the_deep");

    /// <https://oldschool.runescape.wiki/w/Icthlarin%27s_Little_Helper>
    pub const ICTHLARINS_LITTLE_HELPER: Self = Self::new_static("icthlarins_little_helper");

    /// <https://oldschool.runescape.wiki/w/In_Aid_of_the_Myreque>
    pub const IN_AID_OF_THE_MYREQUE: Self = Self::new_static("in_aid_of_the_myreque");

    /// <https://oldschool.runescape.wiki/w/In_Search_of_the_Myreque>
    pub const IN_SEARCH_OF_THE_MYREQUE: Self = Self::new_static("in_search_of_the_myreque");

    /// <https://oldschool.runescape.wiki/w/Jungle_Potion>
    pub const JUNGLE_POTION: Self = Self::new_static("jungle_potion");

    /// <https://oldschool.runescape.wiki/w/King%27s_Ransom>
    pub const KINGS_RANSOM: Self = Self::new_static("kings_ransom");

    /// <https://oldschool.runescape.wiki/w/A_Kingdom_Divided>
    pub const A_KINGDOM_DIVIDED: Self = Self::new_static("a_kingdom_divided");

    /// <https://oldschool.runescape.wiki/w/Land_of_the_Goblins>
    pub const LAND_OF_THE_GOBLINS: Self = Self::new_static("land_of_the_goblins");

    /// <https://oldschool.runescape.wiki/w/Legends%27_Quest>
    pub const LEGENDS_QUEST: Self = Self::new_static("legends_quest");

    /// <https://oldschool.runescape.wiki/w/Lost_City>
    pub const LOST_CITY: Self = Self::new_static("lost_city");

    /// <https://oldschool.runescape.wiki/w/The_Lost_Tribe>
    pub const THE_LOST_TRIBE: Self = Self::new_static("the_lost_tribe");

    /// <https://oldschool.runescape.wiki/w/Lunar_Diplomacy>
    pub const LUNAR_DIPLOMACY: Self = Self::new_static("lunar_diplomacy");

    /// <https://oldschool.runescape.wiki/w/Making_Friends_with_My_Arm>
    pub const MAKING_FRIENDS_WITH_MY_ARM: Self = Self::new_static("making_friends_with_my_arm");

    /// <https://oldschool.runescape.wiki/w/Making_History>
    pub const MAKING_HISTORY: Self = Self::new_static("making_history");

    /// <https://oldschool.runescape.wiki/w/Meat_and_Greet>
    pub const MEAT_AND_GREET: Self = Self::new_static("meat_and_greet");

    /// <https://oldschool.runescape.wiki/w/Merlin%27s_Crystal>
    pub const MERLINS_CRYSTAL: Self = Self::new_static("merlins_crystal");

    /// <https://oldschool.runescape.wiki/w/Monk%27s_Friend>
    pub const MONKS_FRIEND: Self = Self::new_static("monks_friend");

    /// <https://oldschool.runescape.wiki/w/Monkey_Madness_I>
    pub const MONKEY_MADNESS_I: Self = Self::new_static("monkey_madness_i");

    /// <https://oldschool.runescape.wiki/w/Monkey_Madness_II>
    pub const MONKEY_MADNESS_II: Self = Self::new_static("monkey_madness_ii");

    /// <https://oldschool.runescape.wiki/w/Mountain_Daughter>
    pub const MOUNTAIN_DAUGHTER: Self = Self::new_static("mountain_daughter");

    /// <https://oldschool.runescape.wiki/w/Mourning%27s_End_Part_I>
    pub const MOURNINGS_END_PART_I: Self = Self::new_static("mournings_end_part_i");

    /// <https://oldschool.runescape.wiki/w/Mourning%27s_End_Part_II>
    pub const MOURNINGS_END_PART_II: Self = Self::new_static("mournings_end_part_ii");

    /// <https://oldschool.runescape.wiki/w/Murder_Mystery>
    pub const MURDER_MYSTERY: Self = Self::new_static("murder_mystery");

    /// <https://oldschool.runescape.wiki/w/My_Arm%27s_Big_Adventure>
    pub const MY_ARMS_BIG_ADVENTURE: Self = Self::new_static("my_arms_big_adventure");

    /// <https://oldschool.runescape.wiki/w/Nature_Spirit>
    pub const NATURE_SPIRIT: Self = Self::new_static("nature_spirit");

    /// <https://oldschool.runescape.wiki/w/A_Night_at_the_Theatre>
    pub const A_NIGHT_AT_THE_THEATRE: Self = Self::new_static("a_night_at_the_theatre");

    /// <https://oldschool.runescape.wiki/w/Observatory_Quest>
    pub const OBSERVATORY_QUEST: Self = Self::new_static("observatory_quest");

    /// <https://oldschool.runescape.wiki/w/Olaf%27s_Quest>
    pub const OLAFS_QUEST: Self = Self::new_static("olafs_quest");

    /// <https://oldschool.runescape.wiki/w/One_Small_Favour>
    pub const ONE_SMALL_FAVOUR: Self = Self::new_static("one_small_favour");

    /// <https://oldschool.runescape.wiki/w/Pandemonium>
    pub const PANDEMONIUM: Self = Self::new_static("pandemonium");

    /// <https://oldschool.runescape.wiki/w/The_Path_of_Glouphrie>
    pub const THE_PATH_OF_GLOUPHRIE: Self = Self::new_static("the_path_of_glouphrie");

    /// <https://oldschool.runescape.wiki/w/Perilous_Moons>
    pub const PERILOUS_MOONS: Self = Self::new_static("perilous_moons");

    /// <https://oldschool.runescape.wiki/w/Plague_City>
    pub const PLAGUE_CITY: Self = Self::new_static("plague_city");

    /// <https://oldschool.runescape.wiki/w/A_Porcine_of_Interest>
    pub const A_PORCINE_OF_INTEREST: Self = Self::new_static("a_porcine_of_interest");

    /// <https://oldschool.runescape.wiki/w/Priest_in_Peril>
    pub const PRIEST_IN_PERIL: Self = Self::new_static("priest_in_peril");

    /// <https://oldschool.runescape.wiki/w/Prying_Times>
    pub const PRYING_TIMES: Self = Self::new_static("prying_times");

    /// <https://oldschool.runescape.wiki/w/The_Queen_of_Thieves>
    pub const THE_QUEEN_OF_THIEVES: Self = Self::new_static("the_queen_of_thieves");

    /// <https://oldschool.runescape.wiki/w/Rag_and_Bone_Man_I>
    pub const RAG_AND_BONE_MAN_I: Self = Self::new_static("rag_and_bone_man_i");

    /// <https://oldschool.runescape.wiki/w/Rag_and_Bone_Man_II>
    pub const RAG_AND_BONE_MAN_II: Self = Self::new_static("rag_and_bone_man_ii");

    /// <https://oldschool.runescape.wiki/w/Ratcatchers>
    pub const RATCATCHERS: Self = Self::new_static("ratcatchers");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster>
    pub const RECIPE_FOR_DISASTER: Self = Self::new_static("recipe_for_disaster");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Another_Cook%27s_Quest>
    pub const RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST: Self =
        Self::new_static("recipe_for_disaster_another_cooks_quest");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_the_Mountain_Dwarf>
    pub const RECIPE_FOR_DISASTER_FREEING_THE_MOUNTAIN_DWARF: Self =
        Self::new_static("recipe_for_disaster_freeing_the_mountain_dwarf");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_the_Goblin_generals>
    pub const RECIPE_FOR_DISASTER_FREEING_THE_GOBLIN_GENERALS: Self =
        Self::new_static("recipe_for_disaster_freeing_the_goblin_generals");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Pirate_Pete>
    pub const RECIPE_FOR_DISASTER_FREEING_PIRATE_PETE: Self =
        Self::new_static("recipe_for_disaster_freeing_pirate_pete");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_the_Lumbridge_Guide>
    pub const RECIPE_FOR_DISASTER_FREEING_THE_LUMBRIDGE_GUIDE: Self =
        Self::new_static("recipe_for_disaster_freeing_the_lumbridge_guide");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Evil_Dave>
    pub const RECIPE_FOR_DISASTER_FREEING_EVIL_DAVE: Self =
        Self::new_static("recipe_for_disaster_freeing_evil_dave");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Skrach_Uglogwee>
    pub const RECIPE_FOR_DISASTER_FREEING_SKRACH_UGLOGWEE: Self =
        Self::new_static("recipe_for_disaster_freeing_skrach_uglogwee");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Sir_Amik_Varze>
    pub const RECIPE_FOR_DISASTER_FREEING_SIR_AMIK_VARZE: Self =
        Self::new_static("recipe_for_disaster_freeing_sir_amik_varze");

    /// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_King_Awowogei>
    pub const RECIPE_FOR_DISASTER_FREEING_KING_AWOWOGEI: Self =
        Self::new_static("recipe_for_disaster_freeing_king_awowogei");

    /// <https://oldschool.runescape.wiki/w/Recruitment_Drive>
    pub const RECRUITMENT_DRIVE: Self = Self::new_static("recruitment_drive");

    /// <https://oldschool.runescape.wiki/w/Regicide>
    pub const REGICIDE: Self = Self::new_static("regicide");

    /// <https://oldschool.runescape.wiki/w/The_Ribbiting_Tale_of_a_Lily_Pad_Labour_Dispute>
    pub const THE_RIBBITING_TALE_OF_A_LILY_PAD_LABOUR_DISPUTE: Self =
        Self::new_static("the_ribbiting_tale_of_a_lily_pad_labour_dispute");

    /// <https://oldschool.runescape.wiki/w/Roving_Elves>
    pub const ROVING_ELVES: Self = Self::new_static("roving_elves");

    /// <https://oldschool.runescape.wiki/w/Royal_Trouble>
    pub const ROYAL_TROUBLE: Self = Self::new_static("royal_trouble");

    /// <https://oldschool.runescape.wiki/w/Rum_Deal>
    pub const RUM_DEAL: Self = Self::new_static("rum_deal");

    /// <https://oldschool.runescape.wiki/w/Scorpion_Catcher>
    pub const SCORPION_CATCHER: Self = Self::new_static("scorpion_catcher");

    /// <https://oldschool.runescape.wiki/w/Scrambled!>
    pub const SCRAMBLED: Self = Self::new_static("scrambled");

    /// <https://oldschool.runescape.wiki/w/Sea_Slug>
    pub const SEA_SLUG: Self = Self::new_static("sea_slug");

    /// <https://oldschool.runescape.wiki/w/Secrets_of_the_North>
    pub const SECRETS_OF_THE_NORTH: Self = Self::new_static("secrets_of_the_north");

    /// <https://oldschool.runescape.wiki/w/Shades_of_Mort%27ton>
    pub const SHADES_OF_MORTTON: Self = Self::new_static("shades_of_mortton");

    /// <https://oldschool.runescape.wiki/w/Shadow_of_the_Storm>
    pub const SHADOW_OF_THE_STORM: Self = Self::new_static("shadow_of_the_storm");

    /// <https://oldschool.runescape.wiki/w/Shadows_of_Custodia>
    pub const SHADOWS_OF_CUSTODIA: Self = Self::new_static("shadows_of_custodia");

    /// <https://oldschool.runescape.wiki/w/Sheep_Herder>
    pub const SHEEP_HERDER: Self = Self::new_static("sheep_herder");

    /// <https://oldschool.runescape.wiki/w/Shilo_Village>
    pub const SHILO_VILLAGE: Self = Self::new_static("shilo_village");

    /// <https://oldschool.runescape.wiki/w/Sins_of_the_Father>
    pub const SINS_OF_THE_FATHER: Self = Self::new_static("sins_of_the_father");

    /// <https://oldschool.runescape.wiki/w/Sleeping_Giants>
    pub const SLEEPING_GIANTS: Self = Self::new_static("sleeping_giants");

    /// <https://oldschool.runescape.wiki/w/The_Slug_Menace>
    pub const THE_SLUG_MENACE: Self = Self::new_static("the_slug_menace");

    /// <https://oldschool.runescape.wiki/w/Song_of_the_Elves>
    pub const SONG_OF_THE_ELVES: Self = Self::new_static("song_of_the_elves");

    /// <https://oldschool.runescape.wiki/w/A_Soul%27s_Bane>
    pub const A_SOULS_BANE: Self = Self::new_static("a_souls_bane");

    /// <https://oldschool.runescape.wiki/w/Spirits_of_the_Elid>
    pub const SPIRITS_OF_THE_ELID: Self = Self::new_static("spirits_of_the_elid");

    /// <https://oldschool.runescape.wiki/w/Swan_Song>
    pub const SWAN_SONG: Self = Self::new_static("swan_song");

    /// <https://oldschool.runescape.wiki/w/Tai_Bwo_Wannai_Trio>
    pub const TAI_BWO_WANNAI_TRIO: Self = Self::new_static("tai_bwo_wannai_trio");

    /// <https://oldschool.runescape.wiki/w/A_Tail_of_Two_Cats>
    pub const A_TAIL_OF_TWO_CATS: Self = Self::new_static("a_tail_of_two_cats");

    /// <https://oldschool.runescape.wiki/w/Tale_of_the_Righteous>
    pub const TALE_OF_THE_RIGHTEOUS: Self = Self::new_static("tale_of_the_righteous");

    /// <https://oldschool.runescape.wiki/w/A_Taste_of_Hope>
    pub const A_TASTE_OF_HOPE: Self = Self::new_static("a_taste_of_hope");

    /// <https://oldschool.runescape.wiki/w/Tears_of_Guthix>
    pub const TEARS_OF_GUTHIX: Self = Self::new_static("tears_of_guthix");

    /// <https://oldschool.runescape.wiki/w/Temple_of_Ikov>
    pub const TEMPLE_OF_IKOV: Self = Self::new_static("temple_of_ikov");

    /// <https://oldschool.runescape.wiki/w/Temple_of_the_Eye>
    pub const TEMPLE_OF_THE_EYE: Self = Self::new_static("temple_of_the_eye");

    /// <https://oldschool.runescape.wiki/w/Throne_of_Miscellania>
    pub const THRONE_OF_MISCELLANIA: Self = Self::new_static("throne_of_miscellania");

    /// <https://oldschool.runescape.wiki/w/The_Tourist_Trap>
    pub const THE_TOURIST_TRAP: Self = Self::new_static("the_tourist_trap");

    /// <https://oldschool.runescape.wiki/w/Tower_of_Life>
    pub const TOWER_OF_LIFE: Self = Self::new_static("tower_of_life");

    /// <https://oldschool.runescape.wiki/w/Tree_Gnome_Village>
    pub const TREE_GNOME_VILLAGE: Self = Self::new_static("tree_gnome_village");

    /// <https://oldschool.runescape.wiki/w/Tribal_Totem>
    pub const TRIBAL_TOTEM: Self = Self::new_static("tribal_totem");

    /// <https://oldschool.runescape.wiki/w/Troll_Romance>
    pub const TROLL_ROMANCE: Self = Self::new_static("troll_romance");

    /// <https://oldschool.runescape.wiki/w/Troll_Stronghold>
    pub const TROLL_STRONGHOLD: Self = Self::new_static("troll_stronghold");

    /// <https://oldschool.runescape.wiki/w/Troubled_Tortugans>
    pub const TROUBLED_TORTUGANS: Self = Self::new_static("troubled_tortugans");

    /// <https://oldschool.runescape.wiki/w/Twilight%27s_Promise>
    pub const TWILIGHTS_PROMISE: Self = Self::new_static("twilights_promise");

    /// <https://oldschool.runescape.wiki/w/Underground_Pass>
    pub const UNDERGROUND_PASS: Self = Self::new_static("underground_pass");

    /// <https://oldschool.runescape.wiki/w/Wanted!>
    pub const WANTED: Self = Self::new_static("wanted");

    /// <https://oldschool.runescape.wiki/w/Watchtower>
    pub const WATCHTOWER: Self = Self::new_static("watchtower");

    /// <https://oldschool.runescape.wiki/w/Waterfall_Quest>
    pub const WATERFALL_QUEST: Self = Self::new_static("waterfall_quest");

    /// <https://oldschool.runescape.wiki/w/What_Lies_Below>
    pub const WHAT_LIES_BELOW: Self = Self::new_static("what_lies_below");

    /// <https://oldschool.runescape.wiki/w/While_Guthix_Sleeps>
    pub const WHILE_GUTHIX_SLEEPS: Self = Self::new_static("while_guthix_sleeps");

    /// <https://oldschool.runescape.wiki/w/Witch%27s_House>
    pub const WITCHS_HOUSE: Self = Self::new_static("witchs_house");

    /// <https://oldschool.runescape.wiki/w/Zogre_Flesh_Eaters>
    pub const ZOGRE_FLESH_EATERS: Self = Self::new_static("zogre_flesh_eaters");
}

#[derive(Debug, Error)]
#[error("quest not found: {0:?}")]
pub struct QuestNotFoundError(pub QuestId);

impl QuestId {
    pub fn all() -> &'static [QuestId] {
        &ALL_QUEST_IDS
    }

    pub fn quest(&self) -> Result<Arc<Quest>, QuestNotFoundError> {
        QUESTS_BY_ID
            .get(self)
            .cloned()
            .ok_or_else(|| QuestNotFoundError(self.clone()))
    }
}

static ALL_QUEST_IDS: LazyLock<Vec<QuestId>> = LazyLock::new(|| {
    vec![
        QuestId::BELOW_ICE_MOUNTAIN,
        QuestId::BLACK_KNIGHTS_FORTRESS,
        QuestId::COOKS_ASSISTANT,
        QuestId::THE_CORSAIR_CURSE,
        QuestId::DEMON_SLAYER,
        QuestId::DORICS_QUEST,
        QuestId::DRAGON_SLAYER_I,
        QuestId::ERNEST_THE_CHICKEN,
        QuestId::GOBLIN_DIPLOMACY,
        QuestId::IMP_CATCHER,
        QuestId::THE_KNIGHTS_SWORD,
        QuestId::MISTHALIN_MYSTERY,
        QuestId::PIRATES_TREASURE,
        QuestId::PRINCE_ALI_RESCUE,
        QuestId::THE_RESTLESS_GHOST,
        QuestId::ROMEO_AND_JULIET,
        QuestId::RUNE_MYSTERIES,
        QuestId::SHEEP_SHEARER,
        QuestId::SHIELD_OF_ARRAV,
        QuestId::VAMPYRE_SLAYER,
        QuestId::WITCHS_POTION,
        QuestId::X_MARKS_THE_SPOT,
        QuestId::ANIMAL_MAGNETISM,
        QuestId::ANOTHER_SLICE_OF_HAM,
        QuestId::THE_ASCENT_OF_ARCEUUS,
        QuestId::AT_FIRST_LIGHT,
        QuestId::BENEATH_CURSED_SANDS,
        QuestId::BETWEEN_A_ROCK,
        QuestId::BIG_CHOMPY_BIRD_HUNTING,
        QuestId::BIOHAZARD,
        QuestId::BONE_VOYAGE,
        QuestId::CABIN_FEVER,
        QuestId::CHILDREN_OF_THE_SUN,
        QuestId::CLIENT_OF_KOUREND,
        QuestId::CLOCK_TOWER,
        QuestId::COLD_WAR,
        QuestId::CONTACT,
        QuestId::CREATURE_OF_FENKENSTRAIN,
        QuestId::CURRENT_AFFAIRS,
        QuestId::THE_CURSE_OF_ARRAV,
        QuestId::DARKNESS_OF_HALLOWVALE,
        QuestId::DEATH_ON_THE_ISLE,
        QuestId::DEATH_PLATEAU,
        QuestId::DEATH_TO_THE_DORGESHUUN,
        QuestId::DEFENDER_OF_VARROCK,
        QuestId::THE_DEPTHS_OF_DESPAIR,
        QuestId::DESERT_TREASURE_I,
        QuestId::DESERT_TREASURE_II,
        QuestId::DEVIOUS_MINDS,
        QuestId::THE_DIG_SITE,
        QuestId::DRAGON_SLAYER_II,
        QuestId::DREAM_MENTOR,
        QuestId::DRUIDIC_RITUAL,
        QuestId::DWARF_CANNON,
        QuestId::EADGARS_RUSE,
        QuestId::EAGLES_PEAK,
        QuestId::ELEMENTAL_WORKSHOP_I,
        QuestId::ELEMENTAL_WORKSHOP_II,
        QuestId::ENAKHRAS_LAMENT,
        QuestId::ENLIGHTENED_JOURNEY,
        QuestId::ETHICALLY_ACQUIRED_ANTIQUITIES,
        QuestId::THE_EYES_OF_GLOUPHRIE,
        QuestId::FAIRYTALE_I,
        QuestId::FAIRYTALE_II,
        QuestId::FAMILY_CREST,
        QuestId::THE_FEUD,
        QuestId::FIGHT_ARENA,
        QuestId::THE_FINAL_DAWN,
        QuestId::FISHING_CONTEST,
        QuestId::FORGETTABLE_TALE,
        QuestId::THE_FORSAKEN_TOWER,
        QuestId::THE_FREMENNIK_EXILES,
        QuestId::THE_FREMENNIK_ISLES,
        QuestId::THE_FREMENNIK_TRIALS,
        QuestId::THE_GARDEN_OF_DEATH,
        QuestId::GARDEN_OF_TRANQUILLITY,
        QuestId::GERTRUDES_CAT,
        QuestId::GETTING_AHEAD,
        QuestId::GHOSTS_AHOY,
        QuestId::THE_GIANT_DWARF,
        QuestId::THE_GOLEM,
        QuestId::THE_GRAND_TREE,
        QuestId::THE_GREAT_BRAIN_ROBBERY,
        QuestId::GRIM_TALES,
        QuestId::THE_HAND_IN_THE_SAND,
        QuestId::HAUNTED_MINE,
        QuestId::HAZEEL_CULT,
        QuestId::THE_HEART_OF_DARKNESS,
        QuestId::HEROES_QUEST,
        QuestId::HOLY_GRAIL,
        QuestId::HORROR_FROM_THE_DEEP,
        QuestId::ICTHLARINS_LITTLE_HELPER,
        QuestId::IN_AID_OF_THE_MYREQUE,
        QuestId::IN_SEARCH_OF_THE_MYREQUE,
        QuestId::JUNGLE_POTION,
        QuestId::A_KINGDOM_DIVIDED,
        QuestId::KINGS_RANSOM,
        QuestId::LAND_OF_THE_GOBLINS,
        QuestId::LEGENDS_QUEST,
        QuestId::LOST_CITY,
        QuestId::THE_LOST_TRIBE,
        QuestId::LUNAR_DIPLOMACY,
        QuestId::MAKING_FRIENDS_WITH_MY_ARM,
        QuestId::MAKING_HISTORY,
        QuestId::MEAT_AND_GREET,
        QuestId::MERLINS_CRYSTAL,
        QuestId::MONKS_FRIEND,
        QuestId::MONKEY_MADNESS_I,
        QuestId::MONKEY_MADNESS_II,
        QuestId::MOUNTAIN_DAUGHTER,
        QuestId::MOURNINGS_END_PART_I,
        QuestId::MOURNINGS_END_PART_II,
        QuestId::MURDER_MYSTERY,
        QuestId::MY_ARMS_BIG_ADVENTURE,
        QuestId::NATURE_SPIRIT,
        QuestId::A_NIGHT_AT_THE_THEATRE,
        QuestId::OBSERVATORY_QUEST,
        QuestId::OLAFS_QUEST,
        QuestId::ONE_SMALL_FAVOUR,
        QuestId::PANDEMONIUM,
        QuestId::THE_PATH_OF_GLOUPHRIE,
        QuestId::PERILOUS_MOONS,
        QuestId::PLAGUE_CITY,
        QuestId::A_PORCINE_OF_INTEREST,
        QuestId::PRIEST_IN_PERIL,
        QuestId::PRYING_TIMES,
        QuestId::THE_QUEEN_OF_THIEVES,
        QuestId::RAG_AND_BONE_MAN_I,
        QuestId::RAG_AND_BONE_MAN_II,
        QuestId::RATCATCHERS,
        QuestId::RECIPE_FOR_DISASTER,
        QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST,
        QuestId::RECIPE_FOR_DISASTER_FREEING_THE_MOUNTAIN_DWARF,
        QuestId::RECIPE_FOR_DISASTER_FREEING_THE_GOBLIN_GENERALS,
        QuestId::RECIPE_FOR_DISASTER_FREEING_PIRATE_PETE,
        QuestId::RECIPE_FOR_DISASTER_FREEING_THE_LUMBRIDGE_GUIDE,
        QuestId::RECIPE_FOR_DISASTER_FREEING_EVIL_DAVE,
        QuestId::RECIPE_FOR_DISASTER_FREEING_SKRACH_UGLOGWEE,
        QuestId::RECIPE_FOR_DISASTER_FREEING_SIR_AMIK_VARZE,
        QuestId::RECIPE_FOR_DISASTER_FREEING_KING_AWOWOGEI,
        QuestId::RECRUITMENT_DRIVE,
        QuestId::REGICIDE,
        QuestId::THE_RIBBITING_TALE_OF_A_LILY_PAD_LABOUR_DISPUTE,
        QuestId::ROVING_ELVES,
        QuestId::ROYAL_TROUBLE,
        QuestId::RUM_DEAL,
        QuestId::SCORPION_CATCHER,
        QuestId::SCRAMBLED,
        QuestId::SEA_SLUG,
        QuestId::SECRETS_OF_THE_NORTH,
        QuestId::SHADES_OF_MORTTON,
        QuestId::SHADOW_OF_THE_STORM,
        QuestId::SHADOWS_OF_CUSTODIA,
        QuestId::SHEEP_HERDER,
        QuestId::SHILO_VILLAGE,
        QuestId::SINS_OF_THE_FATHER,
        QuestId::SLEEPING_GIANTS,
        QuestId::THE_SLUG_MENACE,
        QuestId::SONG_OF_THE_ELVES,
        QuestId::A_SOULS_BANE,
        QuestId::SPIRITS_OF_THE_ELID,
        QuestId::SWAN_SONG,
        QuestId::TAI_BWO_WANNAI_TRIO,
        QuestId::A_TAIL_OF_TWO_CATS,
        QuestId::TALE_OF_THE_RIGHTEOUS,
        QuestId::A_TASTE_OF_HOPE,
        QuestId::TEARS_OF_GUTHIX,
        QuestId::TEMPLE_OF_IKOV,
        QuestId::TEMPLE_OF_THE_EYE,
        QuestId::THRONE_OF_MISCELLANIA,
        QuestId::THE_TOURIST_TRAP,
        QuestId::TOWER_OF_LIFE,
        QuestId::TREE_GNOME_VILLAGE,
        QuestId::TRIBAL_TOTEM,
        QuestId::TROLL_ROMANCE,
        QuestId::TROLL_STRONGHOLD,
        QuestId::TROUBLED_TORTUGANS,
        QuestId::TWILIGHTS_PROMISE,
        QuestId::UNDERGROUND_PASS,
        QuestId::WANTED,
        QuestId::WATCHTOWER,
        QuestId::WATERFALL_QUEST,
        QuestId::WHAT_LIES_BELOW,
        QuestId::WHILE_GUTHIX_SLEEPS,
        QuestId::WITCHS_HOUSE,
        QuestId::ZOGRE_FLESH_EATERS,
    ]
});

static QUESTS_BY_ID: LazyLock<HashMap<QuestId, Arc<Quest>>> = LazyLock::new(make_quests_by_id);

fn quest_req(id: QuestId) -> QuestRequirement {
    QuestRequirement::Quest(id)
}

fn skill_req(skill: Skill, level: Level) -> QuestRequirement {
    QuestRequirement::Skill { skill, level }
}

fn quest_point_req(quest_points: u16) -> QuestRequirement {
    QuestRequirement::QuestPoints(quest_points)
}

fn xp_reward(skill: Skill, xp: Xp) -> QuestReward {
    QuestReward::Xp { skill, xp }
}

fn make_quests_by_id() -> HashMap<QuestId, Arc<Quest>> {
    let mut quests = HashMap::new();

    let free_quests = [
        below_ice_mountain(),
        black_knights_fortress(),
        cooks_assistant(),
        the_corsair_curse(),
        demon_slayer(),
        dorics_quest(),
        dragon_slayer_i(),
        ernest_the_chicken(),
        goblin_diplomacy(),
        imp_catcher(),
        the_knights_sword(),
        misthalin_mystery(),
        pirates_treasure(),
        prince_ali_rescue(),
        the_restless_ghost(),
        romeo_and_juliet(),
        rune_mysteries(),
        sheep_shearer(),
        shield_of_arrav(),
        vampyre_slayer(),
        witchs_potion(),
        x_marks_the_spot(),
    ];
    for quest in free_quests {
        quests.insert(quest.id.clone(), Arc::new(quest));
    }

    let members_quests = [
        animal_magnetism(),
        another_slice_of_ham(),
        the_ascent_of_arceuus(),
        at_first_light(),
        beneath_cursed_sands(),
        between_a_rock(),
        big_chompy_bird_hunting(),
        biohazard(),
        bone_voyage(),
        cabin_fever(),
        children_of_the_sun(),
        client_of_kourend(),
        clock_tower(),
        cold_war(),
        contact(),
        creature_of_fenkenstrain(),
        current_affairs(),
        the_curse_of_arrav(),
        darkness_of_hallowvale(),
        death_on_the_isle(),
        death_plateau(),
        death_to_the_dorgeshuun(),
        defender_of_varrock(),
        the_depths_of_despair(),
        desert_treasure_i(),
        desert_treasure_ii(),
        devious_minds(),
        the_dig_site(),
        dragon_slayer_ii(),
        dream_mentor(),
        druidic_ritual(),
        dwarf_cannon(),
        eadgars_ruse(),
        eagles_peak(),
        elemental_workshop_i(),
        elemental_workshop_ii(),
        enakhras_lament(),
        enlightened_journey(),
        ethically_acquired_antiquities(),
        the_eyes_of_glouphrie(),
        fairytale_i(),
        fairytale_ii(),
        family_crest(),
        the_feud(),
        fight_arena(),
        the_final_dawn(),
        fishing_contest(),
        forgettable_tale(),
        the_forsaken_tower(),
        the_fremennik_exiles(),
        the_fremennik_isles(),
        the_fremennik_trials(),
        the_garden_of_death(),
        garden_of_tranquillity(),
        gertrudes_cat(),
        getting_ahead(),
        ghosts_ahoy(),
        the_giant_dwarf(),
        the_golem(),
        the_grand_tree(),
        the_great_brain_robbery(),
        grim_tales(),
        the_hand_in_the_sand(),
        haunted_mine(),
        hazeel_cult(),
        the_heart_of_darkness(),
        heroes_quest(),
        holy_grail(),
        horror_from_the_deep(),
        icthlarins_little_helper(),
        in_aid_of_the_myreque(),
        in_search_of_the_myreque(),
        jungle_potion(),
        kings_ransom(),
        a_kingdom_divided(),
        land_of_the_goblins(),
        legends_quest(),
        lost_city(),
        the_lost_tribe(),
        lunar_diplomacy(),
        making_friends_with_my_arm(),
        making_history(),
        meat_and_greet(),
        merlins_crystal(),
        monks_friend(),
        monkey_madness_i(),
        monkey_madness_ii(),
        mountain_daughter(),
        mournings_end_part_i(),
        mournings_end_part_ii(),
        murder_mystery(),
        my_arms_big_adventure(),
        nature_spirit(),
        a_night_at_the_theatre(),
        observatory_quest(),
        olafs_quest(),
        one_small_favour(),
        pandemonium(),
        the_path_of_glouphrie(),
        perilous_moons(),
        plague_city(),
        a_porcine_of_interest(),
        priest_in_peril(),
        prying_times(),
        the_queen_of_thieves(),
        rag_and_bone_man_i(),
        rag_and_bone_man_ii(),
        ratcatchers(),
        recipe_for_disaster(),
        recipe_for_disaster_another_cooks_quest(),
        recipe_for_disaster_freeing_the_mountain_dwarf(),
        recipe_for_disaster_freeing_the_goblin_generals(),
        recipe_for_disaster_freeing_pirate_pete(),
        recipe_for_disaster_freeing_the_lumbridge_guide(),
        recipe_for_disaster_freeing_evil_dave(),
        recipe_for_disaster_freeing_skrach_uglogwee(),
        recipe_for_disaster_freeing_sir_amik_varze(),
        recipe_for_disaster_freeing_king_awowogei(),
        recruitment_drive(),
        regicide(),
        the_ribbiting_tale_of_a_lily_pad_labour_dispute(),
        roving_elves(),
        royal_trouble(),
        rum_deal(),
        scorpion_catcher(),
        scrambled(),
        sea_slug(),
        secrets_of_the_north(),
        shades_of_mortton(),
        shadow_of_the_storm(),
        shadows_of_custodia(),
        sheep_herder(),
        shilo_village(),
        sins_of_the_father(),
        sleeping_giants(),
        the_slug_menace(),
        song_of_the_elves(),
        a_souls_bane(),
        spirits_of_the_elid(),
        swan_song(),
        tai_bwo_wannai_trio(),
        a_tail_of_two_cats(),
        tale_of_the_righteous(),
        a_taste_of_hope(),
        tears_of_guthix(),
        temple_of_ikov(),
        temple_of_the_eye(),
        throne_of_miscellania(),
        the_tourist_trap(),
        tower_of_life(),
        tree_gnome_village(),
        tribal_totem(),
        troll_romance(),
        troll_stronghold(),
        troubled_tortugans(),
        twilights_promise(),
        underground_pass(),
        wanted(),
        watchtower(),
        waterfall_quest(),
        what_lies_below(),
        while_guthix_sleeps(),
        witchs_house(),
        zogre_flesh_eaters(),
    ];
    for quest in members_quests {
        quests.insert(quest.id.clone(), Arc::new(quest));
    }

    quests
}

/// <https://oldschool.runescape.wiki/w/Below_Ice_Mountain>
fn below_ice_mountain() -> Quest {
    Quest {
        id: QuestId::BELOW_ICE_MOUNTAIN,
        name: "Below Ice Mountain".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([quest_point_req(16)]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Black_Knights%27_Fortress>
fn black_knights_fortress() -> Quest {
    Quest {
        id: QuestId::BLACK_KNIGHTS_FORTRESS,
        name: "Black Knights' Fortress".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([quest_point_req(12)]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Cook%27s_Assistant>
fn cooks_assistant() -> Quest {
    Quest {
        id: QuestId::COOKS_ASSISTANT,
        name: "Cook's Assistant".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Cooking, Xp(300.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Corsair_Curse>
fn the_corsair_curse() -> Quest {
    Quest {
        id: QuestId::THE_CORSAIR_CURSE,
        name: "The Corsair Curse".to_string(),
        quest_points: 2,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Demon_Slayer>
fn demon_slayer() -> Quest {
    Quest {
        id: QuestId::DEMON_SLAYER,
        name: "Demon Slayer".to_string(),
        quest_points: 3,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Doric%27s_Quest>
fn dorics_quest() -> Quest {
    Quest {
        id: QuestId::DORICS_QUEST,
        name: "Doric's Quest".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Mining, Xp(1_300.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Dragon_Slayer_I>
fn dragon_slayer_i() -> Quest {
    Quest {
        id: QuestId::DRAGON_SLAYER_I,
        name: "Dragon Slayer I".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([quest_point_req(32)]),
        rewards: vec![
            xp_reward(Skill::Strength, Xp(18_650.)),
            xp_reward(Skill::Defence, Xp(18_650.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Ernest_the_Chicken>
fn ernest_the_chicken() -> Quest {
    Quest {
        id: QuestId::ERNEST_THE_CHICKEN,
        name: "Ernest the Chicken".to_string(),
        quest_points: 4,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Goblin_Diplomacy>
fn goblin_diplomacy() -> Quest {
    Quest {
        id: QuestId::GOBLIN_DIPLOMACY,
        name: "Goblin Diplomacy".to_string(),
        quest_points: 5,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Crafting, Xp(200.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Imp_Catcher>
fn imp_catcher() -> Quest {
    Quest {
        id: QuestId::IMP_CATCHER,
        name: "Imp Catcher".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Magic, Xp(875.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Knight%27s_Sword>
fn the_knights_sword() -> Quest {
    Quest {
        id: QuestId::THE_KNIGHTS_SWORD,
        name: "The Knight's Sword".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([skill_req(Skill::Mining, Level(10))]),
        rewards: vec![xp_reward(Skill::Smithing, Xp(12_725.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Misthalin_Mystery>
fn misthalin_mystery() -> Quest {
    Quest {
        id: QuestId::MISTHALIN_MYSTERY,
        name: "Misthalin Mystery".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Crafting, Xp(600.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Pirate%27s_Treasure>
fn pirates_treasure() -> Quest {
    Quest {
        id: QuestId::PIRATES_TREASURE,
        name: "Pirate's Treasure".to_string(),
        quest_points: 2,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Prince_Ali_Rescue>
fn prince_ali_rescue() -> Quest {
    Quest {
        id: QuestId::PRINCE_ALI_RESCUE,
        name: "Prince Ali Rescue".to_string(),
        quest_points: 3,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/The_Restless_Ghost>
fn the_restless_ghost() -> Quest {
    Quest {
        id: QuestId::THE_RESTLESS_GHOST,
        name: "The Restless Ghost".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Prayer, Xp(1_125.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Romeo_%26_Juliet>
fn romeo_and_juliet() -> Quest {
    Quest {
        id: QuestId::ROMEO_AND_JULIET,
        name: "Romeo & Juliet".to_string(),
        quest_points: 5,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Rune_Mysteries>
fn rune_mysteries() -> Quest {
    Quest {
        id: QuestId::RUNE_MYSTERIES,
        name: "Rune Mysteries".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Sheep_Shearer>
fn sheep_shearer() -> Quest {
    Quest {
        id: QuestId::SHEEP_SHEARER,
        name: "Sheep Shearer".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Crafting, Xp(150.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Shield_of_Arrav>
fn shield_of_arrav() -> Quest {
    Quest {
        id: QuestId::SHIELD_OF_ARRAV,
        name: "Shield of Arrav".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Vampyre_Slayer>
fn vampyre_slayer() -> Quest {
    Quest {
        id: QuestId::VAMPYRE_SLAYER,
        name: "Vampyre Slayer".to_string(),
        quest_points: 3,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Attack, Xp(4_825.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Witch%27s_Potion>
fn witchs_potion() -> Quest {
    Quest {
        id: QuestId::WITCHS_POTION,
        name: "Witch's Potion".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Magic, Xp(325.))],
    }
}

/// <https://oldschool.runescape.wiki/w/X_Marks_the_Spot>
fn x_marks_the_spot() -> Quest {
    Quest {
        id: QuestId::X_MARKS_THE_SPOT,
        name: "X Marks the Spot".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![
            // TODO: Figure out how to model 300 XP lamp.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Animal_Magnetism>
fn animal_magnetism() -> Quest {
    Quest {
        id: QuestId::ANIMAL_MAGNETISM,
        name: "Animal Magnetism".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_RESTLESS_GHOST),
            quest_req(QuestId::ERNEST_THE_CHICKEN),
            quest_req(QuestId::PRIEST_IN_PERIL),
            skill_req(Skill::Slayer, Level(18)),
            skill_req(Skill::Crafting, Level(19)),
            skill_req(Skill::Ranged, Level(30)),
            skill_req(Skill::Woodcutting, Level(35)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(1_000.)),
            xp_reward(Skill::Fletching, Xp(1_000.)),
            xp_reward(Skill::Slayer, Xp(1_000.)),
            xp_reward(Skill::Woodcutting, Xp(2_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Another_Slice_of_H.A.M.>
fn another_slice_of_ham() -> Quest {
    Quest {
        id: QuestId::ANOTHER_SLICE_OF_HAM,
        name: "Another Slice of H.A.M.".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DEATH_TO_THE_DORGESHUUN),
            quest_req(QuestId::THE_GIANT_DWARF),
            quest_req(QuestId::THE_DIG_SITE),
            skill_req(Skill::Attack, Level(15)),
            skill_req(Skill::Prayer, Level(25)),
        ]),
        rewards: vec![
            xp_reward(Skill::Mining, Xp(3_000.)),
            xp_reward(Skill::Prayer, Xp(3_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Ascent_of_Arceuus>
fn the_ascent_of_arceuus() -> Quest {
    Quest {
        id: QuestId::THE_ASCENT_OF_ARCEUUS,
        name: "The Ascent of Arceuus".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CLIENT_OF_KOUREND),
            skill_req(Skill::Hunter, Level(12)),
        ]),
        rewards: vec![
            xp_reward(Skill::Hunter, Xp(1_500.)),
            xp_reward(Skill::Runecraft, Xp(500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/At_First_Light>
fn at_first_light() -> Quest {
    Quest {
        id: QuestId::AT_FIRST_LIGHT,
        name: "At First Light".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CHILDREN_OF_THE_SUN),
            quest_req(QuestId::EAGLES_PEAK),
            skill_req(Skill::Hunter, Level(46)),
            skill_req(Skill::Herblore, Level(30)),
            skill_req(Skill::Construction, Level(27)),
        ]),
        rewards: vec![
            xp_reward(Skill::Hunter, Xp(4_500.)),
            xp_reward(Skill::Construction, Xp(800.)),
            xp_reward(Skill::Herblore, Xp(500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Beneath_Cursed_Sands>
fn beneath_cursed_sands() -> Quest {
    Quest {
        id: QuestId::BENEATH_CURSED_SANDS,
        name: "Beneath Cursed Sands".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CONTACT),
            skill_req(Skill::Agility, Level(62)),
            skill_req(Skill::Crafting, Level(55)),
            skill_req(Skill::Firemaking, Level(55)),
        ]),
        rewards: vec![xp_reward(Skill::Agility, Xp(50_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Between_a_Rock...>
fn between_a_rock() -> Quest {
    Quest {
        id: QuestId::BETWEEN_A_ROCK,
        name: "Between a Rock...".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DWARF_CANNON),
            quest_req(QuestId::FISHING_CONTEST),
            skill_req(Skill::Defence, Level(30)),
            skill_req(Skill::Mining, Level(40)),
            skill_req(Skill::Smithing, Level(50)),
        ]),
        rewards: vec![
            xp_reward(Skill::Defence, Xp(5_000.)),
            xp_reward(Skill::Mining, Xp(5_000.)),
            xp_reward(Skill::Smithing, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Big_Chompy_Bird_Hunting>
fn big_chompy_bird_hunting() -> Quest {
    Quest {
        id: QuestId::BIG_CHOMPY_BIRD_HUNTING,
        name: "Big Chompy Bird Hunting".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Fletching, Level(5)),
            skill_req(Skill::Cooking, Level(30)),
            skill_req(Skill::Ranged, Level(30)),
        ]),
        rewards: vec![
            xp_reward(Skill::Fletching, Xp(262.)),
            xp_reward(Skill::Cooking, Xp(1_470.)),
            xp_reward(Skill::Ranged, Xp(735.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Biohazard>
fn biohazard() -> Quest {
    Quest {
        id: QuestId::BIOHAZARD,
        name: "Biohazard".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([quest_req(QuestId::PLAGUE_CITY)]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(1_250.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Bone_Voyage>
fn bone_voyage() -> Quest {
    Quest {
        id: QuestId::BONE_VOYAGE,
        name: "Bone Voyage".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            // TODO: Figure out how to model kudos requirement.
            quest_req(QuestId::THE_DIG_SITE),
        ]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Cabin_Fever>
fn cabin_fever() -> Quest {
    Quest {
        id: QuestId::CABIN_FEVER,
        name: "Cabin Fever".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PIRATES_TREASURE),
            quest_req(QuestId::RUM_DEAL),
            skill_req(Skill::Agility, Level(42)),
            skill_req(Skill::Crafting, Level(45)),
            skill_req(Skill::Smithing, Level(50)),
            skill_req(Skill::Ranged, Level(40)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(7_000.)),
            xp_reward(Skill::Smithing, Xp(7_000.)),
            xp_reward(Skill::Agility, Xp(7_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Children_of_the_Sun>
fn children_of_the_sun() -> Quest {
    Quest {
        id: QuestId::CHILDREN_OF_THE_SUN,
        name: "Children of the Sun".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Client_of_Kourend>
fn client_of_kourend() -> Quest {
    Quest {
        id: QuestId::CLIENT_OF_KOUREND,
        name: "Client of Kourend".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([quest_req(QuestId::X_MARKS_THE_SPOT)]),
        rewards: vec![
            // TODO: Figure out how to model 2x 500 XP lamps.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Clock_Tower>
fn clock_tower() -> Quest {
    Quest {
        id: QuestId::CLOCK_TOWER,
        name: "Clock Tower".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Cold_War>
fn cold_war() -> Quest {
    Quest {
        id: QuestId::COLD_WAR,
        name: "Cold War".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Hunter, Level(10)),
            skill_req(Skill::Agility, Level(30)),
            skill_req(Skill::Crafting, Level(30)),
            skill_req(Skill::Construction, Level(34)),
            skill_req(Skill::Thieving, Level(15)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(5_000.)),
            xp_reward(Skill::Crafting, Xp(2_000.)),
            xp_reward(Skill::Construction, Xp(1_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Contact!>
fn contact() -> Quest {
    Quest {
        id: QuestId::CONTACT,
        name: "Contact!".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PRINCE_ALI_RESCUE),
            quest_req(QuestId::ICTHLARINS_LITTLE_HELPER),
        ]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(7_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Creature_of_Fenkenstrain>
fn creature_of_fenkenstrain() -> Quest {
    Quest {
        id: QuestId::CREATURE_OF_FENKENSTRAIN,
        name: "Creature of Fenkenstrain".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PRIEST_IN_PERIL),
            quest_req(QuestId::THE_RESTLESS_GHOST),
            skill_req(Skill::Crafting, Level(20)),
            skill_req(Skill::Thieving, Level(25)),
        ]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(1_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Current_Affairs>
fn current_affairs() -> Quest {
    Quest {
        id: QuestId::CURRENT_AFFAIRS,
        name: "Current Affairs".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PANDEMONIUM),
            skill_req(Skill::Sailing, Level(22)),
            skill_req(Skill::Fishing, Level(10)),
        ]),
        rewards: vec![
            xp_reward(Skill::Sailing, Xp(1_400.)),
            xp_reward(Skill::Fishing, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Curse_of_Arrav>
fn the_curse_of_arrav() -> Quest {
    Quest {
        id: QuestId::THE_CURSE_OF_ARRAV,
        name: "The Curse of Arrav".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DEFENDER_OF_VARROCK),
            quest_req(QuestId::TROLL_ROMANCE),
            skill_req(Skill::Mining, Level(64)),
            skill_req(Skill::Ranged, Level(62)),
            skill_req(Skill::Thieving, Level(62)),
            skill_req(Skill::Agility, Level(61)),
            skill_req(Skill::Strength, Level(58)),
            skill_req(Skill::Slayer, Level(37)),
        ]),
        rewards: vec![
            xp_reward(Skill::Mining, Xp(40_000.)),
            xp_reward(Skill::Thieving, Xp(40_000.)),
            xp_reward(Skill::Agility, Xp(40_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Darkness_of_Hallowvale>
fn darkness_of_hallowvale() -> Quest {
    Quest {
        id: QuestId::DARKNESS_OF_HALLOWVALE,
        name: "Darkness of Hallowvale".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::IN_AID_OF_THE_MYREQUE),
            skill_req(Skill::Construction, Level(5)),
            skill_req(Skill::Mining, Level(20)),
            skill_req(Skill::Thieving, Level(22)),
            skill_req(Skill::Agility, Level(26)),
            skill_req(Skill::Crafting, Level(32)),
            skill_req(Skill::Magic, Level(33)),
            skill_req(Skill::Strength, Level(40)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(7_000.)),
            xp_reward(Skill::Thieving, Xp(6_000.)),
            xp_reward(Skill::Construction, Xp(2_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Death_on_the_Isle>
fn death_on_the_isle() -> Quest {
    Quest {
        id: QuestId::DEATH_ON_THE_ISLE,
        name: "Death on the Isle".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CHILDREN_OF_THE_SUN),
            skill_req(Skill::Thieving, Level(34)),
            skill_req(Skill::Agility, Level(32)),
        ]),
        rewards: vec![
            xp_reward(Skill::Thieving, Xp(10_000.)),
            xp_reward(Skill::Agility, Xp(7_500.)),
            xp_reward(Skill::Crafting, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Death_Plateau>
fn death_plateau() -> Quest {
    Quest {
        id: QuestId::DEATH_PLATEAU,
        name: "Death Plateau".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Attack, Xp(3_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Death_to_the_Dorgeshuun>
fn death_to_the_dorgeshuun() -> Quest {
    Quest {
        id: QuestId::DEATH_TO_THE_DORGESHUUN,
        name: "Death to the Dorgeshuun".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_LOST_TRIBE),
            skill_req(Skill::Agility, Level(23)),
            skill_req(Skill::Thieving, Level(23)),
        ]),
        rewards: vec![
            xp_reward(Skill::Thieving, Xp(2_000.)),
            xp_reward(Skill::Ranged, Xp(2_000.)),
        ],
    }
}

fn defender_of_varrock() -> Quest {
    Quest {
        id: QuestId::DEFENDER_OF_VARROCK,
        name: "Defender of Varrock".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::SHIELD_OF_ARRAV),
            quest_req(QuestId::TEMPLE_OF_IKOV),
            quest_req(QuestId::BELOW_ICE_MOUNTAIN),
            quest_req(QuestId::FAMILY_CREST),
            quest_req(QuestId::GARDEN_OF_TRANQUILLITY),
            quest_req(QuestId::WHAT_LIES_BELOW),
            quest_req(QuestId::ROMEO_AND_JULIET),
            quest_req(QuestId::DEMON_SLAYER),
            skill_req(Skill::Smithing, Level(55)),
            skill_req(Skill::Hunter, Level(52)),
        ]),
        rewards: vec![
            xp_reward(Skill::Smithing, Xp(15_000.)),
            xp_reward(Skill::Hunter, Xp(15_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Depths_of_Despair>
fn the_depths_of_despair() -> Quest {
    Quest {
        id: QuestId::THE_DEPTHS_OF_DESPAIR,
        name: "The Depths of Despair".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CLIENT_OF_KOUREND),
            skill_req(Skill::Agility, Level(18)),
        ]),
        rewards: vec![xp_reward(Skill::Agility, Xp(1_500.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Desert_Treasure_I>
fn desert_treasure_i() -> Quest {
    Quest {
        id: QuestId::DESERT_TREASURE_I,
        name: "Desert Treasure I".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_DIG_SITE),
            quest_req(QuestId::TEMPLE_OF_IKOV),
            quest_req(QuestId::THE_TOURIST_TRAP),
            quest_req(QuestId::TROLL_STRONGHOLD),
            quest_req(QuestId::PRIEST_IN_PERIL),
            quest_req(QuestId::WATERFALL_QUEST),
            skill_req(Skill::Thieving, Level(53)),
            skill_req(Skill::Magic, Level(50)),
            skill_req(Skill::Firemaking, Level(50)),
            skill_req(Skill::Slayer, Level(10)),
        ]),
        rewards: vec![xp_reward(Skill::Magic, Xp(20_006.9))],
    }
}

/// <https://oldschool.runescape.wiki/w/Desert_Treasure_II_-_The_Fallen_Empire>
fn desert_treasure_ii() -> Quest {
    Quest {
        id: QuestId::DESERT_TREASURE_II,
        name: "Desert Treasure II - The Fallen Empire".to_string(),
        quest_points: 5,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DESERT_TREASURE_I),
            quest_req(QuestId::SECRETS_OF_THE_NORTH),
            quest_req(QuestId::ENAKHRAS_LAMENT),
            quest_req(QuestId::TEMPLE_OF_THE_EYE),
            quest_req(QuestId::THE_GARDEN_OF_DEATH),
            quest_req(QuestId::BELOW_ICE_MOUNTAIN),
            // TODO: Figure out how to model His Faithful Servants.
            skill_req(Skill::Firemaking, Level(75)),
            skill_req(Skill::Magic, Level(75)),
            skill_req(Skill::Thieving, Level(70)),
            skill_req(Skill::Herblore, Level(62)),
            skill_req(Skill::Runecraft, Level(60)),
            skill_req(Skill::Construction, Level(60)),
        ]),
        rewards: vec![
            // TODO: Figure out how to model combat XP lamps.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Devious_Minds>
fn devious_minds() -> Quest {
    Quest {
        id: QuestId::DEVIOUS_MINDS,
        name: "Devious Minds".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::WANTED),
            quest_req(QuestId::TROLL_STRONGHOLD),
            quest_req(QuestId::DORICS_QUEST),
            // TODO: Figure out how to model Enter the Abyss requirement.
            skill_req(Skill::Smithing, Level(65)),
            skill_req(Skill::Runecraft, Level(50)),
            skill_req(Skill::Fletching, Level(50)),
        ]),
        rewards: vec![
            xp_reward(Skill::Fletching, Xp(5_000.)),
            xp_reward(Skill::Runecraft, Xp(5_000.)),
            xp_reward(Skill::Smithing, Xp(6_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Dig_Site>
fn the_dig_site() -> Quest {
    Quest {
        id: QuestId::THE_DIG_SITE,
        name: "The Dig Site".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Agility, Level(10)),
            skill_req(Skill::Herblore, Level(10)),
            skill_req(Skill::Thieving, Level(25)),
        ]),
        rewards: vec![
            xp_reward(Skill::Mining, Xp(15_300.)),
            xp_reward(Skill::Herblore, Xp(2_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Dragon_Slayer_II>
fn dragon_slayer_ii() -> Quest {
    Quest {
        id: QuestId::DRAGON_SLAYER_II,
        name: "Dragon Slayer II".to_string(),
        quest_points: 5,
        requirements: IndexSet::from_iter([
            quest_point_req(200),
            quest_req(QuestId::LEGENDS_QUEST),
            quest_req(QuestId::DREAM_MENTOR),
            quest_req(QuestId::A_TAIL_OF_TWO_CATS),
            quest_req(QuestId::ANIMAL_MAGNETISM),
            quest_req(QuestId::GHOSTS_AHOY),
            quest_req(QuestId::BONE_VOYAGE),
            quest_req(QuestId::CLIENT_OF_KOUREND),
            skill_req(Skill::Magic, Level(75)),
            skill_req(Skill::Smithing, Level(70)),
            skill_req(Skill::Mining, Level(68)),
            skill_req(Skill::Crafting, Level(62)),
            skill_req(Skill::Agility, Level(60)),
            skill_req(Skill::Thieving, Level(60)),
            skill_req(Skill::Construction, Level(50)),
            skill_req(Skill::Hitpoints, Level(50)),
        ]),
        rewards: vec![
            xp_reward(Skill::Smithing, Xp(80_000.)),
            xp_reward(Skill::Mining, Xp(60_000.)),
            xp_reward(Skill::Agility, Xp(50_000.)),
            xp_reward(Skill::Thieving, Xp(50_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Dream_Mentor>
fn dream_mentor() -> Quest {
    Quest {
        id: QuestId::DREAM_MENTOR,
        name: "Dream Mentor".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::LUNAR_DIPLOMACY),
            quest_req(QuestId::EADGARS_RUSE),
        ]),
        rewards: vec![
            xp_reward(Skill::Hitpoints, Xp(15_000.)),
            xp_reward(Skill::Magic, Xp(10_000.)),
            // TODO: Figure out how to model dreamy lamp.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Druidic_Ritual>
fn druidic_ritual() -> Quest {
    Quest {
        id: QuestId::DRUIDIC_RITUAL,
        name: "Druidic Ritual".to_string(),
        quest_points: 4,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Herblore, Xp(250.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Dwarf_Cannon>
fn dwarf_cannon() -> Quest {
    Quest {
        id: QuestId::DWARF_CANNON,
        name: "Dwarf Cannon".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Crafting, Xp(750.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Eadgar%27s_Ruse>
fn eadgars_ruse() -> Quest {
    Quest {
        id: QuestId::EADGARS_RUSE,
        name: "Eadgar's Ruse".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DRUIDIC_RITUAL),
            quest_req(QuestId::TROLL_STRONGHOLD),
            skill_req(Skill::Herblore, Level(31)),
        ]),
        rewards: vec![xp_reward(Skill::Herblore, Xp(11_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Eagles%27_Peak>
fn eagles_peak() -> Quest {
    Quest {
        id: QuestId::EAGLES_PEAK,
        name: "Eagles' Peak".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([skill_req(Skill::Hunter, Level(27))]),
        rewards: vec![xp_reward(Skill::Hunter, Xp(2_500.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Elemental_Workshop_I>
fn elemental_workshop_i() -> Quest {
    Quest {
        id: QuestId::ELEMENTAL_WORKSHOP_I,
        name: "Elemental Workshop I".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Mining, Level(20)),
            skill_req(Skill::Smithing, Level(20)),
            skill_req(Skill::Crafting, Level(20)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(5_000.)),
            xp_reward(Skill::Smithing, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Elemental_Workshop_II>
fn elemental_workshop_ii() -> Quest {
    Quest {
        id: QuestId::ELEMENTAL_WORKSHOP_II,
        name: "Elemental Workshop II".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::ELEMENTAL_WORKSHOP_I),
            skill_req(Skill::Magic, Level(20)),
            skill_req(Skill::Smithing, Level(30)),
        ]),
        rewards: vec![
            xp_reward(Skill::Smithing, Xp(7_500.)),
            xp_reward(Skill::Crafting, Xp(7_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Enakhra%27s_Lament>
fn enakhras_lament() -> Quest {
    Quest {
        id: QuestId::ENAKHRAS_LAMENT,
        name: "Enakhra's Lament".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Crafting, Level(50)),
            skill_req(Skill::Firemaking, Level(45)),
            skill_req(Skill::Prayer, Level(43)),
            skill_req(Skill::Magic, Level(39)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(7_000.)),
            xp_reward(Skill::Mining, Xp(7_000.)),
            xp_reward(Skill::Firemaking, Xp(7_000.)),
            xp_reward(Skill::Magic, Xp(7_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Enlightened_Journey>
fn enlightened_journey() -> Quest {
    Quest {
        id: QuestId::ENLIGHTENED_JOURNEY,
        name: "Enlightened Journey".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_point_req(20),
            skill_req(Skill::Firemaking, Level(20)),
            skill_req(Skill::Farming, Level(30)),
            skill_req(Skill::Crafting, Level(36)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(2_000.)),
            xp_reward(Skill::Farming, Xp(3_000.)),
            xp_reward(Skill::Woodcutting, Xp(1_500.)),
            xp_reward(Skill::Firemaking, Xp(4_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Ethically_Acquired_Antiquities>
fn ethically_acquired_antiquities() -> Quest {
    Quest {
        id: QuestId::ETHICALLY_ACQUIRED_ANTIQUITIES,
        name: "Ethically Acquired Antiquities".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Thieving, Level(25)),
            quest_req(QuestId::CHILDREN_OF_THE_SUN),
            quest_req(QuestId::SHIELD_OF_ARRAV),
        ]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(6_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Eyes_of_Glouphrie>
fn the_eyes_of_glouphrie() -> Quest {
    Quest {
        id: QuestId::THE_EYES_OF_GLOUPHRIE,
        name: "The Eyes of Glouphrie".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_GRAND_TREE),
            skill_req(Skill::Construction, Level(5)),
            skill_req(Skill::Magic, Level(46)),
        ]),
        rewards: vec![
            xp_reward(Skill::Magic, Xp(12_000.)),
            xp_reward(Skill::Woodcutting, Xp(2_500.)),
            xp_reward(Skill::Runecraft, Xp(6_000.)),
            xp_reward(Skill::Construction, Xp(250.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Fairytale_I_-_Growing_Pains>
fn fairytale_i() -> Quest {
    Quest {
        id: QuestId::FAIRYTALE_I,
        name: "Fairytale I - Growing Pains".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::LOST_CITY),
            quest_req(QuestId::NATURE_SPIRIT),
        ]),
        rewards: vec![
            xp_reward(Skill::Farming, Xp(3_500.)),
            xp_reward(Skill::Attack, Xp(2_000.)),
            xp_reward(Skill::Magic, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Fairytale_II_-_Cure_a_Queen>
fn fairytale_ii() -> Quest {
    Quest {
        id: QuestId::FAIRYTALE_II,
        name: "Fairytale II - Cure a Queen".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::FAIRYTALE_I),
            skill_req(Skill::Thieving, Level(40)),
            skill_req(Skill::Farming, Level(49)),
            skill_req(Skill::Herblore, Level(57)),
        ]),
        rewards: vec![
            xp_reward(Skill::Herblore, Xp(3500.)),
            xp_reward(Skill::Thieving, Xp(2500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Family_Crest>
fn family_crest() -> Quest {
    Quest {
        id: QuestId::FAMILY_CREST,
        name: "Family Crest".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Mining, Level(40)),
            skill_req(Skill::Smithing, Level(40)),
            skill_req(Skill::Magic, Level(59)),
            skill_req(Skill::Crafting, Level(40)),
        ]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/The_Feud>
fn the_feud() -> Quest {
    Quest {
        id: QuestId::THE_FEUD,
        name: "The Feud".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([skill_req(Skill::Thieving, Level(30))]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(15_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Fight_Arena>
fn fight_arena() -> Quest {
    Quest {
        id: QuestId::FIGHT_ARENA,
        name: "Fight Arena".to_string(),
        quest_points: 2,
        requirements: IndexSet::new(),
        rewards: vec![
            xp_reward(Skill::Attack, Xp(12_175.)),
            xp_reward(Skill::Thieving, Xp(2_175.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Final_Dawn>
fn the_final_dawn() -> Quest {
    Quest {
        id: QuestId::THE_FINAL_DAWN,
        name: "The Final Dawn".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_HEART_OF_DARKNESS),
            quest_req(QuestId::PERILOUS_MOONS),
            skill_req(Skill::Thieving, Level(66)),
            skill_req(Skill::Runecraft, Level(52)),
            skill_req(Skill::Fletching, Level(52)),
        ]),
        rewards: vec![
            xp_reward(Skill::Thieving, Xp(55_000.)),
            xp_reward(Skill::Runecraft, Xp(25_000.)),
            xp_reward(Skill::Fletching, Xp(25_000.)),
            // TODO: Figure out how to model 55,000 combat lamp.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Fishing_Contest>
pub fn fishing_contest() -> Quest {
    Quest {
        id: QuestId::FISHING_CONTEST,
        name: "Fishing Contest".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([skill_req(Skill::Fishing, Level(10))]),
        rewards: vec![xp_reward(Skill::Fishing, Xp(2_437.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Forgettable_Tale...>
pub fn forgettable_tale() -> Quest {
    Quest {
        id: QuestId::FORGETTABLE_TALE,
        name: "Forgettable Tale...".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_GIANT_DWARF),
            quest_req(QuestId::FISHING_CONTEST),
            skill_req(Skill::Cooking, Level(22)),
            skill_req(Skill::Farming, Level(17)),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(5_000.)),
            xp_reward(Skill::Farming, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Forsaken_Tower>
pub fn the_forsaken_tower() -> Quest {
    Quest {
        id: QuestId::THE_FORSAKEN_TOWER,
        name: "The Forsaken Tower".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([quest_req(QuestId::CLIENT_OF_KOUREND)]),
        rewards: vec![
            xp_reward(Skill::Mining, Xp(500.)),
            xp_reward(Skill::Smithing, Xp(500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Fremennik_Exiles>
pub fn the_fremennik_exiles() -> Quest {
    Quest {
        id: QuestId::THE_FREMENNIK_EXILES,
        name: "The Fremennik Exiles".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_FREMENNIK_ISLES),
            quest_req(QuestId::LUNAR_DIPLOMACY),
            quest_req(QuestId::MOUNTAIN_DAUGHTER),
            quest_req(QuestId::HEROES_QUEST),
            skill_req(Skill::Crafting, Level(65)),
            skill_req(Skill::Slayer, Level(60)),
            skill_req(Skill::Smithing, Level(60)),
            skill_req(Skill::Fishing, Level(60)),
            skill_req(Skill::Runecraft, Level(55)),
        ]),
        rewards: vec![
            xp_reward(Skill::Slayer, Xp(50_000.)),
            xp_reward(Skill::Crafting, Xp(50_000.)),
            xp_reward(Skill::Runecraft, Xp(30_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Fremennik_Isles>
pub fn the_fremennik_isles() -> Quest {
    Quest {
        id: QuestId::THE_FREMENNIK_ISLES,
        name: "The Fremennik Isles".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_FREMENNIK_TRIALS),
            skill_req(Skill::Construction, Level(20)),
        ]),
        rewards: vec![
            xp_reward(Skill::Woodcutting, Xp(10_000.)),
            xp_reward(Skill::Crafting, Xp(5_000.)),
            xp_reward(Skill::Construction, Xp(5_000.)),
            // TODO: Figure out how to model "Two lots of Combat level 10,000 Combat experience".
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Fremennik_Trials>
pub fn the_fremennik_trials() -> Quest {
    Quest {
        id: QuestId::THE_FREMENNIK_TRIALS,
        name: "The Fremennik Trials".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Fletching, Level(25)),
            skill_req(Skill::Woodcutting, Level(40)),
            skill_req(Skill::Crafting, Level(40)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(2_812.4)),
            xp_reward(Skill::Attack, Xp(2_812.4)),
            xp_reward(Skill::Crafting, Xp(2_812.4)),
            xp_reward(Skill::Defence, Xp(2_812.4)),
            xp_reward(Skill::Fishing, Xp(2_812.4)),
            xp_reward(Skill::Fletching, Xp(2_812.4)),
            xp_reward(Skill::Hitpoints, Xp(2_812.4)),
            xp_reward(Skill::Strength, Xp(2_812.4)),
            xp_reward(Skill::Thieving, Xp(2_812.4)),
            xp_reward(Skill::Woodcutting, Xp(2_812.4)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Garden_of_Death>
fn the_garden_of_death() -> Quest {
    Quest {
        id: QuestId::THE_GARDEN_OF_DEATH,
        name: "The Garden of Death".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([skill_req(Skill::Farming, Level(20))]),
        rewards: vec![xp_reward(Skill::Farming, Xp(10_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Garden_of_Tranquillity>
fn garden_of_tranquillity() -> Quest {
    Quest {
        id: QuestId::GARDEN_OF_TRANQUILLITY,
        name: "Garden of Tranquillity".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CREATURE_OF_FENKENSTRAIN),
            skill_req(Skill::Farming, Level(25)),
        ]),
        rewards: vec![xp_reward(Skill::Farming, Xp(5_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Gertrude%27s_Cat>
fn gertrudes_cat() -> Quest {
    Quest {
        id: QuestId::GERTRUDES_CAT,
        name: "Gertrude's Cat".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Cooking, Xp(1_525.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Getting_Ahead>
fn getting_ahead() -> Quest {
    Quest {
        id: QuestId::GETTING_AHEAD,
        name: "Getting Ahead".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Crafting, Level(30)),
            skill_req(Skill::Construction, Level(26)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(4_000.)),
            xp_reward(Skill::Construction, Xp(3_200.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Ghosts_Ahoy>
fn ghosts_ahoy() -> Quest {
    Quest {
        id: QuestId::GHOSTS_AHOY,
        name: "Ghosts Ahoy".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PRIEST_IN_PERIL),
            quest_req(QuestId::THE_RESTLESS_GHOST),
            skill_req(Skill::Agility, Level(25)),
            skill_req(Skill::Cooking, Level(20)),
        ]),
        rewards: vec![xp_reward(Skill::Prayer, Xp(2_400.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Giant_Dwarf>
fn the_giant_dwarf() -> Quest {
    Quest {
        id: QuestId::THE_GIANT_DWARF,
        name: "The Giant Dwarf".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Crafting, Level(12)),
            skill_req(Skill::Firemaking, Level(16)),
            skill_req(Skill::Magic, Level(33)),
            skill_req(Skill::Thieving, Level(14)),
        ]),
        rewards: vec![
            xp_reward(Skill::Mining, Xp(2_500.)),
            xp_reward(Skill::Smithing, Xp(2_500.)),
            xp_reward(Skill::Crafting, Xp(2_500.)),
            xp_reward(Skill::Magic, Xp(1_500.)),
            xp_reward(Skill::Thieving, Xp(1_500.)),
            xp_reward(Skill::Firemaking, Xp(1_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Golem>
fn the_golem() -> Quest {
    Quest {
        id: QuestId::THE_GOLEM,
        name: "The Golem".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Crafting, Level(20)),
            skill_req(Skill::Thieving, Level(25)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(1_000.)),
            xp_reward(Skill::Thieving, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Grand_Tree>
fn the_grand_tree() -> Quest {
    Quest {
        id: QuestId::THE_GRAND_TREE,
        name: "The Grand Tree".to_string(),
        quest_points: 5,
        requirements: IndexSet::from_iter([skill_req(Skill::Agility, Level(25))]),
        rewards: vec![
            xp_reward(Skill::Attack, Xp(18_400.)),
            xp_reward(Skill::Agility, Xp(7_900.)),
            xp_reward(Skill::Magic, Xp(2_150.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Great_Brain_Robbery>
fn the_great_brain_robbery() -> Quest {
    Quest {
        id: QuestId::THE_GREAT_BRAIN_ROBBERY,
        name: "The Great Brain Robbery".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CREATURE_OF_FENKENSTRAIN),
            quest_req(QuestId::CABIN_FEVER),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_PIRATE_PETE),
            skill_req(Skill::Crafting, Level(16)),
            skill_req(Skill::Construction, Level(30)),
            skill_req(Skill::Prayer, Level(50)),
        ]),
        rewards: vec![
            xp_reward(Skill::Prayer, Xp(6_000.)),
            xp_reward(Skill::Crafting, Xp(3_000.)),
            xp_reward(Skill::Construction, Xp(2_000.)),
            // TODO: Figure out how to model "Blessed lamp".
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Grim_Tales>
fn grim_tales() -> Quest {
    Quest {
        id: QuestId::GRIM_TALES,
        name: "Grim Tales".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::WITCHS_HOUSE),
            skill_req(Skill::Farming, Level(45)),
            skill_req(Skill::Herblore, Level(52)),
            skill_req(Skill::Thieving, Level(58)),
            skill_req(Skill::Agility, Level(59)),
            skill_req(Skill::Woodcutting, Level(71)),
        ]),
        rewards: vec![
            xp_reward(Skill::Woodcutting, Xp(60_000.)),
            xp_reward(Skill::Agility, Xp(25_000.)),
            xp_reward(Skill::Thieving, Xp(25_000.)),
            xp_reward(Skill::Herblore, Xp(15_000.)),
            xp_reward(Skill::Farming, Xp(10_000.)),
            xp_reward(Skill::Hitpoints, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Hand_in_the_Sand>
fn the_hand_in_the_sand() -> Quest {
    Quest {
        id: QuestId::THE_HAND_IN_THE_SAND,
        name: "The Hand in the Sand".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Thieving, Level(17)),
            skill_req(Skill::Crafting, Level(49)),
        ]),
        rewards: vec![
            xp_reward(Skill::Thieving, Xp(1_000.)),
            xp_reward(Skill::Crafting, Xp(9_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Haunted_Mine>
fn haunted_mine() -> Quest {
    Quest {
        id: QuestId::HAUNTED_MINE,
        name: "Haunted Mine".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PRIEST_IN_PERIL),
            skill_req(Skill::Crafting, Level(35)),
        ]),
        rewards: vec![xp_reward(Skill::Strength, Xp(22_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Hazeel_Cult>
fn hazeel_cult() -> Quest {
    Quest {
        id: QuestId::HAZEEL_CULT,
        name: "Hazeel Cult".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Thieving, Xp(1_500.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Heart_of_Darkness>
fn the_heart_of_darkness() -> Quest {
    Quest {
        id: QuestId::THE_HEART_OF_DARKNESS,
        name: "The Heart of Darkness".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::TWILIGHTS_PROMISE),
            skill_req(Skill::Mining, Level(55)),
            skill_req(Skill::Thieving, Level(48)),
            skill_req(Skill::Slayer, Level(48)),
            skill_req(Skill::Agility, Level(46)),
        ]),
        rewards: vec![
            xp_reward(Skill::Mining, Xp(8_000.)),
            xp_reward(Skill::Thieving, Xp(8_000.)),
            xp_reward(Skill::Slayer, Xp(8_000.)),
            xp_reward(Skill::Agility, Xp(8_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Heroes%27_Quest>
fn heroes_quest() -> Quest {
    Quest {
        id: QuestId::HEROES_QUEST,
        name: "Heroes' Quest".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_point_req(55),
            quest_req(QuestId::SHIELD_OF_ARRAV),
            quest_req(QuestId::LOST_CITY),
            quest_req(QuestId::MERLINS_CRYSTAL),
            quest_req(QuestId::DRAGON_SLAYER_I),
            skill_req(Skill::Cooking, Level(53)),
            skill_req(Skill::Fishing, Level(53)),
            skill_req(Skill::Herblore, Level(25)),
            skill_req(Skill::Mining, Level(50)),
        ]),
        rewards: vec![
            xp_reward(Skill::Attack, Xp(3_075.)),
            xp_reward(Skill::Defence, Xp(3_075.)),
            xp_reward(Skill::Strength, Xp(3_075.)),
            xp_reward(Skill::Hitpoints, Xp(3_075.)),
            xp_reward(Skill::Ranged, Xp(2_075.)),
            xp_reward(Skill::Fishing, Xp(2_725.)),
            xp_reward(Skill::Cooking, Xp(2_825.)),
            xp_reward(Skill::Woodcutting, Xp(1_575.)),
            xp_reward(Skill::Firemaking, Xp(1_575.)),
            xp_reward(Skill::Smithing, Xp(2_275.)),
            xp_reward(Skill::Mining, Xp(2_575.)),
            xp_reward(Skill::Herblore, Xp(1_325.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Holy_Grail>
fn holy_grail() -> Quest {
    Quest {
        id: QuestId::HOLY_GRAIL,
        name: "Holy Grail".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::MERLINS_CRYSTAL),
            skill_req(Skill::Attack, Level(20)),
        ]),
        rewards: vec![
            xp_reward(Skill::Prayer, Xp(11_000.)),
            xp_reward(Skill::Defence, Xp(15_300.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Horror_from_the_Deep>
fn horror_from_the_deep() -> Quest {
    Quest {
        id: QuestId::HORROR_FROM_THE_DEEP,
        name: "Horror from the Deep".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Agility, Level(35)),
            // TODO: Figure out how to model Alfred Grimhand's Barcrawl.
        ]),
        rewards: vec![
            xp_reward(Skill::Magic, Xp(4_662.5)),
            xp_reward(Skill::Strength, Xp(4_662.5)),
            xp_reward(Skill::Ranged, Xp(4_662.5)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Icthlarin%27s_Little_Helper>
fn icthlarins_little_helper() -> Quest {
    Quest {
        id: QuestId::ICTHLARINS_LITTLE_HELPER,
        name: "Icthlarin's Little Helper".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([quest_req(QuestId::GERTRUDES_CAT)]),
        rewards: vec![
            xp_reward(Skill::Thieving, Xp(4_500.)),
            xp_reward(Skill::Agility, Xp(4_000.)),
            xp_reward(Skill::Woodcutting, Xp(4_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/In_Aid_of_the_Myreque>
fn in_aid_of_the_myreque() -> Quest {
    Quest {
        id: QuestId::IN_AID_OF_THE_MYREQUE,
        name: "In Aid of the Myreque".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::IN_SEARCH_OF_THE_MYREQUE),
            skill_req(Skill::Agility, Level(25)),
            skill_req(Skill::Crafting, Level(25)),
            skill_req(Skill::Mining, Level(15)),
            skill_req(Skill::Magic, Level(7)),
        ]),
        rewards: vec![
            xp_reward(Skill::Attack, Xp(2_000.)),
            xp_reward(Skill::Strength, Xp(2_000.)),
            xp_reward(Skill::Crafting, Xp(2_000.)),
            xp_reward(Skill::Defence, Xp(2_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/In_Search_of_the_Myreque>
fn in_search_of_the_myreque() -> Quest {
    Quest {
        id: QuestId::IN_SEARCH_OF_THE_MYREQUE,
        name: "In Search of the Myreque".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::NATURE_SPIRIT),
            skill_req(Skill::Agility, Level(25)),
        ]),
        rewards: vec![
            xp_reward(Skill::Attack, Xp(600.)),
            xp_reward(Skill::Defence, Xp(600.)),
            xp_reward(Skill::Strength, Xp(600.)),
            xp_reward(Skill::Hitpoints, Xp(600.)),
            xp_reward(Skill::Crafting, Xp(600.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Jungle_Potion>
fn jungle_potion() -> Quest {
    Quest {
        id: QuestId::JUNGLE_POTION,
        name: "Jungle Potion".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DRUIDIC_RITUAL),
            skill_req(Skill::Herblore, Level(3)),
        ]),
        rewards: vec![xp_reward(Skill::Herblore, Xp(775.))],
    }
}

/// <https://oldschool.runescape.wiki/w/King%27s_Ransom>
fn kings_ransom() -> Quest {
    Quest {
        id: QuestId::KINGS_RANSOM,
        name: "King's Ransom".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::BLACK_KNIGHTS_FORTRESS),
            quest_req(QuestId::HOLY_GRAIL),
            quest_req(QuestId::MURDER_MYSTERY),
            quest_req(QuestId::ONE_SMALL_FAVOUR),
            skill_req(Skill::Magic, Level(45)),
            skill_req(Skill::Defence, Level(65)),
        ]),
        rewards: vec![
            xp_reward(Skill::Defence, Xp(33_000.)),
            xp_reward(Skill::Magic, Xp(5_000.)),
            // TODO: Figure out how to model antique lamp.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/A_Kingdom_Divided>
fn a_kingdom_divided() -> Quest {
    Quest {
        id: QuestId::A_KINGDOM_DIVIDED,
        name: "A Kingdom Divided".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_DEPTHS_OF_DESPAIR),
            quest_req(QuestId::THE_QUEEN_OF_THIEVES),
            quest_req(QuestId::THE_ASCENT_OF_ARCEUUS),
            quest_req(QuestId::THE_FORSAKEN_TOWER),
            quest_req(QuestId::TALE_OF_THE_RIGHTEOUS),
            skill_req(Skill::Agility, Level(54)),
            skill_req(Skill::Thieving, Level(52)),
            skill_req(Skill::Woodcutting, Level(52)),
            skill_req(Skill::Herblore, Level(50)),
            skill_req(Skill::Mining, Level(42)),
            skill_req(Skill::Crafting, Level(38)),
            skill_req(Skill::Magic, Level(35)),
        ]),
        rewards: vec![
            // TODO: Figure out how to model two antique lamps.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Land_of_the_Goblins>
fn land_of_the_goblins() -> Quest {
    Quest {
        id: QuestId::LAND_OF_THE_GOBLINS,
        name: "Land of the Goblins".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::ANOTHER_SLICE_OF_HAM),
            quest_req(QuestId::FISHING_CONTEST),
            skill_req(Skill::Agility, Level(38)),
            skill_req(Skill::Fishing, Level(40)),
            skill_req(Skill::Thieving, Level(45)),
            skill_req(Skill::Herblore, Level(48)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(8_000.)),
            xp_reward(Skill::Fishing, Xp(8_000.)),
            xp_reward(Skill::Thieving, Xp(8_000.)),
            xp_reward(Skill::Herblore, Xp(8_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Legends%27_Quest>
fn legends_quest() -> Quest {
    Quest {
        id: QuestId::LEGENDS_QUEST,
        name: "Legends' Quest".to_string(),
        quest_points: 4,
        requirements: IndexSet::from_iter([
            quest_point_req(107),
            quest_req(QuestId::FAMILY_CREST),
            quest_req(QuestId::HEROES_QUEST),
            quest_req(QuestId::SHILO_VILLAGE),
            quest_req(QuestId::UNDERGROUND_PASS),
            quest_req(QuestId::WATERFALL_QUEST),
            skill_req(Skill::Agility, Level(50)),
            skill_req(Skill::Crafting, Level(50)),
            skill_req(Skill::Herblore, Level(45)),
            skill_req(Skill::Magic, Level(56)),
            skill_req(Skill::Mining, Level(52)),
            skill_req(Skill::Prayer, Level(42)),
            skill_req(Skill::Smithing, Level(50)),
            skill_req(Skill::Strength, Level(50)),
            skill_req(Skill::Thieving, Level(50)),
            skill_req(Skill::Woodcutting, Level(50)),
        ]),
        rewards: vec![
            // TODO: Figure out how to model 30,000 experience each in four skills of choice.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Lost_City>
fn lost_city() -> Quest {
    Quest {
        id: QuestId::LOST_CITY,
        name: "Lost City".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Crafting, Level(31)),
            skill_req(Skill::Woodcutting, Level(36)),
        ]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/The_Lost_Tribe>
fn the_lost_tribe() -> Quest {
    Quest {
        id: QuestId::THE_LOST_TRIBE,
        name: "The Lost Tribe".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::GOBLIN_DIPLOMACY),
            quest_req(QuestId::RUNE_MYSTERIES),
            skill_req(Skill::Agility, Level(13)),
            skill_req(Skill::Thieving, Level(13)),
            skill_req(Skill::Mining, Level(17)),
        ]),
        rewards: vec![xp_reward(Skill::Mining, Xp(3_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Lunar_Diplomacy>
fn lunar_diplomacy() -> Quest {
    Quest {
        id: QuestId::LUNAR_DIPLOMACY,
        name: "Lunar Diplomacy".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_FREMENNIK_TRIALS),
            quest_req(QuestId::LOST_CITY),
            quest_req(QuestId::RUNE_MYSTERIES),
            quest_req(QuestId::SHILO_VILLAGE),
            skill_req(Skill::Herblore, Level(5)),
            skill_req(Skill::Crafting, Level(61)),
            skill_req(Skill::Defence, Level(40)),
            skill_req(Skill::Firemaking, Level(49)),
            skill_req(Skill::Magic, Level(65)),
            skill_req(Skill::Mining, Level(60)),
            skill_req(Skill::Woodcutting, Level(55)),
        ]),
        rewards: vec![
            xp_reward(Skill::Magic, Xp(5_000.)),
            xp_reward(Skill::Runecraft, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Making_Friends_with_My_Arm>
fn making_friends_with_my_arm() -> Quest {
    Quest {
        id: QuestId::MAKING_FRIENDS_WITH_MY_ARM,
        name: "Making Friends with My Arm".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::MY_ARMS_BIG_ADVENTURE),
            quest_req(QuestId::SWAN_SONG),
            quest_req(QuestId::COLD_WAR),
            quest_req(QuestId::ROMEO_AND_JULIET),
            skill_req(Skill::Firemaking, Level(66)),
            skill_req(Skill::Mining, Level(72)),
            skill_req(Skill::Construction, Level(35)),
            skill_req(Skill::Agility, Level(68)),
        ]),
        rewards: vec![
            xp_reward(Skill::Construction, Xp(10_000.)),
            xp_reward(Skill::Firemaking, Xp(40_000.)),
            xp_reward(Skill::Mining, Xp(50_000.)),
            xp_reward(Skill::Agility, Xp(50_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Making_History>
fn making_history() -> Quest {
    Quest {
        id: QuestId::MAKING_HISTORY,
        name: "Making History".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PRIEST_IN_PERIL),
            quest_req(QuestId::THE_RESTLESS_GHOST),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(1_000.)),
            xp_reward(Skill::Prayer, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Meat_and_Greet>
fn meat_and_greet() -> Quest {
    Quest {
        id: QuestId::MEAT_AND_GREET,
        name: "Meat and Greet".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([quest_req(QuestId::CHILDREN_OF_THE_SUN)]),
        rewards: vec![xp_reward(Skill::Cooking, Xp(8_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Merlin%27s_Crystal>
fn merlins_crystal() -> Quest {
    Quest {
        id: QuestId::MERLINS_CRYSTAL,
        name: "Merlin's Crystal".to_string(),
        quest_points: 6,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Monk%27s_Friend>
fn monks_friend() -> Quest {
    Quest {
        id: QuestId::MONKS_FRIEND,
        name: "Monk's Friend".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Woodcutting, Xp(2_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Monkey_Madness_I>
fn monkey_madness_i() -> Quest {
    Quest {
        id: QuestId::MONKEY_MADNESS_I,
        name: "Monkey Madness I".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_GRAND_TREE),
            quest_req(QuestId::TREE_GNOME_VILLAGE),
        ]),
        rewards: vec![
            // TODO: Experience rewards based on player choice.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Monkey_Madness_II>
fn monkey_madness_ii() -> Quest {
    Quest {
        id: QuestId::MONKEY_MADNESS_II,
        name: "Monkey Madness II".to_string(),
        quest_points: 4,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_KING_AWOWOGEI),
            quest_req(QuestId::ENLIGHTENED_JOURNEY),
            quest_req(QuestId::THE_EYES_OF_GLOUPHRIE),
            quest_req(QuestId::TROLL_STRONGHOLD),
            quest_req(QuestId::WATCHTOWER),
            skill_req(Skill::Slayer, Level(69)),
            skill_req(Skill::Crafting, Level(70)),
            skill_req(Skill::Hunter, Level(60)),
            skill_req(Skill::Agility, Level(55)),
            skill_req(Skill::Thieving, Level(55)),
            skill_req(Skill::Firemaking, Level(60)),
        ]),
        rewards: vec![
            xp_reward(Skill::Slayer, Xp(80_000.)),
            xp_reward(Skill::Agility, Xp(60_000.)),
            xp_reward(Skill::Thieving, Xp(50_000.)),
            xp_reward(Skill::Hunter, Xp(50_000.)),
            // TODO: Additional experience rewards based on player choice
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Mountain_Daughter>
fn mountain_daughter() -> Quest {
    Quest {
        id: QuestId::MOUNTAIN_DAUGHTER,
        name: "Mountain Daughter".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([skill_req(Skill::Agility, Level(20))]),
        rewards: vec![
            xp_reward(Skill::Prayer, Xp(2_000.)),
            xp_reward(Skill::Attack, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Mourning%27s_End_Part_I>
fn mournings_end_part_i() -> Quest {
    Quest {
        id: QuestId::MOURNINGS_END_PART_I,
        name: "Mourning's End Part I".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::ROVING_ELVES),
            quest_req(QuestId::BIG_CHOMPY_BIRD_HUNTING),
            quest_req(QuestId::SHEEP_HERDER),
            skill_req(Skill::Ranged, Level(60)),
            skill_req(Skill::Thieving, Level(50)),
        ]),
        rewards: vec![
            xp_reward(Skill::Thieving, Xp(40_000.)),
            xp_reward(Skill::Hitpoints, Xp(25_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Mourning%27s_End_Part_II>
fn mournings_end_part_ii() -> Quest {
    Quest {
        id: QuestId::MOURNINGS_END_PART_II,
        name: "Mourning's End Part II".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([quest_req(QuestId::MOURNINGS_END_PART_I)]),
        rewards: vec![xp_reward(Skill::Agility, Xp(60_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Murder_Mystery>
fn murder_mystery() -> Quest {
    Quest {
        id: QuestId::MURDER_MYSTERY,
        name: "Murder Mystery".to_string(),
        quest_points: 3,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Crafting, Xp(1_406.))],
    }
}

/// <https://oldschool.runescape.wiki/w/My_Arm%27s_Big_Adventure>
fn my_arms_big_adventure() -> Quest {
    Quest {
        id: QuestId::MY_ARMS_BIG_ADVENTURE,
        name: "My Arm's Big Adventure".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::EADGARS_RUSE),
            quest_req(QuestId::THE_FEUD),
            quest_req(QuestId::JUNGLE_POTION),
            skill_req(Skill::Farming, Level(29)),
            skill_req(Skill::Woodcutting, Level(10)),
        ]),
        rewards: vec![
            xp_reward(Skill::Herblore, Xp(10_000.)),
            xp_reward(Skill::Farming, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Nature_Spirit>
fn nature_spirit() -> Quest {
    Quest {
        id: QuestId::NATURE_SPIRIT,
        name: "Nature Spirit".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PRIEST_IN_PERIL),
            quest_req(QuestId::THE_RESTLESS_GHOST),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(3_000.)),
            xp_reward(Skill::Defence, Xp(2_000.)),
            xp_reward(Skill::Hitpoints, Xp(2_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/A_Night_at_the_Theatre>
fn a_night_at_the_theatre() -> Quest {
    Quest {
        id: QuestId::A_NIGHT_AT_THE_THEATRE,
        name: "A Night at the Theatre".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([quest_req(QuestId::A_TASTE_OF_HOPE)]),
        rewards: vec![
            // TODO: Figure out how to model 4 antique lamps.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Observatory_Quest>
fn observatory_quest() -> Quest {
    Quest {
        id: QuestId::OBSERVATORY_QUEST,
        name: "Observatory Quest".to_string(),
        quest_points: 2,
        requirements: IndexSet::new(),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(2_250.)),
            // TODO: There's also the random reward, which can contain XP.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Olaf%27s_Quest>
fn olafs_quest() -> Quest {
    Quest {
        id: QuestId::OLAFS_QUEST,
        name: "Olaf's Quest".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_FREMENNIK_TRIALS),
            skill_req(Skill::Firemaking, Level(40)),
            skill_req(Skill::Woodcutting, Level(50)),
        ]),
        rewards: vec![xp_reward(Skill::Defence, Xp(12_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/One_Small_Favour>
fn one_small_favour() -> Quest {
    Quest {
        id: QuestId::ONE_SMALL_FAVOUR,
        name: "One Small Favour".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RUNE_MYSTERIES),
            quest_req(QuestId::SHILO_VILLAGE),
            skill_req(Skill::Agility, Level(36)),
            skill_req(Skill::Crafting, Level(25)),
            skill_req(Skill::Herblore, Level(18)),
            skill_req(Skill::Smithing, Level(30)),
        ]),
        rewards: vec![
            // TODO: Figure out how to model 2 XP lamps.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Pandemonium>
fn pandemonium() -> Quest {
    Quest {
        id: QuestId::PANDEMONIUM,
        name: "Pandemonium".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Sailing, Xp(300.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Path_of_Glouphrie>
fn the_path_of_glouphrie() -> Quest {
    Quest {
        id: QuestId::THE_PATH_OF_GLOUPHRIE,
        name: "The Path of Glouphrie".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_EYES_OF_GLOUPHRIE),
            quest_req(QuestId::WATERFALL_QUEST),
            quest_req(QuestId::TREE_GNOME_VILLAGE),
            skill_req(Skill::Strength, Level(60)),
            skill_req(Skill::Slayer, Level(56)),
            skill_req(Skill::Thieving, Level(56)),
            skill_req(Skill::Ranged, Level(47)),
            skill_req(Skill::Agility, Level(45)),
        ]),
        rewards: vec![
            // Note: These are all received in lamp form.
            xp_reward(Skill::Strength, Xp(30_000.)),
            xp_reward(Skill::Slayer, Xp(20_000.)),
            xp_reward(Skill::Thieving, Xp(5_000.)),
            xp_reward(Skill::Magic, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Perilous_Moons>
fn perilous_moons() -> Quest {
    Quest {
        id: QuestId::PERILOUS_MOONS,
        name: "Perilous Moons".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::TWILIGHTS_PROMISE),
            skill_req(Skill::Slayer, Level(48)),
            skill_req(Skill::Hunter, Level(20)),
            skill_req(Skill::Fishing, Level(20)),
            skill_req(Skill::Runecraft, Level(20)),
            skill_req(Skill::Construction, Level(10)),
        ]),
        rewards: vec![
            xp_reward(Skill::Slayer, Xp(40_000.)),
            xp_reward(Skill::Runecraft, Xp(5_000.)),
            xp_reward(Skill::Hunter, Xp(5_000.)),
            xp_reward(Skill::Fishing, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Plague_City>
fn plague_city() -> Quest {
    Quest {
        id: QuestId::PLAGUE_CITY,
        name: "Plague City".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Mining, Xp(2_425.))],
    }
}

/// <https://oldschool.runescape.wiki/w/A_Porcine_of_Interest>
fn a_porcine_of_interest() -> Quest {
    Quest {
        id: QuestId::A_PORCINE_OF_INTEREST,
        name: "A Porcine of Interest".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Slayer, Xp(1_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Priest_in_Peril>
fn priest_in_peril() -> Quest {
    Quest {
        id: QuestId::PRIEST_IN_PERIL,
        name: "Priest in Peril".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Prayer, Xp(1_406.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Prying_Times>
fn prying_times() -> Quest {
    Quest {
        id: QuestId::PRYING_TIMES,
        name: "Prying Times".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PANDEMONIUM),
            quest_req(QuestId::THE_KNIGHTS_SWORD),
            skill_req(Skill::Smithing, Level(30)),
            skill_req(Skill::Sailing, Level(12)),
        ]),
        rewards: vec![
            xp_reward(Skill::Sailing, Xp(800.)),
            xp_reward(Skill::Smithing, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Queen_of_Thieves>
fn the_queen_of_thieves() -> Quest {
    Quest {
        id: QuestId::THE_QUEEN_OF_THIEVES,
        name: "The Queen of Thieves".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CLIENT_OF_KOUREND),
            skill_req(Skill::Thieving, Level(20)),
        ]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(2_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Rag_and_Bone_Man_I>
fn rag_and_bone_man_i() -> Quest {
    Quest {
        id: QuestId::RAG_AND_BONE_MAN_I,
        name: "Rag and Bone Man I".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(500.)),
            xp_reward(Skill::Prayer, Xp(500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Rag_and_Bone_Man_II>
fn rag_and_bone_man_ii() -> Quest {
    Quest {
        id: QuestId::RAG_AND_BONE_MAN_II,
        name: "Rag and Bone Man II".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RAG_AND_BONE_MAN_I),
            // TODO: Figure out how to model Skippy and the Mogres miniquest requirement.
            quest_req(QuestId::HORROR_FROM_THE_DEEP),
            quest_req(QuestId::CREATURE_OF_FENKENSTRAIN),
            skill_req(Skill::Slayer, Level(40)),
            skill_req(Skill::Defence, Level(20)),
        ]),
        rewards: vec![xp_reward(Skill::Prayer, Xp(5_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Ratcatchers>
fn ratcatchers() -> Quest {
    Quest {
        id: QuestId::RATCATCHERS,
        name: "Ratcatchers".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([quest_req(QuestId::ICTHLARINS_LITTLE_HELPER)]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(4_500.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster>
fn recipe_for_disaster() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER,
        name: "Recipe for Disaster".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_THE_MOUNTAIN_DWARF),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_THE_GOBLIN_GENERALS),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_PIRATE_PETE),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_THE_LUMBRIDGE_GUIDE),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_EVIL_DAVE),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_SKRACH_UGLOGWEE),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_SIR_AMIK_VARZE),
            quest_req(QuestId::RECIPE_FOR_DISASTER_FREEING_KING_AWOWOGEI),
        ]),
        rewards: vec![
            // TODO: Figure out how to model antique lamp.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Another_Cook%27s_Quest>
fn recipe_for_disaster_another_cooks_quest() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST,
        name: "Recipe for Disaster: Another Cook's Quest".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::COOKS_ASSISTANT),
            skill_req(Skill::Cooking, Level(10)),
        ]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_the_Mountain_Dwarf>
fn recipe_for_disaster_freeing_the_mountain_dwarf() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_THE_MOUNTAIN_DWARF,
        name: "Recipe for Disaster: Freeing the Mountain Dwarf".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::FISHING_CONTEST),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(1_000.)),
            xp_reward(Skill::Slayer, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_the_Goblin_generals>
fn recipe_for_disaster_freeing_the_goblin_generals() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_THE_GOBLIN_GENERALS,
        name: "Recipe for Disaster: Freeing the Goblin Generals".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::GOBLIN_DIPLOMACY),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(1_000.)),
            xp_reward(Skill::Farming, Xp(1_000.)),
            xp_reward(Skill::Crafting, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Pirate_Pete>
fn recipe_for_disaster_freeing_pirate_pete() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_PIRATE_PETE,
        name: "Recipe for Disaster: Freeing Pirate Pete".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            skill_req(Skill::Cooking, Level(31)),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(1_000.)),
            xp_reward(Skill::Crafting, Xp(1_000.)),
            xp_reward(Skill::Fishing, Xp(1_000.)),
            xp_reward(Skill::Smithing, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_the_Lumbridge_Guide>
fn recipe_for_disaster_freeing_the_lumbridge_guide() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_THE_LUMBRIDGE_GUIDE,
        name: "Recipe for Disaster: Freeing the Lumbridge Guide".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::BIG_CHOMPY_BIRD_HUNTING),
            quest_req(QuestId::BIOHAZARD),
            quest_req(QuestId::DEMON_SLAYER),
            quest_req(QuestId::MURDER_MYSTERY),
            quest_req(QuestId::NATURE_SPIRIT),
            quest_req(QuestId::WITCHS_HOUSE),
            skill_req(Skill::Cooking, Level(40)),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(2_500.)),
            xp_reward(Skill::Magic, Xp(2_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Evil_Dave>
fn recipe_for_disaster_freeing_evil_dave() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_EVIL_DAVE,
        name: "Recipe for Disaster: Freeing Evil Dave".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::GERTRUDES_CAT),
            quest_req(QuestId::SHADOW_OF_THE_STORM),
            skill_req(Skill::Cooking, Level(25)),
        ]),
        rewards: vec![xp_reward(Skill::Cooking, Xp(7_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Skrach_Uglogwee>
fn recipe_for_disaster_freeing_skrach_uglogwee() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_SKRACH_UGLOGWEE,
        name: "Recipe for Disaster: Freeing Skrach Uglogwee".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::BIG_CHOMPY_BIRD_HUNTING),
            skill_req(Skill::Cooking, Level(41)),
            skill_req(Skill::Firemaking, Level(20)),
        ]),
        rewards: vec![
            xp_reward(Skill::Woodcutting, Xp(1_500.)),
            xp_reward(Skill::Cooking, Xp(1_500.)),
            xp_reward(Skill::Crafting, Xp(1_500.)),
            xp_reward(Skill::Ranged, Xp(1_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_Sir_Amik_Varze>
fn recipe_for_disaster_freeing_sir_amik_varze() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_SIR_AMIK_VARZE,
        name: "Recipe for Disaster: Freeing Sir Amik Varze".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_point_req(107),
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::FAMILY_CREST),
            quest_req(QuestId::HEROES_QUEST),
            quest_req(QuestId::SHILO_VILLAGE),
            quest_req(QuestId::UNDERGROUND_PASS),
            quest_req(QuestId::WATERFALL_QUEST),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(4_000.)),
            xp_reward(Skill::Hitpoints, Xp(4_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recipe_for_Disaster/Freeing_King_Awowogei>
fn recipe_for_disaster_freeing_king_awowogei() -> Quest {
    Quest {
        id: QuestId::RECIPE_FOR_DISASTER_FREEING_KING_AWOWOGEI,
        name: "Recipe for Disaster: Freeing King Awowogei".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RECIPE_FOR_DISASTER_ANOTHER_COOKS_QUEST),
            quest_req(QuestId::MONKEY_MADNESS_I),
            skill_req(Skill::Cooking, Level(70)),
            skill_req(Skill::Agility, Level(48)),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(10_000.)),
            xp_reward(Skill::Agility, Xp(10_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Recruitment_Drive>
fn recruitment_drive() -> Quest {
    Quest {
        id: QuestId::RECRUITMENT_DRIVE,
        name: "Recruitment Drive".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::BLACK_KNIGHTS_FORTRESS),
            quest_req(QuestId::DRUIDIC_RITUAL),
        ]),
        rewards: vec![
            xp_reward(Skill::Prayer, Xp(1_000.5)),
            xp_reward(Skill::Herblore, Xp(1_000.5)),
            xp_reward(Skill::Agility, Xp(1_000.5)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Regicide>
fn regicide() -> Quest {
    Quest {
        id: QuestId::REGICIDE,
        name: "Regicide".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::UNDERGROUND_PASS),
            skill_req(Skill::Crafting, Level(10)),
            skill_req(Skill::Agility, Level(56)),
        ]),
        rewards: vec![xp_reward(Skill::Agility, Xp(13_750.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Ribbiting_Tale_of_a_Lily_Pad_Labour_Dispute>
fn the_ribbiting_tale_of_a_lily_pad_labour_dispute() -> Quest {
    Quest {
        id: QuestId::THE_RIBBITING_TALE_OF_A_LILY_PAD_LABOUR_DISPUTE,
        name: "The Ribbiting Tale of a Lily Pad Labour Dispute".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CHILDREN_OF_THE_SUN),
            skill_req(Skill::Woodcutting, Level(15)),
        ]),
        rewards: vec![xp_reward(Skill::Woodcutting, Xp(2_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Roving_Elves>
fn roving_elves() -> Quest {
    Quest {
        id: QuestId::ROVING_ELVES,
        name: "Roving Elves".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::REGICIDE),
            skill_req(Skill::Agility, Level(56)),
        ]),
        rewards: vec![xp_reward(Skill::Strength, Xp(10_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Royal_Trouble>
fn royal_trouble() -> Quest {
    Quest {
        id: QuestId::ROYAL_TROUBLE,
        name: "Royal Trouble".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THRONE_OF_MISCELLANIA),
            skill_req(Skill::Agility, Level(40)),
            skill_req(Skill::Slayer, Level(40)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(5_000.)),
            xp_reward(Skill::Slayer, Xp(5_000.)),
            xp_reward(Skill::Hitpoints, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Rum_Deal>
fn rum_deal() -> Quest {
    Quest {
        id: QuestId::RUM_DEAL,
        name: "Rum Deal".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::ZOGRE_FLESH_EATERS),
            quest_req(QuestId::PRIEST_IN_PERIL),
            skill_req(Skill::Crafting, Level(42)),
            skill_req(Skill::Fishing, Level(50)),
            skill_req(Skill::Farming, Level(40)),
            skill_req(Skill::Prayer, Level(47)),
            skill_req(Skill::Slayer, Level(42)),
        ]),
        rewards: vec![
            xp_reward(Skill::Fishing, Xp(7_000.)),
            xp_reward(Skill::Prayer, Xp(7_000.)),
            xp_reward(Skill::Farming, Xp(7_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Scorpion_Catcher>
fn scorpion_catcher() -> Quest {
    Quest {
        id: QuestId::SCORPION_CATCHER,
        name: "Scorpion Catcher".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Prayer, Level(31)),
            // TODO: Alfred Grimhand's Barcrawl requirement.
        ]),
        rewards: vec![xp_reward(Skill::Strength, Xp(6_625.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Scrambled!>
fn scrambled() -> Quest {
    Quest {
        id: QuestId::SCRAMBLED,
        name: "Scrambled!".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CHILDREN_OF_THE_SUN),
            skill_req(Skill::Construction, Level(38)),
            skill_req(Skill::Cooking, Level(36)),
            skill_req(Skill::Smithing, Level(35)),
        ]),
        rewards: vec![
            xp_reward(Skill::Construction, Xp(5_000.)),
            xp_reward(Skill::Cooking, Xp(5_000.)),
            xp_reward(Skill::Smithing, Xp(5_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Sea_Slug>
fn sea_slug() -> Quest {
    Quest {
        id: QuestId::SEA_SLUG,
        name: "Sea Slug".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([skill_req(Skill::Firemaking, Level(30))]),
        rewards: vec![xp_reward(Skill::Fishing, Xp(7_175.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Secrets_of_the_North>
fn secrets_of_the_north() -> Quest {
    Quest {
        id: QuestId::SECRETS_OF_THE_NORTH,
        name: "Secrets of the North".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::MAKING_FRIENDS_WITH_MY_ARM),
            quest_req(QuestId::DEVIOUS_MINDS),
            // TODO: The General's Shadow miniquest requirement.
            quest_req(QuestId::HAZEEL_CULT),
            skill_req(Skill::Agility, Level(69)),
            skill_req(Skill::Thieving, Level(64)),
            skill_req(Skill::Hunter, Level(56)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(60_000.)),
            xp_reward(Skill::Thieving, Xp(50_000.)),
            xp_reward(Skill::Hunter, Xp(40_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Shades_of_Mort%27ton>
fn shades_of_mortton() -> Quest {
    Quest {
        id: QuestId::SHADES_OF_MORTTON,
        name: "Shades of Mort'ton".to_string(),
        quest_points: 3,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PRIEST_IN_PERIL),
            skill_req(Skill::Crafting, Level(20)),
            skill_req(Skill::Herblore, Level(15)),
            skill_req(Skill::Firemaking, Level(5)),
        ]),
        rewards: vec![
            xp_reward(Skill::Herblore, Xp(2_000.)),
            xp_reward(Skill::Crafting, Xp(2_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Shadow_of_the_Storm>
fn shadow_of_the_storm() -> Quest {
    Quest {
        id: QuestId::SHADOW_OF_THE_STORM,
        name: "Shadow of the Storm".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::THE_GOLEM),
            quest_req(QuestId::DEMON_SLAYER),
            skill_req(Skill::Crafting, Level(30)),
        ]),
        rewards: vec![
            // TODO: 10,000 experience in any combat skill other than Prayer.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Shadows_of_Custodia>
fn shadows_of_custodia() -> Quest {
    Quest {
        id: QuestId::SHADOWS_OF_CUSTODIA,
        name: "Shadows of Custodia".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CHILDREN_OF_THE_SUN),
            skill_req(Skill::Slayer, Level(54)),
            skill_req(Skill::Fishing, Level(45)),
            skill_req(Skill::Construction, Level(41)),
            skill_req(Skill::Hunter, Level(36)),
        ]),
        rewards: vec![
            xp_reward(Skill::Slayer, Xp(10_000.)),
            xp_reward(Skill::Hunter, Xp(4_000.)),
            xp_reward(Skill::Fishing, Xp(3_000.)),
            xp_reward(Skill::Construction, Xp(3_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Sheep_Herder>
fn sheep_herder() -> Quest {
    Quest {
        id: QuestId::SHEEP_HERDER,
        name: "Sheep Herder".to_string(),
        quest_points: 4,
        requirements: IndexSet::new(),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Shilo_Village>
fn shilo_village() -> Quest {
    Quest {
        id: QuestId::SHILO_VILLAGE,
        name: "Shilo Village".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::JUNGLE_POTION),
            skill_req(Skill::Crafting, Level(20)),
            skill_req(Skill::Agility, Level(32)),
        ]),
        rewards: vec![xp_reward(Skill::Crafting, Xp(3_875.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Sins_of_the_Father>
fn sins_of_the_father() -> Quest {
    Quest {
        id: QuestId::SINS_OF_THE_FATHER,
        name: "Sins of the Father".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::VAMPYRE_SLAYER),
            quest_req(QuestId::A_TASTE_OF_HOPE),
            skill_req(Skill::Woodcutting, Level(62)),
            skill_req(Skill::Fletching, Level(60)),
            skill_req(Skill::Crafting, Level(56)),
            skill_req(Skill::Agility, Level(52)),
            skill_req(Skill::Attack, Level(50)),
            skill_req(Skill::Slayer, Level(50)),
            skill_req(Skill::Magic, Level(49)),
        ]),
        rewards: vec![
            // TODO: 6x tome of experience (15,000 XP to any skill level 60+).
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Sleeping_Giants>
fn sleeping_giants() -> Quest {
    Quest {
        id: QuestId::SLEEPING_GIANTS,
        name: "Sleeping Giants".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([skill_req(Skill::Smithing, Level(15))]),
        rewards: vec![xp_reward(Skill::Smithing, Xp(6_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/The_Slug_Menace>
fn the_slug_menace() -> Quest {
    Quest {
        id: QuestId::THE_SLUG_MENACE,
        name: "The Slug Menace".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::WANTED),
            quest_req(QuestId::SEA_SLUG),
            skill_req(Skill::Crafting, Level(30)),
            skill_req(Skill::Runecraft, Level(30)),
            skill_req(Skill::Slayer, Level(30)),
            skill_req(Skill::Thieving, Level(30)),
        ]),
        rewards: vec![
            xp_reward(Skill::Crafting, Xp(3_500.)),
            xp_reward(Skill::Runecraft, Xp(3_500.)),
            xp_reward(Skill::Thieving, Xp(3_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Song_of_the_Elves>
fn song_of_the_elves() -> Quest {
    Quest {
        id: QuestId::SONG_OF_THE_ELVES,
        name: "Song of the Elves".to_string(),
        quest_points: 4,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::MOURNINGS_END_PART_II),
            skill_req(Skill::Agility, Level(70)),
            skill_req(Skill::Construction, Level(70)),
            skill_req(Skill::Farming, Level(70)),
            skill_req(Skill::Herblore, Level(70)),
            skill_req(Skill::Hunter, Level(70)),
            skill_req(Skill::Mining, Level(70)),
            skill_req(Skill::Smithing, Level(70)),
            skill_req(Skill::Woodcutting, Level(70)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(40_000.)),
            xp_reward(Skill::Construction, Xp(40_000.)),
            xp_reward(Skill::Farming, Xp(40_000.)),
            xp_reward(Skill::Herblore, Xp(40_000.)),
            xp_reward(Skill::Hunter, Xp(40_000.)),
            xp_reward(Skill::Mining, Xp(40_000.)),
            xp_reward(Skill::Smithing, Xp(40_000.)),
            xp_reward(Skill::Woodcutting, Xp(40_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/A_Soul%27s_Bane>
fn a_souls_bane() -> Quest {
    Quest {
        id: QuestId::A_SOULS_BANE,
        name: "A Soul's Bane".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![
            xp_reward(Skill::Defence, Xp(500.)),
            xp_reward(Skill::Hitpoints, Xp(500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Spirits_of_the_Elid>
fn spirits_of_the_elid() -> Quest {
    Quest {
        id: QuestId::SPIRITS_OF_THE_ELID,
        name: "Spirits of the Elid".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Magic, Level(33)),
            skill_req(Skill::Ranged, Level(37)),
            skill_req(Skill::Mining, Level(37)),
            skill_req(Skill::Thieving, Level(37)),
        ]),
        rewards: vec![
            xp_reward(Skill::Prayer, Xp(8_000.)),
            xp_reward(Skill::Thieving, Xp(1_000.)),
            xp_reward(Skill::Magic, Xp(1_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Swan_Song>
fn swan_song() -> Quest {
    Quest {
        id: QuestId::SWAN_SONG,
        name: "Swan Song".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_point_req(100),
            quest_req(QuestId::ONE_SMALL_FAVOUR),
            quest_req(QuestId::GARDEN_OF_TRANQUILLITY),
            skill_req(Skill::Magic, Level(66)),
            skill_req(Skill::Cooking, Level(62)),
            skill_req(Skill::Fishing, Level(62)),
            skill_req(Skill::Smithing, Level(45)),
            skill_req(Skill::Firemaking, Level(42)),
            skill_req(Skill::Crafting, Level(40)),
        ]),
        rewards: vec![
            xp_reward(Skill::Magic, Xp(15_000.)),
            xp_reward(Skill::Prayer, Xp(10_000.)),
            xp_reward(Skill::Fishing, Xp(50_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Tai_Bwo_Wannai_Trio>
fn tai_bwo_wannai_trio() -> Quest {
    Quest {
        id: QuestId::TAI_BWO_WANNAI_TRIO,
        name: "Tai Bwo Wannai Trio".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::JUNGLE_POTION),
            skill_req(Skill::Agility, Level(15)),
            skill_req(Skill::Cooking, Level(30)),
            skill_req(Skill::Fishing, Level(5)),
        ]),
        rewards: vec![
            xp_reward(Skill::Cooking, Xp(5_000.)),
            xp_reward(Skill::Fishing, Xp(5_000.)),
            xp_reward(Skill::Attack, Xp(2_500.)),
            xp_reward(Skill::Strength, Xp(2_500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/A_Tail_of_Two_Cats>
fn a_tail_of_two_cats() -> Quest {
    Quest {
        id: QuestId::A_TAIL_OF_TWO_CATS,
        name: "A Tail of Two Cats".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([quest_req(QuestId::ICTHLARINS_LITTLE_HELPER)]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Tale_of_the_Righteous>
fn tale_of_the_righteous() -> Quest {
    Quest {
        id: QuestId::TALE_OF_THE_RIGHTEOUS,
        name: "Tale of the Righteous".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::CLIENT_OF_KOUREND),
            skill_req(Skill::Strength, Level(16)),
            skill_req(Skill::Mining, Level(10)),
        ]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/A_Taste_of_Hope>
fn a_taste_of_hope() -> Quest {
    Quest {
        id: QuestId::A_TASTE_OF_HOPE,
        name: "A Taste of Hope".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DARKNESS_OF_HALLOWVALE),
            skill_req(Skill::Crafting, Level(48)),
            skill_req(Skill::Agility, Level(45)),
            skill_req(Skill::Attack, Level(40)),
            skill_req(Skill::Herblore, Level(40)),
            skill_req(Skill::Slayer, Level(38)),
        ]),
        rewards: vec![
            // TODO: Figure out how to model tome of experience.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Tears_of_Guthix>
fn tears_of_guthix() -> Quest {
    Quest {
        id: QuestId::TEARS_OF_GUTHIX,
        name: "Tears of Guthix".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_point_req(43),
            skill_req(Skill::Firemaking, Level(49)),
            skill_req(Skill::Crafting, Level(20)),
            skill_req(Skill::Mining, Level(20)),
        ]),
        rewards: vec![xp_reward(Skill::Crafting, Xp(1_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Temple_of_Ikov>
fn temple_of_ikov() -> Quest {
    Quest {
        id: QuestId::TEMPLE_OF_IKOV,
        name: "Temple of Ikov".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Thieving, Level(42)),
            skill_req(Skill::Ranged, Level(40)),
        ]),
        rewards: vec![
            xp_reward(Skill::Ranged, Xp(10_500.)),
            xp_reward(Skill::Fletching, Xp(8_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Temple_of_the_Eye>
fn temple_of_the_eye() -> Quest {
    Quest {
        id: QuestId::TEMPLE_OF_THE_EYE,
        name: "Temple of the Eye".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            // TODO: Figure out how to handle Enter the Abyss requirement.
            quest_req(QuestId::RUNE_MYSTERIES),
            skill_req(Skill::Runecraft, Level(10)),
        ]),
        rewards: vec![xp_reward(Skill::Runecraft, Xp(5_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Throne_of_Miscellania>
fn throne_of_miscellania() -> Quest {
    Quest {
        id: QuestId::THRONE_OF_MISCELLANIA,
        name: "Throne of Miscellania".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::HEROES_QUEST),
            quest_req(QuestId::THE_FREMENNIK_TRIALS),
            skill_req(Skill::Woodcutting, Level(45)),
            skill_req(Skill::Farming, Level(10)),
            skill_req(Skill::Herblore, Level(35)),
        ]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/The_Tourist_Trap>
fn the_tourist_trap() -> Quest {
    Quest {
        id: QuestId::THE_TOURIST_TRAP,
        name: "The Tourist Trap".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Fletching, Level(10)),
            skill_req(Skill::Smithing, Level(20)),
        ]),
        rewards: vec![
            // TODO: Figure out how to model XP choice.
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Tower_of_Life>
fn tower_of_life() -> Quest {
    Quest {
        id: QuestId::TOWER_OF_LIFE,
        name: "Tower of Life".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([skill_req(Skill::Construction, Level(10))]),
        rewards: vec![
            xp_reward(Skill::Construction, Xp(1_000.)),
            xp_reward(Skill::Crafting, Xp(500.)),
            xp_reward(Skill::Thieving, Xp(500.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Tree_Gnome_Village>
fn tree_gnome_village() -> Quest {
    Quest {
        id: QuestId::TREE_GNOME_VILLAGE,
        name: "Tree Gnome Village".to_string(),
        quest_points: 2,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Attack, Xp(11_450.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Tribal_Totem>
fn tribal_totem() -> Quest {
    Quest {
        id: QuestId::TRIBAL_TOTEM,
        name: "Tribal Totem".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([skill_req(Skill::Thieving, Level(21))]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(1_775.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Troll_Romance>
fn troll_romance() -> Quest {
    Quest {
        id: QuestId::TROLL_ROMANCE,
        name: "Troll Romance".to_string(),
        quest_points: 2,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::TROLL_STRONGHOLD),
            skill_req(Skill::Agility, Level(28)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(8_000.)),
            xp_reward(Skill::Strength, Xp(4_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Troll_Stronghold>
fn troll_stronghold() -> Quest {
    Quest {
        id: QuestId::TROLL_STRONGHOLD,
        name: "Troll Stronghold".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::DEATH_PLATEAU),
            skill_req(Skill::Agility, Level(15)),
        ]),
        rewards: Vec::new(),
    }
}

/// <https://oldschool.runescape.wiki/w/Troubled_Tortugans>
fn troubled_tortugans() -> Quest {
    Quest {
        id: QuestId::TROUBLED_TORTUGANS,
        name: "Troubled Tortugans".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::PANDEMONIUM),
            skill_req(Skill::Slayer, Level(51)),
            skill_req(Skill::Construction, Level(48)),
            skill_req(Skill::Sailing, Level(45)),
            skill_req(Skill::Hunter, Level(45)),
            skill_req(Skill::Woodcutting, Level(40)),
            skill_req(Skill::Crafting, Level(34)),
        ]),
        rewards: vec![
            xp_reward(Skill::Sailing, Xp(10_000.)),
            xp_reward(Skill::Slayer, Xp(8_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Twilight's_Promise>
fn twilights_promise() -> Quest {
    Quest {
        id: QuestId::TWILIGHTS_PROMISE,
        name: "Twilight's Promise".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([quest_req(QuestId::CHILDREN_OF_THE_SUN)]),
        rewards: vec![xp_reward(Skill::Thieving, Xp(3_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Underground_Pass>
fn underground_pass() -> Quest {
    Quest {
        id: QuestId::UNDERGROUND_PASS,
        name: "Underground Pass".to_string(),
        quest_points: 5,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::BIOHAZARD),
            skill_req(Skill::Ranged, Level(25)),
        ]),
        rewards: vec![
            xp_reward(Skill::Agility, Xp(3_000.)),
            xp_reward(Skill::Attack, Xp(3_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Wanted!>
fn wanted() -> Quest {
    Quest {
        id: QuestId::WANTED,
        name: "Wanted!".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_point_req(32),
            quest_req(QuestId::RECRUITMENT_DRIVE),
            quest_req(QuestId::THE_LOST_TRIBE),
            quest_req(QuestId::PRIEST_IN_PERIL),
            // TODO: Figure out how to model Enter the Abyss requirement.
        ]),
        rewards: vec![xp_reward(Skill::Slayer, Xp(5_000.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Watchtower>
fn watchtower() -> Quest {
    Quest {
        id: QuestId::WATCHTOWER,
        name: "Watchtower".to_string(),
        quest_points: 4,
        requirements: IndexSet::from_iter([
            skill_req(Skill::Magic, Level(14)),
            skill_req(Skill::Thieving, Level(15)),
            skill_req(Skill::Agility, Level(25)),
            skill_req(Skill::Herblore, Level(14)),
            skill_req(Skill::Mining, Level(40)),
        ]),
        rewards: vec![xp_reward(Skill::Magic, Xp(15_250.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Waterfall_Quest>
fn waterfall_quest() -> Quest {
    Quest {
        id: QuestId::WATERFALL_QUEST,
        name: "Waterfall Quest".to_string(),
        quest_points: 1,
        requirements: IndexSet::new(),
        rewards: vec![
            xp_reward(Skill::Strength, Xp(13_750.)),
            xp_reward(Skill::Attack, Xp(13_750.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/What_Lies_Below>
fn what_lies_below() -> Quest {
    Quest {
        id: QuestId::WHAT_LIES_BELOW,
        name: "What Lies Below".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::RUNE_MYSTERIES),
            skill_req(Skill::Runecraft, Level(35)),
        ]),
        rewards: vec![
            xp_reward(Skill::Runecraft, Xp(8_000.)),
            xp_reward(Skill::Defence, Xp(2_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/While_Guthix_Sleeps>
fn while_guthix_sleeps() -> Quest {
    Quest {
        id: QuestId::WHILE_GUTHIX_SLEEPS,
        name: "While Guthix Sleeps".to_string(),
        quest_points: 5,
        requirements: IndexSet::from_iter([
            quest_point_req(180),
            quest_req(QuestId::DEFENDER_OF_VARROCK),
            quest_req(QuestId::THE_PATH_OF_GLOUPHRIE),
            quest_req(QuestId::FIGHT_ARENA),
            quest_req(QuestId::DREAM_MENTOR),
            quest_req(QuestId::THE_HAND_IN_THE_SAND),
            quest_req(QuestId::WANTED),
            quest_req(QuestId::TEMPLE_OF_THE_EYE),
            quest_req(QuestId::TEARS_OF_GUTHIX),
            quest_req(QuestId::A_TAIL_OF_TWO_CATS),
            skill_req(Skill::Thieving, Level(72)),
            skill_req(Skill::Magic, Level(67)),
            skill_req(Skill::Agility, Level(66)),
            skill_req(Skill::Farming, Level(65)),
            skill_req(Skill::Herblore, Level(65)),
            skill_req(Skill::Hunter, Level(62)),
            // TODO: Warriors' Guild requirement (Attack + Strength >= 130 OR 99 Attack OR 99 Strength)
        ]),
        rewards: vec![
            xp_reward(Skill::Thieving, Xp(80_000.)),
            xp_reward(Skill::Farming, Xp(75_000.)),
            xp_reward(Skill::Herblore, Xp(75_000.)),
            xp_reward(Skill::Hunter, Xp(50_000.)),
        ],
    }
}

/// <https://oldschool.runescape.wiki/w/Witch%27s_House>
fn witchs_house() -> Quest {
    Quest {
        id: QuestId::WITCHS_HOUSE,
        name: "Witch's House".to_string(),
        quest_points: 4,
        requirements: IndexSet::new(),
        rewards: vec![xp_reward(Skill::Hitpoints, Xp(6_325.))],
    }
}

/// <https://oldschool.runescape.wiki/w/Zogre_Flesh_Eaters>
fn zogre_flesh_eaters() -> Quest {
    Quest {
        id: QuestId::ZOGRE_FLESH_EATERS,
        name: "Zogre Flesh Eaters".to_string(),
        quest_points: 1,
        requirements: IndexSet::from_iter([
            quest_req(QuestId::BIG_CHOMPY_BIRD_HUNTING),
            quest_req(QuestId::JUNGLE_POTION),
            skill_req(Skill::Smithing, Level(4)),
            skill_req(Skill::Herblore, Level(8)),
            skill_req(Skill::Ranged, Level(30)),
        ]),
        rewards: vec![
            xp_reward(Skill::Fletching, Xp(2_000.)),
            xp_reward(Skill::Ranged, Xp(2_000.)),
            xp_reward(Skill::Herblore, Xp(2_000.)),
        ],
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_all_quests_are_defined() {
        const QUEST_COUNT: usize = 177;
        const RFD_QUEST_COUNT: usize = 9;

        assert_eq!(
            QuestId::all().len(),
            QUEST_COUNT + RFD_QUEST_COUNT,
            "mismatched quest count"
        );

        for quest_id in QuestId::all() {
            quest_id.quest().unwrap();
        }
    }

    #[test]
    fn test_total_quest_points() {
        let total_quest_points = QuestId::all()
            .into_iter()
            .map(|id| id.quest().unwrap().quest_points as u32)
            .sum::<u32>();
        assert_eq!(total_quest_points, 331);
    }

    #[test]
    fn test_quest_not_found_error() {
        let quest_id = QuestId::new_static("non_existent_quest");
        let result = quest_id.quest();
        assert_eq!(
            result.expect_err("expected quest to not exist").to_string(),
            r#"quest not found: QuestId("non_existent_quest")"#
        );
    }
}
