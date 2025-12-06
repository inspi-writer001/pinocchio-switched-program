#[allow(unused)]
#[cfg(test)]
mod tests {

    use std::io::Error;

    use litesvm::LiteSVM;
    use litesvm_token::{
        spl_token::{
            self,
            solana_program::{msg, rent::Rent, sysvar::SysvarId},
        },
        CreateAssociatedTokenAccount, CreateAssociatedTokenAccountIdempotent, CreateMint, MintTo,
    };

    use pinocchio::pubkey::pubkey_eq;
    use solana_instruction::{AccountMeta, Instruction};
    use solana_keypair::Keypair;
    use solana_message::Message;
    use solana_native_token::LAMPORTS_PER_SOL;
    use solana_pubkey::Pubkey;
    use solana_signer::Signer;
    use solana_transaction::Transaction;
    use spl_associated_token_account::solana_program::clock::Clock;
    use spl_associated_token_account::solana_program::program_pack::Pack;

    use crate::{
        instruction::{InitializeSwitched, SwitchedInstruction},
        state::GlobalState,
    };

    const PROGRAM_ID: Pubkey = Pubkey::new_from_array(crate::ID); //"CntDHuHyUa1sEyLEYoHbrYdzM2G4VeDHSdQjQXXdRh6E";
    const TOKEN_PROGRAM_ID: Pubkey = spl_token::ID;
    const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

    fn program_id() -> Pubkey {
        PROGRAM_ID
    }

    pub struct ReusableState {
        pub mint: Pubkey,
        // pub streamer_ata: Pubkey,
        pub treasury: Pubkey,
        pub ata_program_id: Pubkey,
        pub token_program_id: Pubkey,
        pub system_program_id: Pubkey,
        // pub streamer_state: (Pubkey, u8),
        pub global_state: (Pubkey, u8),
        pub admin: Keypair,
        pub user: Option<Keypair>,
        pub user_ata: Option<Pubkey>,
        pub user_pda: Option<(Pubkey, u8)>,
    }

    fn setup() -> (LiteSVM, ReusableState) {
        let mut svm = LiteSVM::new();
        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL)
            .expect("Airdrop failed");

        // Load program SO file
        msg!("The path is!! {}", env!("CARGO_MANIFEST_DIR"));

        let bytes = include_bytes!("../../target/deploy/switched_program.so");
        svm.add_program(program_id(), bytes);

        let mint = CreateMint::new(&mut svm, &payer)
            .decimals(6)
            .authority(&payer.pubkey())
            .send()
            .unwrap();
        // msg!("Mint A: {}", mint);

        // Create the maker's associated token account for Mint A
        let treasury = CreateAssociatedTokenAccount::new(&mut svm, &payer, &mint)
            .owner(&payer.pubkey())
            .send()
            .unwrap();
        // msg!("Maker ATA A: {}\n", maker_ata);

        // Derive the PDA for the escrow account using the maker's public key and a seed value
        let global_state = Pubkey::find_program_address(&[b"global_state".as_ref()], &PROGRAM_ID);

        let associated_token_program = ASSOCIATED_TOKEN_PROGRAM_ID.parse::<Pubkey>().unwrap();
        let token_program = TOKEN_PROGRAM_ID;
        let system_program = solana_sdk_ids::system_program::ID;

        let reusable_state = ReusableState {
            mint,
            ata_program_id: associated_token_program,
            token_program_id: token_program,
            system_program_id: system_program,
            treasury,
            admin: payer,
            global_state,
            user: None,
            user_ata: None,
            user_pda: None,
        };
        (svm, reusable_state)
    }

    #[test]
    pub fn create_global_state() -> Result<(), Error> {
        let (mut svm, reusable_state) = setup();

        let mint = reusable_state.mint;
        let payer = reusable_state.admin;
        let treasury_ata = reusable_state.treasury;
        let ata_id = reusable_state.ata_program_id;
        let token_program = reusable_state.token_program_id;
        let system_program = reusable_state.system_program_id;
        let global_state = reusable_state.global_state;

        let ix_data = InitializeSwitched { fee_bps: 100 };

        let ix_data = [
            vec![SwitchedInstruction::Initialize as u8],
            ix_data.to_bytes(),
        ]
        .concat();

        let init_ix = Instruction {
            program_id: program_id(),
            accounts: vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(global_state.0, false),
                AccountMeta::new(treasury_ata, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(system_program, false),
                AccountMeta::new(ata_id, false),
                AccountMeta::new(token_program, false),
                AccountMeta::new(Rent::id(), false),
            ],
            data: ix_data,
        };

        let message = Message::new(&[init_ix], Some(&payer.pubkey()));
        let recent_blockhash = svm.latest_blockhash();

        let transaction = Transaction::new(&[&payer], message, recent_blockhash);

        // Send the transaction and capture the result
        let tx = svm.send_transaction(transaction).unwrap();

        // msg!("tx logs: {:#?}", tx.logs);
        msg!("\nInit transaction sucessful");
        msg!("CUs Consumed: {}", tx.compute_units_consumed);

        let global_state_from_svm = svm.get_account(&reusable_state.global_state.0).unwrap();
        let parsed_account = bytemuck::from_bytes::<GlobalState>(&global_state_from_svm.data);

        assert!(
            pubkey_eq(
                &treasury_ata.to_bytes(),
                &parsed_account.plaftorm_fee_account
            ),
            "Broo the accounts are not the same"
        );

        println!("here's the data onchain: {:?}", parsed_account);

        Ok(())
    }
}
