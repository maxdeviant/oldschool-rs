#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DiaryId(&'static str);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DiaryDifficulty {
    Easy,
    Medium,
    Hard,
    Elite,
}

#[derive(Debug)]
pub struct Diary {
    id: DiaryId,
    name: String,
    difficulty: DiaryDifficulty,
    tasks: Vec<Task>,
}

impl Diary {
    pub fn id(&self) -> DiaryId {
        self.id
    }
}

#[derive(Debug)]
pub struct Task {}

pub const EASY_ARDOUGNE_DIARY: DiaryId = DiaryId("ardougne_easy");

pub const MEDIUM_ARDOUGNE_DIARY: DiaryId = DiaryId("ardougne_medium");

pub const HARD_ARDOUGNE_DIARY: DiaryId = DiaryId("ardougne_hard");

pub const ELITE_ARDOUGNE_DIARY: DiaryId = DiaryId("ardougne_elite");

pub const EASY_DESERT_DIARY: DiaryId = DiaryId("desert_easy");

pub const MEDIUM_DESERT_DIARY: DiaryId = DiaryId("desert_medium");

pub const HARD_DESERT_DIARY: DiaryId = DiaryId("desert_hard");

pub const ELITE_DESERT_DIARY: DiaryId = DiaryId("desert_elite");

pub const EASY_FALADOR_DIARY: DiaryId = DiaryId("falador_easy");

pub const MEDIUM_FALADOR_DIARY: DiaryId = DiaryId("falador_medium");

pub const HARD_FALADOR_DIARY: DiaryId = DiaryId("falador_hard");

pub const ELITE_FALADOR_DIARY: DiaryId = DiaryId("falador_elite");

pub const EASY_FREMENNIK_DIARY: DiaryId = DiaryId("fremennik_easy");

pub const MEDIUM_FREMENNIK_DIARY: DiaryId = DiaryId("fremennik_medium");

pub const HARD_FREMENNIK_DIARY: DiaryId = DiaryId("fremennik_hard");

pub const ELITE_FREMENNIK_DIARY: DiaryId = DiaryId("fremennik_elite");

pub const EASY_KANDARIN_DIARY: DiaryId = DiaryId("kandarin_easy");

pub const MEDIUM_KANDARIN_DIARY: DiaryId = DiaryId("kandarin_medium");

pub const HARD_KANDARIN_DIARY: DiaryId = DiaryId("kandarin_hard");

pub const ELITE_KANDARIN_DIARY: DiaryId = DiaryId("kandarin_elite");

pub const EASY_KARAMJA_DIARY: DiaryId = DiaryId("karamja_easy");

pub const MEDIUM_KARAMJA_DIARY: DiaryId = DiaryId("karamja_medium");

pub const HARD_KARAMJA_DIARY: DiaryId = DiaryId("karamja_hard");

pub const ELITE_KARAMJA_DIARY: DiaryId = DiaryId("karamja_elite");

pub const EASY_KOUREND_AND_KEBOS_DIARY: DiaryId = DiaryId("kourend_and_kebos_easy");

pub const MEDIUM_KOUREND_AND_KEBOS_DIARY: DiaryId = DiaryId("kourend_and_kebos_medium");

pub const HARD_KOUREND_AND_KEBOS_DIARY: DiaryId = DiaryId("kourend_and_kebos_hard");

pub const ELITE_KOUREND_AND_KEBOS_DIARY: DiaryId = DiaryId("kourend_and_kebos_elite");

pub const EASY_LUMBRIDGE_AND_DRAYNOR_DIARY: DiaryId = DiaryId("lumbridge_and_draynor_easy");

pub const MEDIUM_LUMBRIDGE_AND_DRAYNOR_DIARY: DiaryId = DiaryId("lumbridge_and_draynor_medium");

pub const HARD_LUMBRIDGE_AND_DRAYNOR_DIARY: DiaryId = DiaryId("lumbridge_and_draynor_hard");

pub const ELITE_LUMBRIDGE_AND_DRAYNOR_DIARY: DiaryId = DiaryId("lumbridge_and_draynor_elite");

pub const EASY_MORYTANIA_DIARY: DiaryId = DiaryId("morytania_easy");

pub const MEDIUM_MORYTANIA_DIARY: DiaryId = DiaryId("morytania_medium");

pub const HARD_MORYTANIA_DIARY: DiaryId = DiaryId("morytania_hard");

pub const ELITE_MORYTANIA_DIARY: DiaryId = DiaryId("morytania_elite");

pub const EASY_VARROCK_DIARY: DiaryId = DiaryId("varrock_easy");

pub const MEDIUM_VARROCK_DIARY: DiaryId = DiaryId("varrock_medium");

pub const HARD_VARROCK_DIARY: DiaryId = DiaryId("varrock_hard");

pub const ELITE_VARROCK_DIARY: DiaryId = DiaryId("varrock_elite");

pub const EASY_WESTERN_PROVINCES_DIARY: DiaryId = DiaryId("western_provinces_easy");

pub const MEDIUM_WESTERN_PROVINCES_DIARY: DiaryId = DiaryId("western_provinces_medium");

pub const HARD_WESTERN_PROVINCES_DIARY: DiaryId = DiaryId("western_provinces_hard");

pub const ELITE_WESTERN_PROVINCES_DIARY: DiaryId = DiaryId("western_provinces_elite");

pub const EASY_WILDERNESS_DIARY: DiaryId = DiaryId("wilderness_easy");

pub const MEDIUM_WILDERNESS_DIARY: DiaryId = DiaryId("wilderness_medium");

pub const HARD_WILDERNESS_DIARY: DiaryId = DiaryId("wilderness_hard");

pub const ELITE_WILDERNESS_DIARY: DiaryId = DiaryId("wilderness_elite");
