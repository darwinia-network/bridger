#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 4)]
        Balances(balances::Event),
        #[codec(index = 5)]
        Kton(kton::Event),
        #[codec(index = 8)]
        ElectionProviderMultiPhase(election_provider_multi_phase::Event),
        #[codec(index = 9)]
        Staking(staking::Event),
        #[codec(index = 10)]
        Offences(offences::Event),
        #[codec(index = 12)]
        Session(session::Event),
        #[codec(index = 13)]
        Grandpa(grandpa::Event),
        #[codec(index = 32)]
        EcdsaAuthority(ecdsa_authority::Event),
        #[codec(index = 14)]
        ImOnline(im_online::Event),
        #[codec(index = 24)]
        Treasury(treasury::Event),
        #[codec(index = 16)]
        Sudo(sudo::Event),
        #[codec(index = 21)]
        Scheduler(scheduler::Event),
        #[codec(index = 18)]
        BridgePangolinDispatch(bridge_pangolin_dispatch::Event),
        #[codec(index = 17)]
        BridgePangolinMessages(bridge_pangolin_messages::Event),
        #[codec(index = 22)]
        PangolinFeeMarket(pangolin_fee_market::Event),
        #[codec(index = 23)]
        TransactionPause(transaction_pause::Event),
        #[codec(index = 20)]
        Substrate2SubstrateBacking(substrate2_substrate_backing::Event),
        #[codec(index = 25)]
        EVM(evm::Event),
        #[codec(index = 26)]
        Ethereum(ethereum::Event),
        #[codec(index = 31)]
        BaseFee(base_fee::Event),
    }
    pub mod system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct fill_block {
                pub ratio: runtime_types::sp_arithmetic::per_things::Perbill,
            }
            impl ::subxt::Call for fill_block {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "fill_block";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct remark {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_heap_pages {
                pub pages: ::core::primitive::u64,
            }
            impl ::subxt::Call for set_heap_pages {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_heap_pages";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_code {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_code_without_checks {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_code_without_checks {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_code_without_checks";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_changes_trie_config {
                pub changes_trie_config: ::core::option::Option<
                    runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                >,
            }
            impl ::subxt::Call for set_changes_trie_config {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_changes_trie_config";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_storage {
                pub items: ::std::vec::Vec<(
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            }
            impl ::subxt::Call for set_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "set_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct kill_storage {
                pub keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            }
            impl ::subxt::Call for kill_storage {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_storage";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct kill_prefix {
                pub prefix: ::std::vec::Vec<::core::primitive::u8>,
                pub subkeys: ::core::primitive::u32,
            }
            impl ::subxt::Call for kill_prefix {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "kill_prefix";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct remark_with_event {
                pub remark: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for remark_with_event {
                const PALLET: &'static str = "System";
                const FUNCTION: &'static str = "remark_with_event";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn fill_block(
                    &self,
                    ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, fill_block, DispatchError>
                {
                    let call = fill_block { ratio };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remark, DispatchError>
                {
                    let call = remark { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_heap_pages(
                    &self,
                    pages: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_heap_pages, DispatchError>
                {
                    let call = set_heap_pages { pages };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_code, DispatchError>
                {
                    let call = set_code { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_code_without_checks(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_code_without_checks,
                    DispatchError,
                > {
                    let call = set_code_without_checks { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_changes_trie_config(
                    &self,
                    changes_trie_config: ::core::option::Option<
                        runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_changes_trie_config,
                    DispatchError,
                > {
                    let call = set_changes_trie_config {
                        changes_trie_config,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_storage(
                    &self,
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_storage, DispatchError>
                {
                    let call = set_storage { items };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_storage(
                    &self,
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_storage, DispatchError>
                {
                    let call = kill_storage { keys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_prefix(
                    &self,
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_prefix, DispatchError>
                {
                    let call = kill_prefix { prefix, subkeys };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remark_with_event(
                    &self,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remark_with_event, DispatchError>
                {
                    let call = remark_with_event { remark };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::frame_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ExtrinsicSuccess(pub runtime_types::frame_support::weights::DispatchInfo);
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ExtrinsicFailed(
                pub runtime_types::sp_runtime::DispatchError,
                pub runtime_types::frame_support::weights::DispatchInfo,
            );
            impl ::subxt::Event for ExtrinsicFailed {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CodeUpdated;
            impl ::subxt::Event for CodeUpdated {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "CodeUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NewAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct KilledAccount(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Remarked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for Remarked {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "Remarked";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Account";
                type Value = runtime_types::frame_system::AccountInfo<
                    ::core::primitive::u32,
                    runtime_types::drml_common_runtime::impls::AccountData<::core::primitive::u128>,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct ExtrinsicCount;
            impl ::subxt::StorageEntry for ExtrinsicCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockWeight;
            impl ::subxt::StorageEntry for BlockWeight {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockWeight";
                type Value =
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AllExtrinsicsLen;
            impl ::subxt::StorageEntry for AllExtrinsicsLen {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "AllExtrinsicsLen";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BlockHash(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ExtrinsicData(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ExtrinsicData {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExtrinsicData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Number;
            impl ::subxt::StorageEntry for Number {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Number";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ParentHash;
            impl ::subxt::StorageEntry for ParentHash {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ParentHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Digest;
            impl ::subxt::StorageEntry for Digest {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Digest";
                type Value =
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Events;
            impl ::subxt::StorageEntry for Events {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "Events";
                type Value = ::std::vec::Vec<
                    runtime_types::frame_system::EventRecord<
                        runtime_types::pangoro_runtime::Event,
                        ::subxt::sp_core::H256,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventCount;
            impl ::subxt::StorageEntry for EventCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EventTopics(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for EventTopics {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "EventTopics";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct LastRuntimeUpgrade;
            impl ::subxt::StorageEntry for LastRuntimeUpgrade {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "LastRuntimeUpgrade";
                type Value = runtime_types::frame_system::LastRuntimeUpgradeInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToU32RefCount;
            impl ::subxt::StorageEntry for UpgradedToU32RefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToU32RefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradedToTripleRefCount;
            impl ::subxt::StorageEntry for UpgradedToTripleRefCount {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "UpgradedToTripleRefCount";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ExecutionPhase;
            impl ::subxt::StorageEntry for ExecutionPhase {
                const PALLET: &'static str = "System";
                const STORAGE: &'static str = "ExecutionPhase";
                type Value = runtime_types::frame_system::Phase;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::AccountInfo<
                        ::core::primitive::u32,
                        runtime_types::drml_common_runtime::impls::AccountData<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_weight(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::PerDispatchClass<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = BlockWeight;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn all_extrinsics_len(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = AllExtrinsicsLen;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn extrinsic_data(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ExtrinsicData(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn extrinsic_data_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ExtrinsicData>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Number;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn parent_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = ParentHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn digest(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_runtime::generic::digest::Digest<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = Digest;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn events(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_system::EventRecord<
                            runtime_types::pangoro_runtime::Event,
                            ::subxt::sp_core::H256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Events;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = EventCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = EventTopics(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn event_topics_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, EventTopics>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn last_runtime_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::LastRuntimeUpgradeInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = LastRuntimeUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn upgraded_to_u32_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToU32RefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgraded_to_triple_ref_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = UpgradedToTripleRefCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn execution_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::frame_system::Phase>,
                    ::subxt::BasicError,
                > {
                    let entry = ExecutionPhase;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn block_weights(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockWeights,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 242u8, 5u8, 42u8, 1u8, 0u8, 0u8, 0u8, 0u8, 32u8, 74u8, 169u8,
                            209u8, 1u8, 0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                            192u8, 202u8, 72u8, 147u8, 81u8, 1u8, 0u8, 0u8, 1u8, 0u8, 152u8, 247u8,
                            62u8, 93u8, 1u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8, 192u8, 82u8, 155u8,
                            253u8, 197u8, 1u8, 0u8, 0u8, 1u8, 0u8, 32u8, 74u8, 169u8, 209u8, 1u8,
                            0u8, 0u8, 1u8, 0u8, 136u8, 82u8, 106u8, 116u8, 0u8, 0u8, 0u8, 64u8,
                            89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_length(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_system::limits::BlockLength,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 60u8, 0u8, 0u8, 0u8, 80u8, 0u8, 0u8, 0u8, 80u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn block_hash_count(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[96u8, 9u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn db_weight(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::weights::RuntimeDbWeight,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            64u8, 120u8, 125u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 225u8, 245u8, 5u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn version(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_version::RuntimeVersion,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            28u8, 80u8, 97u8, 110u8, 103u8, 111u8, 114u8, 111u8, 28u8, 80u8, 97u8,
                            110u8, 103u8, 111u8, 114u8, 111u8, 0u8, 0u8, 0u8, 0u8, 30u8, 110u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 56u8, 223u8, 106u8, 203u8, 104u8, 153u8,
                            7u8, 96u8, 155u8, 3u8, 0u8, 0u8, 0u8, 55u8, 227u8, 151u8, 252u8, 124u8,
                            145u8, 245u8, 228u8, 1u8, 0u8, 0u8, 0u8, 64u8, 254u8, 58u8, 212u8, 1u8,
                            248u8, 149u8, 154u8, 5u8, 0u8, 0u8, 0u8, 171u8, 60u8, 5u8, 114u8, 41u8,
                            31u8, 235u8, 139u8, 1u8, 0u8, 0u8, 0u8, 203u8, 202u8, 37u8, 227u8,
                            159u8, 20u8, 35u8, 135u8, 2u8, 0u8, 0u8, 0u8, 237u8, 153u8, 197u8,
                            172u8, 178u8, 94u8, 237u8, 245u8, 3u8, 0u8, 0u8, 0u8, 104u8, 122u8,
                            212u8, 74u8, 211u8, 127u8, 3u8, 194u8, 1u8, 0u8, 0u8, 0u8, 188u8,
                            157u8, 137u8, 144u8, 79u8, 91u8, 146u8, 63u8, 1u8, 0u8, 0u8, 0u8,
                            210u8, 188u8, 152u8, 151u8, 238u8, 208u8, 143u8, 21u8, 3u8, 0u8, 0u8,
                            0u8, 247u8, 139u8, 39u8, 139u8, 229u8, 63u8, 69u8, 76u8, 2u8, 0u8, 0u8,
                            0u8, 55u8, 200u8, 187u8, 19u8, 80u8, 169u8, 162u8, 168u8, 1u8, 0u8,
                            0u8, 0u8, 88u8, 34u8, 17u8, 246u8, 91u8, 177u8, 75u8, 137u8, 4u8, 0u8,
                            0u8, 0u8, 230u8, 91u8, 0u8, 228u8, 108u8, 237u8, 208u8, 170u8, 2u8,
                            0u8, 0u8, 0u8, 189u8, 120u8, 37u8, 93u8, 79u8, 238u8, 234u8, 31u8, 4u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[18u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod babe {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct report_equivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct report_equivocation_unsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_session::MembershipProof,
            }
            impl ::subxt::Call for report_equivocation_unsigned {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct plan_config_change {
                pub config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
            }
            impl ::subxt::Call for plan_config_change {
                const PALLET: &'static str = "Babe";
                const FUNCTION: &'static str = "plan_config_change";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, report_equivocation, DispatchError>
                {
                    let call = report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_consensus_slots::EquivocationProof<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                        runtime_types::sp_consensus_babe::app::Public,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation_unsigned,
                    DispatchError,
                > {
                    let call = report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn plan_config_change(
                    &self,
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, plan_config_change, DispatchError>
                {
                    let call = plan_config_change { config };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct EpochIndex;
            impl ::subxt::StorageEntry for EpochIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochIndex";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct GenesisSlot;
            impl ::subxt::StorageEntry for GenesisSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "GenesisSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Randomness;
            impl ::subxt::StorageEntry for Randomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Randomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingEpochConfigChange;
            impl ::subxt::StorageEntry for PendingEpochConfigChange {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "PendingEpochConfigChange";
                type Value = runtime_types::sp_consensus_babe::digests::NextConfigDescriptor;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextRandomness;
            impl ::subxt::StorageEntry for NextRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextRandomness";
                type Value = [::core::primitive::u8; 32usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextAuthorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SegmentIndex;
            impl ::subxt::StorageEntry for SegmentIndex {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "SegmentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnderConstruction(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnderConstruction {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "UnderConstruction";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    [::core::primitive::u8; 32usize],
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Initialized;
            impl ::subxt::StorageEntry for Initialized {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Initialized";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorVrfRandomness;
            impl ::subxt::StorageEntry for AuthorVrfRandomness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "AuthorVrfRandomness";
                type Value = ::core::option::Option<[::core::primitive::u8; 32usize]>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochStart;
            impl ::subxt::StorageEntry for EpochStart {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochStart";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Lateness;
            impl ::subxt::StorageEntry for Lateness {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "Lateness";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct EpochConfig;
            impl ::subxt::StorageEntry for EpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "EpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextEpochConfig;
            impl ::subxt::StorageEntry for NextEpochConfig {
                const PALLET: &'static str = "Babe";
                const STORAGE: &'static str = "NextEpochConfig";
                type Value = runtime_types::sp_consensus_babe::BabeEpochConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn epoch_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = EpochIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn genesis_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = GenesisSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_slot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_consensus_slots::Slot,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentSlot;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = Randomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_epoch_config_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingEpochConfigChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<[::core::primitive::u8; 32usize], ::subxt::BasicError>
                {
                    let entry = NextRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_consensus_babe::app::Public,
                        ::core::primitive::u64,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn segment_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = SegmentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        [::core::primitive::u8; 32usize],
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UnderConstruction(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn under_construction_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnderConstruction>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn initialized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Initialized;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn author_vrf_randomness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    ::subxt::BasicError,
                > {
                    let entry = AuthorVrfRandomness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_start(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u32, ::core::primitive::u32),
                    ::subxt::BasicError,
                > {
                    let entry = EpochStart;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn lateness(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Lateness;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = EpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_epoch_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_consensus_babe::BabeEpochConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextEpochConfig;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn epoch_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[176u8, 4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn expected_block_time(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[112u8, 23u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[3u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod timestamp {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set {
                #[codec(compact)]
                pub now: ::core::primitive::u64,
            }
            impl ::subxt::Call for set {
                const PALLET: &'static str = "Timestamp";
                const FUNCTION: &'static str = "set";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set(
                    &self,
                    now: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set, DispatchError>
                {
                    let call = set { now };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Now;
            impl ::subxt::StorageEntry for Now {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "Now";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidUpdate;
            impl ::subxt::StorageEntry for DidUpdate {
                const PALLET: &'static str = "Timestamp";
                const STORAGE: &'static str = "DidUpdate";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn now(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = Now;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn did_update(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidUpdate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn minimum_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[184u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod balances {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_balance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_balance {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_transfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_transfer {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct transfer_keep_alive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer_keep_alive {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct transfer_all {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for transfer_all {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_unreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_unreserve {
                const PALLET: &'static str = "Balances";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer, DispatchError>
                {
                    let call = transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_balance, DispatchError>
                {
                    let call = set_balance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_transfer, DispatchError>
                {
                    let call = force_transfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_keep_alive, DispatchError>
                {
                    let call = transfer_keep_alive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_all, DispatchError>
                {
                    let call = transfer_all { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_unreserve, DispatchError>
                {
                    let call = force_unreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::darwinia_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Endowed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DustLost(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ReserveRepatriated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Deposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Withdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Account";
                type Value =
                    runtime_types::drml_common_runtime::impls::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::darwinia_balances::pallet::BalanceLock<
                            ::core::primitive::u128,
                        >,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::darwinia_balances::pallet::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Balances";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::darwinia_balances::pallet::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::drml_common_runtime::impls::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::darwinia_balances::pallet::BalanceLock<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::darwinia_balances::pallet::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_balances::pallet::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod kton {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct transfer {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer {
                const PALLET: &'static str = "Kton";
                const FUNCTION: &'static str = "transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_balance {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub new_free: ::core::primitive::u128,
                #[codec(compact)]
                pub new_reserved: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_balance {
                const PALLET: &'static str = "Kton";
                const FUNCTION: &'static str = "set_balance";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_transfer {
                pub source:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_transfer {
                const PALLET: &'static str = "Kton";
                const FUNCTION: &'static str = "force_transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct transfer_keep_alive {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
            }
            impl ::subxt::Call for transfer_keep_alive {
                const PALLET: &'static str = "Kton";
                const FUNCTION: &'static str = "transfer_keep_alive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct transfer_all {
                pub dest:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub keep_alive: ::core::primitive::bool,
            }
            impl ::subxt::Call for transfer_all {
                const PALLET: &'static str = "Kton";
                const FUNCTION: &'static str = "transfer_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_unreserve {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Call for force_unreserve {
                const PALLET: &'static str = "Kton";
                const FUNCTION: &'static str = "force_unreserve";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transfer(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer, DispatchError>
                {
                    let call = transfer { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_balance(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    new_reserved: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_balance, DispatchError>
                {
                    let call = set_balance {
                        who,
                        new_free,
                        new_reserved,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_transfer(
                    &self,
                    source: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_transfer, DispatchError>
                {
                    let call = force_transfer {
                        source,
                        dest,
                        value,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_keep_alive(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_keep_alive, DispatchError>
                {
                    let call = transfer_keep_alive { dest, value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn transfer_all(
                    &self,
                    dest: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transfer_all, DispatchError>
                {
                    let call = transfer_all { dest, keep_alive };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unreserve(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_unreserve, DispatchError>
                {
                    let call = force_unreserve { who, amount };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::darwinia_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Endowed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DustLost(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Transfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BalanceSet(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Reserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Unreserved(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ReserveRepatriated(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            );
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Deposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Withdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Kton";
                const EVENT: &'static str = "Slashed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct TotalIssuance;
            impl ::subxt::StorageEntry for TotalIssuance {
                const PALLET: &'static str = "Kton";
                const STORAGE: &'static str = "TotalIssuance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Account(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Account {
                const PALLET: &'static str = "Kton";
                const STORAGE: &'static str = "Account";
                type Value =
                    runtime_types::drml_common_runtime::impls::AccountData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Locks(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Locks {
                const PALLET: &'static str = "Kton";
                const STORAGE: &'static str = "Locks";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::darwinia_balances::pallet::BalanceLock<
                            ::core::primitive::u128,
                        >,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Reserves(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Reserves {
                const PALLET: &'static str = "Kton";
                const STORAGE: &'static str = "Reserves";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::darwinia_balances::pallet::ReserveData<
                        [::core::primitive::u8; 8usize],
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Kton";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::darwinia_balances::pallet::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn total_issuance(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = TotalIssuance;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::drml_common_runtime::impls::AccountData<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = Account(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Account>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn locks(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::darwinia_balances::pallet::BalanceLock<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Locks(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn locks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Locks>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn reserves(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::darwinia_balances::pallet::ReserveData<
                            [::core::primitive::u8; 8usize],
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Reserves(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserves_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reserves>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_balances::pallet::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn existential_deposit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_locks(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_reserves(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod transaction_payment {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct NextFeeMultiplier;
            impl ::subxt::StorageEntry for NextFeeMultiplier {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "NextFeeMultiplier";
                type Value = runtime_types::sp_arithmetic::fixed_point::FixedU128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "TransactionPayment";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_transaction_payment::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn next_fee_multiplier(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::fixed_point::FixedU128,
                    ::subxt::BasicError,
                > {
                    let entry = NextFeeMultiplier;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_transaction_payment::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn transaction_byte_fee(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            80u8, 195u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn operational_fee_multiplier(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u8, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[5u8][..])?)
                }
                pub fn weight_to_fee(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::frame_support::weights::WeightToFeeCoefficient<
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            4u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 180u8, 196u8, 4u8, 0u8, 1u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod authorship {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_uncles {
                pub new_uncles: ::std::vec::Vec<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for set_uncles {
                const PALLET: &'static str = "Authorship";
                const FUNCTION: &'static str = "set_uncles";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_uncles(
                    &self,
                    new_uncles: ::std::vec::Vec<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_uncles, DispatchError>
                {
                    let call = set_uncles { new_uncles };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Uncles;
            impl ::subxt::StorageEntry for Uncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Uncles";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_authorship::UncleEntryItem<
                        ::core::primitive::u32,
                        ::subxt::sp_core::H256,
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Author;
            impl ::subxt::StorageEntry for Author {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "Author";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetUncles;
            impl ::subxt::StorageEntry for DidSetUncles {
                const PALLET: &'static str = "Authorship";
                const STORAGE: &'static str = "DidSetUncles";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_authorship::UncleEntryItem<
                            ::core::primitive::u32,
                            ::subxt::sp_core::H256,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Uncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn author(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Author;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_uncles(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidSetUncles;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn uncle_generations(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[5u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod election_provider_multi_phase {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct submit_unsigned { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 > > , pub witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , }
            impl ::subxt::Call for submit_unsigned {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_minimum_untrusted_score {
                pub maybe_next_score: ::core::option::Option<[::core::primitive::u128; 3usize]>,
            }
            impl ::subxt::Call for set_minimum_untrusted_score {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_minimum_untrusted_score";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_emergency_election_result {
                pub supports: ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_npos_elections::Support<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                )>,
            }
            impl ::subxt::Call for set_emergency_election_result {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "set_emergency_election_result";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct submit { pub raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 > > , pub num_signed_submissions : :: core :: primitive :: u32 , }
            impl ::subxt::Call for submit {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const FUNCTION: &'static str = "submit";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_unsigned(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 >,
                    witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, submit_unsigned, DispatchError>
                {
                    let call = submit_unsigned {
                        raw_solution: ::std::boxed::Box::new(raw_solution),
                        witness,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_minimum_untrusted_score(
                    &self,
                    maybe_next_score: ::core::option::Option<[::core::primitive::u128; 3usize]>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_minimum_untrusted_score,
                    DispatchError,
                > {
                    let call = set_minimum_untrusted_score { maybe_next_score };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_emergency_election_result(
                    &self,
                    supports: ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_npos_elections::Support<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_emergency_election_result,
                    DispatchError,
                > {
                    let call = set_emergency_election_result { supports };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit(
                    &self,
                    raw_solution : runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 >,
                    num_signed_submissions: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, submit, DispatchError>
                {
                    let call = submit {
                        raw_solution: ::std::boxed::Box::new(raw_solution),
                        num_signed_submissions,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_election_provider_multi_phase::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct SolutionStored(
                pub runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for SolutionStored {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SolutionStored";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ElectionFinalized(
                pub  ::core::option::Option<
                    runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                >,
            );
            impl ::subxt::Event for ElectionFinalized {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "ElectionFinalized";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct SignedPhaseStarted(pub ::core::primitive::u32);
            impl ::subxt::Event for SignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "SignedPhaseStarted";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct UnsignedPhaseStarted(pub ::core::primitive::u32);
            impl ::subxt::Event for UnsignedPhaseStarted {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const EVENT: &'static str = "UnsignedPhaseStarted";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Round;
            impl ::subxt::StorageEntry for Round {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Round";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPhase;
            impl ::subxt::StorageEntry for CurrentPhase {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "CurrentPhase";
                type Value = runtime_types::pallet_election_provider_multi_phase::Phase<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedSolution;
            impl ::subxt::StorageEntry for QueuedSolution {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "QueuedSolution";
                type Value = runtime_types::pallet_election_provider_multi_phase::ReadySolution<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Snapshot;
            impl ::subxt::StorageEntry for Snapshot {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "Snapshot";
                type Value = runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DesiredTargets;
            impl ::subxt::StorageEntry for DesiredTargets {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "DesiredTargets";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SnapshotMetadata;
            impl ::subxt::StorageEntry for SnapshotMetadata {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SnapshotMetadata";
                type Value =
                    runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionNextIndex;
            impl ::subxt::StorageEntry for SignedSubmissionNextIndex {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionNextIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionIndices;
            impl ::subxt::StorageEntry for SignedSubmissionIndices {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionIndices";
                type Value =
                    runtime_types::frame_support::storage::bounded_btree_map::BoundedBTreeMap<
                        [::core::primitive::u128; 3usize],
                        ::core::primitive::u32,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SignedSubmissionsMap(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for SignedSubmissionsMap {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "SignedSubmissionsMap";
                type Value = runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 > ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct MinimumUntrustedScore;
            impl ::subxt::StorageEntry for MinimumUntrustedScore {
                const PALLET: &'static str = "ElectionProviderMultiPhase";
                const STORAGE: &'static str = "MinimumUntrustedScore";
                type Value = [::core::primitive::u128; 3usize];
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn round(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Round;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_phase(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentPhase;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_solution(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_election_provider_multi_phase::ReadySolution<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = QueuedSolution;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn snapshot(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_election_provider_multi_phase::RoundSnapshot<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Snapshot;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn desired_targets(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = DesiredTargets;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn snapshot_metadata(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = SnapshotMetadata;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn signed_submission_next_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = SignedSubmissionNextIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signed_submission_indices(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_btree_map::BoundedBTreeMap<
                        [::core::primitive::u128; 3usize],
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = SignedSubmissionIndices;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn signed_submissions_map (& self , _0 : :: core :: primitive :: u32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < runtime_types :: pallet_election_provider_multi_phase :: signed :: SignedSubmission < :: subxt :: sp_core :: crypto :: AccountId32 , :: core :: primitive :: u128 , runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 > , :: subxt :: BasicError >{
                    let entry = SignedSubmissionsMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signed_submissions_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignedSubmissionsMap>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn minimum_untrusted_score(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<[::core::primitive::u128; 3usize]>,
                    ::subxt::BasicError,
                > {
                    let entry = MinimumUntrustedScore;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_phase(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[44u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn signed_phase(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn solution_improvement_threshold(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[32u8, 161u8, 7u8, 0u8][..],
                    )?)
                }
                pub fn offchain_repeat(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[5u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn miner_tx_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[101u8, 102u8, 102u8, 102u8, 102u8, 102u8, 102u8, 230u8][..],
                    )?)
                }
                pub fn miner_max_weight(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 216u8, 66u8, 105u8, 80u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn signed_max_submissions(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[10u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn signed_max_weight(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 216u8, 66u8, 105u8, 80u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn signed_reward_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            64u8, 66u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn signed_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            64u8, 66u8, 15u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn signed_deposit_byte(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            232u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn signed_deposit_weight(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn voter_snapshot_per_block(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[228u8, 87u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn miner_max_length(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 0u8, 54u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod staking {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct bond {
                pub controller:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub value: runtime_types::darwinia_staking::structs::StakingBalance<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >,
                pub payee: runtime_types::darwinia_staking::structs::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
                pub promise_month: ::core::primitive::u8,
            }
            impl ::subxt::Call for bond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct bond_extra {
                pub max_additional: runtime_types::darwinia_staking::structs::StakingBalance<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >,
                pub promise_month: ::core::primitive::u8,
            }
            impl ::subxt::Call for bond_extra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "bond_extra";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct deposit_extra {
                pub value: ::core::primitive::u128,
                pub promise_month: ::core::primitive::u8,
            }
            impl ::subxt::Call for deposit_extra {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "deposit_extra";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct unbond {
                pub value: runtime_types::darwinia_staking::structs::StakingBalance<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >,
            }
            impl ::subxt::Call for unbond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "unbond";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct withdraw_unbonded {
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for withdraw_unbonded {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "withdraw_unbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct claim_mature_deposits;
            impl ::subxt::Call for claim_mature_deposits {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "claim_mature_deposits";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct try_claim_deposits_with_punish {
                pub expire_time: ::core::primitive::u64,
            }
            impl ::subxt::Call for try_claim_deposits_with_punish {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "try_claim_deposits_with_punish";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct validate {
                pub prefs: runtime_types::darwinia_staking::structs::ValidatorPrefs,
            }
            impl ::subxt::Call for validate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "validate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct nominate {
                pub targets: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for nominate {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "nominate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct chill;
            impl ::subxt::Call for chill {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_payee {
                pub payee: runtime_types::darwinia_staking::structs::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >,
            }
            impl ::subxt::Call for set_payee {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_payee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_controller {
                pub controller:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for set_controller {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_controller";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_validator_count {
                #[codec(compact)]
                pub new: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_validator_count {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct increase_validator_count {
                #[codec(compact)]
                pub additional: ::core::primitive::u32,
            }
            impl ::subxt::Call for increase_validator_count {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "increase_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct scale_validator_count {
                pub factor: runtime_types::sp_arithmetic::per_things::Percent,
            }
            impl ::subxt::Call for scale_validator_count {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "scale_validator_count";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_no_eras;
            impl ::subxt::Call for force_no_eras {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_no_eras";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_new_era;
            impl ::subxt::Call for force_new_era {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_invulnerables {
                pub invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_invulnerables {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_unstake {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_unstake {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_unstake";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_new_era_always;
            impl ::subxt::Call for force_new_era_always {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "force_new_era_always";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct cancel_deferred_slash {
                pub era: ::core::primitive::u32,
                pub slash_indices: ::std::vec::Vec<::core::primitive::u32>,
            }
            impl ::subxt::Call for cancel_deferred_slash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "cancel_deferred_slash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct payout_stakers {
                pub validator_stash: ::subxt::sp_core::crypto::AccountId32,
                pub era: ::core::primitive::u32,
            }
            impl ::subxt::Call for payout_stakers {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "payout_stakers";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct rebond {
                #[codec(compact)]
                pub plan_to_rebond_ring: ::core::primitive::u128,
                #[codec(compact)]
                pub plan_to_rebond_kton: ::core::primitive::u128,
            }
            impl ::subxt::Call for rebond {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "rebond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_history_depth {
                #[codec(compact)]
                pub new_history_depth: ::core::primitive::u32,
                #[codec(compact)]
                pub era_items_deleted: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_history_depth {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_history_depth";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct reap_stash {
                pub stash: ::subxt::sp_core::crypto::AccountId32,
                pub num_slashing_spans: ::core::primitive::u32,
            }
            impl ::subxt::Call for reap_stash {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "reap_stash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct kick {
                pub who: ::std::vec::Vec<
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                >,
            }
            impl ::subxt::Call for kick {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "kick";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_staking_limits {
                pub min_nominator_bond: ::core::primitive::u128,
                pub min_validator_bond: ::core::primitive::u128,
                pub max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                pub max_validator_count: ::core::option::Option<::core::primitive::u32>,
                pub threshold:
                    ::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>,
            }
            impl ::subxt::Call for set_staking_limits {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "set_staking_limits";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct chill_other {
                pub controller: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for chill_other {
                const PALLET: &'static str = "Staking";
                const FUNCTION: &'static str = "chill_other";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn bond(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: runtime_types::darwinia_staking::structs::StakingBalance<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                    payee: runtime_types::darwinia_staking::structs::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    promise_month: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, bond, DispatchError>
                {
                    let call = bond {
                        controller,
                        value,
                        payee,
                        promise_month,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn bond_extra(
                    &self,
                    max_additional: runtime_types::darwinia_staking::structs::StakingBalance<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                    promise_month: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, bond_extra, DispatchError>
                {
                    let call = bond_extra {
                        max_additional,
                        promise_month,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn deposit_extra(
                    &self,
                    value: ::core::primitive::u128,
                    promise_month: ::core::primitive::u8,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, deposit_extra, DispatchError>
                {
                    let call = deposit_extra {
                        value,
                        promise_month,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unbond(
                    &self,
                    value: runtime_types::darwinia_staking::structs::StakingBalance<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, unbond, DispatchError>
                {
                    let call = unbond { value };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn withdraw_unbonded(
                    &self,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, withdraw_unbonded, DispatchError>
                {
                    let call = withdraw_unbonded { num_slashing_spans };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn claim_mature_deposits(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, claim_mature_deposits, DispatchError>
                {
                    let call = claim_mature_deposits {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn try_claim_deposits_with_punish(
                    &self,
                    expire_time: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    try_claim_deposits_with_punish,
                    DispatchError,
                > {
                    let call = try_claim_deposits_with_punish { expire_time };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn validate(
                    &self,
                    prefs: runtime_types::darwinia_staking::structs::ValidatorPrefs,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, validate, DispatchError>
                {
                    let call = validate { prefs };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn nominate(
                    &self,
                    targets: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, nominate, DispatchError>
                {
                    let call = nominate { targets };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, chill, DispatchError>
                {
                    let call = chill {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_payee(
                    &self,
                    payee: runtime_types::darwinia_staking::structs::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_payee, DispatchError>
                {
                    let call = set_payee { payee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_controller(
                    &self,
                    controller: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_controller, DispatchError>
                {
                    let call = set_controller { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_validator_count(
                    &self,
                    new: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_validator_count, DispatchError>
                {
                    let call = set_validator_count { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_validator_count(
                    &self,
                    additional: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    increase_validator_count,
                    DispatchError,
                > {
                    let call = increase_validator_count { additional };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn scale_validator_count(
                    &self,
                    factor: runtime_types::sp_arithmetic::per_things::Percent,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, scale_validator_count, DispatchError>
                {
                    let call = scale_validator_count { factor };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_no_eras(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_no_eras, DispatchError>
                {
                    let call = force_no_eras {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_new_era, DispatchError>
                {
                    let call = force_new_era {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_invulnerables(
                    &self,
                    invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_invulnerables, DispatchError>
                {
                    let call = set_invulnerables { invulnerables };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unstake(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_unstake, DispatchError>
                {
                    let call = force_unstake {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_new_era_always(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_new_era_always, DispatchError>
                {
                    let call = force_new_era_always {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_deferred_slash(
                    &self,
                    era: ::core::primitive::u32,
                    slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_deferred_slash, DispatchError>
                {
                    let call = cancel_deferred_slash { era, slash_indices };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn payout_stakers(
                    &self,
                    validator_stash: ::subxt::sp_core::crypto::AccountId32,
                    era: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, payout_stakers, DispatchError>
                {
                    let call = payout_stakers {
                        validator_stash,
                        era,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn rebond(
                    &self,
                    plan_to_rebond_ring: ::core::primitive::u128,
                    plan_to_rebond_kton: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, rebond, DispatchError>
                {
                    let call = rebond {
                        plan_to_rebond_ring,
                        plan_to_rebond_kton,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_history_depth(
                    &self,
                    new_history_depth: ::core::primitive::u32,
                    era_items_deleted: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_history_depth, DispatchError>
                {
                    let call = set_history_depth {
                        new_history_depth,
                        era_items_deleted,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reap_stash(
                    &self,
                    stash: ::subxt::sp_core::crypto::AccountId32,
                    num_slashing_spans: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, reap_stash, DispatchError>
                {
                    let call = reap_stash {
                        stash,
                        num_slashing_spans,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kick(
                    &self,
                    who: ::std::vec::Vec<
                        ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kick, DispatchError>
                {
                    let call = kick { who };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_staking_limits(
                    &self,
                    min_nominator_bond: ::core::primitive::u128,
                    min_validator_bond: ::core::primitive::u128,
                    max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                    max_validator_count: ::core::option::Option<::core::primitive::u32>,
                    threshold: ::core::option::Option<
                        runtime_types::sp_arithmetic::per_things::Percent,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_staking_limits, DispatchError>
                {
                    let call = set_staking_limits {
                        min_nominator_bond,
                        min_validator_bond,
                        max_nominator_count,
                        max_validator_count,
                        threshold,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn chill_other(
                    &self,
                    controller: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, chill_other, DispatchError>
                {
                    let call = chill_other { controller };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::darwinia_staking::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct EraPaid(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for EraPaid {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "EraPaid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Rewarded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Rewarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Rewarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Slashed(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Slashed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Slashed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct OldSlashingReportDiscarded(pub ::core::primitive::u32);
            impl ::subxt::Event for OldSlashingReportDiscarded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "OldSlashingReportDiscarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct StakersElected;
            impl ::subxt::Event for StakersElected {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakersElected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct RingBonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for RingBonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "RingBonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct KtonBonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for KtonBonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "KtonBonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct RingUnbonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for RingUnbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "RingUnbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct KtonUnbonded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for KtonUnbonded {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "KtonUnbonded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Kicked(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Kicked {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Kicked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct StakingElectionFailed;
            impl ::subxt::Event for StakingElectionFailed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "StakingElectionFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Chilled(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for Chilled {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "Chilled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct PayoutStarted(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for PayoutStarted {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "PayoutStarted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DepositsClaimed(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for DepositsClaimed {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "DepositsClaimed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DepositsClaimedWithPunish(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for DepositsClaimedWithPunish {
                const PALLET: &'static str = "Staking";
                const EVENT: &'static str = "DepositsClaimedWithPunish";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HistoryDepth;
            impl ::subxt::StorageEntry for HistoryDepth {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "HistoryDepth";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorCount;
            impl ::subxt::StorageEntry for ValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinimumValidatorCount;
            impl ::subxt::StorageEntry for MinimumValidatorCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinimumValidatorCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Invulnerables";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Bonded(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Bonded {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Bonded";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct MinNominatorBond;
            impl ::subxt::StorageEntry for MinNominatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinNominatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MinValidatorBond;
            impl ::subxt::StorageEntry for MinValidatorBond {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MinValidatorBond";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Ledger(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Ledger {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Ledger";
                type Value = runtime_types::darwinia_staking::structs::StakingLedger<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Payee(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Payee {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Payee";
                type Value = runtime_types::darwinia_staking::structs::RewardDestination<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Validators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Validators";
                type Value = runtime_types::darwinia_staking::structs::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CounterForValidators;
            impl ::subxt::StorageEntry for CounterForValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForValidators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxValidatorsCount;
            impl ::subxt::StorageEntry for MaxValidatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxValidatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nominators(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Nominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "Nominators";
                type Value = runtime_types::darwinia_staking::structs::Nominations<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct CounterForNominators;
            impl ::subxt::StorageEntry for CounterForNominators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CounterForNominators";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct MaxNominatorsCount;
            impl ::subxt::StorageEntry for MaxNominatorsCount {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "MaxNominatorsCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentEra;
            impl ::subxt::StorageEntry for CurrentEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentEra";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ActiveEra;
            impl ::subxt::StorageEntry for ActiveEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ActiveEra";
                type Value = runtime_types::darwinia_staking::structs::ActiveEraInfo;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ErasStartSessionIndex(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasStartSessionIndex {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStartSessionIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasStakers(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakers {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakers";
                type Value = runtime_types::darwinia_staking::structs::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasStakersClipped(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasStakersClipped {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasStakersClipped";
                type Value = runtime_types::darwinia_staking::structs::Exposure<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasValidatorPrefs(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ErasValidatorPrefs {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorPrefs";
                type Value = runtime_types::darwinia_staking::structs::ValidatorPrefs;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ErasValidatorReward(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasValidatorReward {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasValidatorReward";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasRewardPoints(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasRewardPoints {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasRewardPoints";
                type Value = runtime_types::darwinia_staking::structs::EraRewardPoints<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ErasTotalStake(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ErasTotalStake {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ErasTotalStake";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ForceEra;
            impl ::subxt::StorageEntry for ForceEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ForceEra";
                type Value = runtime_types::darwinia_staking::structs::Forcing;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SlashRewardFraction;
            impl ::subxt::StorageEntry for SlashRewardFraction {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashRewardFraction";
                type Value = runtime_types::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CanceledSlashPayout;
            impl ::subxt::StorageEntry for CanceledSlashPayout {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CanceledSlashPayout";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UnappliedSlashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for UnappliedSlashes {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "UnappliedSlashes";
                type Value = ::std::vec::Vec<
                    runtime_types::darwinia_staking::structs::UnappliedSlash<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct BondedEras;
            impl ::subxt::StorageEntry for BondedEras {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "BondedEras";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSlashInEra(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for ValidatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ValidatorSlashInEra";
                type Value = (
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    runtime_types::darwinia_staking::slashing::RK<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct NominatorSlashInEra(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for NominatorSlashInEra {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "NominatorSlashInEra";
                type Value = runtime_types::darwinia_staking::slashing::RK<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct SlashingSpans(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for SlashingSpans {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SlashingSpans";
                type Value = runtime_types::darwinia_staking::slashing::SlashingSpans;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct SpanSlash(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for SpanSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "SpanSlash";
                type Value = runtime_types::darwinia_staking::slashing::SpanRecord<
                    ::core::primitive::u128,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct EarliestUnappliedSlash;
            impl ::subxt::StorageEntry for EarliestUnappliedSlash {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "EarliestUnappliedSlash";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentPlannedSession;
            impl ::subxt::StorageEntry for CurrentPlannedSession {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "CurrentPlannedSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OffendingValidators;
            impl ::subxt::StorageEntry for OffendingValidators {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "OffendingValidators";
                type Value = ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::darwinia_staking::structs::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ChillThreshold;
            impl ::subxt::StorageEntry for ChillThreshold {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "ChillThreshold";
                type Value = runtime_types::sp_arithmetic::per_things::Percent;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LivingTime;
            impl ::subxt::StorageEntry for LivingTime {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "LivingTime";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PayoutFraction;
            impl ::subxt::StorageEntry for PayoutFraction {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "PayoutFraction";
                type Value = runtime_types::sp_arithmetic::per_things::Perbill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RingPool;
            impl ::subxt::StorageEntry for RingPool {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "RingPool";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct KtonPool;
            impl ::subxt::StorageEntry for KtonPool {
                const PALLET: &'static str = "Staking";
                const STORAGE: &'static str = "KtonPool";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn history_depth(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HistoryDepth;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn minimum_validator_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = MinimumValidatorCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn invulnerables(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Invulnerables;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn bonded(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Bonded(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn bonded_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Bonded>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn min_nominator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = MinNominatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn min_validator_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = MinValidatorBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ledger(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darwinia_staking::structs::StakingLedger<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Ledger(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn ledger_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Ledger>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn payee(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::RewardDestination<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Payee(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn payee_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Payee>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn validators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::ValidatorPrefs,
                    ::subxt::BasicError,
                > {
                    let entry = Validators(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Validators>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CounterForValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_validators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = MaxValidatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darwinia_staking::structs::Nominations<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Nominators(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominators_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Nominators>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn counter_for_nominators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CounterForNominators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn max_nominators_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = MaxNominatorsCount;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn active_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::darwinia_staking::structs::ActiveEraInfo>,
                    ::subxt::BasicError,
                > {
                    let entry = ActiveEra;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStartSessionIndex(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_start_session_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStartSessionIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStakers(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, ErasStakers>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_stakers_clipped(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasStakersClipped(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_stakers_clipped_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasStakersClipped>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_prefs(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::ValidatorPrefs,
                    ::subxt::BasicError,
                > {
                    let entry = ErasValidatorPrefs(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_validator_prefs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorPrefs>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_validator_reward(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = ErasValidatorReward(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn eras_validator_reward_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasValidatorReward>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_reward_points(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::EraRewardPoints<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ErasRewardPoints(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_reward_points_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasRewardPoints>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn eras_total_stake(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ErasTotalStake(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn eras_total_stake_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ErasTotalStake>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn force_era(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::Forcing,
                    ::subxt::BasicError,
                > {
                    let entry = ForceEra;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn slash_reward_fraction(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    let entry = SlashRewardFraction;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn canceled_slash_payout(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CanceledSlashPayout;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::darwinia_staking::structs::UnappliedSlash<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UnappliedSlashes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn unapplied_slashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, UnappliedSlashes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn bonded_eras(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = BondedEras;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        runtime_types::darwinia_staking::slashing::RK<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = ValidatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn validator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ValidatorSlashInEra>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn nominator_slash_in_era(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darwinia_staking::slashing::RK<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NominatorSlashInEra(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn nominator_slash_in_era_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, NominatorSlashInEra>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn slashing_spans(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::darwinia_staking::slashing::SlashingSpans,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = SlashingSpans(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn slashing_spans_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SlashingSpans>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn span_slash(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::slashing::SpanRecord<
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = SpanSlash(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn span_slash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, SpanSlash>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn earliest_unapplied_slash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = EarliestUnappliedSlash;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_planned_session(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentPlannedSession;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn offending_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::bool)>,
                    ::subxt::BasicError,
                > {
                    let entry = OffendingValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::darwinia_staking::structs::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn chill_threshold(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<runtime_types::sp_arithmetic::per_things::Percent>,
                    ::subxt::BasicError,
                > {
                    let entry = ChillThreshold;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn living_time(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = LivingTime;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn payout_fraction(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    let entry = PayoutFraction;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn ring_pool(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = RingPool;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn kton_pool(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = KtonPool;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn sessions_per_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[3u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn slash_defer_duration(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[55u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_nominator_rewarded_per_validator(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[64u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn bonding_duration_in_era(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[56u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn bonding_duration_in_block_number(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[128u8, 19u8, 3u8, 0u8][..],
                    )?)
                }
                pub fn cap(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 232u8, 137u8, 4u8, 35u8, 199u8, 138u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn total_power(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 202u8, 154u8, 59u8][..],
                    )?)
                }
                pub fn max_nominations(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[16u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod offences {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_offences::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Offence(
                pub [::core::primitive::u8; 16usize],
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for Offence {
                const PALLET: &'static str = "Offences";
                const EVENT: &'static str = "Offence";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Reports(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for Reports {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "Reports";
                type Value = runtime_types::sp_staking::offence::OffenceDetails<
                    ::subxt::sp_core::crypto::AccountId32,
                    (
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::darwinia_staking::structs::Exposure<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    ),
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct ConcurrentReportsIndex(
                pub [::core::primitive::u8; 16usize],
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for ConcurrentReportsIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ConcurrentReportsIndex";
                type Value = ::std::vec::Vec<::subxt::sp_core::H256>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct ReportsByKindIndex(pub [::core::primitive::u8; 16usize]);
            impl ::subxt::StorageEntry for ReportsByKindIndex {
                const PALLET: &'static str = "Offences";
                const STORAGE: &'static str = "ReportsByKindIndex";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn reports(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_staking::offence::OffenceDetails<
                            ::subxt::sp_core::crypto::AccountId32,
                            (
                                ::subxt::sp_core::crypto::AccountId32,
                                runtime_types::darwinia_staking::structs::Exposure<
                                    ::subxt::sp_core::crypto::AccountId32,
                                    ::core::primitive::u128,
                                    ::core::primitive::u128,
                                >,
                            ),
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Reports(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reports_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Reports>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn concurrent_reports_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = ConcurrentReportsIndex(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn concurrent_reports_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ConcurrentReportsIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn reports_by_kind_index(
                    &self,
                    _0: [::core::primitive::u8; 16usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = ReportsByKindIndex(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reports_by_kind_index_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReportsByKindIndex>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod historical {
        use super::runtime_types;
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_keys {
                pub keys: runtime_types::pangoro_runtime::pallets::session::SessionKeys,
                pub proof: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for set_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "set_keys";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct purge_keys;
            impl ::subxt::Call for purge_keys {
                const PALLET: &'static str = "Session";
                const FUNCTION: &'static str = "purge_keys";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_keys(
                    &self,
                    keys: runtime_types::pangoro_runtime::pallets::session::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_keys, DispatchError>
                {
                    let call = set_keys { keys, proof };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn purge_keys(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, purge_keys, DispatchError>
                {
                    let call = purge_keys {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_session::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewSession(pub ::core::primitive::u32);
            impl ::subxt::Event for NewSession {
                const PALLET: &'static str = "Session";
                const EVENT: &'static str = "NewSession";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Validators;
            impl ::subxt::StorageEntry for Validators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "Validators";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentIndex;
            impl ::subxt::StorageEntry for CurrentIndex {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "CurrentIndex";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedChanged;
            impl ::subxt::StorageEntry for QueuedChanged {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedChanged";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct QueuedKeys;
            impl ::subxt::StorageEntry for QueuedKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "QueuedKeys";
                type Value = ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::pangoro_runtime::pallets::session::SessionKeys,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DisabledValidators;
            impl ::subxt::StorageEntry for DisabledValidators {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "DisabledValidators";
                type Value = ::std::vec::Vec<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextKeys(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for NextKeys {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "NextKeys";
                type Value = runtime_types::pangoro_runtime::pallets::session::SessionKeys;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct KeyOwner(
                pub runtime_types::sp_core::crypto::KeyTypeId,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for KeyOwner {
                const PALLET: &'static str = "Session";
                const STORAGE: &'static str = "KeyOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Validators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = CurrentIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_changed(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = QueuedChanged;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queued_keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::pangoro_runtime::pallets::session::SessionKeys,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = QueuedKeys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn disabled_validators(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = DisabledValidators;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_keys(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pangoro_runtime::pallets::session::SessionKeys,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextKeys(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_keys_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, NextKeys>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn key_owner(
                    &self,
                    _0: runtime_types::sp_core::crypto::KeyTypeId,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = KeyOwner(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn key_owner_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, KeyOwner>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct report_equivocation {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for report_equivocation {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct report_equivocation_unsigned {
                pub equivocation_proof: ::std::boxed::Box<
                    runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                >,
                pub key_owner_proof: runtime_types::sp_core::Void,
            }
            impl ::subxt::Call for report_equivocation_unsigned {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "report_equivocation_unsigned";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct note_stalled {
                pub delay: ::core::primitive::u32,
                pub best_finalized_block_number: ::core::primitive::u32,
            }
            impl ::subxt::Call for note_stalled {
                const PALLET: &'static str = "Grandpa";
                const FUNCTION: &'static str = "note_stalled";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn report_equivocation(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, report_equivocation, DispatchError>
                {
                    let call = report_equivocation {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn report_equivocation_unsigned(
                    &self,
                    equivocation_proof: runtime_types::sp_finality_grandpa::EquivocationProof<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                    >,
                    key_owner_proof: runtime_types::sp_core::Void,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    report_equivocation_unsigned,
                    DispatchError,
                > {
                    let call = report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box::new(equivocation_proof),
                        key_owner_proof,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn note_stalled(
                    &self,
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, note_stalled, DispatchError>
                {
                    let call = note_stalled {
                        delay,
                        best_finalized_block_number,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_grandpa::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NewAuthorities(
                pub  ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            );
            impl ::subxt::Event for NewAuthorities {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "NewAuthorities";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Paused;
            impl ::subxt::Event for Paused {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Paused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Resumed;
            impl ::subxt::Event for Resumed {
                const PALLET: &'static str = "Grandpa";
                const EVENT: &'static str = "Resumed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct State;
            impl ::subxt::StorageEntry for State {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "State";
                type Value = runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingChange;
            impl ::subxt::StorageEntry for PendingChange {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "PendingChange";
                type Value =
                    runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextForced;
            impl ::subxt::StorageEntry for NextForced {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "NextForced";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Stalled;
            impl ::subxt::StorageEntry for Stalled {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "Stalled";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSetId;
            impl ::subxt::StorageEntry for CurrentSetId {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "CurrentSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SetIdSession(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for SetIdSession {
                const PALLET: &'static str = "Grandpa";
                const STORAGE: &'static str = "SetIdSession";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn state(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_grandpa::StoredState<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = State;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_change(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_grandpa::StoredPendingChange<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = PendingChange;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn next_forced(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = NextForced;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn stalled(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = Stalled;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = CurrentSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn set_id_session(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SetIdSession(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn set_id_session_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SetIdSession>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[3u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod beefy {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "Authorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidatorSetId;
            impl ::subxt::StorageEntry for ValidatorSetId {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "ValidatorSetId";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "Beefy";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn validator_set_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = ValidatorSetId;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<runtime_types::beefy_primitives::crypto::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod message_gadget {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_commitment_contract {
                pub commitment_contract: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for set_commitment_contract {
                const PALLET: &'static str = "MessageGadget";
                const FUNCTION: &'static str = "set_commitment_contract";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_commitment_contract(
                    &self,
                    commitment_contract: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_commitment_contract,
                    DispatchError,
                > {
                    let call = set_commitment_contract {
                        commitment_contract,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct CommitmentContract;
            impl ::subxt::StorageEntry for CommitmentContract {
                const PALLET: &'static str = "MessageGadget";
                const STORAGE: &'static str = "CommitmentContract";
                type Value = runtime_types::primitive_types::H160;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn commitment_contract(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::primitive_types::H160, ::subxt::BasicError>
                {
                    let entry = CommitmentContract;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod ecdsa_authority {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct add_authority {
                pub new: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for add_authority {
                const PALLET: &'static str = "EcdsaAuthority";
                const FUNCTION: &'static str = "add_authority";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct remove_authority {
                pub old: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for remove_authority {
                const PALLET: &'static str = "EcdsaAuthority";
                const FUNCTION: &'static str = "remove_authority";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct swap_authority {
                pub old: runtime_types::primitive_types::H160,
                pub new: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for swap_authority {
                const PALLET: &'static str = "EcdsaAuthority";
                const FUNCTION: &'static str = "swap_authority";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct submit_authorities_change_signature {
                pub address: runtime_types::primitive_types::H160,
                pub signature: runtime_types::sp_core::ecdsa::Signature,
            }
            impl ::subxt::Call for submit_authorities_change_signature {
                const PALLET: &'static str = "EcdsaAuthority";
                const FUNCTION: &'static str = "submit_authorities_change_signature";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct submit_new_message_root_signature {
                pub address: runtime_types::primitive_types::H160,
                pub signature: runtime_types::sp_core::ecdsa::Signature,
            }
            impl ::subxt::Call for submit_new_message_root_signature {
                const PALLET: &'static str = "EcdsaAuthority";
                const FUNCTION: &'static str = "submit_new_message_root_signature";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn add_authority(
                    &self,
                    new: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, add_authority, DispatchError>
                {
                    let call = add_authority { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_authority(
                    &self,
                    old: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_authority, DispatchError>
                {
                    let call = remove_authority { old };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn swap_authority(
                    &self,
                    old: runtime_types::primitive_types::H160,
                    new: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, swap_authority, DispatchError>
                {
                    let call = swap_authority { old, new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_authorities_change_signature(
                    &self,
                    address: runtime_types::primitive_types::H160,
                    signature: runtime_types::sp_core::ecdsa::Signature,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit_authorities_change_signature,
                    DispatchError,
                > {
                    let call = submit_authorities_change_signature { address, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn submit_new_message_root_signature(
                    &self,
                    address: runtime_types::primitive_types::H160,
                    signature: runtime_types::sp_core::ecdsa::Signature,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    submit_new_message_root_signature,
                    DispatchError,
                > {
                    let call = submit_new_message_root_signature { address, signature };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::darwinia_ecdsa_authority::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CollectingAuthoritiesChangeSignatures {
                pub message: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for CollectingAuthoritiesChangeSignatures {
                const PALLET: &'static str = "EcdsaAuthority";
                const EVENT: &'static str = "CollectingAuthoritiesChangeSignatures";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CollectedEnoughAuthoritiesChangeSignatures {
                pub operation: runtime_types::darwinia_ecdsa_authority::primitives::Operation,
                pub message: [::core::primitive::u8; 32usize],
                pub signatures: ::std::vec::Vec<(
                    runtime_types::primitive_types::H160,
                    runtime_types::sp_core::ecdsa::Signature,
                )>,
            }
            impl ::subxt::Event for CollectedEnoughAuthoritiesChangeSignatures {
                const PALLET: &'static str = "EcdsaAuthority";
                const EVENT: &'static str = "CollectedEnoughAuthoritiesChangeSignatures";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CollectingNewMessageRootSignatures {
                pub message: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for CollectingNewMessageRootSignatures {
                const PALLET: &'static str = "EcdsaAuthority";
                const EVENT: &'static str = "CollectingNewMessageRootSignatures";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CollectedEnoughNewMessageRootSignatures {
                pub commitment: runtime_types::darwinia_ecdsa_authority::primitives::Commitment,
                pub message: [::core::primitive::u8; 32usize],
                pub signatures: ::std::vec::Vec<(
                    runtime_types::primitive_types::H160,
                    runtime_types::sp_core::ecdsa::Signature,
                )>,
            }
            impl ::subxt::Event for CollectedEnoughNewMessageRootSignatures {
                const PALLET: &'static str = "EcdsaAuthority";
                const EVENT: &'static str = "CollectedEnoughNewMessageRootSignatures";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "EcdsaAuthority";
                const STORAGE: &'static str = "Authorities";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::primitive_types::H160,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NextAuthorities;
            impl ::subxt::StorageEntry for NextAuthorities {
                const PALLET: &'static str = "EcdsaAuthority";
                const STORAGE: &'static str = "NextAuthorities";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    runtime_types::primitive_types::H160,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Nonce;
            impl ::subxt::StorageEntry for Nonce {
                const PALLET: &'static str = "EcdsaAuthority";
                const STORAGE: &'static str = "Nonce";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthoritiesChangeToSign;
            impl ::subxt::StorageEntry for AuthoritiesChangeToSign {
                const PALLET: &'static str = "EcdsaAuthority";
                const STORAGE: &'static str = "AuthoritiesChangeToSign";
                type Value = (
                    runtime_types::darwinia_ecdsa_authority::primitives::Operation,
                    [::core::primitive::u8; 32usize],
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                        runtime_types::primitive_types::H160,
                        runtime_types::sp_core::ecdsa::Signature,
                    )>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NewMessageRootToSign;
            impl ::subxt::StorageEntry for NewMessageRootToSign {
                const PALLET: &'static str = "EcdsaAuthority";
                const STORAGE: &'static str = "NewMessageRootToSign";
                type Value = (
                    runtime_types::darwinia_ecdsa_authority::primitives::Commitment,
                    [::core::primitive::u8; 32usize],
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                        runtime_types::primitive_types::H160,
                        runtime_types::sp_core::ecdsa::Signature,
                    )>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PreviousMessageRoot;
            impl ::subxt::StorageEntry for PreviousMessageRoot {
                const PALLET: &'static str = "EcdsaAuthority";
                const STORAGE: &'static str = "PreviousMessageRoot";
                type Value = (::core::primitive::u32, ::subxt::sp_core::H256);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::primitive_types::H160,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn next_authorities(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::primitive_types::H160,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = NextAuthorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn nonce(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = Nonce;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authorities_change_to_sign(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::darwinia_ecdsa_authority::primitives::Operation,
                        [::core::primitive::u8; 32usize],
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            runtime_types::primitive_types::H160,
                            runtime_types::sp_core::ecdsa::Signature,
                        )>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = AuthoritiesChangeToSign;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn new_message_root_to_sign(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::darwinia_ecdsa_authority::primitives::Commitment,
                        [::core::primitive::u8; 32usize],
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                            runtime_types::primitive_types::H160,
                            runtime_types::sp_core::ecdsa::Signature,
                        )>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = NewMessageRootToSign;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn previous_message_root(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::subxt::sp_core::H256)>,
                    ::subxt::BasicError,
                > {
                    let entry = PreviousMessageRoot;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_authorities(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[3u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn chain_id(
                    &self,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(&mut &[8u8, 52u8, 53u8][..])?)
                }
                pub fn sign_threshold(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Perbill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 70u8, 195u8, 35u8][..],
                    )?)
                }
                pub fn sync_interval(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[10u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_pending_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod im_online {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct heartbeat {
                pub heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                pub signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
            }
            impl ::subxt::Call for heartbeat {
                const PALLET: &'static str = "ImOnline";
                const FUNCTION: &'static str = "heartbeat";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn heartbeat(
                    &self,
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, heartbeat, DispatchError>
                {
                    let call = heartbeat {
                        heartbeat,
                        signature,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_im_online::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct HeartbeatReceived(
                pub runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            );
            impl ::subxt::Event for HeartbeatReceived {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "HeartbeatReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct AllGood;
            impl ::subxt::Event for AllGood {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "AllGood";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct SomeOffline(
                pub  ::std::vec::Vec<(
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::darwinia_staking::structs::Exposure<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    >,
                )>,
            );
            impl ::subxt::Event for SomeOffline {
                const PALLET: &'static str = "ImOnline";
                const EVENT: &'static str = "SomeOffline";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct HeartbeatAfter;
            impl ::subxt::StorageEntry for HeartbeatAfter {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "HeartbeatAfter";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Keys;
            impl ::subxt::StorageEntry for Keys {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "Keys";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReceivedHeartbeats(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ReceivedHeartbeats {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "ReceivedHeartbeats";
                type Value = runtime_types::frame_support::traits::misc::WrapperOpaque<
                    runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct AuthoredBlocks(
                pub ::core::primitive::u32,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::StorageEntry for AuthoredBlocks {
                const PALLET: &'static str = "ImOnline";
                const STORAGE: &'static str = "AuthoredBlocks";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn heartbeat_after(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HeartbeatAfter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn keys(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Keys;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn received_heartbeats(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::frame_support::traits::misc::WrapperOpaque<
                            runtime_types::pallet_im_online::BoundedOpaqueNetworkState,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ReceivedHeartbeats(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn received_heartbeats_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ReceivedHeartbeats>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn authored_blocks(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AuthoredBlocks(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn authored_blocks_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AuthoredBlocks>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn unsigned_priority(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8][..],
                    )?)
                }
            }
        }
    }
    pub mod authority_discovery {
        use super::runtime_types;
    }
    pub mod treasury {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct propose_spend {
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                pub beneficiary:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for propose_spend {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "propose_spend";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct reject_proposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for reject_proposal {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "reject_proposal";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct approve_proposal {
                #[codec(compact)]
                pub proposal_id: ::core::primitive::u32,
            }
            impl ::subxt::Call for approve_proposal {
                const PALLET: &'static str = "Treasury";
                const FUNCTION: &'static str = "approve_proposal";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn propose_spend(
                    &self,
                    value: ::core::primitive::u128,
                    beneficiary: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, propose_spend, DispatchError>
                {
                    let call = propose_spend { value, beneficiary };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, reject_proposal, DispatchError>
                {
                    let call = reject_proposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_proposal(
                    &self,
                    proposal_id: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, approve_proposal, DispatchError>
                {
                    let call = approve_proposal { proposal_id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_treasury::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Proposed(pub ::core::primitive::u32);
            impl ::subxt::Event for Proposed {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Proposed";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Spending(pub ::core::primitive::u128);
            impl ::subxt::Event for Spending {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Spending";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Awarded(
                pub ::core::primitive::u32,
                pub ::core::primitive::u128,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for Awarded {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Awarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Rejected(pub ::core::primitive::u32, pub ::core::primitive::u128);
            impl ::subxt::Event for Rejected {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rejected";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Burnt(pub ::core::primitive::u128);
            impl ::subxt::Event for Burnt {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Burnt";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Rollover(pub ::core::primitive::u128);
            impl ::subxt::Event for Rollover {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Rollover";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Deposit(pub ::core::primitive::u128);
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Treasury";
                const EVENT: &'static str = "Deposit";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct ProposalCount;
            impl ::subxt::StorageEntry for ProposalCount {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "ProposalCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Proposals(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Proposals {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "Proposals";
                type Value = runtime_types::pallet_treasury::Proposal<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Approvals;
            impl ::subxt::StorageEntry for Approvals {
                const PALLET: &'static str = "Treasury";
                const STORAGE: &'static str = "Approvals";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn proposal_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ProposalCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proposals(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_treasury::Proposal<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Proposals(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn proposals_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proposals>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn approvals(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        ::core::primitive::u32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Approvals;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn proposal_bond(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[80u8, 195u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn proposal_bond_minimum(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 16u8, 165u8, 212u8, 232u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn spend_period(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 70u8, 5u8, 0u8][..],
                    )?)
                }
                pub fn burn(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[16u8, 39u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 97u8, 47u8, 116u8, 114u8, 115u8, 114u8, 121u8][..],
                    )?)
                }
                pub fn max_approvals(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod sudo {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct sudo {
                pub call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
            }
            impl ::subxt::Call for sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct sudo_unchecked_weight {
                pub call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                pub weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for sudo_unchecked_weight {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_unchecked_weight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_key {
                pub new:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
            }
            impl ::subxt::Call for set_key {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "set_key";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct sudo_as {
                pub who:
                    ::subxt::sp_runtime::MultiAddress<::subxt::sp_core::crypto::AccountId32, ()>,
                pub call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
            }
            impl ::subxt::Call for sudo_as {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo_as";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn sudo(
                    &self,
                    call: runtime_types::pangoro_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo, DispatchError>
                {
                    let call = sudo {
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::pangoro_runtime::Call,
                    weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo_unchecked_weight, DispatchError>
                {
                    let call = sudo_unchecked_weight {
                        call: ::std::boxed::Box::new(call),
                        weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_key(
                    &self,
                    new: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_key, DispatchError>
                {
                    let call = set_key { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_as(
                    &self,
                    who: ::subxt::sp_runtime::MultiAddress<
                        ::subxt::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call: runtime_types::pangoro_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo_as, DispatchError>
                {
                    let call = sudo_as {
                        who,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_sudo::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Sudid(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct KeyChanged(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct SudoAsDone(
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for SudoAsDone {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "SudoAsDone";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Key;
            impl ::subxt::StorageEntry for Key {
                const PALLET: &'static str = "Sudo";
                const STORAGE: &'static str = "Key";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn key(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = Key;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod scheduler {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct schedule {
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
            }
            impl ::subxt::Call for schedule {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct cancel {
                pub when: ::core::primitive::u32,
                pub index: ::core::primitive::u32,
            }
            impl ::subxt::Call for cancel {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct schedule_named {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub when: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
            }
            impl ::subxt::Call for schedule_named {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct cancel_named {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for cancel_named {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "cancel_named";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct schedule_after {
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
            }
            impl ::subxt::Call for schedule_after {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_after";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct schedule_named_after {
                pub id: ::std::vec::Vec<::core::primitive::u8>,
                pub after: ::core::primitive::u32,
                pub maybe_periodic:
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                pub priority: ::core::primitive::u8,
                pub call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
            }
            impl ::subxt::Call for schedule_named_after {
                const PALLET: &'static str = "Scheduler";
                const FUNCTION: &'static str = "schedule_named_after";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn schedule(
                    &self,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::pangoro_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, schedule, DispatchError>
                {
                    let call = schedule {
                        when,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel(
                    &self,
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel, DispatchError>
                {
                    let call = cancel { when, index };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::pangoro_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, schedule_named, DispatchError>
                {
                    let call = schedule_named {
                        id,
                        when,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_named(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_named, DispatchError>
                {
                    let call = cancel_named { id };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_after(
                    &self,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::pangoro_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, schedule_after, DispatchError>
                {
                    let call = schedule_after {
                        after,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn schedule_named_after(
                    &self,
                    id: ::std::vec::Vec<::core::primitive::u8>,
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<(
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    )>,
                    priority: ::core::primitive::u8,
                    call: runtime_types::pangoro_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, schedule_named_after, DispatchError>
                {
                    let call = schedule_named_after {
                        id,
                        after,
                        maybe_periodic,
                        priority,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_scheduler::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Scheduled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Scheduled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Scheduled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Canceled(pub ::core::primitive::u32, pub ::core::primitive::u32);
            impl ::subxt::Event for Canceled {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Canceled";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Dispatched(
                pub (::core::primitive::u32, ::core::primitive::u32),
                pub ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for Dispatched {
                const PALLET: &'static str = "Scheduler";
                const EVENT: &'static str = "Dispatched";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Agenda(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Agenda {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Agenda";
                type Value = ::std::vec::Vec<
                    ::core::option::Option<
                        runtime_types::pallet_scheduler::ScheduledV2<
                            runtime_types::pangoro_runtime::Call,
                            ::core::primitive::u32,
                            runtime_types::pangoro_runtime::OriginCaller,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Lookup(pub ::std::vec::Vec<::core::primitive::u8>);
            impl ::subxt::StorageEntry for Lookup {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "Lookup";
                type Value = (::core::primitive::u32, ::core::primitive::u32);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageVersion;
            impl ::subxt::StorageEntry for StorageVersion {
                const PALLET: &'static str = "Scheduler";
                const STORAGE: &'static str = "StorageVersion";
                type Value = runtime_types::pallet_scheduler::Releases;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn agenda(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        ::core::option::Option<
                            runtime_types::pallet_scheduler::ScheduledV2<
                                runtime_types::pangoro_runtime::Call,
                                ::core::primitive::u32,
                                runtime_types::pangoro_runtime::OriginCaller,
                                ::subxt::sp_core::crypto::AccountId32,
                            >,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Agenda(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn agenda_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Agenda>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn lookup(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    ::subxt::BasicError,
                > {
                    let entry = Lookup(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn lookup_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Lookup>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn storage_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_scheduler::Releases,
                    ::subxt::BasicError,
                > {
                    let entry = StorageVersion;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn maximum_weight(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 128u8, 110u8, 135u8, 116u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_scheduled_per_block(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod bridge_pangolin_dispatch {
        use super::runtime_types;
        pub type Event = runtime_types::pallet_bridge_dispatch::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageRejected(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageRejected {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageRejected";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageVersionSpecMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::primitive::u32,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for MessageVersionSpecMismatch {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageVersionSpecMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageWeightMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageWeightMismatch {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageWeightMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageSignatureMismatch(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageSignatureMismatch {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageSignatureMismatch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageCallDecodeFailed(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageCallDecodeFailed {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageCallDecodeFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageCallValidateFailed(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
            );
            impl ::subxt::Event for MessageCallValidateFailed {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageCallValidateFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageDispatchPaymentFailed(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageDispatchPaymentFailed {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageDispatchPaymentFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageDispatched(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                pub ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            );
            impl ::subxt::Event for MessageDispatched {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageDispatched";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct _Dummy;
            impl ::subxt::Event for _Dummy {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "_Dummy";
            }
        }
    }
    pub mod bridge_pangolin_grandpa {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct submit_finality_proof {
                pub finality_target: ::std::boxed::Box<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
                pub justification:
                    runtime_types::bp_header_chain::justification::GrandpaJustification<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
            }
            impl ::subxt::Call for submit_finality_proof {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const FUNCTION: &'static str = "submit_finality_proof";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct initialize {
                pub init_data: runtime_types::bp_header_chain::InitializationData<
                    runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                >,
            }
            impl ::subxt::Call for initialize {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const FUNCTION: &'static str = "initialize";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_owner {
                pub new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_owner {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const FUNCTION: &'static str = "set_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_operational {
                pub operational: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_operational {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const FUNCTION: &'static str = "set_operational";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn submit_finality_proof(
                    &self,
                    finality_target: runtime_types::sp_runtime::generic::header::Header<
                        ::core::primitive::u32,
                        runtime_types::sp_runtime::traits::BlakeTwo256,
                    >,
                    justification : runtime_types :: bp_header_chain :: justification :: GrandpaJustification < runtime_types :: sp_runtime :: generic :: header :: Header < :: core :: primitive :: u32 , runtime_types :: sp_runtime :: traits :: BlakeTwo256 > >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, submit_finality_proof, DispatchError>
                {
                    let call = submit_finality_proof {
                        finality_target: ::std::boxed::Box::new(finality_target),
                        justification,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn initialize(
                    &self,
                    init_data: runtime_types::bp_header_chain::InitializationData<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, initialize, DispatchError>
                {
                    let call = initialize { init_data };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_owner(
                    &self,
                    new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_owner, DispatchError>
                {
                    let call = set_owner { new_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_operational(
                    &self,
                    operational: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_operational, DispatchError>
                {
                    let call = set_operational { operational };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RequestCount;
            impl ::subxt::StorageEntry for RequestCount {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "RequestCount";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InitialHash;
            impl ::subxt::StorageEntry for InitialHash {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "InitialHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct BestFinalized;
            impl ::subxt::StorageEntry for BestFinalized {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "BestFinalized";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ImportedHashes(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for ImportedHashes {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "ImportedHashes";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct ImportedHashesPointer;
            impl ::subxt::StorageEntry for ImportedHashesPointer {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "ImportedHashesPointer";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ImportedHeaders(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for ImportedHeaders {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "ImportedHeaders";
                type Value = runtime_types::sp_runtime::generic::header::Header<
                    ::core::primitive::u32,
                    runtime_types::sp_runtime::traits::BlakeTwo256,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct CurrentAuthoritySet;
            impl ::subxt::StorageEntry for CurrentAuthoritySet {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "CurrentAuthoritySet";
                type Value = runtime_types::bp_header_chain::AuthoritySet;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PalletOwner;
            impl ::subxt::StorageEntry for PalletOwner {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "PalletOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct IsHalted;
            impl ::subxt::StorageEntry for IsHalted {
                const PALLET: &'static str = "BridgePangolinGrandpa";
                const STORAGE: &'static str = "IsHalted";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn request_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = RequestCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn initial_hash(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = InitialHash;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn best_finalized(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BestFinalized;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn imported_hashes(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = ImportedHashes(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn imported_hashes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ImportedHashes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn imported_hashes_pointer(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ImportedHashesPointer;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn imported_headers(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::sp_runtime::generic::header::Header<
                            ::core::primitive::u32,
                            runtime_types::sp_runtime::traits::BlakeTwo256,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ImportedHeaders(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn imported_headers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, ImportedHeaders>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn current_authority_set(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_header_chain::AuthoritySet,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentAuthoritySet;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pallet_owner(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOwner;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn is_halted(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = IsHalted;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn max_requests(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[50u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn headers_to_keep(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 137u8, 1u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod bridge_pangolin_messages {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_owner {
                pub new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_owner {
                const PALLET: &'static str = "BridgePangolinMessages";
                const FUNCTION: &'static str = "set_owner";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_operating_mode {
                pub operating_mode: runtime_types::bp_messages::OperatingMode,
            }
            impl ::subxt::Call for set_operating_mode {
                const PALLET: &'static str = "BridgePangolinMessages";
                const FUNCTION: &'static str = "set_operating_mode";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct update_pallet_parameter { pub parameter : runtime_types :: pangoro_runtime :: bridges_message :: pangolin :: PangoroToPangolinMessagesParameter , }
            impl ::subxt::Call for update_pallet_parameter {
                const PALLET: &'static str = "BridgePangolinMessages";
                const FUNCTION: &'static str = "update_pallet_parameter";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct send_message {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub payload: runtime_types::bp_message_dispatch::MessagePayload<
                    ::subxt::sp_core::crypto::AccountId32,
                    runtime_types::sp_runtime::MultiSigner,
                    runtime_types::sp_runtime::MultiSignature,
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub delivery_and_dispatch_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for send_message {
                const PALLET: &'static str = "BridgePangolinMessages";
                const FUNCTION: &'static str = "send_message";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct increase_message_fee {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub nonce: ::core::primitive::u64,
                pub additional_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for increase_message_fee {
                const PALLET: &'static str = "BridgePangolinMessages";
                const FUNCTION: &'static str = "increase_message_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct receive_messages_proof { pub relayer_id_at_bridged_chain : :: subxt :: sp_core :: crypto :: AccountId32 , pub proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 > , pub messages_count : :: core :: primitive :: u32 , pub dispatch_weight : :: core :: primitive :: u64 , }
            impl ::subxt::Call for receive_messages_proof {
                const PALLET: &'static str = "BridgePangolinMessages";
                const FUNCTION: &'static str = "receive_messages_proof";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct receive_messages_delivery_proof { pub proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 > , pub relayers_state : runtime_types :: bp_messages :: UnrewardedRelayersState , }
            impl ::subxt::Call for receive_messages_delivery_proof {
                const PALLET: &'static str = "BridgePangolinMessages";
                const FUNCTION: &'static str = "receive_messages_delivery_proof";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_owner(
                    &self,
                    new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_owner, DispatchError>
                {
                    let call = set_owner { new_owner };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_operating_mode(
                    &self,
                    operating_mode: runtime_types::bp_messages::OperatingMode,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_operating_mode, DispatchError>
                {
                    let call = set_operating_mode { operating_mode };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_pallet_parameter(
                    &self,
                    parameter : runtime_types :: pangoro_runtime :: bridges_message :: pangolin :: PangoroToPangolinMessagesParameter,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    update_pallet_parameter,
                    DispatchError,
                > {
                    let call = update_pallet_parameter { parameter };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn send_message(
                    &self,
                    lane_id: [::core::primitive::u8; 4usize],
                    payload: runtime_types::bp_message_dispatch::MessagePayload<
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::sp_runtime::MultiSigner,
                        runtime_types::sp_runtime::MultiSignature,
                        ::std::vec::Vec<::core::primitive::u8>,
                    >,
                    delivery_and_dispatch_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, send_message, DispatchError>
                {
                    let call = send_message {
                        lane_id,
                        payload,
                        delivery_and_dispatch_fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn increase_message_fee(
                    &self,
                    lane_id: [::core::primitive::u8; 4usize],
                    nonce: ::core::primitive::u64,
                    additional_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, increase_message_fee, DispatchError>
                {
                    let call = increase_message_fee {
                        lane_id,
                        nonce,
                        additional_fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn receive_messages_proof(
                    &self,
                    relayer_id_at_bridged_chain: ::subxt::sp_core::crypto::AccountId32,
                    proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 >,
                    messages_count: ::core::primitive::u32,
                    dispatch_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, receive_messages_proof, DispatchError>
                {
                    let call = receive_messages_proof {
                        relayer_id_at_bridged_chain,
                        proof,
                        messages_count,
                        dispatch_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn receive_messages_delivery_proof(
                    &self,
                    proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 >,
                    relayers_state: runtime_types::bp_messages::UnrewardedRelayersState,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    receive_messages_delivery_proof,
                    DispatchError,
                > {
                    let call = receive_messages_delivery_proof {
                        proof,
                        relayers_state,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_bridge_messages::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ParameterUpdated (pub runtime_types :: pangoro_runtime :: bridges_message :: pangolin :: PangoroToPangolinMessagesParameter ,) ;
            impl ::subxt::Event for ParameterUpdated {
                const PALLET: &'static str = "BridgePangolinMessages";
                const EVENT: &'static str = "ParameterUpdated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageAccepted(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for MessageAccepted {
                const PALLET: &'static str = "BridgePangolinMessages";
                const EVENT: &'static str = "MessageAccepted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessagesDelivered(
                pub [::core::primitive::u8; 4usize],
                pub runtime_types::bp_messages::DeliveredMessages,
            );
            impl ::subxt::Event for MessagesDelivered {
                const PALLET: &'static str = "BridgePangolinMessages";
                const EVENT: &'static str = "MessagesDelivered";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PalletOwner;
            impl ::subxt::StorageEntry for PalletOwner {
                const PALLET: &'static str = "BridgePangolinMessages";
                const STORAGE: &'static str = "PalletOwner";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PalletOperatingMode;
            impl ::subxt::StorageEntry for PalletOperatingMode {
                const PALLET: &'static str = "BridgePangolinMessages";
                const STORAGE: &'static str = "PalletOperatingMode";
                type Value = runtime_types::bp_messages::OperatingMode;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InboundLanes(pub [::core::primitive::u8; 4usize]);
            impl ::subxt::StorageEntry for InboundLanes {
                const PALLET: &'static str = "BridgePangolinMessages";
                const STORAGE: &'static str = "InboundLanes";
                type Value = runtime_types::bp_messages::InboundLaneData<
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct OutboundLanes(pub [::core::primitive::u8; 4usize]);
            impl ::subxt::StorageEntry for OutboundLanes {
                const PALLET: &'static str = "BridgePangolinMessages";
                const STORAGE: &'static str = "OutboundLanes";
                type Value = runtime_types::bp_messages::OutboundLaneData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct OutboundMessages(pub runtime_types::bp_messages::MessageKey);
            impl ::subxt::StorageEntry for OutboundMessages {
                const PALLET: &'static str = "BridgePangolinMessages";
                const STORAGE: &'static str = "OutboundMessages";
                type Value = runtime_types::bp_messages::MessageData<::core::primitive::u128>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn pallet_owner(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOwner;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn pallet_operating_mode(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::OperatingMode,
                    ::subxt::BasicError,
                > {
                    let entry = PalletOperatingMode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_lanes(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::InboundLaneData<
                        ::subxt::sp_core::crypto::AccountId32,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = InboundLanes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_lanes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InboundLanes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_lanes(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::bp_messages::OutboundLaneData,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundLanes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn outbound_lanes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundLanes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_messages(
                    &self,
                    _0: runtime_types::bp_messages::MessageKey,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::bp_messages::MessageData<::core::primitive::u128>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundMessages(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn outbound_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundMessages>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn bridged_chain_id(
                    &self,
                ) -> ::core::result::Result<[::core::primitive::u8; 4usize], ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[112u8, 97u8, 103u8, 108u8][..],
                    )?)
                }
            }
        }
    }
    pub mod pangolin_fee_market {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct enroll_and_lock_collateral {
                pub lock_collateral: ::core::primitive::u128,
                pub relay_fee: ::core::option::Option<::core::primitive::u128>,
            }
            impl ::subxt::Call for enroll_and_lock_collateral {
                const PALLET: &'static str = "PangolinFeeMarket";
                const FUNCTION: &'static str = "enroll_and_lock_collateral";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct update_locked_collateral {
                pub new_collateral: ::core::primitive::u128,
            }
            impl ::subxt::Call for update_locked_collateral {
                const PALLET: &'static str = "PangolinFeeMarket";
                const FUNCTION: &'static str = "update_locked_collateral";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct update_relay_fee {
                pub new_fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for update_relay_fee {
                const PALLET: &'static str = "PangolinFeeMarket";
                const FUNCTION: &'static str = "update_relay_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct cancel_enrollment;
            impl ::subxt::Call for cancel_enrollment {
                const PALLET: &'static str = "PangolinFeeMarket";
                const FUNCTION: &'static str = "cancel_enrollment";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_slash_protect {
                pub slash_protect: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_slash_protect {
                const PALLET: &'static str = "PangolinFeeMarket";
                const FUNCTION: &'static str = "set_slash_protect";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_assigned_relayers_number {
                pub number: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_assigned_relayers_number {
                const PALLET: &'static str = "PangolinFeeMarket";
                const FUNCTION: &'static str = "set_assigned_relayers_number";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn enroll_and_lock_collateral(
                    &self,
                    lock_collateral: ::core::primitive::u128,
                    relay_fee: ::core::option::Option<::core::primitive::u128>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    enroll_and_lock_collateral,
                    DispatchError,
                > {
                    let call = enroll_and_lock_collateral {
                        lock_collateral,
                        relay_fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_locked_collateral(
                    &self,
                    new_collateral: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    update_locked_collateral,
                    DispatchError,
                > {
                    let call = update_locked_collateral { new_collateral };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn update_relay_fee(
                    &self,
                    new_fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, update_relay_fee, DispatchError>
                {
                    let call = update_relay_fee { new_fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_enrollment(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_enrollment, DispatchError>
                {
                    let call = cancel_enrollment {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_slash_protect(
                    &self,
                    slash_protect: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_slash_protect, DispatchError>
                {
                    let call = set_slash_protect { slash_protect };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_assigned_relayers_number(
                    &self,
                    number: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_assigned_relayers_number,
                    DispatchError,
                > {
                    let call = set_assigned_relayers_number { number };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_fee_market::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Enroll(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for Enroll {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "Enroll";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UpdateLockedCollateral(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UpdateLockedCollateral {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "UpdateLockedCollateral";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UpdateRelayFee(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UpdateRelayFee {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "UpdateRelayFee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CancelEnrollment(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for CancelEnrollment {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "CancelEnrollment";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct UpdateCollateralSlashProtect(pub ::core::primitive::u128);
            impl ::subxt::Event for UpdateCollateralSlashProtect {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "UpdateCollateralSlashProtect";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct UpdateAssignedRelayersNumber(pub ::core::primitive::u32);
            impl ::subxt::Event for UpdateAssignedRelayersNumber {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "UpdateAssignedRelayersNumber";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct FeeMarketSlash(
                pub  runtime_types::pallet_fee_market::types::SlashReport<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                >,
            );
            impl ::subxt::Event for FeeMarketSlash {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "FeeMarketSlash";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OrderCreated(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u128,
                pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub ::core::option::Option<::core::primitive::u32>,
            );
            impl ::subxt::Event for OrderCreated {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "OrderCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OrderReward(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
                pub  runtime_types::pallet_fee_market::s2s::payment::RewardItem<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >,
            );
            impl ::subxt::Event for OrderReward {
                const PALLET: &'static str = "PangolinFeeMarket";
                const EVENT: &'static str = "OrderReward";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RelayersMap(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for RelayersMap {
                const PALLET: &'static str = "PangolinFeeMarket";
                const STORAGE: &'static str = "RelayersMap";
                type Value = runtime_types::pallet_fee_market::types::Relayer<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Relayers;
            impl ::subxt::StorageEntry for Relayers {
                const PALLET: &'static str = "PangolinFeeMarket";
                const STORAGE: &'static str = "Relayers";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssignedRelayers;
            impl ::subxt::StorageEntry for AssignedRelayers {
                const PALLET: &'static str = "PangolinFeeMarket";
                const STORAGE: &'static str = "AssignedRelayers";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_fee_market::types::Relayer<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Orders(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for Orders {
                const PALLET: &'static str = "PangolinFeeMarket";
                const STORAGE: &'static str = "Orders";
                type Value = runtime_types::pallet_fee_market::types::Order<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct CollateralSlashProtect;
            impl ::subxt::StorageEntry for CollateralSlashProtect {
                const PALLET: &'static str = "PangolinFeeMarket";
                const STORAGE: &'static str = "CollateralSlashProtect";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssignedRelayersNumber;
            impl ::subxt::StorageEntry for AssignedRelayersNumber {
                const PALLET: &'static str = "PangolinFeeMarket";
                const STORAGE: &'static str = "AssignedRelayersNumber";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn relayers_map(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::pallet_fee_market::types::Relayer<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = RelayersMap(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn relayers_map_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, RelayersMap>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn relayers(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Relayers;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn assigned_relayers(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<
                            runtime_types::pallet_fee_market::types::Relayer<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::core::primitive::u128,
                            >,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = AssignedRelayers;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn orders(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    _1: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_fee_market::types::Order<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Orders(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn orders_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Orders>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn collateral_slash_protect(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u128>,
                    ::subxt::BasicError,
                > {
                    let entry = CollateralSlashProtect;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn assigned_relayers_number(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AssignedRelayersNumber;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn treasury_pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 97u8, 47u8, 116u8, 114u8, 115u8, 114u8, 121u8][..],
                    )?)
                }
                pub fn lock_id(
                    &self,
                ) -> ::core::result::Result<[::core::primitive::u8; 8usize], ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 97u8, 47u8, 102u8, 101u8, 101u8, 108u8, 102u8][..],
                    )?)
                }
                pub fn minimum_relay_fee(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 214u8, 17u8, 126u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn collateral_per_order(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 116u8, 59u8, 164u8, 11u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn slot(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[44u8, 1u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn assigned_relayers_reward_ratio(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[192u8, 39u8, 9u8, 0u8][..],
                    )?)
                }
                pub fn message_relayers_reward_ratio(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[0u8, 53u8, 12u8, 0u8][..],
                    )?)
                }
                pub fn confirm_relayers_reward_ratio(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[64u8, 13u8, 3u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod transaction_pause {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct pause_transaction {
                pub pallet_name: ::std::vec::Vec<::core::primitive::u8>,
                pub function_name: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for pause_transaction {
                const PALLET: &'static str = "TransactionPause";
                const FUNCTION: &'static str = "pause_transaction";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct unpause_transaction {
                pub pallet_name: ::std::vec::Vec<::core::primitive::u8>,
                pub function_name: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for unpause_transaction {
                const PALLET: &'static str = "TransactionPause";
                const FUNCTION: &'static str = "unpause_transaction";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn pause_transaction(
                    &self,
                    pallet_name: ::std::vec::Vec<::core::primitive::u8>,
                    function_name: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, pause_transaction, DispatchError>
                {
                    let call = pause_transaction {
                        pallet_name,
                        function_name,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unpause_transaction(
                    &self,
                    pallet_name: ::std::vec::Vec<::core::primitive::u8>,
                    function_name: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, unpause_transaction, DispatchError>
                {
                    let call = unpause_transaction {
                        pallet_name,
                        function_name,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::module_transaction_pause::module::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TransactionPaused(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for TransactionPaused {
                const PALLET: &'static str = "TransactionPause";
                const EVENT: &'static str = "TransactionPaused";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TransactionUnpaused(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::Event for TransactionUnpaused {
                const PALLET: &'static str = "TransactionPause";
                const EVENT: &'static str = "TransactionUnpaused";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PausedTransactions(
                pub ::std::vec::Vec<::core::primitive::u8>,
                pub ::std::vec::Vec<::core::primitive::u8>,
            );
            impl ::subxt::StorageEntry for PausedTransactions {
                const PALLET: &'static str = "TransactionPause";
                const STORAGE: &'static str = "PausedTransactions";
                type Value = ();
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn paused_transactions(
                    &self,
                    _0: ::std::vec::Vec<::core::primitive::u8>,
                    _1: ::std::vec::Vec<::core::primitive::u8>,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::option::Option<()>, ::subxt::BasicError>
                {
                    let entry = PausedTransactions(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn paused_transactions_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, PausedTransactions>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod substrate2_substrate_backing {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct register_and_remote_create {
                pub spec_version: ::core::primitive::u32,
                pub weight: ::core::primitive::u64,
                pub fee: ::core::primitive::u128,
            }
            impl ::subxt::Call for register_and_remote_create {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const FUNCTION: &'static str = "register_and_remote_create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct lock_and_remote_issue {
                pub spec_version: ::core::primitive::u32,
                pub weight: ::core::primitive::u64,
                #[codec(compact)]
                pub value: ::core::primitive::u128,
                #[codec(compact)]
                pub fee: ::core::primitive::u128,
                pub recipient: runtime_types::primitive_types::H160,
            }
            impl ::subxt::Call for lock_and_remote_issue {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const FUNCTION: &'static str = "lock_and_remote_issue";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct unlock_from_remote {
                pub token_address: runtime_types::primitive_types::H160,
                pub amount: runtime_types::primitive_types::U256,
                pub recipient: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for unlock_from_remote {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const FUNCTION: &'static str = "unlock_from_remote";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_secure_limited_period {
                pub period: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_secure_limited_period {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const FUNCTION: &'static str = "set_secure_limited_period";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_security_limitation_ring_amount {
                pub limitation: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_security_limitation_ring_amount {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const FUNCTION: &'static str = "set_security_limitation_ring_amount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_remote_mapping_token_factory_account {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Call for set_remote_mapping_token_factory_account {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const FUNCTION: &'static str = "set_remote_mapping_token_factory_account";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn register_and_remote_create(
                    &self,
                    spec_version: ::core::primitive::u32,
                    weight: ::core::primitive::u64,
                    fee: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    register_and_remote_create,
                    DispatchError,
                > {
                    let call = register_and_remote_create {
                        spec_version,
                        weight,
                        fee,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn lock_and_remote_issue(
                    &self,
                    spec_version: ::core::primitive::u32,
                    weight: ::core::primitive::u64,
                    value: ::core::primitive::u128,
                    fee: ::core::primitive::u128,
                    recipient: runtime_types::primitive_types::H160,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, lock_and_remote_issue, DispatchError>
                {
                    let call = lock_and_remote_issue {
                        spec_version,
                        weight,
                        value,
                        fee,
                        recipient,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn unlock_from_remote(
                    &self,
                    token_address: runtime_types::primitive_types::H160,
                    amount: runtime_types::primitive_types::U256,
                    recipient: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, unlock_from_remote, DispatchError>
                {
                    let call = unlock_from_remote {
                        token_address,
                        amount,
                        recipient,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_secure_limited_period(
                    &self,
                    period: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_secure_limited_period,
                    DispatchError,
                > {
                    let call = set_secure_limited_period { period };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_security_limitation_ring_amount(
                    &self,
                    limitation: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_security_limitation_ring_amount,
                    DispatchError,
                > {
                    let call = set_security_limitation_ring_amount { limitation };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_remote_mapping_token_factory_account(
                    &self,
                    account: ::subxt::sp_core::crypto::AccountId32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    set_remote_mapping_token_factory_account,
                    DispatchError,
                > {
                    let call = set_remote_mapping_token_factory_account { account };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::to_substrate_backing::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TokenRegistered(
                pub runtime_types::dp_asset::TokenMetadata,
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for TokenRegistered {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const EVENT: &'static str = "TokenRegistered";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TokenLocked(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for TokenLocked {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const EVENT: &'static str = "TokenLocked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TokenUnlocked(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for TokenUnlocked {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const EVENT: &'static str = "TokenUnlocked";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TokenLockedConfirmed(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
                pub ::core::primitive::bool,
            );
            impl ::subxt::Event for TokenLockedConfirmed {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const EVENT: &'static str = "TokenLockedConfirmed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct RemoteMappingFactoryAddressUpdated(
                pub ::subxt::sp_core::crypto::AccountId32,
            );
            impl ::subxt::Event for RemoteMappingFactoryAddressUpdated {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const EVENT: &'static str = "RemoteMappingFactoryAddressUpdated";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct SecureLimitedPeriod;
            impl ::subxt::StorageEntry for SecureLimitedPeriod {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const STORAGE: &'static str = "SecureLimitedPeriod";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SecureLimitedRingAmount;
            impl ::subxt::StorageEntry for SecureLimitedRingAmount {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const STORAGE: &'static str = "SecureLimitedRingAmount";
                type Value = (::core::primitive::u128, ::core::primitive::u128);
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct TransactionInfos(
                pub [::core::primitive::u8; 4usize],
                pub ::core::primitive::u64,
            );
            impl ::subxt::StorageEntry for TransactionInfos {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const STORAGE: &'static str = "TransactionInfos";
                type Value = (
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct RemoteMappingTokenFactoryAccount;
            impl ::subxt::StorageEntry for RemoteMappingTokenFactoryAccount {
                const PALLET: &'static str = "Substrate2SubstrateBacking";
                const STORAGE: &'static str = "RemoteMappingTokenFactoryAccount";
                type Value = ::subxt::sp_core::crypto::AccountId32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn secure_limited_period(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = SecureLimitedPeriod;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn secure_limited_ring_amount(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (::core::primitive::u128, ::core::primitive::u128),
                    ::subxt::BasicError,
                > {
                    let entry = SecureLimitedRingAmount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn transaction_infos(
                    &self,
                    _0: [::core::primitive::u8; 4usize],
                    _1: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = TransactionInfos(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn transaction_infos_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, TransactionInfos>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn remote_mapping_token_factory_account(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::sp_core::crypto::AccountId32,
                    ::subxt::BasicError,
                > {
                    let entry = RemoteMappingTokenFactoryAccount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 97u8, 47u8, 115u8, 50u8, 115u8, 98u8, 97u8][..],
                    )?)
                }
                pub fn ring_metadata(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::dp_asset::TokenMetadata,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 0u8, 0u8, 109u8, 111u8, 100u8, 108u8, 100u8, 97u8, 47u8,
                            98u8, 114u8, 105u8, 110u8, 103u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 112u8, 80u8, 97u8, 110u8, 103u8, 111u8, 114u8, 111u8, 32u8, 78u8,
                            101u8, 116u8, 119u8, 111u8, 114u8, 107u8, 32u8, 78u8, 97u8, 116u8,
                            105u8, 118u8, 101u8, 32u8, 84u8, 111u8, 107u8, 101u8, 110u8, 20u8,
                            79u8, 82u8, 73u8, 78u8, 71u8, 9u8,
                        ][..],
                    )?)
                }
                pub fn max_lock_ring_amount_per_tx(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 160u8, 114u8, 78u8, 24u8, 9u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
            }
        }
    }
    pub mod evm {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct call {
                pub source: runtime_types::primitive_types::H160,
                pub target: runtime_types::primitive_types::H160,
                pub input: ::std::vec::Vec<::core::primitive::u8>,
                pub value: runtime_types::primitive_types::U256,
                pub gas_limit: ::core::primitive::u64,
                pub max_fee_per_gas: runtime_types::primitive_types::U256,
                pub max_priority_fee_per_gas:
                    ::core::option::Option<runtime_types::primitive_types::U256>,
                pub nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                pub access_list: ::std::vec::Vec<(
                    runtime_types::primitive_types::H160,
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                )>,
            }
            impl ::subxt::Call for call {
                const PALLET: &'static str = "EVM";
                const FUNCTION: &'static str = "call";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct create {
                pub source: runtime_types::primitive_types::H160,
                pub init: ::std::vec::Vec<::core::primitive::u8>,
                pub value: runtime_types::primitive_types::U256,
                pub gas_limit: ::core::primitive::u64,
                pub max_fee_per_gas: runtime_types::primitive_types::U256,
                pub max_priority_fee_per_gas:
                    ::core::option::Option<runtime_types::primitive_types::U256>,
                pub nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                pub access_list: ::std::vec::Vec<(
                    runtime_types::primitive_types::H160,
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                )>,
            }
            impl ::subxt::Call for create {
                const PALLET: &'static str = "EVM";
                const FUNCTION: &'static str = "create";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct create2 {
                pub source: runtime_types::primitive_types::H160,
                pub init: ::std::vec::Vec<::core::primitive::u8>,
                pub salt: ::subxt::sp_core::H256,
                pub value: runtime_types::primitive_types::U256,
                pub gas_limit: ::core::primitive::u64,
                pub max_fee_per_gas: runtime_types::primitive_types::U256,
                pub max_priority_fee_per_gas:
                    ::core::option::Option<runtime_types::primitive_types::U256>,
                pub nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                pub access_list: ::std::vec::Vec<(
                    runtime_types::primitive_types::H160,
                    ::std::vec::Vec<::subxt::sp_core::H256>,
                )>,
            }
            impl ::subxt::Call for create2 {
                const PALLET: &'static str = "EVM";
                const FUNCTION: &'static str = "create2";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn call(
                    &self,
                    source: runtime_types::primitive_types::H160,
                    target: runtime_types::primitive_types::H160,
                    input: ::std::vec::Vec<::core::primitive::u8>,
                    value: runtime_types::primitive_types::U256,
                    gas_limit: ::core::primitive::u64,
                    max_fee_per_gas: runtime_types::primitive_types::U256,
                    max_priority_fee_per_gas: ::core::option::Option<
                        runtime_types::primitive_types::U256,
                    >,
                    nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                    access_list: ::std::vec::Vec<(
                        runtime_types::primitive_types::H160,
                        ::std::vec::Vec<::subxt::sp_core::H256>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, call, DispatchError>
                {
                    let call = call {
                        source,
                        target,
                        input,
                        value,
                        gas_limit,
                        max_fee_per_gas,
                        max_priority_fee_per_gas,
                        nonce,
                        access_list,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create(
                    &self,
                    source: runtime_types::primitive_types::H160,
                    init: ::std::vec::Vec<::core::primitive::u8>,
                    value: runtime_types::primitive_types::U256,
                    gas_limit: ::core::primitive::u64,
                    max_fee_per_gas: runtime_types::primitive_types::U256,
                    max_priority_fee_per_gas: ::core::option::Option<
                        runtime_types::primitive_types::U256,
                    >,
                    nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                    access_list: ::std::vec::Vec<(
                        runtime_types::primitive_types::H160,
                        ::std::vec::Vec<::subxt::sp_core::H256>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create, DispatchError>
                {
                    let call = create {
                        source,
                        init,
                        value,
                        gas_limit,
                        max_fee_per_gas,
                        max_priority_fee_per_gas,
                        nonce,
                        access_list,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn create2(
                    &self,
                    source: runtime_types::primitive_types::H160,
                    init: ::std::vec::Vec<::core::primitive::u8>,
                    salt: ::subxt::sp_core::H256,
                    value: runtime_types::primitive_types::U256,
                    gas_limit: ::core::primitive::u64,
                    max_fee_per_gas: runtime_types::primitive_types::U256,
                    max_priority_fee_per_gas: ::core::option::Option<
                        runtime_types::primitive_types::U256,
                    >,
                    nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                    access_list: ::std::vec::Vec<(
                        runtime_types::primitive_types::H160,
                        ::std::vec::Vec<::subxt::sp_core::H256>,
                    )>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, create2, DispatchError>
                {
                    let call = create2 {
                        source,
                        init,
                        salt,
                        value,
                        gas_limit,
                        max_fee_per_gas,
                        max_priority_fee_per_gas,
                        nonce,
                        access_list,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::darwinia_evm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Log(pub runtime_types::ethereum::log::Log);
            impl ::subxt::Event for Log {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Log";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Created(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for Created {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Created";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CreatedFailed(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for CreatedFailed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "CreatedFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Executed(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "Executed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ExecutedFailed(pub runtime_types::primitive_types::H160);
            impl ::subxt::Event for ExecutedFailed {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "ExecutedFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BalanceDeposit(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::U256,
            );
            impl ::subxt::Event for BalanceDeposit {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "BalanceDeposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BalanceWithdraw(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::U256,
            );
            impl ::subxt::Event for BalanceWithdraw {
                const PALLET: &'static str = "EVM";
                const EVENT: &'static str = "BalanceWithdraw";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct AccountCodes(pub runtime_types::primitive_types::H160);
            impl ::subxt::StorageEntry for AccountCodes {
                const PALLET: &'static str = "EVM";
                const STORAGE: &'static str = "AccountCodes";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct AccountStorages(
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::StorageEntry for AccountStorages {
                const PALLET: &'static str = "EVM";
                const STORAGE: &'static str = "AccountStorages";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn account_codes(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = AccountCodes(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_codes_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AccountCodes>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn account_storages(
                    &self,
                    _0: runtime_types::primitive_types::H160,
                    _1: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = AccountStorages(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn account_storages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, AccountStorages>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod ethereum {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct transact {
                pub transaction: runtime_types::ethereum::transaction::TransactionV2,
            }
            impl ::subxt::Call for transact {
                const PALLET: &'static str = "Ethereum";
                const FUNCTION: &'static str = "transact";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct message_transact {
                pub transaction: runtime_types::ethereum::transaction::TransactionV2,
            }
            impl ::subxt::Call for message_transact {
                const PALLET: &'static str = "Ethereum";
                const FUNCTION: &'static str = "message_transact";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct root_transact {
                pub target: runtime_types::primitive_types::H160,
                pub input: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for root_transact {
                const PALLET: &'static str = "Ethereum";
                const FUNCTION: &'static str = "root_transact";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn transact(
                    &self,
                    transaction: runtime_types::ethereum::transaction::TransactionV2,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, transact, DispatchError>
                {
                    let call = transact { transaction };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn message_transact(
                    &self,
                    transaction: runtime_types::ethereum::transaction::TransactionV2,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, message_transact, DispatchError>
                {
                    let call = message_transact { transaction };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn root_transact(
                    &self,
                    target: runtime_types::primitive_types::H160,
                    input: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, root_transact, DispatchError>
                {
                    let call = root_transact { target, input };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::darwinia_ethereum::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Executed(
                pub runtime_types::primitive_types::H160,
                pub runtime_types::primitive_types::H160,
                pub ::subxt::sp_core::H256,
                pub runtime_types::evm_core::error::ExitReason,
            );
            impl ::subxt::Event for Executed {
                const PALLET: &'static str = "Ethereum";
                const EVENT: &'static str = "Executed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DVMTransfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::U256,
            );
            impl ::subxt::Event for DVMTransfer {
                const PALLET: &'static str = "Ethereum";
                const EVENT: &'static str = "DVMTransfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct KtonDVMTransfer(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::subxt::sp_core::crypto::AccountId32,
                pub runtime_types::primitive_types::U256,
            );
            impl ::subxt::Event for KtonDVMTransfer {
                const PALLET: &'static str = "Ethereum";
                const EVENT: &'static str = "KtonDVMTransfer";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Pending;
            impl ::subxt::StorageEntry for Pending {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "Pending";
                type Value = ::std::vec::Vec<(
                    runtime_types::ethereum::transaction::TransactionV2,
                    runtime_types::fp_rpc::TransactionStatus,
                    runtime_types::ethereum::receipt::ReceiptV3,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentBlock;
            impl ::subxt::StorageEntry for CurrentBlock {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "CurrentBlock";
                type Value = runtime_types::ethereum::block::Block<
                    runtime_types::ethereum::transaction::TransactionV2,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentReceipts;
            impl ::subxt::StorageEntry for CurrentReceipts {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "CurrentReceipts";
                type Value = ::std::vec::Vec<runtime_types::ethereum::receipt::ReceiptV3>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentTransactionStatuses;
            impl ::subxt::StorageEntry for CurrentTransactionStatuses {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "CurrentTransactionStatuses";
                type Value = ::std::vec::Vec<runtime_types::fp_rpc::TransactionStatus>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RemainingRingBalance(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for RemainingRingBalance {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "RemainingRingBalance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct RemainingKtonBalance(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for RemainingKtonBalance {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "RemainingKtonBalance";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct BlockHash(pub runtime_types::primitive_types::U256);
            impl ::subxt::StorageEntry for BlockHash {
                const PALLET: &'static str = "Ethereum";
                const STORAGE: &'static str = "BlockHash";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn pending(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        runtime_types::ethereum::transaction::TransactionV2,
                        runtime_types::fp_rpc::TransactionStatus,
                        runtime_types::ethereum::receipt::ReceiptV3,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Pending;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_block(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::ethereum::block::Block<
                            runtime_types::ethereum::transaction::TransactionV2,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentBlock;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_receipts(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<runtime_types::ethereum::receipt::ReceiptV3>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentReceipts;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn current_transaction_statuses(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        ::std::vec::Vec<runtime_types::fp_rpc::TransactionStatus>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentTransactionStatuses;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn remaining_ring_balance(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = RemainingRingBalance(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn remaining_ring_balance_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, RemainingRingBalance>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn remaining_kton_balance(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = RemainingKtonBalance(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn remaining_kton_balance_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, RemainingKtonBalance>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn block_hash(
                    &self,
                    _0: runtime_types::primitive_types::U256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::sp_core::H256, ::subxt::BasicError>
                {
                    let entry = BlockHash(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn block_hash_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, BlockHash>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 97u8, 114u8, 47u8, 100u8, 118u8, 109u8, 112u8][..],
                    )?)
                }
            }
        }
    }
    pub mod base_fee {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_base_fee_per_gas {
                pub fee: runtime_types::primitive_types::U256,
            }
            impl ::subxt::Call for set_base_fee_per_gas {
                const PALLET: &'static str = "BaseFee";
                const FUNCTION: &'static str = "set_base_fee_per_gas";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_is_active {
                pub is_active: ::core::primitive::bool,
            }
            impl ::subxt::Call for set_is_active {
                const PALLET: &'static str = "BaseFee";
                const FUNCTION: &'static str = "set_is_active";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_elasticity {
                pub elasticity: runtime_types::sp_arithmetic::per_things::Permill,
            }
            impl ::subxt::Call for set_elasticity {
                const PALLET: &'static str = "BaseFee";
                const FUNCTION: &'static str = "set_elasticity";
            }
            pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
                client: &'a ::subxt::Client<T>,
                marker: ::core::marker::PhantomData<(X, A)>,
            }
            impl<'a, T, X, A> TransactionApi<'a, T, X, A>
            where
                T: ::subxt::Config,
                X: ::subxt::SignedExtra<T>,
                A: ::subxt::AccountData,
            {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self {
                        client,
                        marker: ::core::marker::PhantomData,
                    }
                }
                pub fn set_base_fee_per_gas(
                    &self,
                    fee: runtime_types::primitive_types::U256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_base_fee_per_gas, DispatchError>
                {
                    let call = set_base_fee_per_gas { fee };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_is_active(
                    &self,
                    is_active: ::core::primitive::bool,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_is_active, DispatchError>
                {
                    let call = set_is_active { is_active };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_elasticity(
                    &self,
                    elasticity: runtime_types::sp_arithmetic::per_things::Permill,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_elasticity, DispatchError>
                {
                    let call = set_elasticity { elasticity };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_base_fee::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NewBaseFeePerGas(pub runtime_types::primitive_types::U256);
            impl ::subxt::Event for NewBaseFeePerGas {
                const PALLET: &'static str = "BaseFee";
                const EVENT: &'static str = "NewBaseFeePerGas";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BaseFeeOverflow;
            impl ::subxt::Event for BaseFeeOverflow {
                const PALLET: &'static str = "BaseFee";
                const EVENT: &'static str = "BaseFeeOverflow";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct IsActive(pub ::core::primitive::bool);
            impl ::subxt::Event for IsActive {
                const PALLET: &'static str = "BaseFee";
                const EVENT: &'static str = "IsActive";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NewElasticity(pub runtime_types::sp_arithmetic::per_things::Permill);
            impl ::subxt::Event for NewElasticity {
                const PALLET: &'static str = "BaseFee";
                const EVENT: &'static str = "NewElasticity";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct BaseFeePerGas;
            impl ::subxt::StorageEntry for BaseFeePerGas {
                const PALLET: &'static str = "BaseFee";
                const STORAGE: &'static str = "BaseFeePerGas";
                type Value = runtime_types::primitive_types::U256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct IsActive;
            impl ::subxt::StorageEntry for IsActive {
                const PALLET: &'static str = "BaseFee";
                const STORAGE: &'static str = "IsActive";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Elasticity;
            impl ::subxt::StorageEntry for Elasticity {
                const PALLET: &'static str = "BaseFee";
                const STORAGE: &'static str = "Elasticity";
                type Value = runtime_types::sp_arithmetic::per_things::Permill;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct StorageApi<'a, T: ::subxt::Config> {
                client: &'a ::subxt::Client<T>,
            }
            impl<'a, T: ::subxt::Config> StorageApi<'a, T> {
                pub fn new(client: &'a ::subxt::Client<T>) -> Self {
                    Self { client }
                }
                pub async fn base_fee_per_gas(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<runtime_types::primitive_types::U256, ::subxt::BasicError>
                {
                    let entry = BaseFeePerGas;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn is_active(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = IsActive;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn elasticity(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::sp_arithmetic::per_things::Permill,
                    ::subxt::BasicError,
                > {
                    let entry = Elasticity;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod runtime_types {
        use super::runtime_types;
        pub mod beefy_primitives {
            use super::runtime_types;
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            }
        }
        pub mod bitvec {
            use super::runtime_types;
            pub mod order {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Msb0;
            }
        }
        pub mod bp_header_chain {
            use super::runtime_types;
            pub mod justification {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct GrandpaJustification<_0> {
                    pub round: ::core::primitive::u64,
                    pub commit: runtime_types::finality_grandpa::Commit<
                        ::subxt::sp_core::H256,
                        ::core::primitive::u32,
                        runtime_types::sp_finality_grandpa::app::Signature,
                        runtime_types::sp_finality_grandpa::app::Public,
                    >,
                    pub votes_ancestries: ::std::vec::Vec<_0>,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct AuthoritySet {
                pub authorities: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
                pub set_id: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InitializationData<_0> {
                pub header: ::std::boxed::Box<_0>,
                pub authority_list: ::std::vec::Vec<(
                    runtime_types::sp_finality_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
                pub set_id: ::core::primitive::u64,
                pub is_halted: ::core::primitive::bool,
            }
        }
        pub mod bp_message_dispatch {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum CallOrigin<_0, _1, _2> {
                #[codec(index = 0)]
                SourceRoot,
                #[codec(index = 1)]
                TargetAccount(_0, _1, _2),
                #[codec(index = 2)]
                SourceAccount(_0),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessagePayload<_0, _1, _2, _3> {
                pub spec_version: ::core::primitive::u32,
                pub weight: ::core::primitive::u64,
                pub origin: runtime_types::bp_message_dispatch::CallOrigin<_0, _1, _2>,
                pub dispatch_fee_payment: runtime_types::bp_runtime::messages::DispatchFeePayment,
                pub call: _3,
            }
        }
        pub mod bp_messages {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DeliveredMessages {
                pub begin: ::core::primitive::u64,
                pub end: ::core::primitive::u64,
                pub dispatch_results: ::subxt::bitvec::vec::BitVec<
                    ::subxt::bitvec::order::Msb0,
                    ::core::primitive::u8,
                >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InboundLaneData<_0> {
                pub relayers: ::std::vec::Vec<runtime_types::bp_messages::UnrewardedRelayer<_0>>,
                pub last_confirmed_nonce: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageData<_0> {
                pub payload: ::std::vec::Vec<::core::primitive::u8>,
                pub fee: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageKey {
                pub lane_id: [::core::primitive::u8; 4usize],
                pub nonce: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum OperatingMode {
                #[codec(index = 0)]
                Normal,
                #[codec(index = 1)]
                RejectingOutboundMessages,
                #[codec(index = 2)]
                Halted,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OutboundLaneData {
                pub oldest_unpruned_nonce: ::core::primitive::u64,
                pub latest_received_nonce: ::core::primitive::u64,
                pub latest_generated_nonce: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UnrewardedRelayer<_0> {
                pub relayer: _0,
                pub messages: runtime_types::bp_messages::DeliveredMessages,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UnrewardedRelayersState {
                pub unrewarded_relayer_entries: ::core::primitive::u64,
                pub messages_in_oldest_entry: ::core::primitive::u64,
                pub total_messages: ::core::primitive::u64,
            }
        }
        pub mod bp_runtime {
            use super::runtime_types;
            pub mod messages {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum DispatchFeePayment {
                    #[codec(index = 0)]
                    AtSourceChain,
                    #[codec(index = 1)]
                    AtTargetChain,
                }
            }
        }
        pub mod bridge_runtime_common {
            use super::runtime_types;
            pub mod messages {
                use super::runtime_types;
                pub mod source {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct FromBridgedChainMessagesDeliveryProof<_0> {
                        pub bridged_header_hash: _0,
                        pub storage_proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                        pub lane: [::core::primitive::u8; 4usize],
                    }
                }
                pub mod target {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct FromBridgedChainMessagesProof<_0> {
                        pub bridged_header_hash: _0,
                        pub storage_proof: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                        pub lane: [::core::primitive::u8; 4usize],
                        pub nonces_start: ::core::primitive::u64,
                        pub nonces_end: ::core::primitive::u64,
                    }
                }
            }
        }
        pub mod darwinia_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct BalanceLock<_0> {
                    pub id: [::core::primitive::u8; 8usize],
                    pub amount: _0,
                    pub reasons: runtime_types::darwinia_balances::pallet::Reasons,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transfer {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    set_balance {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        new_free: ::core::primitive::u128,
                        #[codec(compact)]
                        new_reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    force_transfer {
                        source: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    transfer_keep_alive {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    transfer_all {
                        dest: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        keep_alive: ::core::primitive::bool,
                    },
                    #[codec(index = 5)]
                    force_unreserve {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        amount: ::core::primitive::u128,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    VestingBalance,
                    #[codec(index = 1)]
                    LiquidityRestrictions,
                    #[codec(index = 2)]
                    InsufficientBalance,
                    #[codec(index = 3)]
                    ExistentialDeposit,
                    #[codec(index = 4)]
                    KeepAlive,
                    #[codec(index = 5)]
                    ExistingVestingSchedule,
                    #[codec(index = 6)]
                    DeadAccount,
                    #[codec(index = 7)]
                    TooManyReserves,
                    #[codec(index = 8)]
                    LockP,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    DustLost(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Transfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    BalanceSet(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    Reserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 5)]
                    Unreserved(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 6)]
                    ReserveRepatriated(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    ),
                    #[codec(index = 7)]
                    Deposit(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 8)]
                    Withdraw(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 9)]
                    Slashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Reasons {
                    #[codec(index = 0)]
                    Fee,
                    #[codec(index = 1)]
                    Misc,
                    #[codec(index = 2)]
                    All,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Releases {
                    #[codec(index = 0)]
                    V1_0_0,
                    #[codec(index = 1)]
                    V2_0_0,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct ReserveData<_0, _1> {
                    pub id: _0,
                    pub amount: _1,
                }
            }
        }
        pub mod darwinia_ecdsa_authority {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    add_authority {
                        new: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 1)]
                    remove_authority {
                        old: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 2)]
                    swap_authority {
                        old: runtime_types::primitive_types::H160,
                        new: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 3)]
                    submit_authorities_change_signature {
                        address: runtime_types::primitive_types::H160,
                        signature: runtime_types::sp_core::ecdsa::Signature,
                    },
                    #[codec(index = 4)]
                    submit_new_message_root_signature {
                        address: runtime_types::primitive_types::H160,
                        signature: runtime_types::sp_core::ecdsa::Signature,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    AuthorityExisted,
                    #[codec(index = 1)]
                    TooManyAuthorities,
                    #[codec(index = 2)]
                    NotAuthority,
                    #[codec(index = 3)]
                    AtLeastOneAuthority,
                    #[codec(index = 4)]
                    OnAuthoritiesChange,
                    #[codec(index = 5)]
                    NoAuthoritiesChange,
                    #[codec(index = 6)]
                    NoNewMessageRoot,
                    #[codec(index = 7)]
                    BadSignature,
                    #[codec(index = 8)]
                    AlreadySubmitted,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    CollectingAuthoritiesChangeSignatures {
                        message: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    CollectedEnoughAuthoritiesChangeSignatures {
                        operation: runtime_types::darwinia_ecdsa_authority::primitives::Operation,
                        message: [::core::primitive::u8; 32usize],
                        signatures: ::std::vec::Vec<(
                            runtime_types::primitive_types::H160,
                            runtime_types::sp_core::ecdsa::Signature,
                        )>,
                    },
                    #[codec(index = 2)]
                    CollectingNewMessageRootSignatures {
                        message: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 3)]
                    CollectedEnoughNewMessageRootSignatures {
                        commitment: runtime_types::darwinia_ecdsa_authority::primitives::Commitment,
                        message: [::core::primitive::u8; 32usize],
                        signatures: ::std::vec::Vec<(
                            runtime_types::primitive_types::H160,
                            runtime_types::sp_core::ecdsa::Signature,
                        )>,
                    },
                }
            }
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Commitment {
                    pub block_number: ::core::primitive::u32,
                    pub message_root: ::subxt::sp_core::H256,
                    pub nonce: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Operation {
                    #[codec(index = 0)]
                    AddMember {
                        new: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 1)]
                    RemoveMember {
                        pre: runtime_types::primitive_types::H160,
                        old: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 2)]
                    SwapMembers {
                        pre: runtime_types::primitive_types::H160,
                        old: runtime_types::primitive_types::H160,
                        new: runtime_types::primitive_types::H160,
                    },
                }
            }
        }
        pub mod darwinia_ethereum {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    transact {
                        transaction: runtime_types::ethereum::transaction::TransactionV2,
                    },
                    #[codec(index = 1)]
                    message_transact {
                        transaction: runtime_types::ethereum::transaction::TransactionV2,
                    },
                    #[codec(index = 2)]
                    root_transact {
                        target: runtime_types::primitive_types::H160,
                        input: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSignature,
                    #[codec(index = 1)]
                    PreLogExists,
                    #[codec(index = 2)]
                    InternalTransactionExitError,
                    #[codec(index = 3)]
                    InternalTransactionRevertError,
                    #[codec(index = 4)]
                    InternalTransactionFatalError,
                    #[codec(index = 5)]
                    ReadyOnlyCall,
                    #[codec(index = 6)]
                    MessageTransactionError,
                    #[codec(index = 7)]
                    MessageValidateError,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Executed(
                        runtime_types::primitive_types::H160,
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::H256,
                        runtime_types::evm_core::error::ExitReason,
                    ),
                    #[codec(index = 1)]
                    DVMTransfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::U256,
                    ),
                    #[codec(index = 2)]
                    KtonDVMTransfer(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::U256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum RawOrigin {
                #[codec(index = 0)]
                EthereumTransaction(runtime_types::primitive_types::H160),
            }
        }
        pub mod darwinia_evm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    call {
                        source: runtime_types::primitive_types::H160,
                        target: runtime_types::primitive_types::H160,
                        input: ::std::vec::Vec<::core::primitive::u8>,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        max_fee_per_gas: runtime_types::primitive_types::U256,
                        max_priority_fee_per_gas:
                            ::core::option::Option<runtime_types::primitive_types::U256>,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                        access_list: ::std::vec::Vec<(
                            runtime_types::primitive_types::H160,
                            ::std::vec::Vec<::subxt::sp_core::H256>,
                        )>,
                    },
                    #[codec(index = 1)]
                    create {
                        source: runtime_types::primitive_types::H160,
                        init: ::std::vec::Vec<::core::primitive::u8>,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        max_fee_per_gas: runtime_types::primitive_types::U256,
                        max_priority_fee_per_gas:
                            ::core::option::Option<runtime_types::primitive_types::U256>,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                        access_list: ::std::vec::Vec<(
                            runtime_types::primitive_types::H160,
                            ::std::vec::Vec<::subxt::sp_core::H256>,
                        )>,
                    },
                    #[codec(index = 2)]
                    create2 {
                        source: runtime_types::primitive_types::H160,
                        init: ::std::vec::Vec<::core::primitive::u8>,
                        salt: ::subxt::sp_core::H256,
                        value: runtime_types::primitive_types::U256,
                        gas_limit: ::core::primitive::u64,
                        max_fee_per_gas: runtime_types::primitive_types::U256,
                        max_priority_fee_per_gas:
                            ::core::option::Option<runtime_types::primitive_types::U256>,
                        nonce: ::core::option::Option<runtime_types::primitive_types::U256>,
                        access_list: ::std::vec::Vec<(
                            runtime_types::primitive_types::H160,
                            ::std::vec::Vec<::subxt::sp_core::H256>,
                        )>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    BalanceLow,
                    #[codec(index = 1)]
                    FeeOverflow,
                    #[codec(index = 2)]
                    PaymentOverflow,
                    #[codec(index = 3)]
                    WithdrawFailed,
                    #[codec(index = 4)]
                    GasPriceTooLow,
                    #[codec(index = 5)]
                    InvalidNonce,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Log(runtime_types::ethereum::log::Log),
                    #[codec(index = 1)]
                    Created(runtime_types::primitive_types::H160),
                    #[codec(index = 2)]
                    CreatedFailed(runtime_types::primitive_types::H160),
                    #[codec(index = 3)]
                    Executed(runtime_types::primitive_types::H160),
                    #[codec(index = 4)]
                    ExecutedFailed(runtime_types::primitive_types::H160),
                    #[codec(index = 5)]
                    BalanceDeposit(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                        runtime_types::primitive_types::U256,
                    ),
                    #[codec(index = 6)]
                    BalanceWithdraw(
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                        runtime_types::primitive_types::U256,
                    ),
                }
            }
        }
        pub mod darwinia_message_gadget {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_commitment_contract {
                        commitment_contract: runtime_types::primitive_types::H160,
                    },
                }
            }
        }
        pub mod darwinia_staking {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    bond {
                        controller: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        value: runtime_types::darwinia_staking::structs::StakingBalance<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                        payee: runtime_types::darwinia_staking::structs::RewardDestination<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                        promise_month: ::core::primitive::u8,
                    },
                    #[codec(index = 1)]
                    bond_extra {
                        max_additional: runtime_types::darwinia_staking::structs::StakingBalance<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                        promise_month: ::core::primitive::u8,
                    },
                    #[codec(index = 2)]
                    deposit_extra {
                        value: ::core::primitive::u128,
                        promise_month: ::core::primitive::u8,
                    },
                    #[codec(index = 3)]
                    unbond {
                        value: runtime_types::darwinia_staking::structs::StakingBalance<
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                        >,
                    },
                    #[codec(index = 4)]
                    withdraw_unbonded {
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    claim_mature_deposits,
                    #[codec(index = 6)]
                    try_claim_deposits_with_punish { expire_time: ::core::primitive::u64 },
                    #[codec(index = 7)]
                    validate {
                        prefs: runtime_types::darwinia_staking::structs::ValidatorPrefs,
                    },
                    #[codec(index = 8)]
                    nominate {
                        targets: ::std::vec::Vec<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 9)]
                    chill,
                    #[codec(index = 10)]
                    set_payee {
                        payee: runtime_types::darwinia_staking::structs::RewardDestination<
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    },
                    #[codec(index = 11)]
                    set_controller {
                        controller: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 12)]
                    set_validator_count {
                        #[codec(compact)]
                        new: ::core::primitive::u32,
                    },
                    #[codec(index = 13)]
                    increase_validator_count {
                        #[codec(compact)]
                        additional: ::core::primitive::u32,
                    },
                    #[codec(index = 14)]
                    scale_validator_count {
                        factor: runtime_types::sp_arithmetic::per_things::Percent,
                    },
                    #[codec(index = 15)]
                    force_no_eras,
                    #[codec(index = 16)]
                    force_new_era,
                    #[codec(index = 17)]
                    set_invulnerables {
                        invulnerables: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 18)]
                    force_unstake {
                        stash: ::subxt::sp_core::crypto::AccountId32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    force_new_era_always,
                    #[codec(index = 20)]
                    cancel_deferred_slash {
                        era: ::core::primitive::u32,
                        slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 21)]
                    payout_stakers {
                        validator_stash: ::subxt::sp_core::crypto::AccountId32,
                        era: ::core::primitive::u32,
                    },
                    #[codec(index = 22)]
                    rebond {
                        #[codec(compact)]
                        plan_to_rebond_ring: ::core::primitive::u128,
                        #[codec(compact)]
                        plan_to_rebond_kton: ::core::primitive::u128,
                    },
                    #[codec(index = 23)]
                    set_history_depth {
                        #[codec(compact)]
                        new_history_depth: ::core::primitive::u32,
                        #[codec(compact)]
                        era_items_deleted: ::core::primitive::u32,
                    },
                    #[codec(index = 24)]
                    reap_stash {
                        stash: ::subxt::sp_core::crypto::AccountId32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 25)]
                    kick {
                        who: ::std::vec::Vec<
                            ::subxt::sp_runtime::MultiAddress<
                                ::subxt::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    #[codec(index = 26)]
                    set_staking_limits {
                        min_nominator_bond: ::core::primitive::u128,
                        min_validator_bond: ::core::primitive::u128,
                        max_nominator_count: ::core::option::Option<::core::primitive::u32>,
                        max_validator_count: ::core::option::Option<::core::primitive::u32>,
                        threshold: ::core::option::Option<
                            runtime_types::sp_arithmetic::per_things::Percent,
                        >,
                    },
                    #[codec(index = 27)]
                    chill_other {
                        controller: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    NotController,
                    #[codec(index = 1)]
                    NotStash,
                    #[codec(index = 2)]
                    AlreadyBonded,
                    #[codec(index = 3)]
                    AlreadyPaired,
                    #[codec(index = 4)]
                    EmptyTargets,
                    #[codec(index = 5)]
                    DuplicateIndex,
                    #[codec(index = 6)]
                    InvalidSlashIndex,
                    #[codec(index = 7)]
                    InsufficientBond,
                    #[codec(index = 8)]
                    NoMoreChunks,
                    #[codec(index = 9)]
                    NoUnlockChunk,
                    #[codec(index = 10)]
                    FundedTarget,
                    #[codec(index = 11)]
                    InvalidEraToReward,
                    #[codec(index = 12)]
                    InvalidNumberOfNominations,
                    #[codec(index = 13)]
                    NotSortedAndUnique,
                    #[codec(index = 14)]
                    AlreadyClaimed,
                    #[codec(index = 15)]
                    IncorrectHistoryDepth,
                    #[codec(index = 16)]
                    IncorrectSlashingSpans,
                    #[codec(index = 17)]
                    BadState,
                    #[codec(index = 18)]
                    TooManyTargets,
                    #[codec(index = 19)]
                    BadTarget,
                    #[codec(index = 20)]
                    CannotChillOther,
                    #[codec(index = 21)]
                    TooManyNominators,
                    #[codec(index = 22)]
                    TooManyValidators,
                    #[codec(index = 23)]
                    PayoutIns,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    EraPaid(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    Rewarded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    Slashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    OldSlashingReportDiscarded(::core::primitive::u32),
                    #[codec(index = 4)]
                    StakersElected,
                    #[codec(index = 5)]
                    RingBonded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    KtonBonded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 7)]
                    RingUnbonded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 8)]
                    KtonUnbonded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 9)]
                    Kicked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 10)]
                    StakingElectionFailed,
                    #[codec(index = 11)]
                    Chilled(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 12)]
                    PayoutStarted(
                        ::core::primitive::u32,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 13)]
                    DepositsClaimed(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 14)]
                    DepositsClaimedWithPunish(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct RK<_0, _1> {
                    pub r: _0,
                    pub k: _0,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct SlashingSpans {
                    pub span_index: ::core::primitive::u32,
                    pub last_start: ::core::primitive::u32,
                    pub last_nonzero_slash: ::core::primitive::u32,
                    pub prior: ::std::vec::Vec<::core::primitive::u32>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct SpanRecord<_0, _1> {
                    pub slashed: runtime_types::darwinia_staking::slashing::RK<_0, _0>,
                    pub paid_out: runtime_types::darwinia_staking::slashing::RK<_0, _0>,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
            }
            pub mod structs {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct ActiveEraInfo {
                    pub index: ::core::primitive::u32,
                    pub start: ::core::option::Option<::core::primitive::u64>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct EraRewardPoints<_0> {
                    pub total: ::core::primitive::u32,
                    pub individual: ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Exposure<_0, _1, _2> {
                    #[codec(compact)]
                    pub own_ring_balance: _1,
                    #[codec(compact)]
                    pub own_kton_balance: _1,
                    pub own_power: ::core::primitive::u32,
                    pub total_power: ::core::primitive::u32,
                    pub others: ::std::vec::Vec<
                        runtime_types::darwinia_staking::structs::IndividualExposure<_0, _1, _1>,
                    >,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Forcing {
                    #[codec(index = 0)]
                    NotForcing,
                    #[codec(index = 1)]
                    ForceNew,
                    #[codec(index = 2)]
                    ForceNone,
                    #[codec(index = 3)]
                    ForceAlways,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct IndividualExposure<_0, _1, _2> {
                    pub who: _0,
                    #[codec(compact)]
                    pub ring_balance: _1,
                    #[codec(compact)]
                    pub kton_balance: _1,
                    pub power: ::core::primitive::u32,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Nominations<_0> {
                    pub targets: ::std::vec::Vec<_0>,
                    pub submitted_in: ::core::primitive::u32,
                    pub suppressed: ::core::primitive::bool,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Releases {
                    #[codec(index = 0)]
                    V1_0_0Ancient,
                    #[codec(index = 1)]
                    V2_0_0,
                    #[codec(index = 2)]
                    V3_0_0,
                    #[codec(index = 3)]
                    V4_0_0,
                    #[codec(index = 4)]
                    V5_0_0,
                    #[codec(index = 5)]
                    V6_0_0,
                    #[codec(index = 6)]
                    V7_0_0,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum RewardDestination<_0> {
                    #[codec(index = 0)]
                    Staked,
                    #[codec(index = 1)]
                    Stash,
                    #[codec(index = 2)]
                    Controller,
                    #[codec(index = 3)]
                    Account(_0),
                    #[codec(index = 4)]
                    None,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum StakingBalance<_0, _1> {
                    #[codec(index = 0)]
                    RingBalance(_0),
                    #[codec(index = 1)]
                    KtonBalance(_0),
                    __Ignore(::core::marker::PhantomData<_1>),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct StakingLedger<_0, _1, _2, _3> {
                    pub stash: _0,
                    #[codec(compact)]
                    pub active: _1,
                    #[codec(compact)]
                    pub active_deposit_ring: _1,
                    #[codec(compact)]
                    pub active_kton: _1,
                    pub deposit_items: ::std::vec::Vec<
                        runtime_types::darwinia_staking::structs::TimeDepositItem<_1>,
                    >,
                    pub ring_staking_lock:
                        runtime_types::darwinia_support::structs::StakingLock<_1, _3>,
                    pub kton_staking_lock:
                        runtime_types::darwinia_support::structs::StakingLock<_1, _3>,
                    pub claimed_rewards: ::std::vec::Vec<_3>,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct TimeDepositItem<_0> {
                    #[codec(compact)]
                    pub value: _0,
                    #[codec(compact)]
                    pub start_time: ::core::primitive::u64,
                    #[codec(compact)]
                    pub expire_time: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct UnappliedSlash<_0, _1, _2> {
                    pub validator: _0,
                    pub own: runtime_types::darwinia_staking::slashing::RK<_1, _1>,
                    pub others: ::std::vec::Vec<(
                        _0,
                        runtime_types::darwinia_staking::slashing::RK<_1, _1>,
                    )>,
                    pub reporters: ::std::vec::Vec<_0>,
                    pub payout: runtime_types::darwinia_staking::slashing::RK<_1, _1>,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct ValidatorPrefs {
                    #[codec(compact)]
                    pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub blocked: ::core::primitive::bool,
                }
            }
        }
        pub mod darwinia_support {
            use super::runtime_types;
            pub mod structs {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct StakingLock<_0, _1> {
                    pub staking_amount: _0,
                    pub unbondings:
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            runtime_types::darwinia_support::structs::Unbonding<_0, _1>,
                        >,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Unbonding<_0, _1> {
                    pub amount: _0,
                    pub until: _1,
                }
            }
        }
        pub mod dp_asset {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TokenMetadata {
                pub token_type: ::core::primitive::u32,
                pub address: runtime_types::primitive_types::H160,
                pub name: ::std::vec::Vec<::core::primitive::u8>,
                pub symbol: ::std::vec::Vec<::core::primitive::u8>,
                pub decimal: ::core::primitive::u8,
            }
        }
        pub mod drml_common_runtime {
            use super::runtime_types;
            pub mod impls {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct AccountData<_0> {
                    pub free: _0,
                    pub reserved: _0,
                    pub free_kton: _0,
                    pub reserved_kton: _0,
                }
            }
        }
        pub mod ethbloom {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Bloom(pub [::core::primitive::u8; 256usize]);
        }
        pub mod ethereum {
            use super::runtime_types;
            pub mod block {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Block<_0> {
                    pub header: runtime_types::ethereum::header::Header,
                    pub transactions: ::std::vec::Vec<_0>,
                    pub ommers: ::std::vec::Vec<runtime_types::ethereum::header::Header>,
                }
            }
            pub mod header {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Header {
                    pub parent_hash: ::subxt::sp_core::H256,
                    pub ommers_hash: ::subxt::sp_core::H256,
                    pub beneficiary: runtime_types::primitive_types::H160,
                    pub state_root: ::subxt::sp_core::H256,
                    pub transactions_root: ::subxt::sp_core::H256,
                    pub receipts_root: ::subxt::sp_core::H256,
                    pub logs_bloom: runtime_types::ethbloom::Bloom,
                    pub difficulty: runtime_types::primitive_types::U256,
                    pub number: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub gas_used: runtime_types::primitive_types::U256,
                    pub timestamp: ::core::primitive::u64,
                    pub extra_data: ::std::vec::Vec<::core::primitive::u8>,
                    pub mix_hash: ::subxt::sp_core::H256,
                    pub nonce: runtime_types::ethereum_types::hash::H64,
                }
            }
            pub mod log {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Log {
                    pub address: runtime_types::primitive_types::H160,
                    pub topics: ::std::vec::Vec<::subxt::sp_core::H256>,
                    pub data: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod receipt {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct EIP658ReceiptData {
                    pub status_code: ::core::primitive::u8,
                    pub used_gas: runtime_types::primitive_types::U256,
                    pub logs_bloom: runtime_types::ethbloom::Bloom,
                    pub logs: ::std::vec::Vec<runtime_types::ethereum::log::Log>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum ReceiptV3 {
                    #[codec(index = 0)]
                    Legacy(runtime_types::ethereum::receipt::EIP658ReceiptData),
                    #[codec(index = 1)]
                    EIP2930(runtime_types::ethereum::receipt::EIP658ReceiptData),
                    #[codec(index = 2)]
                    EIP1559(runtime_types::ethereum::receipt::EIP658ReceiptData),
                }
            }
            pub mod transaction {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct AccessListItem {
                    pub address: runtime_types::primitive_types::H160,
                    pub slots: ::std::vec::Vec<::subxt::sp_core::H256>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct EIP1559Transaction {
                    pub chain_id: ::core::primitive::u64,
                    pub nonce: runtime_types::primitive_types::U256,
                    pub max_priority_fee_per_gas: runtime_types::primitive_types::U256,
                    pub max_fee_per_gas: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub action: runtime_types::ethereum::transaction::TransactionAction,
                    pub value: runtime_types::primitive_types::U256,
                    pub input: ::std::vec::Vec<::core::primitive::u8>,
                    pub access_list:
                        ::std::vec::Vec<runtime_types::ethereum::transaction::AccessListItem>,
                    pub odd_y_parity: ::core::primitive::bool,
                    pub r: ::subxt::sp_core::H256,
                    pub s: ::subxt::sp_core::H256,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct EIP2930Transaction {
                    pub chain_id: ::core::primitive::u64,
                    pub nonce: runtime_types::primitive_types::U256,
                    pub gas_price: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub action: runtime_types::ethereum::transaction::TransactionAction,
                    pub value: runtime_types::primitive_types::U256,
                    pub input: ::std::vec::Vec<::core::primitive::u8>,
                    pub access_list:
                        ::std::vec::Vec<runtime_types::ethereum::transaction::AccessListItem>,
                    pub odd_y_parity: ::core::primitive::bool,
                    pub r: ::subxt::sp_core::H256,
                    pub s: ::subxt::sp_core::H256,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct LegacyTransaction {
                    pub nonce: runtime_types::primitive_types::U256,
                    pub gas_price: runtime_types::primitive_types::U256,
                    pub gas_limit: runtime_types::primitive_types::U256,
                    pub action: runtime_types::ethereum::transaction::TransactionAction,
                    pub value: runtime_types::primitive_types::U256,
                    pub input: ::std::vec::Vec<::core::primitive::u8>,
                    pub signature: runtime_types::ethereum::transaction::TransactionSignature,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum TransactionAction {
                    #[codec(index = 0)]
                    Call(runtime_types::primitive_types::H160),
                    #[codec(index = 1)]
                    Create,
                }
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct TransactionRecoveryId(pub ::core::primitive::u64);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct TransactionSignature {
                    pub v: runtime_types::ethereum::transaction::TransactionRecoveryId,
                    pub r: ::subxt::sp_core::H256,
                    pub s: ::subxt::sp_core::H256,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum TransactionV2 {
                    #[codec(index = 0)]
                    Legacy(runtime_types::ethereum::transaction::LegacyTransaction),
                    #[codec(index = 1)]
                    EIP2930(runtime_types::ethereum::transaction::EIP2930Transaction),
                    #[codec(index = 2)]
                    EIP1559(runtime_types::ethereum::transaction::EIP1559Transaction),
                }
            }
        }
        pub mod ethereum_types {
            use super::runtime_types;
            pub mod hash {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct H64(pub [::core::primitive::u8; 8usize]);
            }
        }
        pub mod evm_core {
            use super::runtime_types;
            pub mod error {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum ExitError {
                    #[codec(index = 0)]
                    StackUnderflow,
                    #[codec(index = 1)]
                    StackOverflow,
                    #[codec(index = 2)]
                    InvalidJump,
                    #[codec(index = 3)]
                    InvalidRange,
                    #[codec(index = 4)]
                    DesignatedInvalid,
                    #[codec(index = 5)]
                    CallTooDeep,
                    #[codec(index = 6)]
                    CreateCollision,
                    #[codec(index = 7)]
                    CreateContractLimit,
                    #[codec(index = 8)]
                    InvalidCode,
                    #[codec(index = 9)]
                    OutOfOffset,
                    #[codec(index = 10)]
                    OutOfGas,
                    #[codec(index = 11)]
                    OutOfFund,
                    #[codec(index = 12)]
                    PCUnderflow,
                    #[codec(index = 13)]
                    CreateEmpty,
                    #[codec(index = 14)]
                    Other(::std::string::String),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum ExitFatal {
                    #[codec(index = 0)]
                    NotSupported,
                    #[codec(index = 1)]
                    UnhandledInterrupt,
                    #[codec(index = 2)]
                    CallErrorAsFatal(runtime_types::evm_core::error::ExitError),
                    #[codec(index = 3)]
                    Other(::std::string::String),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum ExitReason {
                    #[codec(index = 0)]
                    Succeed(runtime_types::evm_core::error::ExitSucceed),
                    #[codec(index = 1)]
                    Error(runtime_types::evm_core::error::ExitError),
                    #[codec(index = 2)]
                    Revert(runtime_types::evm_core::error::ExitRevert),
                    #[codec(index = 3)]
                    Fatal(runtime_types::evm_core::error::ExitFatal),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum ExitRevert {
                    #[codec(index = 0)]
                    Reverted,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum ExitSucceed {
                    #[codec(index = 0)]
                    Stopped,
                    #[codec(index = 1)]
                    Returned,
                    #[codec(index = 2)]
                    Suicided,
                }
            }
        }
        pub mod finality_grandpa {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Commit<_0, _1, _2, _3> {
                pub target_hash: _0,
                pub target_number: _1,
                pub precommits: ::std::vec::Vec<
                    runtime_types::finality_grandpa::SignedPrecommit<_0, _1, _2, _3>,
                >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Equivocation<_0, _1, _2> {
                pub round_number: ::core::primitive::u64,
                pub identity: _0,
                pub first: (_1, _2),
                pub second: (_1, _2),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Precommit<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Prevote<_0, _1> {
                pub target_hash: _0,
                pub target_number: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct SignedPrecommit<_0, _1, _2, _3> {
                pub precommit: runtime_types::finality_grandpa::Precommit<_0, _1>,
                pub signature: _2,
                pub id: _3,
            }
        }
        pub mod fp_rpc {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct TransactionStatus {
                pub transaction_hash: ::subxt::sp_core::H256,
                pub transaction_index: ::core::primitive::u32,
                pub from: runtime_types::primitive_types::H160,
                pub to: ::core::option::Option<runtime_types::primitive_types::H160>,
                pub contract_address: ::core::option::Option<runtime_types::primitive_types::H160>,
                pub logs: ::std::vec::Vec<runtime_types::ethereum::log::Log>,
                pub logs_bloom: runtime_types::ethbloom::Bloom,
            }
        }
        pub mod fp_self_contained {
            use super::runtime_types;
            pub mod unchecked_extrinsic {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                    pub 
                        runtime_types::sp_runtime::generic::unchecked_extrinsic::UncheckedExtrinsic<
                            _0,
                            _1,
                            _2,
                            _3,
                        >,
                );
            }
        }
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
                pub mod bounded_btree_map {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct BoundedBTreeMap<_0, _1>(pub ::std::collections::BTreeMap<_0, _1>);
                }
                pub mod bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
                pub mod weak_bounded_vec {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
                }
            }
            pub mod traits {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct WrapperOpaque<_0>(
                        #[codec(compact)] pub ::core::primitive::u32,
                        pub _0,
                    );
                }
                pub mod tokens {
                    use super::runtime_types;
                    pub mod misc {
                        use super::runtime_types;
                        #[derive(
                            :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                        )]
                        pub enum BalanceStatus {
                            #[codec(index = 0)]
                            Free,
                            #[codec(index = 1)]
                            Reserved,
                        }
                    }
                }
            }
            pub mod weights {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum DispatchClass {
                    #[codec(index = 0)]
                    Normal,
                    #[codec(index = 1)]
                    Operational,
                    #[codec(index = 2)]
                    Mandatory,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct DispatchInfo {
                    pub weight: ::core::primitive::u64,
                    pub class: runtime_types::frame_support::weights::DispatchClass,
                    pub pays_fee: runtime_types::frame_support::weights::Pays,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Pays {
                    #[codec(index = 0)]
                    Yes,
                    #[codec(index = 1)]
                    No,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct PerDispatchClass<_0> {
                    pub normal: _0,
                    pub operational: _0,
                    pub mandatory: _0,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct RuntimeDbWeight {
                    pub read: ::core::primitive::u64,
                    pub write: ::core::primitive::u64,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct WeightToFeeCoefficient<_0> {
                    pub coeff_integer: _0,
                    pub coeff_frac: runtime_types::sp_arithmetic::per_things::Perbill,
                    pub negative: ::core::primitive::bool,
                    pub degree: ::core::primitive::u8,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct PalletId(pub [::core::primitive::u8; 8usize]);
        }
        pub mod frame_system {
            use super::runtime_types;
            pub mod extensions {
                use super::runtime_types;
                pub mod check_genesis {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct CheckGenesis;
                }
                pub mod check_mortality {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
                }
                pub mod check_nonce {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
                }
                pub mod check_spec_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct CheckSpecVersion;
                }
                pub mod check_tx_version {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct CheckTxVersion;
                }
                pub mod check_weight {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct CheckWeight;
                }
            }
            pub mod limits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct BlockLength {
                    pub max: runtime_types::frame_support::weights::PerDispatchClass<
                        ::core::primitive::u32,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct BlockWeights {
                    pub base_block: ::core::primitive::u64,
                    pub max_block: ::core::primitive::u64,
                    pub per_class: runtime_types::frame_support::weights::PerDispatchClass<
                        runtime_types::frame_system::limits::WeightsPerClass,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct WeightsPerClass {
                    pub base_extrinsic: ::core::primitive::u64,
                    pub max_extrinsic: ::core::option::Option<::core::primitive::u64>,
                    pub max_total: ::core::option::Option<::core::primitive::u64>,
                    pub reserved: ::core::option::Option<::core::primitive::u64>,
                }
            }
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    fill_block {
                        ratio: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                    #[codec(index = 1)]
                    remark {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    set_heap_pages { pages: ::core::primitive::u64 },
                    #[codec(index = 3)]
                    set_code {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    set_code_without_checks {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 5)]
                    set_changes_trie_config {
                        changes_trie_config: ::core::option::Option<
                            runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                        >,
                    },
                    #[codec(index = 6)]
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 7)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 8)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    remark_with_event {
                        remark: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidSpecName,
                    #[codec(index = 1)]
                    SpecVersionNeedsToIncrease,
                    #[codec(index = 2)]
                    FailedToExtractRuntimeVersion,
                    #[codec(index = 3)]
                    NonDefaultComposite,
                    #[codec(index = 4)]
                    NonZeroRefCount,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess(runtime_types::frame_support::weights::DispatchInfo),
                    #[codec(index = 1)]
                    ExtrinsicFailed(
                        runtime_types::sp_runtime::DispatchError,
                        runtime_types::frame_support::weights::DispatchInfo,
                    ),
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    KilledAccount(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 5)]
                    Remarked(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::subxt::sp_core::H256,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct AccountInfo<_0, _1> {
                pub nonce: _0,
                pub consumers: _0,
                pub providers: _0,
                pub sufficients: _0,
                pub data: _1,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct EventRecord<_0, _1> {
                pub phase: runtime_types::frame_system::Phase,
                pub event: _0,
                pub topics: ::std::vec::Vec<_1>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct LastRuntimeUpgradeInfo {
                #[codec(compact)]
                pub spec_version: ::core::primitive::u32,
                pub spec_name: ::std::string::String,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Phase {
                #[codec(index = 0)]
                ApplyExtrinsic(::core::primitive::u32),
                #[codec(index = 1)]
                Finalization,
                #[codec(index = 2)]
                Initialization,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(_0),
                #[codec(index = 2)]
                None,
            }
        }
        pub mod module_transaction_pause {
            use super::runtime_types;
            pub mod module {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    pause_transaction {
                        pallet_name: ::std::vec::Vec<::core::primitive::u8>,
                        function_name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    unpause_transaction {
                        pallet_name: ::std::vec::Vec<::core::primitive::u8>,
                        function_name: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    CannotPause,
                    #[codec(index = 1)]
                    InvalidCharacter,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    TransactionPaused(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 1)]
                    TransactionUnpaused(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
            }
        }
        pub mod pallet_authorship {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_uncles {
                        new_uncles: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidUncleParent,
                    #[codec(index = 1)]
                    UnclesAlreadySet,
                    #[codec(index = 2)]
                    TooManyUncles,
                    #[codec(index = 3)]
                    GenesisUncle,
                    #[codec(index = 4)]
                    TooHighUncle,
                    #[codec(index = 5)]
                    UncleAlreadyIncluded,
                    #[codec(index = 6)]
                    OldUncle,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum UncleEntryItem<_0, _1, _2> {
                #[codec(index = 0)]
                InclusionHeight(_0),
                #[codec(index = 1)]
                Uncle(_1, ::core::option::Option<_2>),
            }
        }
        pub mod pallet_babe {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_consensus_slots::EquivocationProof<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                                runtime_types::sp_consensus_babe::app::Public,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_session::MembershipProof,
                    },
                    #[codec(index = 2)]
                    plan_config_change {
                        config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidEquivocationProof,
                    #[codec(index = 1)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 2)]
                    DuplicateOffenceReport,
                }
            }
        }
        pub mod pallet_base_fee {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_base_fee_per_gas {
                        fee: runtime_types::primitive_types::U256,
                    },
                    #[codec(index = 1)]
                    set_is_active { is_active: ::core::primitive::bool },
                    #[codec(index = 2)]
                    set_elasticity {
                        elasticity: runtime_types::sp_arithmetic::per_things::Permill,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewBaseFeePerGas(runtime_types::primitive_types::U256),
                    #[codec(index = 1)]
                    BaseFeeOverflow,
                    #[codec(index = 2)]
                    IsActive(::core::primitive::bool),
                    #[codec(index = 3)]
                    NewElasticity(runtime_types::sp_arithmetic::per_things::Permill),
                }
            }
        }
        pub mod pallet_bridge_dispatch {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    MessageRejected(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    ),
                    #[codec(index = 1)]
                    MessageVersionSpecMismatch(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 2)]
                    MessageWeightMismatch(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    MessageSignatureMismatch(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    ),
                    #[codec(index = 4)]
                    MessageCallDecodeFailed(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                    ),
                    #[codec(index = 5)]
                    MessageCallValidateFailed(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        runtime_types::sp_runtime::transaction_validity::TransactionValidityError,
                    ),
                    #[codec(index = 6)]
                    MessageDispatchPaymentFailed(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 7)]
                    MessageDispatched(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                    #[codec(index = 8)]
                    _Dummy,
                }
            }
        }
        pub mod pallet_bridge_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    submit_finality_proof {
                        finality_target: ::std::boxed::Box<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                        justification:
                            runtime_types::bp_header_chain::justification::GrandpaJustification<
                                runtime_types::sp_runtime::generic::header::Header<
                                    ::core::primitive::u32,
                                    runtime_types::sp_runtime::traits::BlakeTwo256,
                                >,
                            >,
                    },
                    #[codec(index = 1)]
                    initialize {
                        init_data: runtime_types::bp_header_chain::InitializationData<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                    #[codec(index = 2)]
                    set_owner {
                        new_owner: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 3)]
                    set_operational {
                        operational: ::core::primitive::bool,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidJustification,
                    #[codec(index = 1)]
                    InvalidAuthoritySet,
                    #[codec(index = 2)]
                    TooManyRequests,
                    #[codec(index = 3)]
                    OldHeader,
                    #[codec(index = 4)]
                    UnknownHeader,
                    #[codec(index = 5)]
                    UnsupportedScheduledChange,
                    #[codec(index = 6)]
                    NotInitialized,
                    #[codec(index = 7)]
                    AlreadyInitialized,
                    #[codec(index = 8)]
                    Halted,
                    #[codec(index = 9)]
                    StorageRootMismatch,
                }
            }
        }
        pub mod pallet_bridge_messages {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    # [codec (index = 0)] set_owner { new_owner : :: core :: option :: Option < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 1)] set_operating_mode { operating_mode : runtime_types :: bp_messages :: OperatingMode , } , # [codec (index = 2)] update_pallet_parameter { parameter : runtime_types :: pangoro_runtime :: bridges_message :: pangolin :: PangoroToPangolinMessagesParameter , } , # [codec (index = 3)] send_message { lane_id : [:: core :: primitive :: u8 ; 4usize] , payload : runtime_types :: bp_message_dispatch :: MessagePayload < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_runtime :: MultiSigner , runtime_types :: sp_runtime :: MultiSignature , :: std :: vec :: Vec < :: core :: primitive :: u8 > > , delivery_and_dispatch_fee : :: core :: primitive :: u128 , } , # [codec (index = 4)] increase_message_fee { lane_id : [:: core :: primitive :: u8 ; 4usize] , nonce : :: core :: primitive :: u64 , additional_fee : :: core :: primitive :: u128 , } , # [codec (index = 5)] receive_messages_proof { relayer_id_at_bridged_chain : :: subxt :: sp_core :: crypto :: AccountId32 , proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 > , messages_count : :: core :: primitive :: u32 , dispatch_weight : :: core :: primitive :: u64 , } , # [codec (index = 6)] receive_messages_delivery_proof { proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 > , relayers_state : runtime_types :: bp_messages :: UnrewardedRelayersState , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    Halted,
                    #[codec(index = 1)]
                    MessageRejectedByChainVerifier,
                    #[codec(index = 2)]
                    MessageRejectedByLaneVerifier,
                    #[codec(index = 3)]
                    FailedToWithdrawMessageFee,
                    #[codec(index = 4)]
                    TooManyMessagesInTheProof,
                    #[codec(index = 5)]
                    InvalidMessagesProof,
                    #[codec(index = 6)]
                    InvalidMessagesDeliveryProof,
                    #[codec(index = 7)]
                    InvalidUnrewardedRelayers,
                    #[codec(index = 8)]
                    InvalidUnrewardedRelayersState,
                    #[codec(index = 9)]
                    MessageIsAlreadyDelivered,
                    #[codec(index = 10)]
                    MessageIsNotYetSent,
                    #[codec(index = 11)]
                    TryingToConfirmMoreMessagesThanExpected,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    # [codec (index = 0)] ParameterUpdated (runtime_types :: pangoro_runtime :: bridges_message :: pangolin :: PangoroToPangolinMessagesParameter ,) , # [codec (index = 1)] MessageAccepted ([:: core :: primitive :: u8 ; 4usize] , :: core :: primitive :: u64 ,) , # [codec (index = 2)] MessagesDelivered ([:: core :: primitive :: u8 ; 4usize] , runtime_types :: bp_messages :: DeliveredMessages ,) , }
            }
        }
        pub mod pallet_election_provider_multi_phase {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    # [codec (index = 0)] submit_unsigned { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 > > , witness : runtime_types :: pallet_election_provider_multi_phase :: SolutionOrSnapshotSize , } , # [codec (index = 1)] set_minimum_untrusted_score { maybe_next_score : :: core :: option :: Option < [:: core :: primitive :: u128 ; 3usize] > , } , # [codec (index = 2)] set_emergency_election_result { supports : :: std :: vec :: Vec < (:: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_npos_elections :: Support < :: subxt :: sp_core :: crypto :: AccountId32 > ,) > , } , # [codec (index = 3)] submit { raw_solution : :: std :: boxed :: Box < runtime_types :: pallet_election_provider_multi_phase :: RawSolution < runtime_types :: pangoro_runtime :: pallets :: election_provider_multi_phase :: NposCompactSolution16 > > , num_signed_submissions : :: core :: primitive :: u32 , } , }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    PreDispatchEarlySubmission,
                    #[codec(index = 1)]
                    PreDispatchWrongWinnerCount,
                    #[codec(index = 2)]
                    PreDispatchWeakSubmission,
                    #[codec(index = 3)]
                    SignedQueueFull,
                    #[codec(index = 4)]
                    SignedCannotPayDeposit,
                    #[codec(index = 5)]
                    SignedInvalidWitness,
                    #[codec(index = 6)]
                    SignedTooMuchWeight,
                    #[codec(index = 7)]
                    OcwCallWrongEra,
                    #[codec(index = 8)]
                    MissingSnapshotMetadata,
                    #[codec(index = 9)]
                    InvalidSubmissionIndex,
                    #[codec(index = 10)]
                    CallNotAllowed,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    SolutionStored(
                        runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                        ::core::primitive::bool,
                    ),
                    #[codec(index = 1)]
                    ElectionFinalized(
                        ::core::option::Option<
                            runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                        >,
                    ),
                    #[codec(index = 2)]
                    Rewarded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    Slashed(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    SignedPhaseStarted(::core::primitive::u32),
                    #[codec(index = 5)]
                    UnsignedPhaseStarted(::core::primitive::u32),
                }
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct SignedSubmission<_0, _1, _2> {
                    pub who: _0,
                    pub deposit: _1,
                    pub raw_solution:
                        runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
                    pub reward: _1,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum ElectionCompute {
                #[codec(index = 0)]
                OnChain,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned,
                #[codec(index = 3)]
                Fallback,
                #[codec(index = 4)]
                Emergency,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Phase<_0> {
                #[codec(index = 0)]
                Off,
                #[codec(index = 1)]
                Signed,
                #[codec(index = 2)]
                Unsigned((::core::primitive::bool, _0)),
                #[codec(index = 3)]
                Emergency,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct RawSolution<_0> {
                pub solution: _0,
                pub score: [::core::primitive::u128; 3usize],
                pub round: ::core::primitive::u32,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ReadySolution<_0> {
                pub supports: ::std::vec::Vec<(_0, runtime_types::sp_npos_elections::Support<_0>)>,
                pub score: [::core::primitive::u128; 3usize],
                pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct RoundSnapshot<_0> {
                pub voters: ::std::vec::Vec<(_0, ::core::primitive::u64, ::std::vec::Vec<_0>)>,
                pub targets: ::std::vec::Vec<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct SolutionOrSnapshotSize {
                #[codec(compact)]
                pub voters: ::core::primitive::u32,
                #[codec(compact)]
                pub targets: ::core::primitive::u32,
            }
        }
        pub mod pallet_fee_market {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    enroll_and_lock_collateral {
                        lock_collateral: ::core::primitive::u128,
                        relay_fee: ::core::option::Option<::core::primitive::u128>,
                    },
                    #[codec(index = 1)]
                    update_locked_collateral {
                        new_collateral: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    update_relay_fee { new_fee: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    cancel_enrollment,
                    #[codec(index = 4)]
                    set_slash_protect {
                        slash_protect: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    set_assigned_relayers_number { number: ::core::primitive::u32 },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InsufficientBalance,
                    #[codec(index = 1)]
                    AlreadyEnrolled,
                    #[codec(index = 2)]
                    NotEnrolled,
                    #[codec(index = 3)]
                    CollateralTooLow,
                    #[codec(index = 4)]
                    StillHasOrdersNotConfirmed,
                    #[codec(index = 5)]
                    RelayFeeTooLow,
                    #[codec(index = 6)]
                    OccupiedRelayer,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Enroll(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 1)]
                    UpdateLockedCollateral(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    UpdateRelayFee(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    CancelEnrollment(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 4)]
                    UpdateCollateralSlashProtect(::core::primitive::u128),
                    #[codec(index = 5)]
                    UpdateAssignedRelayersNumber(::core::primitive::u32),
                    #[codec(index = 6)]
                    FeeMarketSlash(
                        runtime_types::pallet_fee_market::types::SlashReport<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                        >,
                    ),
                    #[codec(index = 7)]
                    OrderCreated(
                        [::core::primitive::u8; 4usize],
                        ::core::primitive::u64,
                        ::core::primitive::u128,
                        ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        ::core::option::Option<::core::primitive::u32>,
                    ),
                    #[codec(index = 8)]
                    OrderReward(
                        [::core::primitive::u8; 4usize],
                        ::core::primitive::u64,
                        runtime_types::pallet_fee_market::s2s::payment::RewardItem<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    ),
                }
            }
            pub mod s2s {
                use super::runtime_types;
                pub mod payment {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct RewardItem<_0, _1> {
                        pub to_slot_relayer: ::core::option::Option<(_0, _1)>,
                        pub to_treasury: ::core::option::Option<_1>,
                        pub to_message_relayer: ::core::option::Option<(_0, _1)>,
                        pub to_confirm_relayer: ::core::option::Option<(_0, _1)>,
                    }
                }
            }
            pub mod types {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Order<_0, _1, _2> {
                    pub lane: [::core::primitive::u8; 4usize],
                    pub message: ::core::primitive::u64,
                    pub sent_time: _1,
                    pub confirm_time: ::core::option::Option<_1>,
                    pub locked_collateral: _2,
                    pub relayers: ::std::vec::Vec<
                        runtime_types::pallet_fee_market::types::PriorRelayer<_0, _1, _2>,
                    >,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct PriorRelayer<_0, _1, _2> {
                    pub id: _0,
                    pub fee: _2,
                    pub valid_range: ::core::ops::Range<_1>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Relayer<_0, _1> {
                    pub id: _0,
                    pub collateral: _1,
                    pub fee: _1,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct SlashReport<_0, _1, _2> {
                    pub lane: [::core::primitive::u8; 4usize],
                    pub message: ::core::primitive::u64,
                    pub sent_time: _1,
                    pub confirm_time: ::core::option::Option<_1>,
                    pub delay_time: ::core::option::Option<_1>,
                    pub account_id: _0,
                    pub amount: _2,
                }
            }
        }
        pub mod pallet_grandpa {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    report_equivocation {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 1)]
                    report_equivocation_unsigned {
                        equivocation_proof: ::std::boxed::Box<
                            runtime_types::sp_finality_grandpa::EquivocationProof<
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        key_owner_proof: runtime_types::sp_core::Void,
                    },
                    #[codec(index = 2)]
                    note_stalled {
                        delay: ::core::primitive::u32,
                        best_finalized_block_number: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    PauseFailed,
                    #[codec(index = 1)]
                    ResumeFailed,
                    #[codec(index = 2)]
                    ChangePending,
                    #[codec(index = 3)]
                    TooSoon,
                    #[codec(index = 4)]
                    InvalidKeyOwnershipProof,
                    #[codec(index = 5)]
                    InvalidEquivocationProof,
                    #[codec(index = 6)]
                    DuplicateOffenceReport,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewAuthorities(
                        ::std::vec::Vec<(
                            runtime_types::sp_finality_grandpa::app::Public,
                            ::core::primitive::u64,
                        )>,
                    ),
                    #[codec(index = 1)]
                    Paused,
                    #[codec(index = 2)]
                    Resumed,
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct StoredPendingChange<_0> {
                pub scheduled_at: _0,
                pub delay: _0,
                pub next_authorities:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<(
                        runtime_types::sp_finality_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                pub forced: ::core::option::Option<_0>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum StoredState<_0> {
                #[codec(index = 0)]
                Live,
                #[codec(index = 1)]
                PendingPause { scheduled_at: _0, delay: _0 },
                #[codec(index = 2)]
                Paused,
                #[codec(index = 3)]
                PendingResume { scheduled_at: _0, delay: _0 },
            }
        }
        pub mod pallet_im_online {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    heartbeat {
                        heartbeat:
                            runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                        signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidKey,
                    #[codec(index = 1)]
                    DuplicatedHeartbeat,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    HeartbeatReceived(
                        runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                    ),
                    #[codec(index = 1)]
                    AllGood,
                    #[codec(index = 2)]
                    SomeOffline(
                        ::std::vec::Vec<(
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::darwinia_staking::structs::Exposure<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::core::primitive::u128,
                                ::core::primitive::u128,
                            >,
                        )>,
                    ),
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BoundedOpaqueNetworkState {
                pub peer_id:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                pub external_addresses:
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    >,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Heartbeat<_0> {
                pub block_number: _0,
                pub network_state: runtime_types::sp_core::offchain::OpaqueNetworkState,
                pub session_index: _0,
                pub authority_index: _0,
                pub validators_len: _0,
            }
        }
        pub mod pallet_offences {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Offence(
                        [::core::primitive::u8; 16usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                }
            }
        }
        pub mod pallet_scheduler {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    schedule {
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    cancel {
                        when: ::core::primitive::u32,
                        index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    schedule_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        when: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                    },
                    #[codec(index = 3)]
                    cancel_named {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    schedule_after {
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                    },
                    #[codec(index = 5)]
                    schedule_named_after {
                        id: ::std::vec::Vec<::core::primitive::u8>,
                        after: ::core::primitive::u32,
                        maybe_periodic: ::core::option::Option<(
                            ::core::primitive::u32,
                            ::core::primitive::u32,
                        )>,
                        priority: ::core::primitive::u8,
                        call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    FailedToSchedule,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    TargetBlockNumberInPast,
                    #[codec(index = 3)]
                    RescheduleNoChange,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Scheduled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 1)]
                    Canceled(::core::primitive::u32, ::core::primitive::u32),
                    #[codec(index = 2)]
                    Dispatched(
                        (::core::primitive::u32, ::core::primitive::u32),
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Releases {
                #[codec(index = 0)]
                V1,
                #[codec(index = 1)]
                V2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ScheduledV2<_0, _1, _2, _3> {
                pub maybe_id: ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                pub priority: ::core::primitive::u8,
                pub call: _0,
                pub maybe_periodic: ::core::option::Option<(_1, _1)>,
                pub origin: _2,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_3>,
            }
        }
        pub mod pallet_session {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_keys {
                        keys: runtime_types::pangoro_runtime::pallets::session::SessionKeys,
                        proof: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 1)]
                    purge_keys,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InvalidProof,
                    #[codec(index = 1)]
                    NoAssociatedValidatorId,
                    #[codec(index = 2)]
                    DuplicatedKey,
                    #[codec(index = 3)]
                    NoKeys,
                    #[codec(index = 4)]
                    NoAccount,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewSession(::core::primitive::u32),
                }
            }
        }
        pub mod pallet_sudo {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    sudo {
                        call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                        weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    set_key {
                        new: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 3)]
                    sudo_as {
                        who: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                        call: ::std::boxed::Box<runtime_types::pangoro_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    RequireSudo,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Sudid(::core::result::Result<(), runtime_types::sp_runtime::DispatchError>),
                    #[codec(index = 1)]
                    KeyChanged(::subxt::sp_core::crypto::AccountId32),
                    #[codec(index = 2)]
                    SudoAsDone(
                        ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    ),
                }
            }
        }
        pub mod pallet_timestamp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set {
                        #[codec(compact)]
                        now: ::core::primitive::u64,
                    },
                }
            }
        }
        pub mod pallet_transaction_payment {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Releases {
                #[codec(index = 0)]
                V1Ancient,
                #[codec(index = 1)]
                V2,
            }
        }
        pub mod pallet_treasury {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    propose_spend {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        beneficiary: ::subxt::sp_runtime::MultiAddress<
                            ::subxt::sp_core::crypto::AccountId32,
                            (),
                        >,
                    },
                    #[codec(index = 1)]
                    reject_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    approve_proposal {
                        #[codec(compact)]
                        proposal_id: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InsufficientProposersBalance,
                    #[codec(index = 1)]
                    InvalidIndex,
                    #[codec(index = 2)]
                    TooManyApprovals,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Proposed(::core::primitive::u32),
                    #[codec(index = 1)]
                    Spending(::core::primitive::u128),
                    #[codec(index = 2)]
                    Awarded(
                        ::core::primitive::u32,
                        ::core::primitive::u128,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 3)]
                    Rejected(::core::primitive::u32, ::core::primitive::u128),
                    #[codec(index = 4)]
                    Burnt(::core::primitive::u128),
                    #[codec(index = 5)]
                    Rollover(::core::primitive::u128),
                    #[codec(index = 6)]
                    Deposit(::core::primitive::u128),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Proposal<_0, _1> {
                pub proposer: _0,
                pub value: _1,
                pub beneficiary: _0,
                pub bond: _1,
            }
        }
        pub mod pangoro_runtime {
            use super::runtime_types;
            pub mod bridges_message {
                use super::runtime_types;
                pub mod pangolin {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum PangoroToPangolinMessagesParameter {
                        #[codec(index = 0)]
                        PangolinToPangoroConversionRate(
                            runtime_types::sp_arithmetic::fixed_point::FixedU128,
                        ),
                    }
                }
            }
            pub mod pallets {
                use super::runtime_types;
                pub mod election_provider_multi_phase {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct NposCompactSolution16 {
                        pub votes1:
                            ::std::vec::Vec<(::core::primitive::u32, ::core::primitive::u16)>,
                        pub votes2: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            (
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ),
                            ::core::primitive::u16,
                        )>,
                        pub votes3: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 2usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes4: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 3usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes5: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 4usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes6: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 5usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes7: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 6usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes8: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 7usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes9: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 8usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes10: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 9usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes11: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 10usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes12: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 11usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes13: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 12usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes14: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 13usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes15: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 14usize],
                            ::core::primitive::u16,
                        )>,
                        pub votes16: ::std::vec::Vec<(
                            ::core::primitive::u32,
                            [(
                                ::core::primitive::u16,
                                runtime_types::sp_arithmetic::per_things::PerU16,
                            ); 15usize],
                            ::core::primitive::u16,
                        )>,
                    }
                }
                pub mod session {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct SessionKeys {
                        pub babe: runtime_types::sp_consensus_babe::app::Public,
                        pub grandpa: runtime_types::sp_finality_grandpa::app::Public,
                        pub beefy: runtime_types::beefy_primitives::crypto::Public,
                        pub im_online:
                            runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                        pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
                    }
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 2)]
                Babe(runtime_types::pallet_babe::pallet::Call),
                #[codec(index = 3)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 4)]
                Balances(runtime_types::darwinia_balances::pallet::Call),
                #[codec(index = 5)]
                Kton(runtime_types::darwinia_balances::pallet::Call),
                #[codec(index = 7)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 8)]
                ElectionProviderMultiPhase(
                    runtime_types::pallet_election_provider_multi_phase::pallet::Call,
                ),
                #[codec(index = 9)]
                Staking(runtime_types::darwinia_staking::pallet::Call),
                #[codec(index = 12)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 13)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Call),
                #[codec(index = 30)]
                MessageGadget(runtime_types::darwinia_message_gadget::pallet::Call),
                #[codec(index = 32)]
                EcdsaAuthority(runtime_types::darwinia_ecdsa_authority::pallet::Call),
                #[codec(index = 14)]
                ImOnline(runtime_types::pallet_im_online::pallet::Call),
                #[codec(index = 24)]
                Treasury(runtime_types::pallet_treasury::pallet::Call),
                #[codec(index = 16)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 21)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Call),
                #[codec(index = 19)]
                BridgePangolinGrandpa(runtime_types::pallet_bridge_grandpa::pallet::Call),
                #[codec(index = 17)]
                BridgePangolinMessages(runtime_types::pallet_bridge_messages::pallet::Call),
                #[codec(index = 22)]
                PangolinFeeMarket(runtime_types::pallet_fee_market::pallet::Call),
                #[codec(index = 23)]
                TransactionPause(runtime_types::module_transaction_pause::module::Call),
                #[codec(index = 20)]
                Substrate2SubstrateBacking(runtime_types::to_substrate_backing::pallet::Call),
                #[codec(index = 25)]
                EVM(runtime_types::darwinia_evm::pallet::Call),
                #[codec(index = 26)]
                Ethereum(runtime_types::darwinia_ethereum::pallet::Call),
                #[codec(index = 31)]
                BaseFee(runtime_types::pallet_base_fee::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 4)]
                Balances(runtime_types::darwinia_balances::pallet::Event),
                #[codec(index = 5)]
                Kton(runtime_types::darwinia_balances::pallet::Event),
                #[codec(index = 8)]
                ElectionProviderMultiPhase(
                    runtime_types::pallet_election_provider_multi_phase::pallet::Event,
                ),
                #[codec(index = 9)]
                Staking(runtime_types::darwinia_staking::pallet::Event),
                #[codec(index = 10)]
                Offences(runtime_types::pallet_offences::pallet::Event),
                #[codec(index = 12)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 13)]
                Grandpa(runtime_types::pallet_grandpa::pallet::Event),
                #[codec(index = 32)]
                EcdsaAuthority(runtime_types::darwinia_ecdsa_authority::pallet::Event),
                #[codec(index = 14)]
                ImOnline(runtime_types::pallet_im_online::pallet::Event),
                #[codec(index = 24)]
                Treasury(runtime_types::pallet_treasury::pallet::Event),
                #[codec(index = 16)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 21)]
                Scheduler(runtime_types::pallet_scheduler::pallet::Event),
                #[codec(index = 18)]
                BridgePangolinDispatch(runtime_types::pallet_bridge_dispatch::pallet::Event),
                #[codec(index = 17)]
                BridgePangolinMessages(runtime_types::pallet_bridge_messages::pallet::Event),
                #[codec(index = 22)]
                PangolinFeeMarket(runtime_types::pallet_fee_market::pallet::Event),
                #[codec(index = 23)]
                TransactionPause(runtime_types::module_transaction_pause::module::Event),
                #[codec(index = 20)]
                Substrate2SubstrateBacking(runtime_types::to_substrate_backing::pallet::Event),
                #[codec(index = 25)]
                EVM(runtime_types::darwinia_evm::pallet::Event),
                #[codec(index = 26)]
                Ethereum(runtime_types::darwinia_ethereum::pallet::Event),
                #[codec(index = 31)]
                BaseFee(runtime_types::pallet_base_fee::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_system::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
                ),
                #[codec(index = 26)]
                Ethereum(runtime_types::darwinia_ethereum::RawOrigin),
                #[codec(index = 2)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Runtime;
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct H160(pub [::core::primitive::u8; 20usize]);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct U256(pub [::core::primitive::u64; 4usize]);
        }
        pub mod sp_arithmetic {
            use super::runtime_types;
            pub mod fixed_point {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct FixedU128(pub ::core::primitive::u128);
            }
            pub mod per_things {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct PerU16(pub ::core::primitive::u16);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Perbill(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Percent(pub ::core::primitive::u8);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Permill(pub ::core::primitive::u32);
            }
        }
        pub mod sp_authority_discovery {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
        }
        pub mod sp_consensus_babe {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod digests {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum NextConfigDescriptor {
                    #[codec(index = 1)]
                    V1 {
                        c: (::core::primitive::u64, ::core::primitive::u64),
                        allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum AllowedSlots {
                #[codec(index = 0)]
                PrimarySlots,
                #[codec(index = 1)]
                PrimaryAndSecondaryPlainSlots,
                #[codec(index = 2)]
                PrimaryAndSecondaryVRFSlots,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BabeEpochConfiguration {
                pub c: (::core::primitive::u64, ::core::primitive::u64),
                pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct EquivocationProof<_0, _1> {
                pub offender: _1,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub first_header: _0,
                pub second_header: _0,
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct Slot(pub ::core::primitive::u64);
        }
        pub mod sp_core {
            use super::runtime_types;
            pub mod changes_trie {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct ChangesTrieConfiguration {
                    pub digest_interval: ::core::primitive::u32,
                    pub digest_levels: ::core::primitive::u32,
                }
            }
            pub mod crypto {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
            }
            pub mod ecdsa {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Public(pub [::core::primitive::u8; 33usize]);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Signature(pub [::core::primitive::u8; 65usize]);
            }
            pub mod ed25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            pub mod offchain {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct OpaqueMultiaddr(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct OpaqueNetworkState {
                    pub peer_id: runtime_types::sp_core::OpaquePeerId,
                    pub external_addresses:
                        ::std::vec::Vec<runtime_types::sp_core::offchain::OpaqueMultiaddr>,
                }
            }
            pub mod sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Public(pub [::core::primitive::u8; 32usize]);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Signature(pub [::core::primitive::u8; 64usize]);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OpaquePeerId(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Void {}
        }
        pub mod sp_finality_grandpa {
            use super::runtime_types;
            pub mod app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Public(pub runtime_types::sp_core::ed25519::Public);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Equivocation<_0, _1> {
                #[codec(index = 0)]
                Prevote(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Prevote<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
                #[codec(index = 1)]
                Precommit(
                    runtime_types::finality_grandpa::Equivocation<
                        runtime_types::sp_finality_grandpa::app::Public,
                        runtime_types::finality_grandpa::Precommit<_0, _1>,
                        runtime_types::sp_finality_grandpa::app::Signature,
                    >,
                ),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct EquivocationProof<_0, _1> {
                pub set_id: ::core::primitive::u64,
                pub equivocation: runtime_types::sp_finality_grandpa::Equivocation<_0, _1>,
            }
        }
        pub mod sp_npos_elections {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Support<_0> {
                pub total: ::core::primitive::u128,
                pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
            }
        }
        pub mod sp_runtime {
            use super::runtime_types;
            pub mod generic {
                use super::runtime_types;
                pub mod digest {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum ChangesTrieSignal {
                        #[codec(index = 0)]
                        NewConfiguration(
                            ::core::option::Option<
                                runtime_types::sp_core::changes_trie::ChangesTrieConfiguration,
                            >,
                        ),
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct Digest<_0> {
                        pub logs: ::std::vec::Vec<
                            runtime_types::sp_runtime::generic::digest::DigestItem<_0>,
                        >,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum DigestItem<_0> {
                        #[codec(index = 2)]
                        ChangesTrieRoot(_0),
                        #[codec(index = 6)]
                        PreRuntime(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 4)]
                        Consensus(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 5)]
                        Seal(
                            [::core::primitive::u8; 4usize],
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                        #[codec(index = 7)]
                        ChangesTrieSignal(
                            runtime_types::sp_runtime::generic::digest::ChangesTrieSignal,
                        ),
                        #[codec(index = 0)]
                        Other(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        RuntimeEnvironmentUpdated,
                    }
                }
                pub mod era {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Era {
                        #[codec(index = 0)]
                        Immortal,
                        #[codec(index = 1)]
                        Mortal1(::core::primitive::u8),
                        #[codec(index = 2)]
                        Mortal2(::core::primitive::u8),
                        #[codec(index = 3)]
                        Mortal3(::core::primitive::u8),
                        #[codec(index = 4)]
                        Mortal4(::core::primitive::u8),
                        #[codec(index = 5)]
                        Mortal5(::core::primitive::u8),
                        #[codec(index = 6)]
                        Mortal6(::core::primitive::u8),
                        #[codec(index = 7)]
                        Mortal7(::core::primitive::u8),
                        #[codec(index = 8)]
                        Mortal8(::core::primitive::u8),
                        #[codec(index = 9)]
                        Mortal9(::core::primitive::u8),
                        #[codec(index = 10)]
                        Mortal10(::core::primitive::u8),
                        #[codec(index = 11)]
                        Mortal11(::core::primitive::u8),
                        #[codec(index = 12)]
                        Mortal12(::core::primitive::u8),
                        #[codec(index = 13)]
                        Mortal13(::core::primitive::u8),
                        #[codec(index = 14)]
                        Mortal14(::core::primitive::u8),
                        #[codec(index = 15)]
                        Mortal15(::core::primitive::u8),
                        #[codec(index = 16)]
                        Mortal16(::core::primitive::u8),
                        #[codec(index = 17)]
                        Mortal17(::core::primitive::u8),
                        #[codec(index = 18)]
                        Mortal18(::core::primitive::u8),
                        #[codec(index = 19)]
                        Mortal19(::core::primitive::u8),
                        #[codec(index = 20)]
                        Mortal20(::core::primitive::u8),
                        #[codec(index = 21)]
                        Mortal21(::core::primitive::u8),
                        #[codec(index = 22)]
                        Mortal22(::core::primitive::u8),
                        #[codec(index = 23)]
                        Mortal23(::core::primitive::u8),
                        #[codec(index = 24)]
                        Mortal24(::core::primitive::u8),
                        #[codec(index = 25)]
                        Mortal25(::core::primitive::u8),
                        #[codec(index = 26)]
                        Mortal26(::core::primitive::u8),
                        #[codec(index = 27)]
                        Mortal27(::core::primitive::u8),
                        #[codec(index = 28)]
                        Mortal28(::core::primitive::u8),
                        #[codec(index = 29)]
                        Mortal29(::core::primitive::u8),
                        #[codec(index = 30)]
                        Mortal30(::core::primitive::u8),
                        #[codec(index = 31)]
                        Mortal31(::core::primitive::u8),
                        #[codec(index = 32)]
                        Mortal32(::core::primitive::u8),
                        #[codec(index = 33)]
                        Mortal33(::core::primitive::u8),
                        #[codec(index = 34)]
                        Mortal34(::core::primitive::u8),
                        #[codec(index = 35)]
                        Mortal35(::core::primitive::u8),
                        #[codec(index = 36)]
                        Mortal36(::core::primitive::u8),
                        #[codec(index = 37)]
                        Mortal37(::core::primitive::u8),
                        #[codec(index = 38)]
                        Mortal38(::core::primitive::u8),
                        #[codec(index = 39)]
                        Mortal39(::core::primitive::u8),
                        #[codec(index = 40)]
                        Mortal40(::core::primitive::u8),
                        #[codec(index = 41)]
                        Mortal41(::core::primitive::u8),
                        #[codec(index = 42)]
                        Mortal42(::core::primitive::u8),
                        #[codec(index = 43)]
                        Mortal43(::core::primitive::u8),
                        #[codec(index = 44)]
                        Mortal44(::core::primitive::u8),
                        #[codec(index = 45)]
                        Mortal45(::core::primitive::u8),
                        #[codec(index = 46)]
                        Mortal46(::core::primitive::u8),
                        #[codec(index = 47)]
                        Mortal47(::core::primitive::u8),
                        #[codec(index = 48)]
                        Mortal48(::core::primitive::u8),
                        #[codec(index = 49)]
                        Mortal49(::core::primitive::u8),
                        #[codec(index = 50)]
                        Mortal50(::core::primitive::u8),
                        #[codec(index = 51)]
                        Mortal51(::core::primitive::u8),
                        #[codec(index = 52)]
                        Mortal52(::core::primitive::u8),
                        #[codec(index = 53)]
                        Mortal53(::core::primitive::u8),
                        #[codec(index = 54)]
                        Mortal54(::core::primitive::u8),
                        #[codec(index = 55)]
                        Mortal55(::core::primitive::u8),
                        #[codec(index = 56)]
                        Mortal56(::core::primitive::u8),
                        #[codec(index = 57)]
                        Mortal57(::core::primitive::u8),
                        #[codec(index = 58)]
                        Mortal58(::core::primitive::u8),
                        #[codec(index = 59)]
                        Mortal59(::core::primitive::u8),
                        #[codec(index = 60)]
                        Mortal60(::core::primitive::u8),
                        #[codec(index = 61)]
                        Mortal61(::core::primitive::u8),
                        #[codec(index = 62)]
                        Mortal62(::core::primitive::u8),
                        #[codec(index = 63)]
                        Mortal63(::core::primitive::u8),
                        #[codec(index = 64)]
                        Mortal64(::core::primitive::u8),
                        #[codec(index = 65)]
                        Mortal65(::core::primitive::u8),
                        #[codec(index = 66)]
                        Mortal66(::core::primitive::u8),
                        #[codec(index = 67)]
                        Mortal67(::core::primitive::u8),
                        #[codec(index = 68)]
                        Mortal68(::core::primitive::u8),
                        #[codec(index = 69)]
                        Mortal69(::core::primitive::u8),
                        #[codec(index = 70)]
                        Mortal70(::core::primitive::u8),
                        #[codec(index = 71)]
                        Mortal71(::core::primitive::u8),
                        #[codec(index = 72)]
                        Mortal72(::core::primitive::u8),
                        #[codec(index = 73)]
                        Mortal73(::core::primitive::u8),
                        #[codec(index = 74)]
                        Mortal74(::core::primitive::u8),
                        #[codec(index = 75)]
                        Mortal75(::core::primitive::u8),
                        #[codec(index = 76)]
                        Mortal76(::core::primitive::u8),
                        #[codec(index = 77)]
                        Mortal77(::core::primitive::u8),
                        #[codec(index = 78)]
                        Mortal78(::core::primitive::u8),
                        #[codec(index = 79)]
                        Mortal79(::core::primitive::u8),
                        #[codec(index = 80)]
                        Mortal80(::core::primitive::u8),
                        #[codec(index = 81)]
                        Mortal81(::core::primitive::u8),
                        #[codec(index = 82)]
                        Mortal82(::core::primitive::u8),
                        #[codec(index = 83)]
                        Mortal83(::core::primitive::u8),
                        #[codec(index = 84)]
                        Mortal84(::core::primitive::u8),
                        #[codec(index = 85)]
                        Mortal85(::core::primitive::u8),
                        #[codec(index = 86)]
                        Mortal86(::core::primitive::u8),
                        #[codec(index = 87)]
                        Mortal87(::core::primitive::u8),
                        #[codec(index = 88)]
                        Mortal88(::core::primitive::u8),
                        #[codec(index = 89)]
                        Mortal89(::core::primitive::u8),
                        #[codec(index = 90)]
                        Mortal90(::core::primitive::u8),
                        #[codec(index = 91)]
                        Mortal91(::core::primitive::u8),
                        #[codec(index = 92)]
                        Mortal92(::core::primitive::u8),
                        #[codec(index = 93)]
                        Mortal93(::core::primitive::u8),
                        #[codec(index = 94)]
                        Mortal94(::core::primitive::u8),
                        #[codec(index = 95)]
                        Mortal95(::core::primitive::u8),
                        #[codec(index = 96)]
                        Mortal96(::core::primitive::u8),
                        #[codec(index = 97)]
                        Mortal97(::core::primitive::u8),
                        #[codec(index = 98)]
                        Mortal98(::core::primitive::u8),
                        #[codec(index = 99)]
                        Mortal99(::core::primitive::u8),
                        #[codec(index = 100)]
                        Mortal100(::core::primitive::u8),
                        #[codec(index = 101)]
                        Mortal101(::core::primitive::u8),
                        #[codec(index = 102)]
                        Mortal102(::core::primitive::u8),
                        #[codec(index = 103)]
                        Mortal103(::core::primitive::u8),
                        #[codec(index = 104)]
                        Mortal104(::core::primitive::u8),
                        #[codec(index = 105)]
                        Mortal105(::core::primitive::u8),
                        #[codec(index = 106)]
                        Mortal106(::core::primitive::u8),
                        #[codec(index = 107)]
                        Mortal107(::core::primitive::u8),
                        #[codec(index = 108)]
                        Mortal108(::core::primitive::u8),
                        #[codec(index = 109)]
                        Mortal109(::core::primitive::u8),
                        #[codec(index = 110)]
                        Mortal110(::core::primitive::u8),
                        #[codec(index = 111)]
                        Mortal111(::core::primitive::u8),
                        #[codec(index = 112)]
                        Mortal112(::core::primitive::u8),
                        #[codec(index = 113)]
                        Mortal113(::core::primitive::u8),
                        #[codec(index = 114)]
                        Mortal114(::core::primitive::u8),
                        #[codec(index = 115)]
                        Mortal115(::core::primitive::u8),
                        #[codec(index = 116)]
                        Mortal116(::core::primitive::u8),
                        #[codec(index = 117)]
                        Mortal117(::core::primitive::u8),
                        #[codec(index = 118)]
                        Mortal118(::core::primitive::u8),
                        #[codec(index = 119)]
                        Mortal119(::core::primitive::u8),
                        #[codec(index = 120)]
                        Mortal120(::core::primitive::u8),
                        #[codec(index = 121)]
                        Mortal121(::core::primitive::u8),
                        #[codec(index = 122)]
                        Mortal122(::core::primitive::u8),
                        #[codec(index = 123)]
                        Mortal123(::core::primitive::u8),
                        #[codec(index = 124)]
                        Mortal124(::core::primitive::u8),
                        #[codec(index = 125)]
                        Mortal125(::core::primitive::u8),
                        #[codec(index = 126)]
                        Mortal126(::core::primitive::u8),
                        #[codec(index = 127)]
                        Mortal127(::core::primitive::u8),
                        #[codec(index = 128)]
                        Mortal128(::core::primitive::u8),
                        #[codec(index = 129)]
                        Mortal129(::core::primitive::u8),
                        #[codec(index = 130)]
                        Mortal130(::core::primitive::u8),
                        #[codec(index = 131)]
                        Mortal131(::core::primitive::u8),
                        #[codec(index = 132)]
                        Mortal132(::core::primitive::u8),
                        #[codec(index = 133)]
                        Mortal133(::core::primitive::u8),
                        #[codec(index = 134)]
                        Mortal134(::core::primitive::u8),
                        #[codec(index = 135)]
                        Mortal135(::core::primitive::u8),
                        #[codec(index = 136)]
                        Mortal136(::core::primitive::u8),
                        #[codec(index = 137)]
                        Mortal137(::core::primitive::u8),
                        #[codec(index = 138)]
                        Mortal138(::core::primitive::u8),
                        #[codec(index = 139)]
                        Mortal139(::core::primitive::u8),
                        #[codec(index = 140)]
                        Mortal140(::core::primitive::u8),
                        #[codec(index = 141)]
                        Mortal141(::core::primitive::u8),
                        #[codec(index = 142)]
                        Mortal142(::core::primitive::u8),
                        #[codec(index = 143)]
                        Mortal143(::core::primitive::u8),
                        #[codec(index = 144)]
                        Mortal144(::core::primitive::u8),
                        #[codec(index = 145)]
                        Mortal145(::core::primitive::u8),
                        #[codec(index = 146)]
                        Mortal146(::core::primitive::u8),
                        #[codec(index = 147)]
                        Mortal147(::core::primitive::u8),
                        #[codec(index = 148)]
                        Mortal148(::core::primitive::u8),
                        #[codec(index = 149)]
                        Mortal149(::core::primitive::u8),
                        #[codec(index = 150)]
                        Mortal150(::core::primitive::u8),
                        #[codec(index = 151)]
                        Mortal151(::core::primitive::u8),
                        #[codec(index = 152)]
                        Mortal152(::core::primitive::u8),
                        #[codec(index = 153)]
                        Mortal153(::core::primitive::u8),
                        #[codec(index = 154)]
                        Mortal154(::core::primitive::u8),
                        #[codec(index = 155)]
                        Mortal155(::core::primitive::u8),
                        #[codec(index = 156)]
                        Mortal156(::core::primitive::u8),
                        #[codec(index = 157)]
                        Mortal157(::core::primitive::u8),
                        #[codec(index = 158)]
                        Mortal158(::core::primitive::u8),
                        #[codec(index = 159)]
                        Mortal159(::core::primitive::u8),
                        #[codec(index = 160)]
                        Mortal160(::core::primitive::u8),
                        #[codec(index = 161)]
                        Mortal161(::core::primitive::u8),
                        #[codec(index = 162)]
                        Mortal162(::core::primitive::u8),
                        #[codec(index = 163)]
                        Mortal163(::core::primitive::u8),
                        #[codec(index = 164)]
                        Mortal164(::core::primitive::u8),
                        #[codec(index = 165)]
                        Mortal165(::core::primitive::u8),
                        #[codec(index = 166)]
                        Mortal166(::core::primitive::u8),
                        #[codec(index = 167)]
                        Mortal167(::core::primitive::u8),
                        #[codec(index = 168)]
                        Mortal168(::core::primitive::u8),
                        #[codec(index = 169)]
                        Mortal169(::core::primitive::u8),
                        #[codec(index = 170)]
                        Mortal170(::core::primitive::u8),
                        #[codec(index = 171)]
                        Mortal171(::core::primitive::u8),
                        #[codec(index = 172)]
                        Mortal172(::core::primitive::u8),
                        #[codec(index = 173)]
                        Mortal173(::core::primitive::u8),
                        #[codec(index = 174)]
                        Mortal174(::core::primitive::u8),
                        #[codec(index = 175)]
                        Mortal175(::core::primitive::u8),
                        #[codec(index = 176)]
                        Mortal176(::core::primitive::u8),
                        #[codec(index = 177)]
                        Mortal177(::core::primitive::u8),
                        #[codec(index = 178)]
                        Mortal178(::core::primitive::u8),
                        #[codec(index = 179)]
                        Mortal179(::core::primitive::u8),
                        #[codec(index = 180)]
                        Mortal180(::core::primitive::u8),
                        #[codec(index = 181)]
                        Mortal181(::core::primitive::u8),
                        #[codec(index = 182)]
                        Mortal182(::core::primitive::u8),
                        #[codec(index = 183)]
                        Mortal183(::core::primitive::u8),
                        #[codec(index = 184)]
                        Mortal184(::core::primitive::u8),
                        #[codec(index = 185)]
                        Mortal185(::core::primitive::u8),
                        #[codec(index = 186)]
                        Mortal186(::core::primitive::u8),
                        #[codec(index = 187)]
                        Mortal187(::core::primitive::u8),
                        #[codec(index = 188)]
                        Mortal188(::core::primitive::u8),
                        #[codec(index = 189)]
                        Mortal189(::core::primitive::u8),
                        #[codec(index = 190)]
                        Mortal190(::core::primitive::u8),
                        #[codec(index = 191)]
                        Mortal191(::core::primitive::u8),
                        #[codec(index = 192)]
                        Mortal192(::core::primitive::u8),
                        #[codec(index = 193)]
                        Mortal193(::core::primitive::u8),
                        #[codec(index = 194)]
                        Mortal194(::core::primitive::u8),
                        #[codec(index = 195)]
                        Mortal195(::core::primitive::u8),
                        #[codec(index = 196)]
                        Mortal196(::core::primitive::u8),
                        #[codec(index = 197)]
                        Mortal197(::core::primitive::u8),
                        #[codec(index = 198)]
                        Mortal198(::core::primitive::u8),
                        #[codec(index = 199)]
                        Mortal199(::core::primitive::u8),
                        #[codec(index = 200)]
                        Mortal200(::core::primitive::u8),
                        #[codec(index = 201)]
                        Mortal201(::core::primitive::u8),
                        #[codec(index = 202)]
                        Mortal202(::core::primitive::u8),
                        #[codec(index = 203)]
                        Mortal203(::core::primitive::u8),
                        #[codec(index = 204)]
                        Mortal204(::core::primitive::u8),
                        #[codec(index = 205)]
                        Mortal205(::core::primitive::u8),
                        #[codec(index = 206)]
                        Mortal206(::core::primitive::u8),
                        #[codec(index = 207)]
                        Mortal207(::core::primitive::u8),
                        #[codec(index = 208)]
                        Mortal208(::core::primitive::u8),
                        #[codec(index = 209)]
                        Mortal209(::core::primitive::u8),
                        #[codec(index = 210)]
                        Mortal210(::core::primitive::u8),
                        #[codec(index = 211)]
                        Mortal211(::core::primitive::u8),
                        #[codec(index = 212)]
                        Mortal212(::core::primitive::u8),
                        #[codec(index = 213)]
                        Mortal213(::core::primitive::u8),
                        #[codec(index = 214)]
                        Mortal214(::core::primitive::u8),
                        #[codec(index = 215)]
                        Mortal215(::core::primitive::u8),
                        #[codec(index = 216)]
                        Mortal216(::core::primitive::u8),
                        #[codec(index = 217)]
                        Mortal217(::core::primitive::u8),
                        #[codec(index = 218)]
                        Mortal218(::core::primitive::u8),
                        #[codec(index = 219)]
                        Mortal219(::core::primitive::u8),
                        #[codec(index = 220)]
                        Mortal220(::core::primitive::u8),
                        #[codec(index = 221)]
                        Mortal221(::core::primitive::u8),
                        #[codec(index = 222)]
                        Mortal222(::core::primitive::u8),
                        #[codec(index = 223)]
                        Mortal223(::core::primitive::u8),
                        #[codec(index = 224)]
                        Mortal224(::core::primitive::u8),
                        #[codec(index = 225)]
                        Mortal225(::core::primitive::u8),
                        #[codec(index = 226)]
                        Mortal226(::core::primitive::u8),
                        #[codec(index = 227)]
                        Mortal227(::core::primitive::u8),
                        #[codec(index = 228)]
                        Mortal228(::core::primitive::u8),
                        #[codec(index = 229)]
                        Mortal229(::core::primitive::u8),
                        #[codec(index = 230)]
                        Mortal230(::core::primitive::u8),
                        #[codec(index = 231)]
                        Mortal231(::core::primitive::u8),
                        #[codec(index = 232)]
                        Mortal232(::core::primitive::u8),
                        #[codec(index = 233)]
                        Mortal233(::core::primitive::u8),
                        #[codec(index = 234)]
                        Mortal234(::core::primitive::u8),
                        #[codec(index = 235)]
                        Mortal235(::core::primitive::u8),
                        #[codec(index = 236)]
                        Mortal236(::core::primitive::u8),
                        #[codec(index = 237)]
                        Mortal237(::core::primitive::u8),
                        #[codec(index = 238)]
                        Mortal238(::core::primitive::u8),
                        #[codec(index = 239)]
                        Mortal239(::core::primitive::u8),
                        #[codec(index = 240)]
                        Mortal240(::core::primitive::u8),
                        #[codec(index = 241)]
                        Mortal241(::core::primitive::u8),
                        #[codec(index = 242)]
                        Mortal242(::core::primitive::u8),
                        #[codec(index = 243)]
                        Mortal243(::core::primitive::u8),
                        #[codec(index = 244)]
                        Mortal244(::core::primitive::u8),
                        #[codec(index = 245)]
                        Mortal245(::core::primitive::u8),
                        #[codec(index = 246)]
                        Mortal246(::core::primitive::u8),
                        #[codec(index = 247)]
                        Mortal247(::core::primitive::u8),
                        #[codec(index = 248)]
                        Mortal248(::core::primitive::u8),
                        #[codec(index = 249)]
                        Mortal249(::core::primitive::u8),
                        #[codec(index = 250)]
                        Mortal250(::core::primitive::u8),
                        #[codec(index = 251)]
                        Mortal251(::core::primitive::u8),
                        #[codec(index = 252)]
                        Mortal252(::core::primitive::u8),
                        #[codec(index = 253)]
                        Mortal253(::core::primitive::u8),
                        #[codec(index = 254)]
                        Mortal254(::core::primitive::u8),
                        #[codec(index = 255)]
                        Mortal255(::core::primitive::u8),
                    }
                }
                pub mod header {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct Header<_0, _1> {
                        pub parent_hash: ::subxt::sp_core::H256,
                        #[codec(compact)]
                        pub number: _0,
                        pub state_root: ::subxt::sp_core::H256,
                        pub extrinsics_root: ::subxt::sp_core::H256,
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest<
                            ::subxt::sp_core::H256,
                        >,
                        #[codec(skip)]
                        pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                    }
                }
                pub mod unchecked_extrinsic {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                        pub ::std::vec::Vec<::core::primitive::u8>,
                        #[codec(skip)] pub ::core::marker::PhantomData<(_0, _1, _2, _3)>,
                    );
                }
            }
            pub mod multiaddress {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum MultiAddress<_0, _1> {
                    #[codec(index = 0)]
                    Id(_0),
                    #[codec(index = 1)]
                    Index(#[codec(compact)] _1),
                    #[codec(index = 2)]
                    Raw(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 3)]
                    Address32([::core::primitive::u8; 32usize]),
                    #[codec(index = 4)]
                    Address20([::core::primitive::u8; 20usize]),
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct BlakeTwo256;
            }
            pub mod transaction_validity {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum InvalidTransaction {
                    #[codec(index = 0)]
                    Call,
                    #[codec(index = 1)]
                    Payment,
                    #[codec(index = 2)]
                    Future,
                    #[codec(index = 3)]
                    Stale,
                    #[codec(index = 4)]
                    BadProof,
                    #[codec(index = 5)]
                    AncientBirthBlock,
                    #[codec(index = 6)]
                    ExhaustsResources,
                    #[codec(index = 7)]
                    Custom(::core::primitive::u8),
                    #[codec(index = 8)]
                    BadMandatory,
                    #[codec(index = 9)]
                    MandatoryDispatch,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum TransactionValidityError {
                    #[codec(index = 0)]
                    Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                    #[codec(index = 1)]
                    Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum UnknownTransaction {
                    #[codec(index = 0)]
                    CannotLookup,
                    #[codec(index = 1)]
                    NoUnsignedValidator,
                    #[codec(index = 2)]
                    Custom(::core::primitive::u8),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum ArithmeticError {
                #[codec(index = 0)]
                Underflow,
                #[codec(index = 1)]
                Overflow,
                #[codec(index = 2)]
                DivisionByZero,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum DispatchError {
                #[codec(index = 0)]
                Other,
                #[codec(index = 1)]
                CannotLookup,
                #[codec(index = 2)]
                BadOrigin,
                #[codec(index = 3)]
                Module {
                    index: ::core::primitive::u8,
                    error: ::core::primitive::u8,
                },
                #[codec(index = 4)]
                ConsumerRemaining,
                #[codec(index = 5)]
                NoProviders,
                #[codec(index = 6)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 7)]
                Arithmetic(runtime_types::sp_runtime::ArithmeticError),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum MultiSignature {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Signature),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Signature),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Signature),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum MultiSigner {
                #[codec(index = 0)]
                Ed25519(runtime_types::sp_core::ed25519::Public),
                #[codec(index = 1)]
                Sr25519(runtime_types::sp_core::sr25519::Public),
                #[codec(index = 2)]
                Ecdsa(runtime_types::sp_core::ecdsa::Public),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum TokenError {
                #[codec(index = 0)]
                NoFunds,
                #[codec(index = 1)]
                WouldDie,
                #[codec(index = 2)]
                BelowMinimum,
                #[codec(index = 3)]
                CannotCreate,
                #[codec(index = 4)]
                UnknownAsset,
                #[codec(index = 5)]
                Frozen,
                #[codec(index = 6)]
                Unsupported,
            }
        }
        pub mod sp_session {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MembershipProof {
                pub session: ::core::primitive::u32,
                pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub validator_count: ::core::primitive::u32,
            }
        }
        pub mod sp_staking {
            use super::runtime_types;
            pub mod offence {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct OffenceDetails<_0, _1> {
                    pub offender: _1,
                    pub reporters: ::std::vec::Vec<_0>,
                }
            }
        }
        pub mod sp_version {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct RuntimeVersion {
                pub spec_name: ::std::string::String,
                pub impl_name: ::std::string::String,
                pub authoring_version: ::core::primitive::u32,
                pub spec_version: ::core::primitive::u32,
                pub impl_version: ::core::primitive::u32,
                pub apis:
                    ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
                pub transaction_version: ::core::primitive::u32,
            }
        }
        pub mod to_substrate_backing {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    register_and_remote_create {
                        spec_version: ::core::primitive::u32,
                        weight: ::core::primitive::u64,
                        fee: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    lock_and_remote_issue {
                        spec_version: ::core::primitive::u32,
                        weight: ::core::primitive::u64,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        #[codec(compact)]
                        fee: ::core::primitive::u128,
                        recipient: runtime_types::primitive_types::H160,
                    },
                    #[codec(index = 2)]
                    unlock_from_remote {
                        token_address: runtime_types::primitive_types::H160,
                        amount: runtime_types::primitive_types::U256,
                        recipient: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    set_secure_limited_period { period: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    set_security_limitation_ring_amount { limitation: ::core::primitive::u128 },
                    #[codec(index = 5)]
                    set_remote_mapping_token_factory_account {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    InsufficientBalance,
                    #[codec(index = 1)]
                    RingLockLimited,
                    #[codec(index = 2)]
                    RingDailyLimited,
                    #[codec(index = 3)]
                    NonceDuplicated,
                    #[codec(index = 4)]
                    UnsupportedToken,
                    #[codec(index = 5)]
                    InvalidRecipient,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    TokenRegistered(
                        runtime_types::dp_asset::TokenMetadata,
                        ::subxt::sp_core::crypto::AccountId32,
                    ),
                    #[codec(index = 1)]
                    TokenLocked(
                        [::core::primitive::u8; 4usize],
                        ::core::primitive::u64,
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::crypto::AccountId32,
                        runtime_types::primitive_types::H160,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 2)]
                    TokenUnlocked(
                        [::core::primitive::u8; 4usize],
                        ::core::primitive::u64,
                        runtime_types::primitive_types::H160,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 3)]
                    TokenLockedConfirmed(
                        [::core::primitive::u8; 4usize],
                        ::core::primitive::u64,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                        ::core::primitive::bool,
                    ),
                    #[codec(index = 4)]
                    RemoteMappingFactoryAddressUpdated(::subxt::sp_core::crypto::AccountId32),
                }
            }
        }
    }
    #[doc = r" The default error type returned when there is a runtime issue."]
    pub type DispatchError = self::runtime_types::sp_runtime::DispatchError;
    pub struct ErrorDetails {
        pub pallet: &'static str,
        pub error: &'static str,
        pub docs: &'static str,
    }
    impl DispatchError {
        pub fn details(&self) -> Option<ErrorDetails> {
            if let Self::Module { index, error } = self {
                match (index , error) { (0u8 , 0u8) => Some (ErrorDetails { pallet : "System" , error : "InvalidSpecName" , docs : "The name of specification does not match between the current runtime\nand the new runtime." }) , (0u8 , 1u8) => Some (ErrorDetails { pallet : "System" , error : "SpecVersionNeedsToIncrease" , docs : "The specification version is not allowed to decrease between the current runtime\nand the new runtime." }) , (0u8 , 2u8) => Some (ErrorDetails { pallet : "System" , error : "FailedToExtractRuntimeVersion" , docs : "Failed to extract the runtime version from the new runtime.\n\nEither calling `Core_version` or decoding `RuntimeVersion` failed." }) , (0u8 , 3u8) => Some (ErrorDetails { pallet : "System" , error : "NonDefaultComposite" , docs : "Suicide called when the account has non-default composite data." }) , (0u8 , 4u8) => Some (ErrorDetails { pallet : "System" , error : "NonZeroRefCount" , docs : "There is a non-zero reference count preventing the account from being purged." }) , (2u8 , 0u8) => Some (ErrorDetails { pallet : "Babe" , error : "InvalidEquivocationProof" , docs : "An equivocation proof provided as part of an equivocation report is invalid." }) , (2u8 , 1u8) => Some (ErrorDetails { pallet : "Babe" , error : "InvalidKeyOwnershipProof" , docs : "A key ownership proof provided as part of an equivocation report is invalid." }) , (2u8 , 2u8) => Some (ErrorDetails { pallet : "Babe" , error : "DuplicateOffenceReport" , docs : "A given equivocation report is valid but already previously reported." }) , (4u8 , 0u8) => Some (ErrorDetails { pallet : "Balances" , error : "VestingBalance" , docs : "Vesting balance too high to send value." }) , (4u8 , 1u8) => Some (ErrorDetails { pallet : "Balances" , error : "LiquidityRestrictions" , docs : "Account liquidity restrictions prevent withdrawal." }) , (4u8 , 2u8) => Some (ErrorDetails { pallet : "Balances" , error : "InsufficientBalance" , docs : "Balance too low to send value." }) , (4u8 , 3u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistentialDeposit" , docs : "Value too low to create account due to existential deposit." }) , (4u8 , 4u8) => Some (ErrorDetails { pallet : "Balances" , error : "KeepAlive" , docs : "Transfer/payment would kill account." }) , (4u8 , 5u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistingVestingSchedule" , docs : "A vesting schedule already exists for this account." }) , (4u8 , 6u8) => Some (ErrorDetails { pallet : "Balances" , error : "DeadAccount" , docs : "Beneficiary account must pre-exist." }) , (4u8 , 7u8) => Some (ErrorDetails { pallet : "Balances" , error : "TooManyReserves" , docs : "Number of named reserves exceed MaxReserves" }) , (4u8 , 8u8) => Some (ErrorDetails { pallet : "Balances" , error : "LockP" , docs : "Lock - POISONED." }) , (5u8 , 0u8) => Some (ErrorDetails { pallet : "Kton" , error : "VestingBalance" , docs : "Vesting balance too high to send value." }) , (5u8 , 1u8) => Some (ErrorDetails { pallet : "Kton" , error : "LiquidityRestrictions" , docs : "Account liquidity restrictions prevent withdrawal." }) , (5u8 , 2u8) => Some (ErrorDetails { pallet : "Kton" , error : "InsufficientBalance" , docs : "Balance too low to send value." }) , (5u8 , 3u8) => Some (ErrorDetails { pallet : "Kton" , error : "ExistentialDeposit" , docs : "Value too low to create account due to existential deposit." }) , (5u8 , 4u8) => Some (ErrorDetails { pallet : "Kton" , error : "KeepAlive" , docs : "Transfer/payment would kill account." }) , (5u8 , 5u8) => Some (ErrorDetails { pallet : "Kton" , error : "ExistingVestingSchedule" , docs : "A vesting schedule already exists for this account." }) , (5u8 , 6u8) => Some (ErrorDetails { pallet : "Kton" , error : "DeadAccount" , docs : "Beneficiary account must pre-exist." }) , (5u8 , 7u8) => Some (ErrorDetails { pallet : "Kton" , error : "TooManyReserves" , docs : "Number of named reserves exceed MaxReserves" }) , (5u8 , 8u8) => Some (ErrorDetails { pallet : "Kton" , error : "LockP" , docs : "Lock - POISONED." }) , (7u8 , 0u8) => Some (ErrorDetails { pallet : "Authorship" , error : "InvalidUncleParent" , docs : "The uncle parent not in the chain." }) , (7u8 , 1u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UnclesAlreadySet" , docs : "Uncles already set in the block." }) , (7u8 , 2u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooManyUncles" , docs : "Too many uncles." }) , (7u8 , 3u8) => Some (ErrorDetails { pallet : "Authorship" , error : "GenesisUncle" , docs : "The uncle is genesis." }) , (7u8 , 4u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooHighUncle" , docs : "The uncle is too high in chain." }) , (7u8 , 5u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UncleAlreadyIncluded" , docs : "The uncle is already included." }) , (7u8 , 6u8) => Some (ErrorDetails { pallet : "Authorship" , error : "OldUncle" , docs : "The uncle isn't recent enough to be included." }) , (8u8 , 0u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "PreDispatchEarlySubmission" , docs : "Submission was too early." }) , (8u8 , 1u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "PreDispatchWrongWinnerCount" , docs : "Wrong number of winners presented." }) , (8u8 , 2u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "PreDispatchWeakSubmission" , docs : "Submission was too weak, score-wise." }) , (8u8 , 3u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedQueueFull" , docs : "The queue was full, and the solution was not better than any of the existing ones." }) , (8u8 , 4u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedCannotPayDeposit" , docs : "The origin failed to pay the deposit." }) , (8u8 , 5u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedInvalidWitness" , docs : "Witness data to dispatchable is invalid." }) , (8u8 , 6u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "SignedTooMuchWeight" , docs : "The signed submission consumes too much weight" }) , (8u8 , 7u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "OcwCallWrongEra" , docs : "OCW submitted solution for wrong round" }) , (8u8 , 8u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "MissingSnapshotMetadata" , docs : "Snapshot metadata should exist but didn't." }) , (8u8 , 9u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "InvalidSubmissionIndex" , docs : "`Self::insert_submission` returned an invalid index." }) , (8u8 , 10u8) => Some (ErrorDetails { pallet : "ElectionProviderMultiPhase" , error : "CallNotAllowed" , docs : "The call is not allowed at this point." }) , (9u8 , 0u8) => Some (ErrorDetails { pallet : "Staking" , error : "NotController" , docs : "Not a controller account." }) , (9u8 , 1u8) => Some (ErrorDetails { pallet : "Staking" , error : "NotStash" , docs : "Not a stash account." }) , (9u8 , 2u8) => Some (ErrorDetails { pallet : "Staking" , error : "AlreadyBonded" , docs : "Stash is already bonded." }) , (9u8 , 3u8) => Some (ErrorDetails { pallet : "Staking" , error : "AlreadyPaired" , docs : "Controller is already paired." }) , (9u8 , 4u8) => Some (ErrorDetails { pallet : "Staking" , error : "EmptyTargets" , docs : "Targets cannot be empty." }) , (9u8 , 5u8) => Some (ErrorDetails { pallet : "Staking" , error : "DuplicateIndex" , docs : "Duplicate index." }) , (9u8 , 6u8) => Some (ErrorDetails { pallet : "Staking" , error : "InvalidSlashIndex" , docs : "Slash record index out of bounds." }) , (9u8 , 7u8) => Some (ErrorDetails { pallet : "Staking" , error : "InsufficientBond" , docs : "Can not bond with value less than minimum required." }) , (9u8 , 8u8) => Some (ErrorDetails { pallet : "Staking" , error : "NoMoreChunks" , docs : "Can not schedule more unlock chunks." }) , (9u8 , 9u8) => Some (ErrorDetails { pallet : "Staking" , error : "NoUnlockChunk" , docs : "Can not rebond without unlocking chunks." }) , (9u8 , 10u8) => Some (ErrorDetails { pallet : "Staking" , error : "FundedTarget" , docs : "Attempting to target a stash that still has funds." }) , (9u8 , 11u8) => Some (ErrorDetails { pallet : "Staking" , error : "InvalidEraToReward" , docs : "Invalid era to reward." }) , (9u8 , 12u8) => Some (ErrorDetails { pallet : "Staking" , error : "InvalidNumberOfNominations" , docs : "Invalid number of nominations." }) , (9u8 , 13u8) => Some (ErrorDetails { pallet : "Staking" , error : "NotSortedAndUnique" , docs : "Items are not sorted and unique." }) , (9u8 , 14u8) => Some (ErrorDetails { pallet : "Staking" , error : "AlreadyClaimed" , docs : "Rewards for this era have already been claimed for this validator." }) , (9u8 , 15u8) => Some (ErrorDetails { pallet : "Staking" , error : "IncorrectHistoryDepth" , docs : "Incorrect previous history depth input provided." }) , (9u8 , 16u8) => Some (ErrorDetails { pallet : "Staking" , error : "IncorrectSlashingSpans" , docs : "Incorrect number of slashing spans provided." }) , (9u8 , 17u8) => Some (ErrorDetails { pallet : "Staking" , error : "BadState" , docs : "Internal state has become somehow corrupted and the operation cannot continue." }) , (9u8 , 18u8) => Some (ErrorDetails { pallet : "Staking" , error : "TooManyTargets" , docs : "Too many nomination targets supplied." }) , (9u8 , 19u8) => Some (ErrorDetails { pallet : "Staking" , error : "BadTarget" , docs : "A nomination target was supplied that was blocked or otherwise not a validator." }) , (9u8 , 20u8) => Some (ErrorDetails { pallet : "Staking" , error : "CannotChillOther" , docs : "The user has enough bond and thus cannot be chilled forcefully by an external person." }) , (9u8 , 21u8) => Some (ErrorDetails { pallet : "Staking" , error : "TooManyNominators" , docs : "There are too many nominators in the system. Governance needs to adjust the staking\nsettings to keep things safe for the runtime." }) , (9u8 , 22u8) => Some (ErrorDetails { pallet : "Staking" , error : "TooManyValidators" , docs : "There are too many validators in the system. Governance needs to adjust the staking\nsettings to keep things safe for the runtime." }) , (9u8 , 23u8) => Some (ErrorDetails { pallet : "Staking" , error : "PayoutIns" , docs : "Payout - INSUFFICIENT" }) , (12u8 , 0u8) => Some (ErrorDetails { pallet : "Session" , error : "InvalidProof" , docs : "Invalid ownership proof." }) , (12u8 , 1u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAssociatedValidatorId" , docs : "No associated validator ID for account." }) , (12u8 , 2u8) => Some (ErrorDetails { pallet : "Session" , error : "DuplicatedKey" , docs : "Registered duplicate key." }) , (12u8 , 3u8) => Some (ErrorDetails { pallet : "Session" , error : "NoKeys" , docs : "No keys are associated with this account." }) , (12u8 , 4u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAccount" , docs : "Key setting account is not live, so it's impossible to associate keys." }) , (13u8 , 0u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "PauseFailed" , docs : "Attempt to signal GRANDPA pause when the authority set isn't live\n(either paused or already pending pause)." }) , (13u8 , 1u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ResumeFailed" , docs : "Attempt to signal GRANDPA resume when the authority set isn't paused\n(either live or already pending resume)." }) , (13u8 , 2u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "ChangePending" , docs : "Attempt to signal GRANDPA change with one already pending." }) , (13u8 , 3u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "TooSoon" , docs : "Cannot signal forced change so soon after last." }) , (13u8 , 4u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidKeyOwnershipProof" , docs : "A key ownership proof provided as part of an equivocation report is invalid." }) , (13u8 , 5u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "InvalidEquivocationProof" , docs : "An equivocation proof provided as part of an equivocation report is invalid." }) , (13u8 , 6u8) => Some (ErrorDetails { pallet : "Grandpa" , error : "DuplicateOffenceReport" , docs : "A given equivocation report is valid but already previously reported." }) , (32u8 , 0u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "AuthorityExisted" , docs : "The authority is already existed." }) , (32u8 , 1u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "TooManyAuthorities" , docs : "Too many authorities." }) , (32u8 , 2u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "NotAuthority" , docs : "This is not an authority." }) , (32u8 , 3u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "AtLeastOneAuthority" , docs : "Require at least one authority. Not allow to decrease below one." }) , (32u8 , 4u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "OnAuthoritiesChange" , docs : "Currently, the authorities is changing." }) , (32u8 , 5u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "NoAuthoritiesChange" , docs : "Didn't find any authorities changes to sign." }) , (32u8 , 6u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "NoNewMessageRoot" , docs : "Didn't find any new message root to sign." }) , (32u8 , 7u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "BadSignature" , docs : "Failed to verify the signature." }) , (32u8 , 8u8) => Some (ErrorDetails { pallet : "EcdsaAuthority" , error : "AlreadySubmitted" , docs : "This authority had already finished his duty." }) , (14u8 , 0u8) => Some (ErrorDetails { pallet : "ImOnline" , error : "InvalidKey" , docs : "Non existent public key." }) , (14u8 , 1u8) => Some (ErrorDetails { pallet : "ImOnline" , error : "DuplicatedHeartbeat" , docs : "Duplicated heartbeat." }) , (24u8 , 0u8) => Some (ErrorDetails { pallet : "Treasury" , error : "InsufficientProposersBalance" , docs : "Proposer's balance is too low." }) , (24u8 , 1u8) => Some (ErrorDetails { pallet : "Treasury" , error : "InvalidIndex" , docs : "No proposal or bounty at that index." }) , (24u8 , 2u8) => Some (ErrorDetails { pallet : "Treasury" , error : "TooManyApprovals" , docs : "Too many approvals in the queue." }) , (16u8 , 0u8) => Some (ErrorDetails { pallet : "Sudo" , error : "RequireSudo" , docs : "Sender must be the Sudo account" }) , (21u8 , 0u8) => Some (ErrorDetails { pallet : "Scheduler" , error : "FailedToSchedule" , docs : "Failed to schedule a call" }) , (21u8 , 1u8) => Some (ErrorDetails { pallet : "Scheduler" , error : "NotFound" , docs : "Cannot find the scheduled call." }) , (21u8 , 2u8) => Some (ErrorDetails { pallet : "Scheduler" , error : "TargetBlockNumberInPast" , docs : "Given target block number is in the past." }) , (21u8 , 3u8) => Some (ErrorDetails { pallet : "Scheduler" , error : "RescheduleNoChange" , docs : "Reschedule failed because it does not change scheduled time." }) , (19u8 , 0u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "InvalidJustification" , docs : "The given justification is invalid for the given header." }) , (19u8 , 1u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "InvalidAuthoritySet" , docs : "The authority set from the underlying header chain is invalid." }) , (19u8 , 2u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "TooManyRequests" , docs : "There are too many requests for the current window to handle." }) , (19u8 , 3u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "OldHeader" , docs : "The header being imported is older than the best finalized header known to the pallet." }) , (19u8 , 4u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "UnknownHeader" , docs : "The header is unknown to the pallet." }) , (19u8 , 5u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "UnsupportedScheduledChange" , docs : "The scheduled authority set change found in the header is unsupported by the pallet.\n\nThis is the case for non-standard (e.g forced) authority set changes." }) , (19u8 , 6u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "NotInitialized" , docs : "The pallet is not yet initialized." }) , (19u8 , 7u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "AlreadyInitialized" , docs : "The pallet has already been initialized." }) , (19u8 , 8u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "Halted" , docs : "All pallet operations are halted." }) , (19u8 , 9u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "StorageRootMismatch" , docs : "The storage proof doesn't contains storage root. So it is invalid for given header." }) , (17u8 , 0u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "Halted" , docs : "All pallet operations are halted." }) , (17u8 , 1u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageRejectedByChainVerifier" , docs : "Message has been treated as invalid by chain verifier." }) , (17u8 , 2u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageRejectedByLaneVerifier" , docs : "Message has been treated as invalid by lane verifier." }) , (17u8 , 3u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "FailedToWithdrawMessageFee" , docs : "Submitter has failed to pay fee for delivering and dispatching messages." }) , (17u8 , 4u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "TooManyMessagesInTheProof" , docs : "The transaction brings too many messages." }) , (17u8 , 5u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidMessagesProof" , docs : "Invalid messages has been submitted." }) , (17u8 , 6u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidMessagesDeliveryProof" , docs : "Invalid messages delivery proof has been submitted." }) , (17u8 , 7u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidUnrewardedRelayers" , docs : "The bridged chain has invalid `UnrewardedRelayers` in its storage (fatal for the lane)." }) , (17u8 , 8u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidUnrewardedRelayersState" , docs : "The relayer has declared invalid unrewarded relayers state in the\n`receive_messages_delivery_proof` call." }) , (17u8 , 9u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageIsAlreadyDelivered" , docs : "The message someone is trying to work with (i.e. increase fee) is already-delivered." }) , (17u8 , 10u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageIsNotYetSent" , docs : "The message someone is trying to work with (i.e. increase fee) is not yet sent." }) , (17u8 , 11u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "TryingToConfirmMoreMessagesThanExpected" , docs : "The number of actually confirmed messages is going to be larger than the number of\nmessages in the proof. This may mean that this or bridged chain storage is corrupted." }) , (22u8 , 0u8) => Some (ErrorDetails { pallet : "PangolinFeeMarket" , error : "InsufficientBalance" , docs : "Insufficient balance." }) , (22u8 , 1u8) => Some (ErrorDetails { pallet : "PangolinFeeMarket" , error : "AlreadyEnrolled" , docs : "The relayer has been enrolled." }) , (22u8 , 2u8) => Some (ErrorDetails { pallet : "PangolinFeeMarket" , error : "NotEnrolled" , docs : "This relayer doesn't enroll ever." }) , (22u8 , 3u8) => Some (ErrorDetails { pallet : "PangolinFeeMarket" , error : "CollateralTooLow" , docs : "Locked collateral is too low to cover one order." }) , (22u8 , 4u8) => Some (ErrorDetails { pallet : "PangolinFeeMarket" , error : "StillHasOrdersNotConfirmed" , docs : "Update locked collateral is not allow since some orders are not confirm." }) , (22u8 , 5u8) => Some (ErrorDetails { pallet : "PangolinFeeMarket" , error : "RelayFeeTooLow" , docs : "The fee is lower than MinimumRelayFee." }) , (22u8 , 6u8) => Some (ErrorDetails { pallet : "PangolinFeeMarket" , error : "OccupiedRelayer" , docs : "The relayer is occupied, and can't cancel enrollment now." }) , (23u8 , 0u8) => Some (ErrorDetails { pallet : "TransactionPause" , error : "CannotPause" , docs : "can not pause" }) , (23u8 , 1u8) => Some (ErrorDetails { pallet : "TransactionPause" , error : "InvalidCharacter" , docs : "invalid character encoding" }) , (20u8 , 0u8) => Some (ErrorDetails { pallet : "Substrate2SubstrateBacking" , error : "InsufficientBalance" , docs : "Insufficient balance." }) , (20u8 , 1u8) => Some (ErrorDetails { pallet : "Substrate2SubstrateBacking" , error : "RingLockLimited" , docs : "Ring Lock LIMITED." }) , (20u8 , 2u8) => Some (ErrorDetails { pallet : "Substrate2SubstrateBacking" , error : "RingDailyLimited" , docs : "Redeem Daily Limited" }) , (20u8 , 3u8) => Some (ErrorDetails { pallet : "Substrate2SubstrateBacking" , error : "NonceDuplicated" , docs : "Message nonce duplicated." }) , (20u8 , 4u8) => Some (ErrorDetails { pallet : "Substrate2SubstrateBacking" , error : "UnsupportedToken" , docs : "Unsupported token" }) , (20u8 , 5u8) => Some (ErrorDetails { pallet : "Substrate2SubstrateBacking" , error : "InvalidRecipient" , docs : "Invalid recipient" }) , (25u8 , 0u8) => Some (ErrorDetails { pallet : "EVM" , error : "BalanceLow" , docs : "Not enough balance to perform action" }) , (25u8 , 1u8) => Some (ErrorDetails { pallet : "EVM" , error : "FeeOverflow" , docs : "Calculating total fee overflowed" }) , (25u8 , 2u8) => Some (ErrorDetails { pallet : "EVM" , error : "PaymentOverflow" , docs : "Calculating total payment overflowed" }) , (25u8 , 3u8) => Some (ErrorDetails { pallet : "EVM" , error : "WithdrawFailed" , docs : "Withdraw fee failed" }) , (25u8 , 4u8) => Some (ErrorDetails { pallet : "EVM" , error : "GasPriceTooLow" , docs : "Gas price is too low." }) , (25u8 , 5u8) => Some (ErrorDetails { pallet : "EVM" , error : "InvalidNonce" , docs : "Nonce is invalid" }) , (26u8 , 0u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "InvalidSignature" , docs : "Signature is invalid." }) , (26u8 , 1u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "PreLogExists" , docs : "Pre-log is present, therefore transact is not allowed." }) , (26u8 , 2u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "InternalTransactionExitError" , docs : "The internal transaction failed." }) , (26u8 , 3u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "InternalTransactionRevertError" , docs : "" }) , (26u8 , 4u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "InternalTransactionFatalError" , docs : "" }) , (26u8 , 5u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "ReadyOnlyCall" , docs : "The internal call failed." }) , (26u8 , 6u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "MessageTransactionError" , docs : "Message transaction invalid" }) , (26u8 , 7u8) => Some (ErrorDetails { pallet : "Ethereum" , error : "MessageValidateError" , docs : "Message validate invalid" }) , _ => None }
            } else {
                None
            }
        }
    }
    #[doc = r" The default storage entry from which to fetch an account nonce, required for"]
    #[doc = r" constructing a transaction."]
    pub enum DefaultAccountData {}
    impl ::subxt::AccountData for DefaultAccountData {
        type StorageEntry = self::system::storage::Account;
        type AccountId = ::subxt::sp_core::crypto::AccountId32;
        type Index = ::core::primitive::u32;
        fn nonce(result: &<Self::StorageEntry as ::subxt::StorageEntry>::Value) -> Self::Index {
            result.nonce
        }
        fn storage_entry(account_id: Self::AccountId) -> Self::StorageEntry {
            self::system::storage::Account(account_id)
        }
    }
    pub struct RuntimeApi<T: ::subxt::Config, X, A = DefaultAccountData> {
        pub client: ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<T, X, A> ::core::convert::From<::subxt::Client<T>> for RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        fn from(client: ::subxt::Client<T>) -> Self {
            Self {
                client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    impl<'a, T, X, A> RuntimeApi<T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn constants(&'a self) -> ConstantsApi {
            ConstantsApi
        }
        pub fn storage(&'a self) -> StorageApi<'a, T> {
            StorageApi {
                client: &self.client,
            }
        }
        pub fn tx(&'a self) -> TransactionApi<'a, T, X, A> {
            TransactionApi {
                client: &self.client,
                marker: ::core::marker::PhantomData,
            }
        }
    }
    pub struct ConstantsApi;
    impl ConstantsApi {
        pub fn system(&self) -> system::constants::ConstantsApi {
            system::constants::ConstantsApi
        }
        pub fn babe(&self) -> babe::constants::ConstantsApi {
            babe::constants::ConstantsApi
        }
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn kton(&self) -> kton::constants::ConstantsApi {
            kton::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi {
            authorship::constants::ConstantsApi
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::constants::ConstantsApi {
            election_provider_multi_phase::constants::ConstantsApi
        }
        pub fn staking(&self) -> staking::constants::ConstantsApi {
            staking::constants::ConstantsApi
        }
        pub fn grandpa(&self) -> grandpa::constants::ConstantsApi {
            grandpa::constants::ConstantsApi
        }
        pub fn ecdsa_authority(&self) -> ecdsa_authority::constants::ConstantsApi {
            ecdsa_authority::constants::ConstantsApi
        }
        pub fn im_online(&self) -> im_online::constants::ConstantsApi {
            im_online::constants::ConstantsApi
        }
        pub fn treasury(&self) -> treasury::constants::ConstantsApi {
            treasury::constants::ConstantsApi
        }
        pub fn scheduler(&self) -> scheduler::constants::ConstantsApi {
            scheduler::constants::ConstantsApi
        }
        pub fn bridge_pangolin_grandpa(&self) -> bridge_pangolin_grandpa::constants::ConstantsApi {
            bridge_pangolin_grandpa::constants::ConstantsApi
        }
        pub fn bridge_pangolin_messages(
            &self,
        ) -> bridge_pangolin_messages::constants::ConstantsApi {
            bridge_pangolin_messages::constants::ConstantsApi
        }
        pub fn pangolin_fee_market(&self) -> pangolin_fee_market::constants::ConstantsApi {
            pangolin_fee_market::constants::ConstantsApi
        }
        pub fn substrate2_substrate_backing(
            &self,
        ) -> substrate2_substrate_backing::constants::ConstantsApi {
            substrate2_substrate_backing::constants::ConstantsApi
        }
        pub fn ethereum(&self) -> ethereum::constants::ConstantsApi {
            ethereum::constants::ConstantsApi
        }
    }
    pub struct StorageApi<'a, T: ::subxt::Config> {
        client: &'a ::subxt::Client<T>,
    }
    impl<'a, T> StorageApi<'a, T>
    where
        T: ::subxt::Config,
    {
        pub fn system(&self) -> system::storage::StorageApi<'a, T> {
            system::storage::StorageApi::new(self.client)
        }
        pub fn babe(&self) -> babe::storage::StorageApi<'a, T> {
            babe::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn kton(&self) -> kton::storage::StorageApi<'a, T> {
            kton::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::storage::StorageApi<'a, T> {
            election_provider_multi_phase::storage::StorageApi::new(self.client)
        }
        pub fn staking(&self) -> staking::storage::StorageApi<'a, T> {
            staking::storage::StorageApi::new(self.client)
        }
        pub fn offences(&self) -> offences::storage::StorageApi<'a, T> {
            offences::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::storage::StorageApi<'a, T> {
            grandpa::storage::StorageApi::new(self.client)
        }
        pub fn beefy(&self) -> beefy::storage::StorageApi<'a, T> {
            beefy::storage::StorageApi::new(self.client)
        }
        pub fn message_gadget(&self) -> message_gadget::storage::StorageApi<'a, T> {
            message_gadget::storage::StorageApi::new(self.client)
        }
        pub fn ecdsa_authority(&self) -> ecdsa_authority::storage::StorageApi<'a, T> {
            ecdsa_authority::storage::StorageApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::storage::StorageApi<'a, T> {
            im_online::storage::StorageApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::storage::StorageApi<'a, T> {
            treasury::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::storage::StorageApi<'a, T> {
            scheduler::storage::StorageApi::new(self.client)
        }
        pub fn bridge_pangolin_grandpa(
            &self,
        ) -> bridge_pangolin_grandpa::storage::StorageApi<'a, T> {
            bridge_pangolin_grandpa::storage::StorageApi::new(self.client)
        }
        pub fn bridge_pangolin_messages(
            &self,
        ) -> bridge_pangolin_messages::storage::StorageApi<'a, T> {
            bridge_pangolin_messages::storage::StorageApi::new(self.client)
        }
        pub fn pangolin_fee_market(&self) -> pangolin_fee_market::storage::StorageApi<'a, T> {
            pangolin_fee_market::storage::StorageApi::new(self.client)
        }
        pub fn transaction_pause(&self) -> transaction_pause::storage::StorageApi<'a, T> {
            transaction_pause::storage::StorageApi::new(self.client)
        }
        pub fn substrate2_substrate_backing(
            &self,
        ) -> substrate2_substrate_backing::storage::StorageApi<'a, T> {
            substrate2_substrate_backing::storage::StorageApi::new(self.client)
        }
        pub fn evm(&self) -> evm::storage::StorageApi<'a, T> {
            evm::storage::StorageApi::new(self.client)
        }
        pub fn ethereum(&self) -> ethereum::storage::StorageApi<'a, T> {
            ethereum::storage::StorageApi::new(self.client)
        }
        pub fn base_fee(&self) -> base_fee::storage::StorageApi<'a, T> {
            base_fee::storage::StorageApi::new(self.client)
        }
    }
    pub struct TransactionApi<'a, T: ::subxt::Config, X, A> {
        client: &'a ::subxt::Client<T>,
        marker: ::core::marker::PhantomData<(X, A)>,
    }
    impl<'a, T, X, A> TransactionApi<'a, T, X, A>
    where
        T: ::subxt::Config,
        X: ::subxt::SignedExtra<T>,
        A: ::subxt::AccountData,
    {
        pub fn system(&self) -> system::calls::TransactionApi<'a, T, X, A> {
            system::calls::TransactionApi::new(self.client)
        }
        pub fn babe(&self) -> babe::calls::TransactionApi<'a, T, X, A> {
            babe::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X, A> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X, A> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn kton(&self) -> kton::calls::TransactionApi<'a, T, X, A> {
            kton::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T, X, A> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn election_provider_multi_phase(
            &self,
        ) -> election_provider_multi_phase::calls::TransactionApi<'a, T, X, A> {
            election_provider_multi_phase::calls::TransactionApi::new(self.client)
        }
        pub fn staking(&self) -> staking::calls::TransactionApi<'a, T, X, A> {
            staking::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T, X, A> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn grandpa(&self) -> grandpa::calls::TransactionApi<'a, T, X, A> {
            grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn message_gadget(&self) -> message_gadget::calls::TransactionApi<'a, T, X, A> {
            message_gadget::calls::TransactionApi::new(self.client)
        }
        pub fn ecdsa_authority(&self) -> ecdsa_authority::calls::TransactionApi<'a, T, X, A> {
            ecdsa_authority::calls::TransactionApi::new(self.client)
        }
        pub fn im_online(&self) -> im_online::calls::TransactionApi<'a, T, X, A> {
            im_online::calls::TransactionApi::new(self.client)
        }
        pub fn treasury(&self) -> treasury::calls::TransactionApi<'a, T, X, A> {
            treasury::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X, A> {
            sudo::calls::TransactionApi::new(self.client)
        }
        pub fn scheduler(&self) -> scheduler::calls::TransactionApi<'a, T, X, A> {
            scheduler::calls::TransactionApi::new(self.client)
        }
        pub fn bridge_pangolin_grandpa(
            &self,
        ) -> bridge_pangolin_grandpa::calls::TransactionApi<'a, T, X, A> {
            bridge_pangolin_grandpa::calls::TransactionApi::new(self.client)
        }
        pub fn bridge_pangolin_messages(
            &self,
        ) -> bridge_pangolin_messages::calls::TransactionApi<'a, T, X, A> {
            bridge_pangolin_messages::calls::TransactionApi::new(self.client)
        }
        pub fn pangolin_fee_market(
            &self,
        ) -> pangolin_fee_market::calls::TransactionApi<'a, T, X, A> {
            pangolin_fee_market::calls::TransactionApi::new(self.client)
        }
        pub fn transaction_pause(&self) -> transaction_pause::calls::TransactionApi<'a, T, X, A> {
            transaction_pause::calls::TransactionApi::new(self.client)
        }
        pub fn substrate2_substrate_backing(
            &self,
        ) -> substrate2_substrate_backing::calls::TransactionApi<'a, T, X, A> {
            substrate2_substrate_backing::calls::TransactionApi::new(self.client)
        }
        pub fn evm(&self) -> evm::calls::TransactionApi<'a, T, X, A> {
            evm::calls::TransactionApi::new(self.client)
        }
        pub fn ethereum(&self) -> ethereum::calls::TransactionApi<'a, T, X, A> {
            ethereum::calls::TransactionApi::new(self.client)
        }
        pub fn base_fee(&self) -> base_fee::calls::TransactionApi<'a, T, X, A> {
            base_fee::calls::TransactionApi::new(self.client)
        }
    }
}
