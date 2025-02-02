use anchor_lang::prelude::*;

declare_id!("5cdQ4zLH1uNSb4ooU4uXNvKmHSrr7Hv46XR7kMVR12Nw");

// const PROJECT_DESCRIPTION: &str = "StoryChain is a decentralized collaborative storytelling platform \
// where users can create branching narratives. Each story node represents a continuation of its \
// parent story, allowing for multiple creative paths and endings...";

const MAX_CONTENT: usize = 500;
const MAX_CHILDREN: usize = 16;
const MAX_LATEST_NODES: usize = 16;

#[program]
pub mod storychain {
    use super::*;

    pub fn initialize_program(ctx: Context<InitializeProgram>) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        global_state.root = ctx.accounts.authority.key();
        global_state.main = global_state.root;
        global_state.node_counter = 0;

        // // Converting a byte slice to a fixed-size array
        // let byte_slice: &[u8] = PROJECT_DESCRIPTION.as_bytes();
        // let mut byte_array: [u8; MAX_CONTENT] = [0; MAX_CONTENT];  // initialize with 0s

        // // Copy the content into the fixed-size array
        // let len_to_copy = byte_slice.len().min(MAX_CONTENT);
        // byte_array[..len_to_copy].copy_from_slice(&byte_slice[..len_to_copy]);

        // let add_story_node_context = &mut ctx.accounts;
        // let account_info_list = vec![&ctx.accounts.global_state.root.to_account_info()];
        
        // add_story_node(
        //     Context::new(
        //         ctx.accounts.story_node.clone(),
        //         &global_state.to_account_info().key(),
        //         add_story_node_context,
        //         &account_info_list,
        //     ),
        //     None,
        //     byte_array,
        // );
        Ok(())
    }

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        let user_info = &mut ctx.accounts.user_info;
        user_info.latest_nodes = vec![];
        Ok(())
    }


    pub fn add_story_node(
        ctx: Context<AddStoryNode>,
        parent_id: Option<Pubkey>,
        content: [u8; MAX_CONTENT],
    ) -> Result<()> {
        let story_node = &mut ctx.accounts.story_node;
        story_node.author = ctx.accounts.author.key();
        match parent_id {
            Some(id) => story_node.parent_id = id,
            None => {
                let _ = Err::<(), ErrorCode>(ErrorCode::InvalidParent.into());
            }
        }
        story_node.content = content;
        story_node.children = vec![];

        ctx.accounts.global_state.node_counter += 1;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 8,    // discriminator + root_id + main_id + node_counter
        seeds = [b"GLOBAL_STATE"],  // Anchor automatically derives PDA
        bump
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + (4 + MAX_LATEST_NODES * 32) // discriminator + latest_nodes
    )]
    pub user_info: Account<'info, UserInfo>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddStoryNode<'info> {
    #[account(mut, seeds = [b"GLOBAL_STATE"], bump)]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        init,
        payer = author,
        space = 8 + 32 + 32 + MAX_CONTENT + (4 + MAX_CHILDREN * 32) // discriminator + author + parent_id + content + children
    )]
    pub story_node: Account<'info, StoryNode>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GlobalState {
    pub root: Pubkey,
    pub main: Pubkey,
    pub node_counter: u64,
}

#[account]
pub struct StoryNode {
    pub author: Pubkey,
    pub parent_id: Pubkey,
    pub content: [u8; MAX_CONTENT],
    pub children: Vec<Pubkey>,
}

#[account]
pub struct UserInfo {
    pub latest_nodes: Vec<Pubkey>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid parent node")]
    InvalidParent,
}