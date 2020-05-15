use std::collections::BTreeMap;

use lazy_static::lazy_static;

pub mod accounts;
pub mod optimal_quest_guide;
pub mod quests;
pub mod skills;
pub mod wintertodt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Xp(pub f32);

impl Xp {
    fn max(self, other: Self) -> Self {
        Xp(f32::max(self.0, other.0))
    }
}

impl std::ops::Sub<Xp> for Xp {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Level(pub i32);

impl Default for Level {
    fn default() -> Self {
        Level(1)
    }
}

impl From<Xp> for Level {
    fn from(xp: Xp) -> Self {
        level_from_xp(xp)
    }
}

lazy_static! {
    static ref XP_TABLE: BTreeMap<Level, Xp> = vec![
        (1, 0.0),
        (2, 83.0),
        (3, 174.0),
        (4, 276.0),
        (5, 388.0),
        (6, 512.0),
        (7, 650.0),
        (8, 801.0),
        (9, 969.0),
        (10, 1_154.0),
        (11, 1_358.0),
        (12, 1_584.0),
        (13, 1_833.0),
        (14, 2_107.0),
        (15, 2_411.0),
        (16, 2_746.0),
        (17, 3_115.0),
        (18, 3_523.0),
        (19, 3_973.0),
        (20, 4_470.0),
        (21, 5_018.0),
        (22, 5_624.0),
        (23, 6_291.0),
        (24, 7_028.0),
        (25, 7_842.0),
        (26, 8_740.0),
        (27, 9_730.0),
        (28, 10_824.0),
        (29, 12_031.0),
        (30, 13_363.0),
        (31, 14_833.0),
        (32, 16_456.0),
        (33, 18_247.0),
        (34, 20_224.0),
        (35, 22_406.0),
        (36, 24_815.0),
        (37, 27_473.0),
        (38, 30_408.0),
        (39, 33_648.0),
        (40, 37_224.0),
        (41, 41_171.0),
        (42, 45_529.0),
        (43, 50_339.0),
        (44, 55_649.0),
        (45, 61_512.0),
        (46, 67_983.0),
        (47, 75_127.0),
        (48, 83_014.0),
        (49, 91_721.0),
        (50, 101_333.0),
        (51, 111_945.0),
        (52, 123_660.0),
        (53, 136_594.0),
        (54, 150_872.0),
        (55, 166_636.0),
        (56, 184_040.0),
        (57, 203_254.0),
        (58, 224_466.0),
        (59, 247_886.0),
        (60, 273_742.0),
        (61, 302_288.0),
        (62, 333_804.0),
        (63, 368_599.0),
        (64, 407_015.0),
        (65, 449_428.0),
        (66, 496_254.0),
        (67, 547_953.0),
        (68, 605_032.0),
        (69, 668_051.0),
        (70, 737_627.0),
        (71, 814_445.0),
        (72, 899_257.0),
        (73, 992_895.0),
        (74, 1_096_278.0),
        (75, 1_210_421.0),
        (76, 1_336_443.0),
        (77, 1_475_581.0),
        (78, 1_629_200.0),
        (79, 1_798_808.0),
        (80, 1_986_068.0),
        (81, 2_192_818.0),
        (82, 2_421_087.0),
        (83, 2_673_114.0),
        (84, 2_951_373.0),
        (85, 3_258_594.0),
        (86, 3_597_792.0),
        (87, 3_972_294.0),
        (88, 4_385_776.0),
        (89, 4_842_295.0),
        (90, 5_346_332.0),
        (91, 5_902_831.0),
        (92, 6_517_253.0),
        (93, 7_195_629.0),
        (94, 7_944_614.0),
        (95, 8_771_558.0),
        (96, 9_684_577.0),
        (97, 10_692_629.0),
        (98, 11_805_606.0),
        (99, 13_034_431.0),
    ]
    .into_iter()
    .map(|(level, xp)| (Level(level), Xp(xp)))
    .collect();
}

fn xp_for_level(level: &Level) -> Xp {
    *XP_TABLE.get(&level).unwrap()
}

pub fn level_from_xp(xp: Xp) -> Level {
    for (level, xp_for_level) in XP_TABLE.iter().rev() {
        if (xp_for_level <= &xp) {
            return *level;
        }
    }

    return Level::default();
}

pub fn xp_til_level(xp: Xp, target_level: &Level) -> Xp {
    Xp::max(xp_for_level(target_level) - xp, Xp::default())
}

pub fn progress_towards_level(xp: Xp, target_level: &Level, starting_level: Option<&Level>) -> f32 {
    let current_level = level_from_xp(xp);
    if &current_level >= target_level {
        return 100.0;
    }

    let starting_xp = starting_level
        .map(|starting_level| {
            if starting_level > &current_level {
                Xp::default()
            } else {
                xp - xp_for_level(starting_level)
            }
        })
        .unwrap_or_default();

    (xp - starting_xp).0 / xp_for_level(target_level).0
}

#[cfg(test)]
mod tests {
    use super::{level_from_xp, xp_til_level, Level, Xp};

    #[test]
    fn test_level_from_xp_with_0_xp() {
        assert_eq!(level_from_xp(Xp(0.0)), Level(1));
    }

    #[test]
    fn test_level_from_xp_with_negative_xp() {
        assert_eq!(level_from_xp(Xp(-200.0)), Level(1));
    }

    #[test]
    fn test_level_from_xp_with_10_000xp() {
        assert_eq!(level_from_xp(Xp(10_000.0)), Level(27));
    }

    #[test]
    fn test_level_from_xp_with_13_034_431xp() {
        assert_eq!(level_from_xp(Xp(13_034_431.0)), Level(99));
    }

    #[test]
    fn test_xp_til_level() {
        assert_eq!(xp_til_level(Xp(1_000.0), &Level(10)), Xp(154.0));
    }

    #[test]
    fn test_xp_til_level_with_passed_level() {
        assert_eq!(xp_til_level(Xp(1_629_200.0), &Level(30)), Xp(0.0));
    }
}
