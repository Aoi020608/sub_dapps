use bytemuck::{Pod, Zeroable};
use jito_bytemuck::{
    types::{PodBool, PodU32, PodU64},
    AccountDeserialize, Discriminator,
};
use shank::ShankAccount;
use solana_program::{account_info::AccountInfo, msg, program_error::ProgramError, pubkey::Pubkey};

pub const MAX_MESSAGE_LENGTH: usize = 64;

#[derive(Debug, Clone, Copy, Zeroable, ShankAccount, Pod, AccountDeserialize)]
#[repr(C)]
pub struct Message {
    /// Epoch
    epoch: PodU64,

    /// Message Data
    keyword: [u8; 64],
}

impl Message {
    /// Initiallize a new Message
    pub fn new(epoch: u64, keyword: &str) -> Self {
        let mut keyword_data = [0; 64];
        keyword_data.copy_from_slice(keyword.as_bytes());

        Self {
            epoch: PodU64::from(epoch),
            keyword: keyword_data,
        }
    }

    /// Seeds of Message Account
    pub fn seeds(epoch: u64) -> Vec<Vec<u8>> {
        vec![b"message".to_vec(), epoch.to_be_bytes().to_vec()]
    }

    /// Find the program address of Message Account
    pub fn find_program_address(program_id: &Pubkey, epoch: u64) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = Pubkey::find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Load Message Account
    pub fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
        epoch: u64,
    ) -> Result<(), ProgramError> {
        if account.owner.ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.data.borrow()[0].ne(&Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account
            .key
            .ne(&Self::find_program_address(program_id, epoch).0)
        {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }

    pub fn epoch(&self) -> u64 {
        self.epoch.into()
    }

    pub fn keyword(&self) -> String {
        String::from_utf8(self.keyword.to_vec()).unwrap()
    }
}
