#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

pub mod wintertodt;

lazy_static! {
    static ref XP_TABLE: HashMap<i32, i32> = {
        let mut xp_table = HashMap::new();
        xp_table.insert(1, 0);
        xp_table.insert(2, 83);
        xp_table.insert(3, 174);
        xp_table.insert(4, 276);
        xp_table.insert(5, 388);
        xp_table.insert(6, 512);
        xp_table.insert(7, 650);
        xp_table.insert(8, 801);
        xp_table.insert(9, 969);
        xp_table.insert(10, 1_154);
        xp_table.insert(11, 1_358);
        xp_table.insert(12, 1_584);
        xp_table.insert(13, 1_833);
        xp_table.insert(14, 2_107);
        xp_table.insert(15, 2_411);
        xp_table.insert(16, 2_746);
        xp_table.insert(17, 3_115);
        xp_table.insert(18, 3_523);
        xp_table.insert(19, 3_973);
        xp_table.insert(20, 4_470);
        xp_table.insert(21, 5_018);
        xp_table.insert(22, 5_624);
        xp_table.insert(23, 6_291);
        xp_table.insert(24, 7_028);
        xp_table.insert(25, 7_842);
        xp_table.insert(26, 8_740);
        xp_table.insert(27, 9_730);
        xp_table.insert(28, 10_824);
        xp_table.insert(29, 12_031);
        xp_table.insert(30, 13_363);
        xp_table.insert(31, 14_833);
        xp_table.insert(32, 16_456);
        xp_table.insert(33, 18_247);
        xp_table.insert(34, 20_224);
        xp_table.insert(35, 22_406);
        xp_table.insert(36, 24_815);
        xp_table.insert(37, 27_473);
        xp_table.insert(38, 30_408);
        xp_table.insert(39, 33_648);
        xp_table.insert(40, 37_224);
        xp_table.insert(41, 41_171);
        xp_table.insert(42, 45_529);
        xp_table.insert(43, 50_339);
        xp_table.insert(44, 55_649);
        xp_table.insert(45, 61_512);
        xp_table.insert(46, 67_983);
        xp_table.insert(47, 75_127);
        xp_table.insert(48, 83_014);
        xp_table.insert(49, 91_721);
        xp_table.insert(50, 101_333);
        xp_table.insert(51, 111_945);
        xp_table.insert(52, 123_660);
        xp_table.insert(53, 136_594);
        xp_table.insert(54, 150_872);
        xp_table.insert(55, 166_636);
        xp_table.insert(56, 184_040);
        xp_table.insert(57, 203_254);
        xp_table.insert(58, 224_466);
        xp_table.insert(59, 247_886);
        xp_table.insert(60, 273_742);
        xp_table.insert(61, 302_288);
        xp_table.insert(62, 333_804);
        xp_table.insert(63, 368_599);
        xp_table.insert(64, 407_015);
        xp_table.insert(65, 449_428);
        xp_table.insert(66, 496_254);
        xp_table.insert(67, 547_953);
        xp_table.insert(68, 605_032);
        xp_table.insert(69, 668_051);
        xp_table.insert(70, 737_627);
        xp_table.insert(71, 814_445);
        xp_table.insert(72, 899_257);
        xp_table.insert(73, 992_895);
        xp_table.insert(74, 1_096_278);
        xp_table.insert(75, 1_210_421);
        xp_table.insert(76, 1_336_443);
        xp_table.insert(77, 1_475_581);
        xp_table.insert(78, 1_629_200);
        xp_table.insert(79, 1_798_808);
        xp_table.insert(80, 1_986_068);
        xp_table.insert(81, 2_192_818);
        xp_table.insert(82, 2_421_087);
        xp_table.insert(83, 2_673_114);
        xp_table.insert(84, 2_951_373);
        xp_table.insert(85, 3_258_594);
        xp_table.insert(86, 3_597_792);
        xp_table.insert(87, 3_972_294);
        xp_table.insert(88, 4_385_776);
        xp_table.insert(89, 4_842_295);
        xp_table.insert(90, 5_346_332);
        xp_table.insert(91, 5_902_831);
        xp_table.insert(92, 6_517_253);
        xp_table.insert(93, 7_195_629);
        xp_table.insert(94, 7_944_614);
        xp_table.insert(95, 8_771_558);
        xp_table.insert(96, 9_684_577);
        xp_table.insert(97, 10_692_629);
        xp_table.insert(98, 11_805_606);
        xp_table.insert(99, 13_034_431);
        xp_table
    };
}

pub fn level_from_xp(xp: f32) -> i32 {
    for level in 1..99 {
        let xp_for_level = XP_TABLE.get(&level).unwrap().clone() as f32;
        if xp < xp_for_level {
            return if level > 1 { level - 1 } else { 1 };
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::level_from_xp;

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
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
