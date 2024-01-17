#![no_std]

multiversx_sc::imports!();

use random::*;
use slime::{slime_genes::*, Slime};

#[multiversx_sc::contract]
pub trait SlimeGeneticAlg {
    #[init]
    fn init(&self) {}

    // endpoints

    #[endpoint(generateSlimeGenes)]
    fn generate_slime_genes(&self, matron: Slime, sire: Slime) -> SlimeGenes {
        let mut random = Random::new(
            self.blockchain().get_block_random_seed(),
            self.blockchain().get_tx_hash(),
        );

        let genes = SlimeGenes::get_random(&mut random);
        let random_slime = Slime::new(genes, self.blockchain().get_block_timestamp(), 0, 0, 0);

        let att = ((matron.genes.attack as u16 + sire.genes.attack as u16 + random_slime.genes.attack as u16) / 3) as u8;
        let def = ((matron.genes.defense as u16 + sire.genes.defense as u16 + random_slime.genes.defense as u16) / 3) as u8;
        let hp = ((matron.genes.hp as u16 + sire.genes.hp as u16 + random_slime.genes.hp as u16) / 3) as u8;
        let color = SlimeGenes::genes_to_color(att, def, hp);

        SlimeGenes {
            attack: att,
            defense: def,
            hp: hp,
            slime_color: color
        }
    }
}
