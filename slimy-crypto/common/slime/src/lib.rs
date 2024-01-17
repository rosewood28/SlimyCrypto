#![no_std]

multiversx_sc::derive_imports!();

pub mod color;
pub mod slime_genes;

use slime_genes::*;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Slime {
    pub genes: SlimeGenes,
    pub birth_time: u64,   // timestamp
    pub matron_id: u32,
    pub sire_id: u32,
    pub nr_children: u16,
    pub generation: u16,     // max(sire_gen, matron_gen) + 1. Generation also influences cooldown.
}

impl Slime {
    pub fn new(
        genes: SlimeGenes,
        birth_time: u64,
        matron_id: u32,
        sire_id: u32,
        generation: u16,
    ) -> Self {
        Slime {
            genes,
            birth_time,
            matron_id,
            sire_id,
            nr_children: 0,
            generation,
        }
    }
}

impl Slime {
    pub fn get_attack(&self) -> u8 {
        self.genes.attack
    }

    pub fn get_defense(&self) -> u8 {
        self.genes.defense
    }

    pub fn get_hp(&self) -> u8 {
        self.genes.hp
    }
}

// The default Slime, which is not a valid slime. Used for Slime with ID 0
impl Default for Slime {
    fn default() -> Self {
        Slime {
            genes: SlimeGenes::default(),
            birth_time: 0,
            matron_id: 0,
            sire_id: 0,
            nr_children: 0,
            generation: 0,
        }
    }
}
