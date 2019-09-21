use std::collections::HashMap;

use lazy_static::lazy_static;

pub mod wintertodt;

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Level(pub i32);

lazy_static! {
    static ref XP_TABLE: HashMap<Level, i32> = vec![
        (Level(1), 0),
        (Level(2), 83),
        (Level(3), 174),
        (Level(4), 276),
        (Level(5), 388),
        (Level(6), 512),
        (Level(7), 650),
        (Level(8), 801),
        (Level(9), 969),
        (Level(10), 1_154),
        (Level(11), 1_358),
        (Level(12), 1_584),
        (Level(13), 1_833),
        (Level(14), 2_107),
        (Level(15), 2_411),
        (Level(16), 2_746),
        (Level(17), 3_115),
        (Level(18), 3_523),
        (Level(19), 3_973),
        (Level(20), 4_470),
        (Level(21), 5_018),
        (Level(22), 5_624),
        (Level(23), 6_291),
        (Level(24), 7_028),
        (Level(25), 7_842),
        (Level(26), 8_740),
        (Level(27), 9_730),
        (Level(28), 10_824),
        (Level(29), 12_031),
        (Level(30), 13_363),
        (Level(31), 14_833),
        (Level(32), 16_456),
        (Level(33), 18_247),
        (Level(34), 20_224),
        (Level(35), 22_406),
        (Level(36), 24_815),
        (Level(37), 27_473),
        (Level(38), 30_408),
        (Level(39), 33_648),
        (Level(40), 37_224),
        (Level(41), 41_171),
        (Level(42), 45_529),
        (Level(43), 50_339),
        (Level(44), 55_649),
        (Level(45), 61_512),
        (Level(46), 67_983),
        (Level(47), 75_127),
        (Level(48), 83_014),
        (Level(49), 91_721),
        (Level(50), 101_333),
        (Level(51), 111_945),
        (Level(52), 123_660),
        (Level(53), 136_594),
        (Level(54), 150_872),
        (Level(55), 166_636),
        (Level(56), 184_040),
        (Level(57), 203_254),
        (Level(58), 224_466),
        (Level(59), 247_886),
        (Level(60), 273_742),
        (Level(61), 302_288),
        (Level(62), 333_804),
        (Level(63), 368_599),
        (Level(64), 407_015),
        (Level(65), 449_428),
        (Level(66), 496_254),
        (Level(67), 547_953),
        (Level(68), 605_032),
        (Level(69), 668_051),
        (Level(70), 737_627),
        (Level(71), 814_445),
        (Level(72), 899_257),
        (Level(73), 992_895),
        (Level(74), 1_096_278),
        (Level(75), 1_210_421),
        (Level(76), 1_336_443),
        (Level(77), 1_475_581),
        (Level(78), 1_629_200),
        (Level(79), 1_798_808),
        (Level(80), 1_986_068),
        (Level(81), 2_192_818),
        (Level(82), 2_421_087),
        (Level(83), 2_673_114),
        (Level(84), 2_951_373),
        (Level(85), 3_258_594),
        (Level(86), 3_597_792),
        (Level(87), 3_972_294),
        (Level(88), 4_385_776),
        (Level(89), 4_842_295),
        (Level(90), 5_346_332),
        (Level(91), 5_902_831),
        (Level(92), 6_517_253),
        (Level(93), 7_195_629),
        (Level(94), 7_944_614),
        (Level(95), 8_771_558),
        (Level(96), 9_684_577),
        (Level(97), 10_692_629),
        (Level(98), 11_805_606),
        (Level(99), 13_034_431),
    ]
    .into_iter()
    .collect();
}

fn xp_for_level(level: &Level) -> f32 {
    XP_TABLE.get(&level).unwrap().clone() as f32
}

pub fn level_from_xp(xp: f32) -> Level {
    for level in 1..99 {
        if xp < xp_for_level(&Level(level)) {
            return if level > 1 {
                Level(level - 1)
            } else {
                Level(1)
            };
        }
    }

    unreachable!()
}

pub fn xp_til_level(xp: f32, target_level: &Level) -> f32 {
    f32::max(xp_for_level(target_level) - xp, 0.0)
}

pub fn progress_towards_level(
    xp: f32,
    target_level: &Level,
    starting_level: Option<&Level>,
) -> f32 {
    let current_level = level_from_xp(xp);
    if &current_level >= target_level {
        return 100.0;
    }

    let starting_xp = starting_level
        .map(|starting_level| {
            if starting_level > &current_level {
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
    use super::{level_from_xp, xp_til_level, Level};

    #[test]
    fn test_level_from_xp_with_0_xp() {
        assert_eq!(level_from_xp(0.0), Level(1));
    }

    #[test]
    fn test_level_from_xp_with_negative_xp() {
        assert_eq!(level_from_xp(-200.0), Level(1));
    }

    #[test]
    fn test_level_from_xp_with_10_000xp() {
        assert_eq!(level_from_xp(10_000.0), Level(27));
    }

    #[test]
    fn test_xp_til_level() {
        assert_eq!(xp_til_level(1_000.0, &Level(10)), 154.0);
    }

    #[test]
    fn test_xp_til_level_with_passed_level() {
        assert_eq!(xp_til_level(1_629_200.0, &Level(30)), 0.0);
    }
}
