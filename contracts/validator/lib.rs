#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod validator {
    #[ink(storage)]
    pub struct Validator {}

    impl Validator {
        #[ink(constructor)]
        pub fn new() -> Self { Self {} }

        #[ink(message)]
        pub fn validate_chess_moves(&self, _moves: Vec<String>) -> Option<AccountId> {
            // Stub: Replay moves, return winner
            None
        }

        #[ink(message)]
        pub fn validate_tetris_scores(&self, _scores: (u32, u32)) -> Option<AccountId> {
            //Stub for Tetris
            None
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut validator = Validator::new(false);
            //assert_eq!(validator.get(), false);
            //validator.flip();
            //assert_eq!(validator.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = ValidatorRef::new(false);
            let contract = client
                .instantiate("validator", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<Validator>();

            let get = call_builder.validate_chess_moves();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            /*
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));
            */

            Ok(())
        }
    }
}
