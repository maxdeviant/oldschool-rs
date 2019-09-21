use std::collections::HashMap;

use lazy_static::lazy_static;

pub mod wintertodt;

lazy_static! {
    static ref XP_TABLE: HashMap<i32, i32> = vec![
        (1, 0),
        (2, 83),
        (3, 174),
        (4, 276),
        (5, 388),
        (6, 512),
        (7, 650),
        (8, 801),
        (9, 969),
        (10, 1_154),
        (11, 1_358),
        (12, 1_584),
        (13, 1_833),
        (14, 2_107),
        (15, 2_411),
        (16, 2_746),
        (17, 3_115),
        (18, 3_523),
        (19, 3_973),
        (20, 4_470),
        (21, 5_018),
        (22, 5_624),
        (23, 6_291),
        (24, 7_028),
        (25, 7_842),
        (26, 8_740),
        (27, 9_730),
        (28, 10_824),
        (29, 12_031),
        (30, 13_363),
        (31, 14_833),
        (32, 16_456),
        (33, 18_247),
        (34, 20_224),
        (35, 22_406),
        (36, 24_815),
        (37, 27_473),
        (38, 30_408),
        (39, 33_648),
        (40, 37_224),
        (41, 41_171),
        (42, 45_529),
        (43, 50_339),
        (44, 55_649),
        (45, 61_512),
        (46, 67_983),
        (47, 75_127),
        (48, 83_014),
        (49, 91_721),
        (50, 101_333),
        (51, 111_945),
        (52, 123_660),
        (53, 136_594),
        (54, 150_872),
        (55, 166_636),
        (56, 184_040),
        (57, 203_254),
        (58, 224_466),
        (59, 247_886),
        (60, 273_742),
        (61, 302_288),
        (62, 333_804),
        (63, 368_599),
        (64, 407_015),
        (65, 449_428),
        (66, 496_254),
        (67, 547_953),
        (68, 605_032),
        (69, 668_051),
        (70, 737_627),
        (71, 814_445),
        (72, 899_257),
        (73, 992_895),
        (74, 1_096_278),
        (75, 1_210_421),
        (76, 1_336_443),
        (77, 1_475_581),
        (78, 1_629_200),
        (79, 1_798_808),
        (80, 1_986_068),
        (81, 2_192_818),
        (82, 2_421_087),
        (83, 2_673_114),
        (84, 2_951_373),
        (85, 3_258_594),
        (86, 3_597_792),
        (87, 3_972_294),
        (88, 4_385_776),
        (89, 4_842_295),
        (90, 5_346_332),
        (91, 5_902_831),
        (92, 6_517_253),
        (93, 7_195_629),
        (94, 7_944_614),
        (95, 8_771_558),
        (96, 9_684_577),
        (97, 10_692_629),
        (98, 11_805_606),
        (99, 13_034_431),
    ]
    .into_iter()
    .collect();
}

fn xp_for_level(level: i32) -> f32 {
    XP_TABLE.get(&level).unwrap().clone() as f32
}

pub fn level_from_xp(xp: f32) -> i32 {
    for level in 1..99 {
        if xp < xp_for_level(level) {
            return if level > 1 { level - 1 } else { 1 };
        }
    }

    unreachable!()
}

pub fn xp_til_level(xp: f32, target_level: i32) -> f32 {
    f32::max(xp_for_level(target_level) - xp, 0.0)
}

pub fn progress_towards_level(xp: f32, target_level: i32, starting_level: Option<i32>) -> f32 {
    let current_level = level_from_xp(xp);
    if current_level >= target_level {
        return 100.0;
    }

    let starting_xp = starting_level
        .map(|starting_level| {
            if starting_level > current_level {
                0.0
            } else {
                xp - xp_for_level(starting_level)
            }
        })
        .unwrap_or(0.0);

    (xp - starting_xp) / xp_for_level(target_level)
}

#[cfg(test)]
mod tests {
    use super::{level_from_xp, xp_til_level};

    #[test]
    fn test_level_from_xp_with_0_xp() {
        assert_eq!(level_from_xp(0.0), 1);
    }

    #[test]
    fn test_level_from_xp_with_negative_xp() {
        assert_eq!(level_from_xp(-200.0), 1);
    }

    #[test]
    fn test_level_from_xp_with_10_000xp() {
        assert_eq!(level_from_xp(10_000.0), 27);
    }

    #[test]
    fn test_xp_til_level() {
        assert_eq!(xp_til_level(1_000.0, 10), 154.0);
    }

    #[test]
    fn test_xp_til_level_with_passed_level() {
        assert_eq!(xp_til_level(1_629_200.0, 30), 0.0);
    }
}
