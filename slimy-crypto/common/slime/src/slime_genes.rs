multiversx_sc::derive_imports!();

use super::color::*;
use random::*;

const MIN_ATT: u8 = 10;
const MAX_ATT: u8 = 25;
const MIN_DEF: u8 = 5;
const MAX_DEF: u8 = 20;
const MIN_HP: u8 = 100;
const MAX_HP: u8 = 200;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, Clone, TypeAbi)]
pub struct SlimeGenes {
    pub attack: u8,
    pub defense: u8,
    pub hp: u8,
    pub slime_color: Color,
}

impl Default for SlimeGenes {
    fn default() -> Self {
        SlimeGenes {
            attack: MIN_ATT,
            defense: MIN_DEF,
            hp: MIN_HP,
            slime_color: Self::genes_to_color(MIN_ATT, MIN_DEF, MIN_HP)
        }
    }
}

impl SlimeGenes {
    fn map_range(min: u8, max: u8, val: u8) -> u8 {
        ((val - min) as u16 * 255 / (max - min) as u16) as u8
    }

    pub fn genes_to_color(att: u8, def: u8, hp: u8) -> Color {
        Color::from_u8(
            Self::map_range(MIN_ATT, MAX_ATT, att),
            Self::map_range(MIN_DEF, MAX_DEF, def),
            Self::map_range(MIN_HP, MAX_HP, hp),
        )
    }
}

impl Randomizeable for SlimeGenes {
    fn get_random(random: &mut Random) -> Self {
        let att = random.quadded_u8(MIN_ATT, MAX_ATT);
        let def = random.quadded_u8(MIN_DEF, MAX_DEF);
        let hp = random.quadded_u8(MIN_HP, MAX_HP);

        SlimeGenes {
            attack: att,
            defense: def,
            hp: hp,
            slime_color: Self::genes_to_color(att, def, hp),
        }
    }
}
