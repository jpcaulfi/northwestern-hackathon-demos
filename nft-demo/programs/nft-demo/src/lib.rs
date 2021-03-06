use anchor_lang::prelude::*;

pub mod mint;
pub mod sell;

use mint::*;
use sell::*;


// declare_id!("7umNQcBicLAFfxyNGsGzZVk2YfT2GmFgJ3GpzetgCLiK");
declare_id!("i25WSpXBBR8uk2YohMHVjkG67JuM8KEY7fZiTeLNrxQ");


#[program]
pub mod nft_demo {
    use super::*;

    pub fn mint(
        ctx: Context<MintNft>, 
        metadata_title: String, 
        metadata_symbol: String, 
        metadata_uri: String,
    ) -> Result<()> {
        mint::mint(
            ctx,
            metadata_title,
            metadata_symbol,
            metadata_uri,
        )
    }

    pub fn sell(
        ctx: Context<SellNft>,
        sale_lamports: u64
    ) -> Result<()> {
        sell::sell(
            ctx,
            sale_lamports,
        )
    }
}