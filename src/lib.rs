use std::collections::HashMap;

use lazy_static::lazy_static;

pub mod wintertodt;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Level(pub i32);

lazy_static! {
    static ref XP_TABLE: HashMap<Level, Xp> = vec![
        (Level(1), Xp(0.0)),
        (Level(2), Xp(83.0)),
        (Level(3), Xp(174.0)),
        (Level(4), Xp(276.0)),
        (Level(5), Xp(388.0)),
        (Level(6), Xp(512.0)),
        (Level(7), Xp(650.0)),
        (Level(8), Xp(801.0)),
        (Level(9), Xp(969.0)),
        (Level(10), Xp(1_154.0)),
        (Level(11), Xp(1_358.0)),
        (Level(12), Xp(1_584.0)),
        (Level(13), Xp(1_833.0)),
        (Level(14), Xp(2_107.0)),
        (Level(15), Xp(2_411.0)),
        (Level(16), Xp(2_746.0)),
        (Level(17), Xp(3_115.0)),
        (Level(18), Xp(3_523.0)),
        (Level(19), Xp(3_973.0)),
        (Level(20), Xp(4_470.0)),
        (Level(21), Xp(5_018.0)),
        (Level(22), Xp(5_624.0)),
        (Level(23), Xp(6_291.0)),
        (Level(24), Xp(7_028.0)),
        (Level(25), Xp(7_842.0)),
        (Level(26), Xp(8_740.0)),
        (Level(27), Xp(9_730.0)),
        (Level(28), Xp(10_824.0)),
        (Level(29), Xp(12_031.0)),
        (Level(30), Xp(13_363.0)),
        (Level(31), Xp(14_833.0)),
        (Level(32), Xp(16_456.0)),
        (Level(33), Xp(18_247.0)),
        (Level(34), Xp(20_224.0)),
        (Level(35), Xp(22_406.0)),
        (Level(36), Xp(24_815.0)),
        (Level(37), Xp(27_473.0)),
        (Level(38), Xp(30_408.0)),
        (Level(39), Xp(33_648.0)),
        (Level(40), Xp(37_224.0)),
        (Level(41), Xp(41_171.0)),
        (Level(42), Xp(45_529.0)),
        (Level(43), Xp(50_339.0)),
        (Level(44), Xp(55_649.0)),
        (Level(45), Xp(61_512.0)),
        (Level(46), Xp(67_983.0)),
        (Level(47), Xp(75_127.0)),
        (Level(48), Xp(83_014.0)),
        (Level(49), Xp(91_721.0)),
        (Level(50), Xp(101_333.0)),
        (Level(51), Xp(111_945.0)),
        (Level(52), Xp(123_660.0)),
        (Level(53), Xp(136_594.0)),
        (Level(54), Xp(150_872.0)),
        (Level(55), Xp(166_636.0)),
        (Level(56), Xp(184_040.0)),
        (Level(57), Xp(203_254.0)),
        (Level(58), Xp(224_466.0)),
        (Level(59), Xp(247_886.0)),
        (Level(60), Xp(273_742.0)),
        (Level(61), Xp(302_288.0)),
        (Level(62), Xp(333_804.0)),
        (Level(63), Xp(368_599.0)),
        (Level(64), Xp(407_015.0)),
        (Level(65), Xp(449_428.0)),
        (Level(66), Xp(496_254.0)),
        (Level(67), Xp(547_953.0)),
        (Level(68), Xp(605_032.0)),
        (Level(69), Xp(668_051.0)),
        (Level(70), Xp(737_627.0)),
        (Level(71), Xp(814_445.0)),
        (Level(72), Xp(899_257.0)),
        (Level(73), Xp(992_895.0)),
        (Level(74), Xp(1_096_278.0)),
        (Level(75), Xp(1_210_421.0)),
        (Level(76), Xp(1_336_443.0)),
        (Level(77), Xp(1_475_581.0)),
        (Level(78), Xp(1_629_200.0)),
        (Level(79), Xp(1_798_808.0)),
        (Level(80), Xp(1_986_068.0)),
        (Level(81), Xp(2_192_818.0)),
        (Level(82), Xp(2_421_087.0)),
        (Level(83), Xp(2_673_114.0)),
        (Level(84), Xp(2_951_373.0)),
        (Level(85), Xp(3_258_594.0)),
        (Level(86), Xp(3_597_792.0)),
        (Level(87), Xp(3_972_294.0)),
        (Level(88), Xp(4_385_776.0)),
        (Level(89), Xp(4_842_295.0)),
        (Level(90), Xp(5_346_332.0)),
        (Level(91), Xp(5_902_831.0)),
        (Level(92), Xp(6_517_253.0)),
        (Level(93), Xp(7_195_629.0)),
        (Level(94), Xp(7_944_614.0)),
        (Level(95), Xp(8_771_558.0)),
        (Level(96), Xp(9_684_577.0)),
        (Level(97), Xp(10_692_629.0)),
        (Level(98), Xp(11_805_606.0)),
        (Level(99), Xp(13_034_431.0)),
    ]
    .into_iter()
    .collect();
}

fn xp_for_level(level: &Level) -> Xp {
    *XP_TABLE.get(&level).unwrap()
}

pub fn level_from_xp(xp: Xp) -> Level {
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

pub fn xp_til_level(xp: Xp, target_level: &Level) -> Xp {
    Xp::max(xp_for_level(target_level) - xp, Xp(0.0))
}

pub fn progress_towards_level(xp: Xp, target_level: &Level, starting_level: Option<&Level>) -> f32 {
    let current_level = level_from_xp(xp);
    if &current_level >= target_level {
        return 100.0;
    }

    let starting_xp = starting_level
        .map(|starting_level| {
            if starting_level > &current_level {
                Xp(0.0)
            } else {
                xp - xp_for_level(starting_level)
            }
        })
        .unwrap_or(Xp(0.0));

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
    fn test_xp_til_level() {
        assert_eq!(xp_til_level(Xp(1_000.0), &Level(10)), Xp(154.0));
    }

    #[test]
    fn test_xp_til_level_with_passed_level() {
        assert_eq!(xp_til_level(Xp(1_629_200.0), &Level(30)), Xp(0.0));
    }
}
