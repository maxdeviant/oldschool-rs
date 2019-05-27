use std::cmp;

pub struct Wintertodt {
    pub firemaking_level: i32,
    pub hitpoints_level: i32,
    pub warm_items_worn: i32,
    pub braziers_lit: i32,
}

impl Wintertodt {
    fn warm_items_mitigation(&self) -> i32 {
        cmp::max(self.warm_items_worn, 4)
    }

    fn lit_braziers_mitigation(&self) -> i32 {
        cmp::max(self.braziers_lit, 3)
    }

    pub fn standard_attack_damage(&self) -> i32 {
        (16 - self.warm_items_mitigation() - 2 * self.lit_braziers_mitigation())
            * (self.hitpoints_level + 1)
            / self.firemaking_level
    }

    pub fn brazier_attack_damage(&self) -> i32 {
        ((10 - self.warm_items_mitigation()) * (self.hitpoints_level + 1) / self.firemaking_level)
            * 2
    }

    pub fn area_attack_damage(&self) -> i32 {
        ((10 - self.warm_items_mitigation()) * (self.hitpoints_level + 1) / self.firemaking_level)
            * 3
    }
}

#[cfg(test)]
mod tests {
    use super::Wintertodt;

    #[test]
    fn test_standard_attack_damage() {
        let wintertodt = Wintertodt {
            firemaking_level: 65,
            hitpoints_level: 57,
            warm_items_worn: 4,
            braziers_lit: 4,
        };

        assert_eq!(wintertodt.standard_attack_damage(), 3);
    }

    #[test]
    fn test_brazier_attack_damage() {
        let wintertodt = Wintertodt {
            firemaking_level: 65,
            hitpoints_level: 57,
            warm_items_worn: 4,
            braziers_lit: 4,
        };

        assert_eq!(wintertodt.brazier_attack_damage(), 10);
    }

    #[test]
    fn test_area_attack_damage() {
        let wintertodt = Wintertodt {
            firemaking_level: 65,
            hitpoints_level: 57,
            warm_items_worn: 4,
            braziers_lit: 4,
        };

        assert_eq!(wintertodt.area_attack_damage(), 16);
    }
}
