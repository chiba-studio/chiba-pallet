use crate::{self as chiba, ChibaSwapAction};
use frame_support::{dispatch::DispatchResult, parameter_types};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Nft: orml_nft::{Pallet, Call, Storage},
        Chiba: chiba::{Pallet, Call, Storage, Event<T>},
        AtomicSwap: pallet_atomic_swap::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl orml_nft::Config for Test {
    type ClassId = u64;
    type TokenId = u64;
    type ClassData = chiba::ClassData;
    type TokenData = chiba::TokenData;
}

parameter_types! {
    pub const ProofLimit: u32 = 10_000;
}

impl chiba::Config for Test {
    type Event = Event;
    type Currency = Balances;
}

impl pallet_atomic_swap::Config for Test {
    type Event = Event;
    type SwapAction = ChibaSwapAction<Test>;
    type ProofLimit = ProofLimit;
}

pub const ALICE: u64 = 221;
pub const BOB: u64 = 1983;
pub const CURATOR: u64 = 128;

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(ALICE, 1 << 60), (BOB, 1 << 60)],
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| {
        System::set_block_number(1);
    });

    ext
}

pub fn last_event() -> crate::mock::Event {
    frame_system::Pallet::<crate::mock::Test>::events()
        .pop()
        .expect("NO EVENTS")
        .event
}

pub fn create_default_collection() -> DispatchResult {
    Chiba::create_collection(
        Origin::signed(ALICE),
        Default::default(),
        Default::default(),
    )
}

pub fn mint_default_token() -> DispatchResult {
    create_default_collection()?;
    Chiba::mint(
        Origin::signed(ALICE),
        Default::default(),
        Default::default(),
        Default::default(),
    )
}
