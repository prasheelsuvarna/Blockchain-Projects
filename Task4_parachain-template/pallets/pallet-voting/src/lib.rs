#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::EnsureOrigin,
    };
    use frame_system::pallet_prelude::*;
    use sp_std::{prelude::*, str};

    /// Pallet configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The event type for emitting voting-related events.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Number of blocks after which proposals expire.
        #[pallet::constant]
        type ProposalExpiration: Get<BlockNumberFor<Self>>;
        /// Maximum length for proposal title and description.
        #[pallet::constant]
        type MaxLength: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Storage for proposals: ID -> (title, description, creator, expiry block, for_votes, against_votes, passed).
    #[pallet::storage]
    pub type Proposals<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // Proposal ID
        (
            BoundedVec<u8, T::MaxLength>, // Title
            BoundedVec<u8, T::MaxLength>, // Description
            T::AccountId,                 // Creator
            BlockNumberFor<T>,            // Expiry block
            u32,                          // For votes
            u32,                          // Against votes
            Option<bool>,                 // Outcome (None if active, Some(true) if passed, Some(false) if failed)
        ),
    >;

    /// Storage for votes: (Proposal ID, Account) -> Vote (For/Against).
    #[pallet::storage]
    pub type Votes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32,           // Proposal ID
        Blake2_128Concat,
        T::AccountId,  // Voter
        bool,          // true = For, false = Against
        OptionQuery,
    >;

    /// Next available proposal ID.
    #[pallet::storage]
    pub type NextProposalId<T> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A proposal has been created [proposal_id, creator].
        ProposalCreated { proposal_id: u32, creator: T::AccountId },
        /// A vote has been cast [proposal_id, voter, is_for].
        VoteCast { proposal_id: u32, voter: T::AccountId, is_for: bool },
        /// A proposal’s outcome has been determined [proposal_id, passed].
        ProposalOutcome { proposal_id: u32, passed: bool },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Proposal ID already exists.
        ProposalExists,
        /// Proposal does not exist.
        ProposalNotFound,
        /// Proposal has expired.
        ProposalExpired,
        /// Already voted on this proposal.
        AlreadyVoted,
        /// Title or description exceeds max length.
        TextTooLong,
        /// Arithmetic overflow in vote counting.
        Overflow,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new proposal.
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_proposal(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;
            let proposal_id = NextProposalId::<T>::get();

            let title = BoundedVec::try_from(title).map_err(|_| Error::<T>::TextTooLong)?;
            ensure!(title.len() <= T::MaxLength::get() as usize, Error::<T>::TextTooLong);
            let description = BoundedVec::try_from(description).map_err(|_| Error::<T>::TextTooLong)?;
            ensure!(description.len() <= T::MaxLength::get() as usize, Error::<T>::TextTooLong);

            ensure!(!Proposals::<T>::contains_key(proposal_id), Error::<T>::ProposalExists);

            let expiry = frame_system::Pallet::<T>::block_number() + T::ProposalExpiration::get();
            Proposals::<T>::insert(
                proposal_id,
                (title, description, creator.clone(), expiry, 0, 0, None),
            );
            NextProposalId::<T>::put(proposal_id + 1);

            Self::deposit_event(Event::ProposalCreated { proposal_id, creator });
            Ok(())
        }

        /// Cast a vote on a proposal (true = For, false = Against).
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn vote(
            origin: OriginFor<T>,
            proposal_id: u32,
            is_for: bool,
        ) -> DispatchResult {
            let voter = ensure_signed(origin)?;

            let (title, description, creator, expiry, for_votes, against_votes, outcome) =
                Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;

            ensure!(outcome.is_none(), Error::<T>::ProposalExpired);
            ensure!(
                frame_system::Pallet::<T>::block_number() < expiry,
                Error::<T>::ProposalExpired
            );
            ensure!(
                !Votes::<T>::contains_key(proposal_id, &voter),
                Error::<T>::AlreadyVoted
            );

            Votes::<T>::insert(proposal_id, &voter, is_for);
            let new_for_votes = if is_for { for_votes + 1 } else { for_votes };
            let new_against_votes = if !is_for { against_votes + 1 } else { against_votes };

            Proposals::<T>::insert(
                proposal_id,
                (
                    title,
                    description,
                    creator,
                    expiry,
                    new_for_votes,
                    new_against_votes,
                    None,
                ),
            );

            Self::deposit_event(Event::VoteCast { proposal_id, voter, is_for });
            Ok(())
        }

        /// Finalize a proposal’s outcome (called manually for simplicity; could be hooked).
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn finalize_proposal(origin: OriginFor<T>, proposal_id: u32) -> DispatchResult {
            let _ = ensure_signed(origin)?; // Anyone can finalize for simplicity

            let (title, description, creator, expiry, for_votes, against_votes, outcome) =
                Proposals::<T>::get(proposal_id).ok_or(Error::<T>::ProposalNotFound)?;

            ensure!(outcome.is_none(), Error::<T>::ProposalExpired);
            ensure!(
                frame_system::Pallet::<T>::block_number() >= expiry,
                Error::<T>::ProposalNotFound // Could be a new error, but reusing for simplicity
            );

            let total_votes = for_votes.checked_add(against_votes).ok_or(Error::<T>::Overflow)?;
            let passed = if total_votes > 0 {
                for_votes > total_votes / 2
            } else {
                false // No votes means failed
            };

            Proposals::<T>::insert(
                proposal_id,
                (
                    title,
                    description,
                    creator,
                    expiry,
                    for_votes,
                    against_votes,
                    Some(passed),
                ),
            );

            Self::deposit_event(Event::ProposalOutcome { proposal_id, passed });
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frame_support::{
        assert_ok, assert_noop,
        traits::{ConstU32, ConstU64},
    };
    use frame_system::RawOrigin;
    use sp_runtime::{
        traits::{BlakeTwo256, IdentityLookup},
        BuildStorage,
    };

    type Block = frame_system::mocking::MockBlock<Test>;

    // Configure a mock runtime
    frame_support::construct_runtime!(
        pub enum Test {
            System: frame_system,
            Voting: pallet_voting,
        }
    );

    impl frame_system::Config for Test {
        type BaseCallFilter = frame_support::traits::Everything;
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type RuntimeOrigin = RuntimeOrigin;
        type RuntimeCall = RuntimeCall;
        type Hash = sp_core::H256;
        type Hashing = BlakeTwo256;
        type AccountId = u64;
        type Lookup = IdentityLookup<Self::AccountId>;
        type Block = Block;
        type RuntimeEvent = RuntimeEvent;
        type BlockHashCount = ConstU64<250>;
        type Version = ();
        type PalletInfo = PalletInfo;
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ConstU32<16>;
        type RuntimeTask = ();
        type SingleBlockMigrations = ();
        type MultiBlockMigrator = ();
        type PreInherents = ();
        type PostInherents = ();
        type PostTransactions = ();
    }

    impl Config for Test {
        type RuntimeEvent = RuntimeEvent;
        type ProposalExpiration = ConstU64<10>; // 10 blocks
        type MaxLength = ConstU32<100>;         // 100 bytes max for title/desc
    }

    fn new_test_ext() -> sp_io::TestExternalities {
        let storage = frame_system::GenesisConfig::<Test>::default()
            .build_storage()
            .unwrap();
        storage.into()
    }

    #[test]
    fn test_proposal_creation() {
        new_test_ext().execute_with(|| {
            let title = b"Test Proposal".to_vec();
            let desc = b"Description".to_vec();
            assert_ok!(Voting::create_proposal(
                RuntimeOrigin::signed(1),
                title.clone(),
                desc.clone()
            ));
            let (stored_title, stored_desc, creator, expiry, for_votes, against_votes, outcome) =
                Voting::proposals(0).unwrap();
            assert_eq!(stored_title, BoundedVec::try_from(title).unwrap());
            assert_eq!(stored_desc, BoundedVec::try_from(desc).unwrap());
            assert_eq!(creator, 1);
            assert_eq!(for_votes, 0);
            assert_eq!(against_votes, 0);
            assert_eq!(outcome, None);
            assert_eq!(Voting::next_proposal_id(), 1);
        });
    }

    #[test]
    fn test_voting_process() {
        new_test_ext().execute_with(|| {
            assert_ok!(Voting::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Desc".to_vec()
            ));
            assert_ok!(Voting::vote(RuntimeOrigin::signed(2), 0, true));
            assert_ok!(Voting::vote(RuntimeOrigin::signed(3), 0, false));
            assert_noop!(
                Voting::vote(RuntimeOrigin::signed(2), 0, true),
                Error::<Test>::AlreadyVoted
            );
            let (.., for_votes, against_votes, _) = Voting::proposals(0).unwrap();
            assert_eq!(for_votes, 1);
            assert_eq!(against_votes, 1);
            assert_eq!(Voting::votes(0, 2), Some(true));
            assert_eq!(Voting::votes(0, 3), Some(false));
        });
    }

    #[test]
    fn test_expiry_logic() {
        new_test_ext().execute_with(|| {
            assert_ok!(Voting::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Desc".to_vec()
            ));
            System::set_block_number(11); // Past expiry (10 blocks)
            assert_noop!(
                Voting::vote(RuntimeOrigin::signed(2), 0, true),
                Error::<Test>::ProposalExpired
            );
        });
    }

    #[test]
    fn test_voting_results() {
        new_test_ext().execute_with(|| {
            assert_ok!(Voting::create_proposal(
                RuntimeOrigin::signed(1),
                b"Test".to_vec(),
                b"Desc".to_vec()
            ));
            assert_ok!(Voting::vote(RuntimeOrigin::signed(2), 0, true));
            assert_ok!(Voting::vote(RuntimeOrigin::signed(3), 0, true));
            assert_ok!(Voting::vote(RuntimeOrigin::signed(4), 0, false));
            System::set_block_number(11);
            assert_ok!(Voting::finalize_proposal(RuntimeOrigin::signed(1), 0));
            let (.., for_votes, against_votes, outcome) = Voting::proposals(0).unwrap();
            assert_eq!(for_votes, 2);
            assert_eq!(against_votes, 1);
            assert_eq!(outcome, Some(true)); // 2/3 > 50%
        });
    }
}