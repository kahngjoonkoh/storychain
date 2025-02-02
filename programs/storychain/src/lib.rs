use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("5cdQ4zLH1uNSb4ooU4uXNvKmHSrr7Hv46XR7kMVR12Nw");

const PROJECT_DESCRIPTION: &str = "StoryChain is a decentralized collaborative storytelling platform \
where users can create branching narratives. Each story node represents a continuation of its \
parent story, allowing for multiple creative paths and endings...";


const MAX_CHILDREN: &u64 = 16;

#[program]
pub mod storychain {
    use super::*;  

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let root_story_node = &mut ctx.account.root_story_node;
        let story_tree_state = &mut ctx.account.story_tree_state;

        root_story_node.id = 0;
        root_story_node.content = PROJECT_DESCRIPTION.to_string();
        root_story_node.children = vec![];
    
        story_tree_state.root = root_story_node.key();
        story_tree_state.node_counter = 0;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8  // discriminator + root pubkey + node_counter
    )]
    pub story_tree_state: Account<'info, StoryTreeState>,
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 500 + (4 + (MAX_CHILDREN * 8))  // discriminator + id + content + children
    )]
    pub root_story_node: Account<'info, RootStoryNode>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StoryTreeState {
    pub root: Pubkey,
    pub 
    pub node_counter: u64,
    
}

#[account]
pub struct RootStoryNode {
    pub id: u64,
    pub content: String,
    pub children: Vec<u64>,
}

#[account]
pub struct StoryNode {
    pub id: u64,
    pub author: Pubkey,
    pub content: String,
    pub parent_id: u64,
    pub children: Vec<u64>,
}
