#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod nft_subscription {
    use ink::storage::Mapping;
    use ink::env::AccountId;
    use ink::prelude::vec::Vec;

    #[derive(scale::Encode, scale::Decode, Clone, PartialEq, Eq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SubscriptionNFT {
        pub owner: AccountId,
        pub expiry: u64,  // Unix timestamp for expiry
    }

    #[ink(storage)]
    pub struct NftSubscription {
        nfts: Mapping<AccountId, SubscriptionNFT>, // Stores NFTs for each account
        content_uri: String,                      // URI of the premium content
    }

    impl NftSubscription {
        #[ink(constructor)]
        pub fn new(content_uri: String) -> Self {
            Self {
                nfts: Mapping::default(),
                content_uri,
            }
        }

        /// Mint a new subscription NFT for the caller
        #[ink(message)]
        pub fn mint_nft(&mut self, duration: u64) {
            let caller = self.env().caller();
            let current_time = self.env().block_timestamp();
            let expiry = current_time + duration;
            
            let nft = SubscriptionNFT {
                owner: caller,
                expiry,
            };

            self.nfts.insert(caller, &nft);
        }

        /// Check if the caller has a valid (non-expired) NFT subscription
        #[ink(message)]
        pub fn has_access(&self) -> bool {
            let caller = self.env().caller();
            if let Some(nft) = self.nfts.get(caller) {
                if self.env().block_timestamp() < nft.expiry {
                    return true;
                }
            }
            false
        }

        /// Fetch the premium content URI if the caller has a valid subscription
        #[ink(message)]
        pub fn get_content(&self) -> Option<String> {
            if self.has_access() {
                return Some(self.content_uri.clone());
            }
            None
        }

        /// Extend the subscription for the caller
        #[ink(message)]
        pub fn extend_subscription(&mut self, additional_time: u64) {
            let caller = self.env().caller();
            if let Some(mut nft) = self.nfts.get(caller) {
                let current_time = self.env().block_timestamp();
                nft.expiry = core::cmp::max(nft.expiry, current_time) + additional_time;
                self.nfts.insert(caller, &nft);
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_minting_and_access() {
            let mut contract = NftSubscription::new(String::from("ipfs://content_hash"));
            let duration = 1000;  // Example subscription duration in ms

            // Mint a new NFT for the caller
            contract.mint_nft(duration);

            // Ensure the caller has access after minting
            assert!(contract.has_access());

            // Fetch content and verify it's the correct content URI
            assert_eq!(contract.get_content(), Some(String::from("ipfs://content_hash")));
        }

        #[ink::test]
        fn test_expired_nft() {
            let mut contract = NftSubscription::new(String::from("ipfs://content_hash"));
            let duration = 1;  // Very short duration

            // Mint a new NFT
            contract.mint_nft(duration);

            // Simulate passing of time
            ink::env::test::advance_block_timestamp_by(10);

            // Ensure the NFT is now expired
            assert!(!contract.has_access());
        }
    }
}
