#[allow(dead_code, unused_imports, non_camel_case_types)]
pub mod api {
    #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
    pub enum Event {
        #[codec(index = 0)]
        System(system::Event),
        #[codec(index = 1)]
        ParachainSystem(parachain_system::Event),
        #[codec(index = 5)]
        Balances(balances::Event),
        #[codec(index = 8)]
        CollatorSelection(collator_selection::Event),
        #[codec(index = 9)]
        Session(session::Event),
        #[codec(index = 12)]
        XcmpQueue(xcmp_queue::Event),
        #[codec(index = 13)]
        PolkadotXcm(polkadot_xcm::Event),
        #[codec(index = 14)]
        CumulusXcm(cumulus_xcm::Event),
        #[codec(index = 15)]
        DmpQueue(dmp_queue::Event),
        #[codec(index = 16)]
        Utility(utility::Event),
        #[codec(index = 17)]
        Multisig(multisig::Event),
        #[codec(index = 18)]
        Proxy(proxy::Event),
        #[codec(index = 19)]
        Sudo(sudo::Event),
        #[codec(index = 21)]
        BridgePangolinMessages(bridge_pangolin_messages::Event),
        #[codec(index = 22)]
        BridgePangolinDispatch(bridge_pangolin_dispatch::Event),
        #[codec(index = 23)]
        FeeMarket(fee_market::Event),
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
            pub struct ExtrinsicSuccess {
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
            impl ::subxt::Event for ExtrinsicSuccess {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "ExtrinsicSuccess";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ExtrinsicFailed {
                pub dispatch_error: runtime_types::sp_runtime::DispatchError,
                pub dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
            }
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
            pub struct NewAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for NewAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "NewAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct KilledAccount {
                pub account: ::subxt::sp_core::crypto::AccountId32,
            }
            impl ::subxt::Event for KilledAccount {
                const PALLET: &'static str = "System";
                const EVENT: &'static str = "KilledAccount";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Remarked {
                pub sender: ::subxt::sp_core::crypto::AccountId32,
                pub hash: ::subxt::sp_core::H256,
            }
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
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
                type Value = runtime_types::sp_runtime::generic::digest::Digest;
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
                        runtime_types::pangolin_parachain_runtime::Event,
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
                        runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
                    runtime_types::sp_runtime::generic::digest::Digest,
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
                            runtime_types::pangolin_parachain_runtime::Event,
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
                            0u8, 242u8, 5u8, 42u8, 1u8, 0u8, 0u8, 0u8, 0u8, 136u8, 82u8, 106u8,
                            116u8, 0u8, 0u8, 0u8, 64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8,
                            192u8, 210u8, 44u8, 118u8, 81u8, 0u8, 0u8, 0u8, 1u8, 0u8, 230u8, 189u8,
                            79u8, 87u8, 0u8, 0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            64u8, 89u8, 115u8, 7u8, 0u8, 0u8, 0u8, 0u8, 1u8, 192u8, 116u8, 193u8,
                            144u8, 110u8, 0u8, 0u8, 0u8, 1u8, 0u8, 136u8, 82u8, 106u8, 116u8, 0u8,
                            0u8, 0u8, 1u8, 0u8, 162u8, 148u8, 26u8, 29u8, 0u8, 0u8, 0u8, 64u8,
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
                            72u8, 80u8, 97u8, 110u8, 103u8, 111u8, 108u8, 105u8, 110u8, 32u8, 80u8,
                            97u8, 114u8, 97u8, 99u8, 104u8, 97u8, 105u8, 110u8, 72u8, 80u8, 97u8,
                            110u8, 103u8, 111u8, 108u8, 105u8, 110u8, 32u8, 80u8, 97u8, 114u8,
                            97u8, 99u8, 104u8, 97u8, 105u8, 110u8, 1u8, 0u8, 0u8, 0u8, 3u8, 0u8,
                            0u8, 0u8, 1u8, 0u8, 0u8, 0u8, 44u8, 223u8, 106u8, 203u8, 104u8, 153u8,
                            7u8, 96u8, 155u8, 4u8, 0u8, 0u8, 0u8, 55u8, 227u8, 151u8, 252u8, 124u8,
                            145u8, 245u8, 228u8, 1u8, 0u8, 0u8, 0u8, 64u8, 254u8, 58u8, 212u8, 1u8,
                            248u8, 149u8, 154u8, 5u8, 0u8, 0u8, 0u8, 210u8, 188u8, 152u8, 151u8,
                            238u8, 208u8, 143u8, 21u8, 3u8, 0u8, 0u8, 0u8, 247u8, 139u8, 39u8,
                            139u8, 229u8, 63u8, 69u8, 76u8, 2u8, 0u8, 0u8, 0u8, 171u8, 60u8, 5u8,
                            114u8, 41u8, 31u8, 235u8, 139u8, 1u8, 0u8, 0u8, 0u8, 221u8, 113u8,
                            141u8, 92u8, 197u8, 50u8, 98u8, 212u8, 1u8, 0u8, 0u8, 0u8, 188u8,
                            157u8, 137u8, 144u8, 79u8, 91u8, 146u8, 63u8, 1u8, 0u8, 0u8, 0u8, 55u8,
                            200u8, 187u8, 19u8, 80u8, 169u8, 162u8, 168u8, 1u8, 0u8, 0u8, 0u8,
                            234u8, 147u8, 227u8, 241u8, 111u8, 61u8, 105u8, 98u8, 2u8, 0u8, 0u8,
                            0u8, 45u8, 6u8, 198u8, 53u8, 50u8, 185u8, 56u8, 227u8, 1u8, 0u8, 0u8,
                            0u8, 1u8, 0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn ss58_prefix(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[42u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod parachain_system {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            // #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            // pub struct set_validation_data {
            //     pub data:
            //         runtime_types::cumulus_primitives_parachain_inherent::ParachainInherentData,
            // }
            // impl ::subxt::Call for set_validation_data {
            //     const PALLET: &'static str = "ParachainSystem";
            //     const FUNCTION: &'static str = "set_validation_data";
            // }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct sudo_send_upward_message {
                pub message: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for sudo_send_upward_message {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "sudo_send_upward_message";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct authorize_upgrade {
                pub code_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for authorize_upgrade {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "authorize_upgrade";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct enact_authorized_upgrade {
                pub code: ::std::vec::Vec<::core::primitive::u8>,
            }
            impl ::subxt::Call for enact_authorized_upgrade {
                const PALLET: &'static str = "ParachainSystem";
                const FUNCTION: &'static str = "enact_authorized_upgrade";
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
                // pub fn set_validation_data(
                //     &self,
                //     data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData,
                // ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_validation_data, DispatchError>
                // {
                //     let call = set_validation_data { data };
                //     ::subxt::SubmittableExtrinsic::new(self.client, call)
                // }
                pub fn sudo_send_upward_message(
                    &self,
                    message: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    sudo_send_upward_message,
                    DispatchError,
                > {
                    let call = sudo_send_upward_message { message };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn authorize_upgrade(
                    &self,
                    code_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, authorize_upgrade, DispatchError>
                {
                    let call = authorize_upgrade { code_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn enact_authorized_upgrade(
                    &self,
                    code: ::std::vec::Vec<::core::primitive::u8>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    enact_authorized_upgrade,
                    DispatchError,
                > {
                    let call = enact_authorized_upgrade { code };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_parachain_system::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ValidationFunctionStored;
            impl ::subxt::Event for ValidationFunctionStored {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionStored";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ValidationFunctionApplied(pub ::core::primitive::u32);
            impl ::subxt::Event for ValidationFunctionApplied {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionApplied";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ValidationFunctionDiscarded;
            impl ::subxt::Event for ValidationFunctionDiscarded {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "ValidationFunctionDiscarded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UpgradeAuthorized(pub ::subxt::sp_core::H256);
            impl ::subxt::Event for UpgradeAuthorized {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "UpgradeAuthorized";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct DownwardMessagesReceived(pub ::core::primitive::u32);
            impl ::subxt::Event for DownwardMessagesReceived {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesReceived";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DownwardMessagesProcessed(
                pub ::core::primitive::u64,
                pub ::subxt::sp_core::H256,
            );
            impl ::subxt::Event for DownwardMessagesProcessed {
                const PALLET: &'static str = "ParachainSystem";
                const EVENT: &'static str = "DownwardMessagesProcessed";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct PendingValidationCode;
            impl ::subxt::StorageEntry for PendingValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "PendingValidationCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct NewValidationCode;
            impl ::subxt::StorageEntry for NewValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "NewValidationCode";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ValidationData;
            impl ::subxt::StorageEntry for ValidationData {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ValidationData";
                type Value = runtime_types::polkadot_primitives::v1::PersistedValidationData<
                    ::subxt::sp_core::H256,
                    ::core::primitive::u32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct DidSetValidationCode;
            impl ::subxt::StorageEntry for DidSetValidationCode {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "DidSetValidationCode";
                type Value = ::core::primitive::bool;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpgradeRestrictionSignal;
            impl ::subxt::StorageEntry for UpgradeRestrictionSignal {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "UpgradeRestrictionSignal";
                type Value = ::core::option::Option<
                    runtime_types::polkadot_primitives::v1::UpgradeRestriction,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct RelevantMessagingState;
            impl ::subxt::StorageEntry for RelevantMessagingState {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "RelevantMessagingState";
                type Value = runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot ;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HostConfiguration;
            impl ::subxt::StorageEntry for HostConfiguration {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HostConfiguration";
                type Value = runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastDmqMqcHead;
            impl ::subxt::StorageEntry for LastDmqMqcHead {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "LastDmqMqcHead";
                type Value =
                    runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            //# error[E0277]: the trait bound `BTreeMap<primitives::Id, MessageQueueChain>: Decode` is not satisfied
            //# --> D:\dev\darwinia-network\bridger\frame\assistants\pangolin-parachain-subxt\src\runtime.rs:975:17
            //# |
            //# 975 | /                 type Value = ::std::collections::BTreeMap<
            //# 976 | |                     runtime_types::polkadot_parachain::primitives::Id,
            //#                             977 | |                     runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
            //#                                                         978 | |                 >;
            //# | |__________________^ the trait `Decode` is not implemented for `BTreeMap<primitives::Id, MessageQueueChain>`
            //# |
            //# = help: the following implementations were found:
            //# <BTreeMap<K, V> as Decode>
            //# note: required by a bound in `subxt::StorageEntry::Value`
            //# --> d:\opt\scoop\persist\rustup\.cargo\git\checkouts\subxt-52715947a5b6313f\0dadaa5\subxt\src\storage.rs:50:17
            //# |
            //# 50  |     type Value: Decode;
            //# |                 ^^^^^^ required by this bound in `subxt::StorageEntry::Value`
            // pub struct LastHrmpMqcHeads;
            // impl ::subxt::StorageEntry for LastHrmpMqcHeads {
            //     const PALLET: &'static str = "ParachainSystem";
            //     const STORAGE: &'static str = "LastHrmpMqcHeads";
            //     type Value = ::std::collections::BTreeMap<
            //         runtime_types::polkadot_parachain::primitives::Id,
            //         runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
            //     >;
            //     fn key(&self) -> ::subxt::StorageEntryKey {
            //         ::subxt::StorageEntryKey::Plain
            //     }
            // }
            pub struct ProcessedDownwardMessages;
            impl ::subxt::StorageEntry for ProcessedDownwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ProcessedDownwardMessages";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpWatermark;
            impl ::subxt::StorageEntry for HrmpWatermark {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HrmpWatermark";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct HrmpOutboundMessages;
            impl ::subxt::StorageEntry for HrmpOutboundMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "HrmpOutboundMessages";
                type Value = ::std::vec::Vec<
                    runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                        runtime_types::polkadot_parachain::primitives::Id,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct UpwardMessages;
            impl ::subxt::StorageEntry for UpwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "UpwardMessages";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PendingUpwardMessages;
            impl ::subxt::StorageEntry for PendingUpwardMessages {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "PendingUpwardMessages";
                type Value = ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AnnouncedHrmpMessagesPerCandidate;
            impl ::subxt::StorageEntry for AnnouncedHrmpMessagesPerCandidate {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "AnnouncedHrmpMessagesPerCandidate";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedXcmpWeightOverride;
            impl ::subxt::StorageEntry for ReservedXcmpWeightOverride {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ReservedXcmpWeightOverride";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct ReservedDmpWeightOverride;
            impl ::subxt::StorageEntry for ReservedDmpWeightOverride {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "ReservedDmpWeightOverride";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AuthorizedUpgrade;
            impl ::subxt::StorageEntry for AuthorizedUpgrade {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "AuthorizedUpgrade";
                type Value = ::subxt::sp_core::H256;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CustomValidationHeadData;
            impl ::subxt::StorageEntry for CustomValidationHeadData {
                const PALLET: &'static str = "ParachainSystem";
                const STORAGE: &'static str = "CustomValidationHeadData";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
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
                pub async fn pending_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = PendingValidationCode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn new_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = NewValidationCode;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn validation_data(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::PersistedValidationData<
                            ::subxt::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = ValidationData;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn did_set_validation_code(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::bool, ::subxt::BasicError>
                {
                    let entry = DidSetValidationCode;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upgrade_restriction_signal(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::UpgradeRestriction,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = UpgradeRestrictionSignal;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }                pub async fn relevant_messaging_state (& self , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < :: core :: option :: Option < runtime_types :: cumulus_pallet_parachain_system :: relay_state_snapshot :: MessagingStateSnapshot > , :: subxt :: BasicError >{
                    let entry = RelevantMessagingState;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn host_configuration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::polkadot_primitives::v1::AbridgedHostConfiguration,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = HostConfiguration;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn last_dmq_mqc_head(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
                    ::subxt::BasicError,
                > {
                    let entry = LastDmqMqcHead;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                // pub async fn last_hrmp_mqc_heads(
                //     &self,
                //     hash: ::core::option::Option<T::Hash>,
                // ) -> ::core::result::Result<
                //     ::std::collections::BTreeMap<
                //         runtime_types::polkadot_parachain::primitives::Id,
                //         runtime_types::cumulus_primitives_parachain_inherent::MessageQueueChain,
                //     >,
                //     ::subxt::BasicError,
                // > {
                //     let entry = LastHrmpMqcHeads;
                //     self.client.storage().fetch_or_default(&entry, hash).await
                // }
                pub async fn processed_downward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = ProcessedDownwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_watermark(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = HrmpWatermark;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn hrmp_outbound_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = HrmpOutboundMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn upward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = UpwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pending_upward_messages(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = PendingUpwardMessages;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announced_hrmp_messages_per_candidate(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AnnouncedHrmpMessagesPerCandidate;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn reserved_xcmp_weight_override(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = ReservedXcmpWeightOverride;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn reserved_dmp_weight_override(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = ReservedDmpWeightOverride;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn authorized_upgrade(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::subxt::sp_core::H256>,
                    ::subxt::BasicError,
                > {
                    let entry = AuthorizedUpgrade;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn custom_validation_head_data(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ::subxt::BasicError,
                > {
                    let entry = CustomValidationHeadData;
                    self.client.storage().fetch(&entry, hash).await
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
                        &mut &[112u8, 23u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod parachain_info {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct ParachainId;
            impl ::subxt::StorageEntry for ParachainId {
                const PALLET: &'static str = "ParachainInfo";
                const STORAGE: &'static str = "ParachainId";
                type Value = runtime_types::polkadot_parachain::primitives::Id;
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
                pub async fn parachain_id(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::subxt::BasicError,
                > {
                    let entry = ParachainId;
                    self.client.storage().fetch_or_default(&entry, hash).await
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
        pub type Event = runtime_types::pallet_balances::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Endowed {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub free_balance: ::core::primitive::u128,
            }
            impl ::subxt::Event for Endowed {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Endowed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DustLost {
                pub account: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for DustLost {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "DustLost";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Transfer {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Transfer {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Transfer";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BalanceSet {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub free: ::core::primitive::u128,
                pub reserved: ::core::primitive::u128,
            }
            impl ::subxt::Event for BalanceSet {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "BalanceSet";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Reserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Reserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Reserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Unreserved {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Unreserved {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Unreserved";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ReserveRepatriated {
                pub from: ::subxt::sp_core::crypto::AccountId32,
                pub to: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
                pub destination_status:
                    runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
            }
            impl ::subxt::Event for ReserveRepatriated {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "ReserveRepatriated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Deposit {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Deposit {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Deposit";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Withdraw {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
            impl ::subxt::Event for Withdraw {
                const PALLET: &'static str = "Balances";
                const EVENT: &'static str = "Withdraw";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Slashed {
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub amount: ::core::primitive::u128,
            }
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
                type Value = runtime_types::pallet_balances::AccountData<::core::primitive::u128>;
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
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
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
                    runtime_types::pallet_balances::ReserveData<
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
                type Value = runtime_types::pallet_balances::Releases;
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
                    runtime_types::pallet_balances::AccountData<::core::primitive::u128>,
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
                        runtime_types::pallet_balances::BalanceLock<::core::primitive::u128>,
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
                        runtime_types::pallet_balances::ReserveData<
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
                    runtime_types::pallet_balances::Releases,
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
                            0u8, 128u8, 224u8, 55u8, 121u8, 195u8, 17u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
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
                            4u8, 0u8, 8u8, 175u8, 47u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 1u8,
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
                        &mut &[0u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod collator_selection {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_invulnerables {
                pub new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Call for set_invulnerables {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "set_invulnerables";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_desired_candidates {
                pub max: ::core::primitive::u32,
            }
            impl ::subxt::Call for set_desired_candidates {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "set_desired_candidates";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct set_candidacy_bond {
                pub bond: ::core::primitive::u128,
            }
            impl ::subxt::Call for set_candidacy_bond {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "set_candidacy_bond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct register_as_candidate;
            impl ::subxt::Call for register_as_candidate {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "register_as_candidate";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct leave_intent;
            impl ::subxt::Call for leave_intent {
                const PALLET: &'static str = "CollatorSelection";
                const FUNCTION: &'static str = "leave_intent";
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
                pub fn set_invulnerables(
                    &self,
                    new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_invulnerables, DispatchError>
                {
                    let call = set_invulnerables { new };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_desired_candidates(
                    &self,
                    max: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_desired_candidates, DispatchError>
                {
                    let call = set_desired_candidates { max };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn set_candidacy_bond(
                    &self,
                    bond: ::core::primitive::u128,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, set_candidacy_bond, DispatchError>
                {
                    let call = set_candidacy_bond { bond };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn register_as_candidate(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, register_as_candidate, DispatchError>
                {
                    let call = register_as_candidate {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn leave_intent(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, leave_intent, DispatchError>
                {
                    let call = leave_intent {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_collator_selection::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NewInvulnerables(pub ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>);
            impl ::subxt::Event for NewInvulnerables {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewInvulnerables";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewDesiredCandidates(pub ::core::primitive::u32);
            impl ::subxt::Event for NewDesiredCandidates {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewDesiredCandidates";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct NewCandidacyBond(pub ::core::primitive::u128);
            impl ::subxt::Event for NewCandidacyBond {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "NewCandidacyBond";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CandidateAdded(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for CandidateAdded {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "CandidateAdded";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CandidateRemoved(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for CandidateRemoved {
                const PALLET: &'static str = "CollatorSelection";
                const EVENT: &'static str = "CandidateRemoved";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Invulnerables;
            impl ::subxt::StorageEntry for Invulnerables {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "Invulnerables";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Candidates;
            impl ::subxt::StorageEntry for Candidates {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "Candidates";
                type Value = ::std::vec::Vec<
                    runtime_types::pallet_collator_selection::pallet::CandidateInfo<
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    >,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct LastAuthoredBlock(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for LastAuthoredBlock {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "LastAuthoredBlock";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct DesiredCandidates;
            impl ::subxt::StorageEntry for DesiredCandidates {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "DesiredCandidates";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CandidacyBond;
            impl ::subxt::StorageEntry for CandidacyBond {
                const PALLET: &'static str = "CollatorSelection";
                const STORAGE: &'static str = "CandidacyBond";
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
                pub async fn candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::pallet_collator_selection::pallet::CandidateInfo<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Candidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn last_authored_block(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = LastAuthoredBlock(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn last_authored_block_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, LastAuthoredBlock>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn desired_candidates(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = DesiredCandidates;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn candidacy_bond(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    let entry = CandidacyBond;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod session {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct set_keys {
                pub keys: runtime_types::pangolin_parachain_runtime::pallets::session::SessionKeys,
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
                    keys: runtime_types::pangolin_parachain_runtime::pallets::session::SessionKeys,
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
            pub struct NewSession {
                pub session_index: ::core::primitive::u32,
            }
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
                    runtime_types::pangolin_parachain_runtime::pallets::session::SessionKeys,
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
                type Value =
                    runtime_types::pangolin_parachain_runtime::pallets::session::SessionKeys;
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
                        runtime_types::pangolin_parachain_runtime::pallets::session::SessionKeys,
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
                        runtime_types::pangolin_parachain_runtime::pallets::session::SessionKeys,
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
    pub mod aura {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentSlot;
            impl ::subxt::StorageEntry for CurrentSlot {
                const PALLET: &'static str = "Aura";
                const STORAGE: &'static str = "CurrentSlot";
                type Value = runtime_types::sp_consensus_slots::Slot;
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
                    runtime_types::frame_support::storage::weak_bounded_vec::WeakBoundedVec<
                        runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
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
            }
        }
    }
    pub mod aura_ext {
        use super::runtime_types;
        pub mod storage {
            use super::runtime_types;
            pub struct Authorities;
            impl ::subxt::StorageEntry for Authorities {
                const PALLET: &'static str = "AuraExt";
                const STORAGE: &'static str = "Authorities";
                type Value =
                    ::std::vec::Vec<runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public>;
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
                    ::std::vec::Vec<runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public>,
                    ::subxt::BasicError,
                > {
                    let entry = Authorities;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod xcmp_queue {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct service_overweight {
                pub index: ::core::primitive::u64,
                pub weight_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for service_overweight {
                const PALLET: &'static str = "XcmpQueue";
                const FUNCTION: &'static str = "service_overweight";
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
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, service_overweight, DispatchError>
                {
                    let call = service_overweight {
                        index,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_xcmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Success(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for Success {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Success";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Fail(
                pub ::core::option::Option<::subxt::sp_core::H256>,
                pub runtime_types::xcm::v2::traits::Error,
            );
            impl ::subxt::Event for Fail {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "Fail";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BadVersion(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for BadVersion {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BadFormat(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for BadFormat {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "BadFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UpwardMessageSent(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for UpwardMessageSent {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "UpwardMessageSent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct XcmpMessageSent(pub ::core::option::Option<::subxt::sp_core::H256>);
            impl ::subxt::Event for XcmpMessageSent {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "XcmpMessageSent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OverweightEnqueued(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for OverweightEnqueued {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OverweightServiced(pub ::core::primitive::u64, pub ::core::primitive::u64);
            impl ::subxt::Event for OverweightServiced {
                const PALLET: &'static str = "XcmpQueue";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct InboundXcmpStatus;
            impl ::subxt::StorageEntry for InboundXcmpStatus {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "InboundXcmpStatus";
                type Value = ::std::vec::Vec<
                    runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct InboundXcmpMessages(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u32,
            );
            impl ::subxt::StorageEntry for InboundXcmpMessages {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "InboundXcmpMessages";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct OutboundXcmpStatus;
            impl ::subxt::StorageEntry for OutboundXcmpStatus {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "OutboundXcmpStatus";
                type Value = ::std::vec::Vec<
                    runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct OutboundXcmpMessages(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub ::core::primitive::u16,
            );
            impl ::subxt::StorageEntry for OutboundXcmpMessages {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "OutboundXcmpMessages";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(
                            &self.0,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                        ::subxt::StorageMapKey::new(&self.1, ::subxt::StorageHasher::Twox64Concat),
                    ])
                }
            }
            pub struct SignalMessages(pub runtime_types::polkadot_parachain::primitives::Id);
            impl ::subxt::StorageEntry for SignalMessages {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "SignalMessages";
                type Value = ::std::vec::Vec<::core::primitive::u8>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct QueueConfig;
            impl ::subxt::StorageEntry for QueueConfig {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "QueueConfig";
                type Value = runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Overweight(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Overweight {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "Overweight";
                type Value = (
                    runtime_types::polkadot_parachain::primitives::Id,
                    ::core::primitive::u32,
                    ::std::vec::Vec<::core::primitive::u8>,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct OverweightCount;
            impl ::subxt::StorageEntry for OverweightCount {
                const PALLET: &'static str = "XcmpQueue";
                const STORAGE: &'static str = "OverweightCount";
                type Value = ::core::primitive::u64;
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
                pub async fn inbound_xcmp_status(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::cumulus_pallet_xcmp_queue::InboundChannelDetails,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = InboundXcmpStatus;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_xcmp_messages(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    _1: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = InboundXcmpMessages(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn inbound_xcmp_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, InboundXcmpMessages>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn outbound_xcmp_status(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<
                        runtime_types::cumulus_pallet_xcmp_queue::OutboundChannelDetails,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundXcmpStatus;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn outbound_xcmp_messages(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    _1: ::core::primitive::u16,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = OutboundXcmpMessages(_0, _1);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn outbound_xcmp_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, OutboundXcmpMessages>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn signal_messages(
                    &self,
                    _0: runtime_types::polkadot_parachain::primitives::Id,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<::core::primitive::u8>,
                    ::subxt::BasicError,
                > {
                    let entry = SignalMessages(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn signal_messages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SignalMessages>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn queue_config(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_xcmp_queue::QueueConfigData,
                    ::subxt::BasicError,
                > {
                    let entry = QueueConfig;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn overweight(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Overweight(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Overweight>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn overweight_count(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = OverweightCount;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
            }
        }
    }
    pub mod polkadot_xcm {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct send {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
            }
            impl ::subxt::Call for send {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "send";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct teleport_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for teleport_assets {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "teleport_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct reserve_transfer_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
            }
            impl ::subxt::Call for reserve_transfer_assets {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "reserve_transfer_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct execute {
                pub message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for execute {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "execute";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_xcm_version {
                pub location:
                    ::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
                pub xcm_version: ::core::primitive::u32,
            }
            impl ::subxt::Call for force_xcm_version {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_default_xcm_version {
                pub maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
            }
            impl ::subxt::Call for force_default_xcm_version {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_default_xcm_version";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_subscribe_version_notify {
                pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
            }
            impl ::subxt::Call for force_subscribe_version_notify {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_subscribe_version_notify";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct force_unsubscribe_version_notify {
                pub location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
            }
            impl ::subxt::Call for force_unsubscribe_version_notify {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "force_unsubscribe_version_notify";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct limited_reserve_transfer_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            impl ::subxt::Call for limited_reserve_transfer_assets {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "limited_reserve_transfer_assets";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct limited_teleport_assets {
                pub dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                pub assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                pub fee_asset_item: ::core::primitive::u32,
                pub weight_limit: runtime_types::xcm::v2::WeightLimit,
            }
            impl ::subxt::Call for limited_teleport_assets {
                const PALLET: &'static str = "PolkadotXcm";
                const FUNCTION: &'static str = "limited_teleport_assets";
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
                pub fn send(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    message: runtime_types::xcm::VersionedXcm,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, send, DispatchError>
                {
                    let call = send {
                        dest: ::std::boxed::Box::new(dest),
                        message: ::std::boxed::Box::new(message),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, teleport_assets, DispatchError>
                {
                    let call = teleport_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    reserve_transfer_assets,
                    DispatchError,
                > {
                    let call = reserve_transfer_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
                        fee_asset_item,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn execute(
                    &self,
                    message: runtime_types::xcm::VersionedXcm,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, execute, DispatchError>
                {
                    let call = execute {
                        message: ::std::boxed::Box::new(message),
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_xcm_version(
                    &self,
                    location: runtime_types::xcm::v1::multilocation::MultiLocation,
                    xcm_version: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, force_xcm_version, DispatchError>
                {
                    let call = force_xcm_version {
                        location: ::std::boxed::Box::new(location),
                        xcm_version,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_default_xcm_version(
                    &self,
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_default_xcm_version,
                    DispatchError,
                > {
                    let call = force_default_xcm_version { maybe_xcm_version };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_subscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_subscribe_version_notify,
                    DispatchError,
                > {
                    let call = force_subscribe_version_notify {
                        location: ::std::boxed::Box::new(location),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn force_unsubscribe_version_notify(
                    &self,
                    location: runtime_types::xcm::VersionedMultiLocation,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    force_unsubscribe_version_notify,
                    DispatchError,
                > {
                    let call = force_unsubscribe_version_notify {
                        location: ::std::boxed::Box::new(location),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn limited_reserve_transfer_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    limited_reserve_transfer_assets,
                    DispatchError,
                > {
                    let call = limited_reserve_transfer_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
                        fee_asset_item,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn limited_teleport_assets(
                    &self,
                    dest: runtime_types::xcm::VersionedMultiLocation,
                    beneficiary: runtime_types::xcm::VersionedMultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                ) -> ::subxt::SubmittableExtrinsic<
                    'a,
                    T,
                    X,
                    A,
                    limited_teleport_assets,
                    DispatchError,
                > {
                    let call = limited_teleport_assets {
                        dest: ::std::boxed::Box::new(dest),
                        beneficiary: ::std::boxed::Box::new(beneficiary),
                        assets: ::std::boxed::Box::new(assets),
                        fee_asset_item,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Attempted(pub runtime_types::xcm::v2::traits::Outcome);
            impl ::subxt::Event for Attempted {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Attempted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Sent(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::v2::Xcm,
            );
            impl ::subxt::Event for Sent {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Sent";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UnexpectedResponse(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for UnexpectedResponse {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "UnexpectedResponse";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ResponseReady(
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::Response,
            );
            impl ::subxt::Event for ResponseReady {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseReady";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Notified(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for Notified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "Notified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NotifyOverweight(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyOverweight {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyOverweight";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NotifyDispatchError(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDispatchError {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDispatchError";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NotifyDecodeFailed(
                pub ::core::primitive::u64,
                pub ::core::primitive::u8,
                pub ::core::primitive::u8,
            );
            impl ::subxt::Event for NotifyDecodeFailed {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyDecodeFailed";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InvalidResponder(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub ::core::option::Option<runtime_types::xcm::v1::multilocation::MultiLocation>,
            );
            impl ::subxt::Event for InvalidResponder {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponder";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InvalidResponderVersion(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for InvalidResponderVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "InvalidResponderVersion";
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ResponseTaken(pub ::core::primitive::u64);
            impl ::subxt::Event for ResponseTaken {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "ResponseTaken";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct AssetsTrapped(
                pub ::subxt::sp_core::H256,
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub runtime_types::xcm::VersionedMultiAssets,
            );
            impl ::subxt::Event for AssetsTrapped {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "AssetsTrapped";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct VersionChangeNotified(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for VersionChangeNotified {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "VersionChangeNotified";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct SupportedVersionChanged(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u32,
            );
            impl ::subxt::Event for SupportedVersionChanged {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "SupportedVersionChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NotifyTargetSendFail(
                pub runtime_types::xcm::v1::multilocation::MultiLocation,
                pub ::core::primitive::u64,
                pub runtime_types::xcm::v2::traits::Error,
            );
            impl ::subxt::Event for NotifyTargetSendFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetSendFail";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NotifyTargetMigrationFail(
                pub runtime_types::xcm::VersionedMultiLocation,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for NotifyTargetMigrationFail {
                const PALLET: &'static str = "PolkadotXcm";
                const EVENT: &'static str = "NotifyTargetMigrationFail";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct QueryCounter;
            impl ::subxt::StorageEntry for QueryCounter {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "QueryCounter";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Queries(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Queries {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "Queries";
                type Value = runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct AssetTraps(pub ::subxt::sp_core::H256);
            impl ::subxt::StorageEntry for AssetTraps {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "AssetTraps";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
                    )])
                }
            }
            pub struct SafeXcmVersion;
            impl ::subxt::StorageEntry for SafeXcmVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "SafeXcmVersion";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct SupportedVersion(
                pub ::core::primitive::u32,
                pub runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for SupportedVersion {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "SupportedVersion";
                type Value = ::core::primitive::u32;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionNotifiers(
                pub ::core::primitive::u32,
                pub runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for VersionNotifiers {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "VersionNotifiers";
                type Value = ::core::primitive::u64;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionNotifyTargets(
                pub ::core::primitive::u32,
                pub runtime_types::xcm::VersionedMultiLocation,
            );
            impl ::subxt::StorageEntry for VersionNotifyTargets {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "VersionNotifyTargets";
                type Value = (
                    ::core::primitive::u64,
                    ::core::primitive::u64,
                    ::core::primitive::u32,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct VersionDiscoveryQueue;
            impl ::subxt::StorageEntry for VersionDiscoveryQueue {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "VersionDiscoveryQueue";
                type Value = runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                    runtime_types::xcm::VersionedMultiLocation,
                    ::core::primitive::u32,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct CurrentMigration;
            impl ::subxt::StorageEntry for CurrentMigration {
                const PALLET: &'static str = "PolkadotXcm";
                const STORAGE: &'static str = "CurrentMigration";
                type Value = runtime_types::pallet_xcm::pallet::VersionMigrationStage;
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
                pub async fn query_counter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u64, ::subxt::BasicError>
                {
                    let entry = QueryCounter;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn queries(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_xcm::pallet::QueryStatus<::core::primitive::u32>,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Queries(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn queries_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Queries>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn asset_traps(
                    &self,
                    _0: ::subxt::sp_core::H256,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    let entry = AssetTraps(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn asset_traps_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, AssetTraps>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn safe_xcm_version(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SafeXcmVersion;
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn supported_version(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u32>,
                    ::subxt::BasicError,
                > {
                    let entry = SupportedVersion(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn supported_version_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, SupportedVersion>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_notifiers(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<::core::primitive::u64>,
                    ::subxt::BasicError,
                > {
                    let entry = VersionNotifiers(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn version_notifiers_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VersionNotifiers>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_notify_targets(
                    &self,
                    _0: ::core::primitive::u32,
                    _1: runtime_types::xcm::VersionedMultiLocation,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = VersionNotifyTargets(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn version_notify_targets_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, VersionNotifyTargets>,
                    ::subxt::BasicError,
                > {
                    self.client.storage().iter(hash).await
                }
                pub async fn version_discovery_queue(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u32,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = VersionDiscoveryQueue;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn current_migration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_xcm::pallet::VersionMigrationStage,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = CurrentMigration;
                    self.client.storage().fetch(&entry, hash).await
                }
            }
        }
    }
    pub mod cumulus_xcm {
        use super::runtime_types;
        pub type Event = runtime_types::cumulus_pallet_xcm::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InvalidFormat(pub [::core::primitive::u8; 8usize]);
            impl ::subxt::Event for InvalidFormat {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 8usize]);
            impl ::subxt::Event for UnsupportedVersion {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ExecutedDownward(
                pub [::core::primitive::u8; 8usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::Event for ExecutedDownward {
                const PALLET: &'static str = "CumulusXcm";
                const EVENT: &'static str = "ExecutedDownward";
            }
        }
    }
    pub mod dmp_queue {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct service_overweight {
                pub index: ::core::primitive::u64,
                pub weight_limit: ::core::primitive::u64,
            }
            impl ::subxt::Call for service_overweight {
                const PALLET: &'static str = "DmpQueue";
                const FUNCTION: &'static str = "service_overweight";
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
                pub fn service_overweight(
                    &self,
                    index: ::core::primitive::u64,
                    weight_limit: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, service_overweight, DispatchError>
                {
                    let call = service_overweight {
                        index,
                        weight_limit,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::cumulus_pallet_dmp_queue::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InvalidFormat(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for InvalidFormat {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "InvalidFormat";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UnsupportedVersion(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::Event for UnsupportedVersion {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "UnsupportedVersion";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ExecutedDownward(
                pub [::core::primitive::u8; 32usize],
                pub runtime_types::xcm::v2::traits::Outcome,
            );
            impl ::subxt::Event for ExecutedDownward {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "ExecutedDownward";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct WeightExhausted(
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for WeightExhausted {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "WeightExhausted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OverweightEnqueued(
                pub [::core::primitive::u8; 32usize],
                pub ::core::primitive::u64,
                pub ::core::primitive::u64,
            );
            impl ::subxt::Event for OverweightEnqueued {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightEnqueued";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OverweightServiced(pub ::core::primitive::u64, pub ::core::primitive::u64);
            impl ::subxt::Event for OverweightServiced {
                const PALLET: &'static str = "DmpQueue";
                const EVENT: &'static str = "OverweightServiced";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Configuration;
            impl ::subxt::StorageEntry for Configuration {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "Configuration";
                type Value = runtime_types::cumulus_pallet_dmp_queue::ConfigData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct PageIndex;
            impl ::subxt::StorageEntry for PageIndex {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "PageIndex";
                type Value = runtime_types::cumulus_pallet_dmp_queue::PageIndexData;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct Pages(pub ::core::primitive::u32);
            impl ::subxt::StorageEntry for Pages {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "Pages";
                type Value = ::std::vec::Vec<(
                    ::core::primitive::u32,
                    ::std::vec::Vec<::core::primitive::u8>,
                )>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Blake2_128Concat,
                    )])
                }
            }
            pub struct Overweight(pub ::core::primitive::u64);
            impl ::subxt::StorageEntry for Overweight {
                const PALLET: &'static str = "DmpQueue";
                const STORAGE: &'static str = "Overweight";
                type Value = (
                    ::core::primitive::u32,
                    ::std::vec::Vec<::core::primitive::u8>,
                );
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
                pub async fn configuration(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_dmp_queue::ConfigData,
                    ::subxt::BasicError,
                > {
                    let entry = Configuration;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn page_index(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    runtime_types::cumulus_pallet_dmp_queue::PageIndexData,
                    ::subxt::BasicError,
                > {
                    let entry = PageIndex;
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pages(
                    &self,
                    _0: ::core::primitive::u32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::std::vec::Vec<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Pages(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn pages_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Pages>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn overweight(
                    &self,
                    _0: ::core::primitive::u64,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Overweight(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn overweight_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Overweight>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
    }
    pub mod utility {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct batch {
                pub calls: ::std::vec::Vec<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for batch {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct as_derivative {
                pub index: ::core::primitive::u16,
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for as_derivative {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "as_derivative";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct batch_all {
                pub calls: ::std::vec::Vec<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for batch_all {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "batch_all";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct dispatch_as {
                pub as_origin:
                    ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::OriginCaller>,
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for dispatch_as {
                const PALLET: &'static str = "Utility";
                const FUNCTION: &'static str = "dispatch_as";
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
                pub fn batch(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::pangolin_parachain_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, batch, DispatchError>
                {
                    let call = batch { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_derivative(
                    &self,
                    index: ::core::primitive::u16,
                    call: runtime_types::pangolin_parachain_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_derivative, DispatchError>
                {
                    let call = as_derivative {
                        index,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn batch_all(
                    &self,
                    calls: ::std::vec::Vec<runtime_types::pangolin_parachain_runtime::Call>,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, batch_all, DispatchError>
                {
                    let call = batch_all { calls };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn dispatch_as(
                    &self,
                    as_origin: runtime_types::pangolin_parachain_runtime::OriginCaller,
                    call: runtime_types::pangolin_parachain_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, dispatch_as, DispatchError>
                {
                    let call = dispatch_as {
                        as_origin: ::std::boxed::Box::new(as_origin),
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_utility::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BatchInterrupted {
                pub index: ::core::primitive::u32,
                pub error: runtime_types::sp_runtime::DispatchError,
            }
            impl ::subxt::Event for BatchInterrupted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchInterrupted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BatchCompleted;
            impl ::subxt::Event for BatchCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "BatchCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ItemCompleted;
            impl ::subxt::Event for ItemCompleted {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "ItemCompleted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct DispatchedAs {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for DispatchedAs {
                const PALLET: &'static str = "Utility";
                const EVENT: &'static str = "DispatchedAs";
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn batched_calls_limit(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[170u8, 42u8, 0u8, 0u8][..],
                    )?)
                }
            }
        }
    }
    pub mod multisig {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct as_multi_threshold_1 {
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for as_multi_threshold_1 {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi_threshold_1";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call:
                    ::subxt::WrapperKeepOpaque<runtime_types::pangolin_parachain_runtime::Call>,
                pub store_call: ::core::primitive::bool,
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct approve_as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub maybe_timepoint: ::core::option::Option<
                    runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                >,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub max_weight: ::core::primitive::u64,
            }
            impl ::subxt::Call for approve_as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "approve_as_multi";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct cancel_as_multi {
                pub threshold: ::core::primitive::u16,
                pub other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Call for cancel_as_multi {
                const PALLET: &'static str = "Multisig";
                const FUNCTION: &'static str = "cancel_as_multi";
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
                pub fn as_multi_threshold_1(
                    &self,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    call: runtime_types::pangolin_parachain_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_multi_threshold_1, DispatchError>
                {
                    let call = as_multi_threshold_1 {
                        other_signatories,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::subxt::WrapperKeepOpaque<
                        runtime_types::pangolin_parachain_runtime::Call,
                    >,
                    store_call: ::core::primitive::bool,
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, as_multi, DispatchError>
                {
                    let call = as_multi {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call,
                        store_call,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn approve_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: ::core::primitive::u64,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, approve_as_multi, DispatchError>
                {
                    let call = approve_as_multi {
                        threshold,
                        other_signatories,
                        maybe_timepoint,
                        call_hash,
                        max_weight,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn cancel_as_multi(
                    &self,
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, cancel_as_multi, DispatchError>
                {
                    let call = cancel_as_multi {
                        threshold,
                        other_signatories,
                        timepoint,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_multisig::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct NewMultisig {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for NewMultisig {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "NewMultisig";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MultisigApproval {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigApproval {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigApproval";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MultisigExecuted {
                pub approving: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for MultisigExecuted {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MultisigCancelled {
                pub cancelling: ::subxt::sp_core::crypto::AccountId32,
                pub timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                pub multisig: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: [::core::primitive::u8; 32usize],
            }
            impl ::subxt::Event for MultisigCancelled {
                const PALLET: &'static str = "Multisig";
                const EVENT: &'static str = "MultisigCancelled";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Multisigs(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub [::core::primitive::u8; 32usize],
            );
            impl ::subxt::StorageEntry for Multisigs {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Multisigs";
                type Value = runtime_types::pallet_multisig::Multisig<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                    ::subxt::sp_core::crypto::AccountId32,
                >;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![
                        ::subxt::StorageMapKey::new(&self.0, ::subxt::StorageHasher::Twox64Concat),
                        ::subxt::StorageMapKey::new(
                            &self.1,
                            ::subxt::StorageHasher::Blake2_128Concat,
                        ),
                    ])
                }
            }
            pub struct Calls(pub [::core::primitive::u8; 32usize]);
            impl ::subxt::StorageEntry for Calls {
                const PALLET: &'static str = "Multisig";
                const STORAGE: &'static str = "Calls";
                type Value = (
                    ::subxt::WrapperKeepOpaque<runtime_types::pangolin_parachain_runtime::Call>,
                    ::subxt::sp_core::crypto::AccountId32,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Identity,
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
                pub async fn multisigs(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    _1: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<
                        runtime_types::pallet_multisig::Multisig<
                            ::core::primitive::u32,
                            ::core::primitive::u128,
                            ::subxt::sp_core::crypto::AccountId32,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = Multisigs(_0, _1);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn multisigs_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Multisigs>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn calls(
                    &self,
                    _0: [::core::primitive::u8; 32usize],
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::core::option::Option<(
                        ::subxt::WrapperKeepOpaque<runtime_types::pangolin_parachain_runtime::Call>,
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    )>,
                    ::subxt::BasicError,
                > {
                    let entry = Calls(_0);
                    self.client.storage().fetch(&entry, hash).await
                }
                pub async fn calls_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Calls>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
            }
        }
        pub mod constants {
            use super::runtime_types;
            pub struct ConstantsApi;
            impl ConstantsApi {
                pub fn deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 192u8, 97u8, 200u8, 229u8, 21u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn deposit_factor(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 221u8, 14u8, 233u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_signatories(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u16, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(&mut &[100u8, 0u8][..])?)
                }
            }
        }
    }
    pub mod proxy {
        use super::runtime_types;
        pub mod calls {
            use super::runtime_types;
            type DispatchError = runtime_types::sp_runtime::DispatchError;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct proxy {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type: ::core::option::Option<
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                >,
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct add_proxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for add_proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "add_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct remove_proxy {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Call for remove_proxy {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxy";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct remove_proxies;
            impl ::subxt::Call for remove_proxies {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_proxies";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct anonymous {
                pub proxy_type:
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                pub delay: ::core::primitive::u32,
                pub index: ::core::primitive::u16,
            }
            impl ::subxt::Call for anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct kill_anonymous {
                pub spawner: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                pub index: ::core::primitive::u16,
                #[codec(compact)]
                pub height: ::core::primitive::u32,
                #[codec(compact)]
                pub ext_index: ::core::primitive::u32,
            }
            impl ::subxt::Call for kill_anonymous {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "kill_anonymous";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct announce {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for announce {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "announce";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct remove_announcement {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for remove_announcement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "remove_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct reject_announcement {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Call for reject_announcement {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "reject_announcement";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct proxy_announced {
                pub delegate: ::subxt::sp_core::crypto::AccountId32,
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub force_proxy_type: ::core::option::Option<
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                >,
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for proxy_announced {
                const PALLET: &'static str = "Proxy";
                const FUNCTION: &'static str = "proxy_announced";
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
                pub fn proxy(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                    >,
                    call: runtime_types::pangolin_parachain_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, proxy, DispatchError>
                {
                    let call = proxy {
                        real,
                        force_proxy_type,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn add_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type : runtime_types :: pangolin_parachain_runtime :: pallets :: proxy :: ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, add_proxy, DispatchError>
                {
                    let call = add_proxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxy(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type : runtime_types :: pangolin_parachain_runtime :: pallets :: proxy :: ProxyType,
                    delay: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_proxy, DispatchError>
                {
                    let call = remove_proxy {
                        delegate,
                        proxy_type,
                        delay,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_proxies(
                    &self,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_proxies, DispatchError>
                {
                    let call = remove_proxies {};
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn anonymous(
                    &self,
                    proxy_type : runtime_types :: pangolin_parachain_runtime :: pallets :: proxy :: ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, anonymous, DispatchError>
                {
                    let call = anonymous {
                        proxy_type,
                        delay,
                        index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn kill_anonymous(
                    &self,
                    spawner: ::subxt::sp_core::crypto::AccountId32,
                    proxy_type : runtime_types :: pangolin_parachain_runtime :: pallets :: proxy :: ProxyType,
                    index: ::core::primitive::u16,
                    height: ::core::primitive::u32,
                    ext_index: ::core::primitive::u32,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, kill_anonymous, DispatchError>
                {
                    let call = kill_anonymous {
                        spawner,
                        proxy_type,
                        index,
                        height,
                        ext_index,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn announce(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, announce, DispatchError>
                {
                    let call = announce { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn remove_announcement(
                    &self,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, remove_announcement, DispatchError>
                {
                    let call = remove_announcement { real, call_hash };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn reject_announcement(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    call_hash: ::subxt::sp_core::H256,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, reject_announcement, DispatchError>
                {
                    let call = reject_announcement {
                        delegate,
                        call_hash,
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn proxy_announced(
                    &self,
                    delegate: ::subxt::sp_core::crypto::AccountId32,
                    real: ::subxt::sp_core::crypto::AccountId32,
                    force_proxy_type: ::core::option::Option<
                        runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                    >,
                    call: runtime_types::pangolin_parachain_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, proxy_announced, DispatchError>
                {
                    let call = proxy_announced {
                        delegate,
                        real,
                        force_proxy_type,
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
            }
        }
        pub type Event = runtime_types::pallet_proxy::pallet::Event;
        pub mod events {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ProxyExecuted {
                pub result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for ProxyExecuted {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyExecuted";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct AnonymousCreated {
                pub anonymous: ::subxt::sp_core::crypto::AccountId32,
                pub who: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                pub disambiguation_index: ::core::primitive::u16,
            }
            impl ::subxt::Event for AnonymousCreated {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "AnonymousCreated";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Announced {
                pub real: ::subxt::sp_core::crypto::AccountId32,
                pub proxy: ::subxt::sp_core::crypto::AccountId32,
                pub call_hash: ::subxt::sp_core::H256,
            }
            impl ::subxt::Event for Announced {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "Announced";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ProxyAdded {
                pub delegator: ::subxt::sp_core::crypto::AccountId32,
                pub delegatee: ::subxt::sp_core::crypto::AccountId32,
                pub proxy_type:
                    runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                pub delay: ::core::primitive::u32,
            }
            impl ::subxt::Event for ProxyAdded {
                const PALLET: &'static str = "Proxy";
                const EVENT: &'static str = "ProxyAdded";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct Proxies(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Proxies {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Proxies";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::ProxyDefinition<
                            ::subxt::sp_core::crypto::AccountId32,
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Map(vec![::subxt::StorageMapKey::new(
                        &self.0,
                        ::subxt::StorageHasher::Twox64Concat,
                    )])
                }
            }
            pub struct Announcements(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for Announcements {
                const PALLET: &'static str = "Proxy";
                const STORAGE: &'static str = "Announcements";
                type Value = (
                    runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                        runtime_types::pallet_proxy::Announcement<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::subxt::sp_core::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    ::core::primitive::u128,
                );
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
                }                pub async fn proxies (& self , _0 : :: subxt :: sp_core :: crypto :: AccountId32 , hash : :: core :: option :: Option < T :: Hash > ,) -> :: core :: result :: Result < (runtime_types :: frame_support :: storage :: bounded_vec :: BoundedVec < runtime_types :: pallet_proxy :: ProxyDefinition < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: pangolin_parachain_runtime :: pallets :: proxy :: ProxyType , :: core :: primitive :: u32 > > , :: core :: primitive :: u128 ,) , :: subxt :: BasicError >{
                    let entry = Proxies(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn proxies_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<::subxt::KeyIter<'a, T, Proxies>, ::subxt::BasicError>
                {
                    self.client.storage().iter(hash).await
                }
                pub async fn announcements(
                    &self,
                    _0: ::subxt::sp_core::crypto::AccountId32,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    (
                        runtime_types::frame_support::storage::bounded_vec::BoundedVec<
                            runtime_types::pallet_proxy::Announcement<
                                ::subxt::sp_core::crypto::AccountId32,
                                ::subxt::sp_core::H256,
                                ::core::primitive::u32,
                            >,
                        >,
                        ::core::primitive::u128,
                    ),
                    ::subxt::BasicError,
                > {
                    let entry = Announcements(_0);
                    self.client.storage().fetch_or_default(&entry, hash).await
                }
                pub async fn announcements_iter(
                    &self,
                    hash: ::core::option::Option<T::Hash>,
                ) -> ::core::result::Result<
                    ::subxt::KeyIter<'a, T, Announcements>,
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
                pub fn proxy_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 64u8, 22u8, 50u8, 136u8, 17u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn proxy_deposit_factor(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 232u8, 83u8, 87u8, 0u8, 3u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn max_proxies(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[32u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn max_pending(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[32u8, 0u8, 0u8, 0u8][..],
                    )?)
                }
                pub fn announcement_deposit_base(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 128u8, 205u8, 117u8, 66u8, 18u8, 71u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn announcement_deposit_factor(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 208u8, 167u8, 174u8, 0u8, 6u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8,
                        ][..],
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
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
            }
            impl ::subxt::Call for sudo {
                const PALLET: &'static str = "Sudo";
                const FUNCTION: &'static str = "sudo";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct sudo_unchecked_weight {
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
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
                pub call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
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
                    call: runtime_types::pangolin_parachain_runtime::Call,
                ) -> ::subxt::SubmittableExtrinsic<'a, T, X, A, sudo, DispatchError>
                {
                    let call = sudo {
                        call: ::std::boxed::Box::new(call),
                    };
                    ::subxt::SubmittableExtrinsic::new(self.client, call)
                }
                pub fn sudo_unchecked_weight(
                    &self,
                    call: runtime_types::pangolin_parachain_runtime::Call,
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
                    call: runtime_types::pangolin_parachain_runtime::Call,
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
            pub struct Sudid {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
            impl ::subxt::Event for Sudid {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "Sudid";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct KeyChanged {
                pub old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
            }
            impl ::subxt::Event for KeyChanged {
                const PALLET: &'static str = "Sudo";
                const EVENT: &'static str = "KeyChanged";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct SudoAsDone {
                pub sudo_result:
                    ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
            }
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
                    ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    ::subxt::BasicError,
                > {
                    let entry = Key;
                    self.client.storage().fetch(&entry, hash).await
                }
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
            pub struct update_pallet_parameter { pub parameter : runtime_types :: pangolin_parachain_runtime :: bridges_message :: pangolin :: PangolinParachainToPangolinParameter , }
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
                    parameter : runtime_types :: pangolin_parachain_runtime :: bridges_message :: pangolin :: PangolinParachainToPangolinParameter,
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
            pub struct ParameterUpdated (pub runtime_types :: pangolin_parachain_runtime :: bridges_message :: pangolin :: PangolinParachainToPangolinParameter ,) ;
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
            pub struct MessageCallRejected(
                pub [::core::primitive::u8; 4usize],
                pub ([::core::primitive::u8; 4usize], ::core::primitive::u64),
            );
            impl ::subxt::Event for MessageCallRejected {
                const PALLET: &'static str = "BridgePangolinDispatch";
                const EVENT: &'static str = "MessageCallRejected";
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
    pub mod fee_market {
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
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
                const FUNCTION: &'static str = "update_relay_fee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct cancel_enrollment;
            impl ::subxt::Call for cancel_enrollment {
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
                const EVENT: &'static str = "Enroll";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UpdateLockedCollateral(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UpdateLockedCollateral {
                const PALLET: &'static str = "FeeMarket";
                const EVENT: &'static str = "UpdateLockedCollateral";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct UpdateRelayFee(
                pub ::subxt::sp_core::crypto::AccountId32,
                pub ::core::primitive::u128,
            );
            impl ::subxt::Event for UpdateRelayFee {
                const PALLET: &'static str = "FeeMarket";
                const EVENT: &'static str = "UpdateRelayFee";
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct CancelEnrollment(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::Event for CancelEnrollment {
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
                const EVENT: &'static str = "FeeMarketSlash";
            }
        }
        pub mod storage {
            use super::runtime_types;
            pub struct RelayersMap(pub ::subxt::sp_core::crypto::AccountId32);
            impl ::subxt::StorageEntry for RelayersMap {
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
                const STORAGE: &'static str = "Relayers";
                type Value = ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssignedRelayers;
            impl ::subxt::StorageEntry for AssignedRelayers {
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
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
                const PALLET: &'static str = "FeeMarket";
                const STORAGE: &'static str = "CollateralSlashProtect";
                type Value = ::core::primitive::u128;
                fn key(&self) -> ::subxt::StorageEntryKey {
                    ::subxt::StorageEntryKey::Plain
                }
            }
            pub struct AssignedRelayersNumber;
            impl ::subxt::StorageEntry for AssignedRelayersNumber {
                const PALLET: &'static str = "FeeMarket";
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
                    ::core::option::Option<
                        runtime_types::pallet_fee_market::types::Relayer<
                            ::subxt::sp_core::crypto::AccountId32,
                            ::core::primitive::u128,
                        >,
                    >,
                    ::subxt::BasicError,
                > {
                    let entry = RelayersMap(_0);
                    self.client.storage().fetch(&entry, hash).await
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
                    ::core::option::Option<::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>>,
                    ::subxt::BasicError,
                > {
                    let entry = Relayers;
                    self.client.storage().fetch(&entry, hash).await
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
                pub fn pallet_id(
                    &self,
                ) -> ::core::result::Result<
                    runtime_types::frame_support::PalletId,
                    ::subxt::BasicError,
                > {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[100u8, 97u8, 47u8, 102u8, 101u8, 101u8, 109u8, 107u8][..],
                    )?)
                }
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
                            0u8, 0u8, 220u8, 206u8, 134u8, 180u8, 42u8, 208u8, 0u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn collateral_per_order(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u128, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[
                            0u8, 0u8, 136u8, 177u8, 22u8, 175u8, 227u8, 181u8, 2u8, 0u8, 0u8, 0u8,
                            0u8, 0u8, 0u8, 0u8,
                        ][..],
                    )?)
                }
                pub fn slot(
                    &self,
                ) -> ::core::result::Result<::core::primitive::u32, ::subxt::BasicError>
                {
                    Ok(::subxt::codec::Decode::decode(
                        &mut &[88u8, 2u8, 0u8, 0u8][..],
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
    pub mod runtime_types {
        use super::runtime_types;
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
        pub mod cumulus_pallet_dmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    service_overweight {
                        index: ::core::primitive::u64,
                        weight_limit: ::core::primitive::u64,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    Unknown,
                    #[codec(index = 1)]
                    OverLimit,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    InvalidFormat([::core::primitive::u8; 32usize]),
                    #[codec(index = 1)]
                    UnsupportedVersion([::core::primitive::u8; 32usize]),
                    #[codec(index = 2)]
                    ExecutedDownward(
                        [::core::primitive::u8; 32usize],
                        runtime_types::xcm::v2::traits::Outcome,
                    ),
                    #[codec(index = 3)]
                    WeightExhausted(
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 4)]
                    OverweightEnqueued(
                        [::core::primitive::u8; 32usize],
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 5)]
                    OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
                }
            }
            #[derive(
                :: subxt :: codec :: Encode,
                :: subxt :: codec :: Decode,
                Debug,
                Clone,
                :: subxt :: codec :: CompactAs,
            )]
            pub struct ConfigData {
                pub max_individual: ::core::primitive::u64,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct PageIndexData {
                pub begin_used: ::core::primitive::u32,
                pub end_used: ::core::primitive::u32,
                pub overweight_count: ::core::primitive::u64,
            }
        }
        pub mod cumulus_pallet_parachain_system {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    // # [codec (index = 0)] set_validation_data {
                    //     data : runtime_types :: cumulus_primitives_parachain_inherent :: ParachainInherentData ,
                    // } ,
                    #[codec(index = 1)]
                    sudo_send_upward_message {
                        message: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 2)]
                    authorize_upgrade { code_hash: ::subxt::sp_core::H256 },
                    #[codec(index = 3)]
                    enact_authorized_upgrade {
                        code: ::std::vec::Vec<::core::primitive::u8>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    OverlappingUpgrades,
                    #[codec(index = 1)]
                    ProhibitedByPolkadot,
                    #[codec(index = 2)]
                    TooBig,
                    #[codec(index = 3)]
                    ValidationDataNotAvailable,
                    #[codec(index = 4)]
                    HostConfigurationNotAvailable,
                    #[codec(index = 5)]
                    NotScheduled,
                    #[codec(index = 6)]
                    NothingAuthorized,
                    #[codec(index = 7)]
                    Unauthorized,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ValidationFunctionStored,
                    #[codec(index = 1)]
                    ValidationFunctionApplied(::core::primitive::u32),
                    #[codec(index = 2)]
                    ValidationFunctionDiscarded,
                    #[codec(index = 3)]
                    UpgradeAuthorized(::subxt::sp_core::H256),
                    #[codec(index = 4)]
                    DownwardMessagesReceived(::core::primitive::u32),
                    #[codec(index = 5)]
                    DownwardMessagesProcessed(::core::primitive::u64, ::subxt::sp_core::H256),
                }
            }
            pub mod relay_state_snapshot {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct MessagingStateSnapshot {
                    pub dmq_mqc_head: ::subxt::sp_core::H256,
                    pub relay_dispatch_queue_size: (::core::primitive::u32, ::core::primitive::u32),
                    pub ingress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
                    )>,
                    pub egress_channels: ::std::vec::Vec<(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_primitives::v1::AbridgedHrmpChannel,
                    )>,
                }
            }
        }
        pub mod cumulus_pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {}
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    InvalidFormat([::core::primitive::u8; 8usize]),
                    #[codec(index = 1)]
                    UnsupportedVersion([::core::primitive::u8; 8usize]),
                    #[codec(index = 2)]
                    ExecutedDownward(
                        [::core::primitive::u8; 8usize],
                        runtime_types::xcm::v2::traits::Outcome,
                    ),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    Relay,
                    #[codec(index = 1)]
                    SiblingParachain(runtime_types::polkadot_parachain::primitives::Id),
                }
            }
        }
        pub mod cumulus_pallet_xcmp_queue {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    service_overweight {
                        index: ::core::primitive::u64,
                        weight_limit: ::core::primitive::u64,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    FailedToSend,
                    #[codec(index = 1)]
                    BadXcmOrigin,
                    #[codec(index = 2)]
                    BadXcm,
                    #[codec(index = 3)]
                    BadOverweightIndex,
                    #[codec(index = 4)]
                    WeightOverLimit,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Success(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 1)]
                    Fail(
                        ::core::option::Option<::subxt::sp_core::H256>,
                        runtime_types::xcm::v2::traits::Error,
                    ),
                    #[codec(index = 2)]
                    BadVersion(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 3)]
                    BadFormat(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 4)]
                    UpwardMessageSent(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 5)]
                    XcmpMessageSent(::core::option::Option<::subxt::sp_core::H256>),
                    #[codec(index = 6)]
                    OverweightEnqueued(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 7)]
                    OverweightServiced(::core::primitive::u64, ::core::primitive::u64),
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InboundChannelDetails {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub state: runtime_types::cumulus_pallet_xcmp_queue::InboundState,
                pub message_metadata: ::std::vec::Vec<(
                    ::core::primitive::u32,
                    runtime_types::polkadot_parachain::primitives::XcmpMessageFormat,
                )>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum InboundState {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OutboundChannelDetails {
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
                pub state: runtime_types::cumulus_pallet_xcmp_queue::OutboundState,
                pub signals_exist: ::core::primitive::bool,
                pub first_index: ::core::primitive::u16,
                pub last_index: ::core::primitive::u16,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum OutboundState {
                #[codec(index = 0)]
                Ok,
                #[codec(index = 1)]
                Suspended,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct QueueConfigData {
                pub suspend_threshold: ::core::primitive::u32,
                pub drop_threshold: ::core::primitive::u32,
                pub resume_threshold: ::core::primitive::u32,
                pub threshold_weight: ::core::primitive::u64,
                pub weight_restrict_decay: ::core::primitive::u64,
                pub xcmp_max_individual_weight: ::core::primitive::u64,
            }
        }
        pub mod cumulus_primitives_parachain_inherent {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct MessageQueueChain(pub ::subxt::sp_core::H256);
            //# error[E0277]: the trait bound `BTreeMap<primitives::Id, Vec<InboundHrmpMessage<u32>>>: Decode` is not satisfied
            //# --> D:\dev\darwinia-network\bridger\frame\assistants\pangolin-parachain-subxt\src\runtime.rs:6921:17
            //# |
            //# 6921 |                 pub horizontal_messages: ::std::collections::BTreeMap<
            //# |                 ^^^ the trait `Decode` is not implemented for `BTreeMap<primitives::Id, Vec<InboundHrmpMessage<u32>>>`
            //# |
            //# = help: the following implementations were found:
            //# <BTreeMap<K, V> as Decode>
            //# note: required by a bound in `parity_scale_codec::Decode::decode`
            //# --> d:/opt/scoop/persist/rustup/.cargo\registry\src\github.com-1ecc6299db9ec823\parity-scale-codec-2.3.1\src\codec.rs:284:15
            //# |
            //# 284  |     fn decode<I: Input>(input: &mut I) -> Result<Self, Error>;
            //# |                  ^^^^^ required by this bound in `parity_scale_codec::Decode::decode`
            // #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            // pub struct ParachainInherentData {
            //     pub validation_data:
            //         runtime_types::polkadot_primitives::v1::PersistedValidationData<
            //             ::subxt::sp_core::H256,
            //             ::core::primitive::u32,
            //         >,
            //     pub relay_chain_state: runtime_types::sp_trie::storage_proof::StorageProof,
            //     pub downward_messages: ::std::vec::Vec<
            //         runtime_types::polkadot_core_primitives::InboundDownwardMessage<
            //             ::core::primitive::u32,
            //         >,
            //     >,
            //     pub horizontal_messages: ::std::collections::BTreeMap<
            //         runtime_types::polkadot_parachain::primitives::Id,
            //         ::std::vec::Vec<
            //             runtime_types::polkadot_core_primitives::InboundHrmpMessage<
            //                 ::core::primitive::u32,
            //             >,
            //         >,
            //     >,
            // }
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
            pub struct Precommit<_0, _1> {
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
        pub mod frame_support {
            use super::runtime_types;
            pub mod storage {
                use super::runtime_types;
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
                    pub struct WrapperKeepOpaque<_0>(
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
                pub mod check_non_zero_sender {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct CheckNonZeroSender;
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
                    set_storage {
                        items: ::std::vec::Vec<(
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        )>,
                    },
                    #[codec(index = 6)]
                    kill_storage {
                        keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                    },
                    #[codec(index = 7)]
                    kill_prefix {
                        prefix: ::std::vec::Vec<::core::primitive::u8>,
                        subkeys: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
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
                    #[codec(index = 5)]
                    CallFiltered,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ExtrinsicSuccess {
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 1)]
                    ExtrinsicFailed {
                        dispatch_error: runtime_types::sp_runtime::DispatchError,
                        dispatch_info: runtime_types::frame_support::weights::DispatchInfo,
                    },
                    #[codec(index = 2)]
                    CodeUpdated,
                    #[codec(index = 3)]
                    NewAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 4)]
                    KilledAccount {
                        account: ::subxt::sp_core::crypto::AccountId32,
                    },
                    #[codec(index = 5)]
                    Remarked {
                        sender: ::subxt::sp_core::crypto::AccountId32,
                        hash: ::subxt::sp_core::H256,
                    },
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
        pub mod pallet_balances {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
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
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Endowed {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        free_balance: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    DustLost {
                        account: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    Transfer {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    BalanceSet {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        free: ::core::primitive::u128,
                        reserved: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    Reserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    Unreserved {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 6)]
                    ReserveRepatriated {
                        from: ::subxt::sp_core::crypto::AccountId32,
                        to: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        destination_status:
                            runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                    },
                    #[codec(index = 7)]
                    Deposit {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    Withdraw {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    Slashed {
                        who: ::subxt::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub misc_frozen: _0,
                pub fee_frozen: _0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::Reasons,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Releases {
                #[codec(index = 0)]
                V1_0_0,
                #[codec(index = 1)]
                V2_0_0,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
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
                    MessageCallRejected(
                        [::core::primitive::u8; 4usize],
                        ([::core::primitive::u8; 4usize], ::core::primitive::u64),
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
                    # [codec (index = 0)] set_owner { new_owner : :: core :: option :: Option < :: subxt :: sp_core :: crypto :: AccountId32 > , } , # [codec (index = 1)] set_operating_mode { operating_mode : runtime_types :: bp_messages :: OperatingMode , } , # [codec (index = 2)] update_pallet_parameter { parameter : runtime_types :: pangolin_parachain_runtime :: bridges_message :: pangolin :: PangolinParachainToPangolinParameter , } , # [codec (index = 3)] send_message { lane_id : [:: core :: primitive :: u8 ; 4usize] , payload : runtime_types :: bp_message_dispatch :: MessagePayload < :: subxt :: sp_core :: crypto :: AccountId32 , runtime_types :: sp_runtime :: MultiSigner , runtime_types :: sp_runtime :: MultiSignature , :: std :: vec :: Vec < :: core :: primitive :: u8 > > , delivery_and_dispatch_fee : :: core :: primitive :: u128 , } , # [codec (index = 4)] increase_message_fee { lane_id : [:: core :: primitive :: u8 ; 4usize] , nonce : :: core :: primitive :: u64 , additional_fee : :: core :: primitive :: u128 , } , # [codec (index = 5)] receive_messages_proof { relayer_id_at_bridged_chain : :: subxt :: sp_core :: crypto :: AccountId32 , proof : runtime_types :: bridge_runtime_common :: messages :: target :: FromBridgedChainMessagesProof < :: subxt :: sp_core :: H256 > , messages_count : :: core :: primitive :: u32 , dispatch_weight : :: core :: primitive :: u64 , } , # [codec (index = 6)] receive_messages_delivery_proof { proof : runtime_types :: bridge_runtime_common :: messages :: source :: FromBridgedChainMessagesDeliveryProof < :: subxt :: sp_core :: H256 > , relayers_state : runtime_types :: bp_messages :: UnrewardedRelayersState , } , }
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
                    # [codec (index = 0)] ParameterUpdated (runtime_types :: pangolin_parachain_runtime :: bridges_message :: pangolin :: PangolinParachainToPangolinParameter ,) , # [codec (index = 1)] MessageAccepted ([:: core :: primitive :: u8 ; 4usize] , :: core :: primitive :: u64 ,) , # [codec (index = 2)] MessagesDelivered ([:: core :: primitive :: u8 ; 4usize] , runtime_types :: bp_messages :: DeliveredMessages ,) , }
            }
        }
        pub mod pallet_collator_selection {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    set_invulnerables {
                        new: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 1)]
                    set_desired_candidates { max: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    set_candidacy_bond { bond: ::core::primitive::u128 },
                    #[codec(index = 3)]
                    register_as_candidate,
                    #[codec(index = 4)]
                    leave_intent,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct CandidateInfo<_0, _1> {
                    pub who: _0,
                    pub deposit: _1,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCandidates,
                    #[codec(index = 1)]
                    TooFewCandidates,
                    #[codec(index = 2)]
                    Unknown,
                    #[codec(index = 3)]
                    Permission,
                    #[codec(index = 4)]
                    AlreadyCandidate,
                    #[codec(index = 5)]
                    NotCandidate,
                    #[codec(index = 6)]
                    AlreadyInvulnerable,
                    #[codec(index = 7)]
                    NoAssociatedValidatorId,
                    #[codec(index = 8)]
                    ValidatorNotRegistered,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewInvulnerables(::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>),
                    #[codec(index = 1)]
                    NewDesiredCandidates(::core::primitive::u32),
                    #[codec(index = 2)]
                    NewCandidacyBond(::core::primitive::u128),
                    #[codec(index = 3)]
                    CandidateAdded(
                        ::subxt::sp_core::crypto::AccountId32,
                        ::core::primitive::u128,
                    ),
                    #[codec(index = 4)]
                    CandidateRemoved(::subxt::sp_core::crypto::AccountId32),
                }
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
                    StillHasOrdersNotConfirmed,
                    #[codec(index = 4)]
                    RelayFeeTooLow,
                    #[codec(index = 5)]
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
        pub mod pallet_multisig {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    as_multi_threshold_1 {
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call: ::subxt::WrapperKeepOpaque<
                            runtime_types::pangolin_parachain_runtime::Call,
                        >,
                        store_call: ::core::primitive::bool,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 2)]
                    approve_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        maybe_timepoint: ::core::option::Option<
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        >,
                        call_hash: [::core::primitive::u8; 32usize],
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    cancel_as_multi {
                        threshold: ::core::primitive::u16,
                        other_signatories: ::std::vec::Vec<::subxt::sp_core::crypto::AccountId32>,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    MinimumThreshold,
                    #[codec(index = 1)]
                    AlreadyApproved,
                    #[codec(index = 2)]
                    NoApprovalsNeeded,
                    #[codec(index = 3)]
                    TooFewSignatories,
                    #[codec(index = 4)]
                    TooManySignatories,
                    #[codec(index = 5)]
                    SignatoriesOutOfOrder,
                    #[codec(index = 6)]
                    SenderInSignatories,
                    #[codec(index = 7)]
                    NotFound,
                    #[codec(index = 8)]
                    NotOwner,
                    #[codec(index = 9)]
                    NoTimepoint,
                    #[codec(index = 10)]
                    WrongTimepoint,
                    #[codec(index = 11)]
                    UnexpectedTimepoint,
                    #[codec(index = 12)]
                    MaxWeightTooLow,
                    #[codec(index = 13)]
                    AlreadyStored,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    NewMultisig {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 1)]
                    MultisigApproval {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    MultisigExecuted {
                        approving: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 3)]
                    MultisigCancelled {
                        cancelling: ::subxt::sp_core::crypto::AccountId32,
                        timepoint:
                            runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                        multisig: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: [::core::primitive::u8; 32usize],
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Multisig<_0, _1, _2> {
                pub when: runtime_types::pallet_multisig::Timepoint<_0>,
                pub deposit: _1,
                pub depositor: _2,
                pub approvals: ::std::vec::Vec<_2>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Timepoint<_0> {
                pub height: _0,
                pub index: _0,
            }
        }
        pub mod pallet_proxy {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    proxy {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type: ::core::option::Option<
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        >,
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    add_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type:
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    remove_proxy {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type:
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    remove_proxies,
                    #[codec(index = 4)]
                    anonymous {
                        proxy_type:
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        delay: ::core::primitive::u32,
                        index: ::core::primitive::u16,
                    },
                    #[codec(index = 5)]
                    kill_anonymous {
                        spawner: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type:
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        index: ::core::primitive::u16,
                        #[codec(compact)]
                        height: ::core::primitive::u32,
                        #[codec(compact)]
                        ext_index: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    announce {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 7)]
                    remove_announcement {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 8)]
                    reject_announcement {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 9)]
                    proxy_announced {
                        delegate: ::subxt::sp_core::crypto::AccountId32,
                        real: ::subxt::sp_core::crypto::AccountId32,
                        force_proxy_type: ::core::option::Option<
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        >,
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    TooMany,
                    #[codec(index = 1)]
                    NotFound,
                    #[codec(index = 2)]
                    NotProxy,
                    #[codec(index = 3)]
                    Unproxyable,
                    #[codec(index = 4)]
                    Duplicate,
                    #[codec(index = 5)]
                    NoPermission,
                    #[codec(index = 6)]
                    Unannounced,
                    #[codec(index = 7)]
                    NoSelfProxy,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    ProxyExecuted {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    AnonymousCreated {
                        anonymous: ::subxt::sp_core::crypto::AccountId32,
                        who: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type:
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        disambiguation_index: ::core::primitive::u16,
                    },
                    #[codec(index = 2)]
                    Announced {
                        real: ::subxt::sp_core::crypto::AccountId32,
                        proxy: ::subxt::sp_core::crypto::AccountId32,
                        call_hash: ::subxt::sp_core::H256,
                    },
                    #[codec(index = 3)]
                    ProxyAdded {
                        delegator: ::subxt::sp_core::crypto::AccountId32,
                        delegatee: ::subxt::sp_core::crypto::AccountId32,
                        proxy_type:
                            runtime_types::pangolin_parachain_runtime::pallets::proxy::ProxyType,
                        delay: ::core::primitive::u32,
                    },
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Announcement<_0, _1, _2> {
                pub real: _0,
                pub call_hash: _1,
                pub height: _2,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct ProxyDefinition<_0, _1, _2> {
                pub delegate: _0,
                pub proxy_type: _1,
                pub delay: _2,
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
                    # [codec (index = 0)] set_keys { keys : runtime_types :: pangolin_parachain_runtime :: pallets :: session :: SessionKeys , proof : :: std :: vec :: Vec < :: core :: primitive :: u8 > , } , # [codec (index = 1)] purge_keys , }
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
                    NewSession {
                        session_index: ::core::primitive::u32,
                    },
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
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    sudo_unchecked_weight {
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
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
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
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
                    Sudid {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 1)]
                    KeyChanged {
                        old_sudoer: ::core::option::Option<::subxt::sp_core::crypto::AccountId32>,
                    },
                    #[codec(index = 2)]
                    SudoAsDone {
                        sudo_result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
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
        pub mod pallet_utility {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    batch {
                        calls: ::std::vec::Vec<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                    #[codec(index = 1)]
                    as_derivative {
                        index: ::core::primitive::u16,
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                    #[codec(index = 2)]
                    batch_all {
                        calls: ::std::vec::Vec<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                    #[codec(index = 3)]
                    dispatch_as {
                        as_origin: ::std::boxed::Box<
                            runtime_types::pangolin_parachain_runtime::OriginCaller,
                        >,
                        call: ::std::boxed::Box<runtime_types::pangolin_parachain_runtime::Call>,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    TooManyCalls,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    BatchInterrupted {
                        index: ::core::primitive::u32,
                        error: runtime_types::sp_runtime::DispatchError,
                    },
                    #[codec(index = 1)]
                    BatchCompleted,
                    #[codec(index = 2)]
                    ItemCompleted,
                    #[codec(index = 3)]
                    DispatchedAs {
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                }
            }
        }
        pub mod pallet_xcm {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Call {
                    #[codec(index = 0)]
                    send {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                    },
                    #[codec(index = 1)]
                    teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    execute {
                        message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    force_xcm_version {
                        location:
                            ::std::boxed::Box<runtime_types::xcm::v1::multilocation::MultiLocation>,
                        xcm_version: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    force_default_xcm_version {
                        maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 6)]
                    force_subscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 7)]
                    force_unsubscribe_version_notify {
                        location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    },
                    #[codec(index = 8)]
                    limited_reserve_transfer_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 9)]
                    limited_teleport_assets {
                        dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                        assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                        fee_asset_item: ::core::primitive::u32,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Error {
                    #[codec(index = 0)]
                    Unreachable,
                    #[codec(index = 1)]
                    SendFailure,
                    #[codec(index = 2)]
                    Filtered,
                    #[codec(index = 3)]
                    UnweighableMessage,
                    #[codec(index = 4)]
                    DestinationNotInvertible,
                    #[codec(index = 5)]
                    Empty,
                    #[codec(index = 6)]
                    CannotReanchor,
                    #[codec(index = 7)]
                    TooManyAssets,
                    #[codec(index = 8)]
                    InvalidOrigin,
                    #[codec(index = 9)]
                    BadVersion,
                    #[codec(index = 10)]
                    BadLocation,
                    #[codec(index = 11)]
                    NoSubscription,
                    #[codec(index = 12)]
                    AlreadySubscribed,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Event {
                    #[codec(index = 0)]
                    Attempted(runtime_types::xcm::v2::traits::Outcome),
                    #[codec(index = 1)]
                    Sent(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::v2::Xcm,
                    ),
                    #[codec(index = 2)]
                    UnexpectedResponse(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 3)]
                    ResponseReady(::core::primitive::u64, runtime_types::xcm::v2::Response),
                    #[codec(index = 4)]
                    Notified(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 5)]
                    NotifyOverweight(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                        ::core::primitive::u64,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    NotifyDispatchError(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 7)]
                    NotifyDecodeFailed(
                        ::core::primitive::u64,
                        ::core::primitive::u8,
                        ::core::primitive::u8,
                    ),
                    #[codec(index = 8)]
                    InvalidResponder(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        ::core::option::Option<
                            runtime_types::xcm::v1::multilocation::MultiLocation,
                        >,
                    ),
                    #[codec(index = 9)]
                    InvalidResponderVersion(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 10)]
                    ResponseTaken(::core::primitive::u64),
                    #[codec(index = 11)]
                    AssetsTrapped(
                        ::subxt::sp_core::H256,
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        runtime_types::xcm::VersionedMultiAssets,
                    ),
                    #[codec(index = 12)]
                    VersionChangeNotified(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 13)]
                    SupportedVersionChanged(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 14)]
                    NotifyTargetSendFail(
                        runtime_types::xcm::v1::multilocation::MultiLocation,
                        ::core::primitive::u64,
                        runtime_types::xcm::v2::traits::Error,
                    ),
                    #[codec(index = 15)]
                    NotifyTargetMigrationFail(
                        runtime_types::xcm::VersionedMultiLocation,
                        ::core::primitive::u64,
                    ),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Origin {
                    #[codec(index = 0)]
                    Xcm(runtime_types::xcm::v1::multilocation::MultiLocation),
                    #[codec(index = 1)]
                    Response(runtime_types::xcm::v1::multilocation::MultiLocation),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum QueryStatus<_0> {
                    #[codec(index = 0)]
                    Pending {
                        responder: runtime_types::xcm::VersionedMultiLocation,
                        maybe_notify:
                            ::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
                        timeout: _0,
                    },
                    #[codec(index = 1)]
                    VersionNotifier {
                        origin: runtime_types::xcm::VersionedMultiLocation,
                        is_active: ::core::primitive::bool,
                    },
                    #[codec(index = 2)]
                    Ready {
                        response: runtime_types::xcm::VersionedResponse,
                        at: _0,
                    },
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum VersionMigrationStage {
                    #[codec(index = 0)]
                    MigrateSupportedVersion,
                    #[codec(index = 1)]
                    MigrateVersionNotifiers,
                    #[codec(index = 2)]
                    NotifyCurrentTargets(
                        ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                    ),
                    #[codec(index = 3)]
                    MigrateAndNotifyOldTargets,
                }
            }
        }
        pub mod pangolin_parachain_runtime {
            use super::runtime_types;
            pub mod bridges_message {
                use super::runtime_types;
                pub mod pangolin {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum PangolinParachainToPangolinParameter {
                        #[codec(index = 0)]
                        PangolinToPangolinParachainConversionRate(
                            runtime_types::sp_arithmetic::fixed_point::FixedU128,
                        ),
                    }
                }
            }
            pub mod pallets {
                use super::runtime_types;
                pub mod proxy {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum ProxyType {
                        #[codec(index = 0)]
                        Any,
                        #[codec(index = 1)]
                        NonTransfer,
                        #[codec(index = 2)]
                        CancelProxy,
                        #[codec(index = 3)]
                        Collator,
                    }
                }
                pub mod session {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct SessionKeys {
                        pub aura: runtime_types::sp_consensus_aura::sr25519::app_sr25519::Public,
                    }
                }
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Call {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Call),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Call),
                #[codec(index = 3)]
                Timestamp(runtime_types::pallet_timestamp::pallet::Call),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Call),
                #[codec(index = 7)]
                Authorship(runtime_types::pallet_authorship::pallet::Call),
                #[codec(index = 8)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Call),
                #[codec(index = 9)]
                Session(runtime_types::pallet_session::pallet::Call),
                #[codec(index = 12)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Call),
                #[codec(index = 13)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Call),
                #[codec(index = 15)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Call),
                #[codec(index = 16)]
                Utility(runtime_types::pallet_utility::pallet::Call),
                #[codec(index = 17)]
                Multisig(runtime_types::pallet_multisig::pallet::Call),
                #[codec(index = 18)]
                Proxy(runtime_types::pallet_proxy::pallet::Call),
                #[codec(index = 19)]
                Sudo(runtime_types::pallet_sudo::pallet::Call),
                #[codec(index = 20)]
                BridgePangolinGrandpa(runtime_types::pallet_bridge_grandpa::pallet::Call),
                #[codec(index = 21)]
                BridgePangolinMessages(runtime_types::pallet_bridge_messages::pallet::Call),
                #[codec(index = 23)]
                FeeMarket(runtime_types::pallet_fee_market::pallet::Call),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum Event {
                #[codec(index = 0)]
                System(runtime_types::frame_system::pallet::Event),
                #[codec(index = 1)]
                ParachainSystem(runtime_types::cumulus_pallet_parachain_system::pallet::Event),
                #[codec(index = 5)]
                Balances(runtime_types::pallet_balances::pallet::Event),
                #[codec(index = 8)]
                CollatorSelection(runtime_types::pallet_collator_selection::pallet::Event),
                #[codec(index = 9)]
                Session(runtime_types::pallet_session::pallet::Event),
                #[codec(index = 12)]
                XcmpQueue(runtime_types::cumulus_pallet_xcmp_queue::pallet::Event),
                #[codec(index = 13)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Event),
                #[codec(index = 14)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Event),
                #[codec(index = 15)]
                DmpQueue(runtime_types::cumulus_pallet_dmp_queue::pallet::Event),
                #[codec(index = 16)]
                Utility(runtime_types::pallet_utility::pallet::Event),
                #[codec(index = 17)]
                Multisig(runtime_types::pallet_multisig::pallet::Event),
                #[codec(index = 18)]
                Proxy(runtime_types::pallet_proxy::pallet::Event),
                #[codec(index = 19)]
                Sudo(runtime_types::pallet_sudo::pallet::Event),
                #[codec(index = 21)]
                BridgePangolinMessages(runtime_types::pallet_bridge_messages::pallet::Event),
                #[codec(index = 22)]
                BridgePangolinDispatch(runtime_types::pallet_bridge_dispatch::pallet::Event),
                #[codec(index = 23)]
                FeeMarket(runtime_types::pallet_fee_market::pallet::Event),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum OriginCaller {
                #[codec(index = 0)]
                system(
                    runtime_types::frame_system::RawOrigin<::subxt::sp_core::crypto::AccountId32>,
                ),
                #[codec(index = 13)]
                PolkadotXcm(runtime_types::pallet_xcm::pallet::Origin),
                #[codec(index = 14)]
                CumulusXcm(runtime_types::cumulus_pallet_xcm::pallet::Origin),
                #[codec(index = 3)]
                Void(runtime_types::sp_core::Void),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct Runtime;
        }
        pub mod polkadot_core_primitives {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InboundDownwardMessage<_0> {
                pub sent_at: _0,
                pub msg: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct InboundHrmpMessage<_0> {
                pub sent_at: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct OutboundHrmpMessage<_0> {
                pub recipient: _0,
                pub data: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod polkadot_parachain {
            use super::runtime_types;
            pub mod primitives {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: codec :: Encode,
                    :: subxt :: codec :: Decode,
                    Debug,
                    Clone,
                    :: subxt :: codec :: CompactAs,
                )]
                pub struct Id(pub ::core::primitive::u32);
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum XcmpMessageFormat {
                    #[codec(index = 0)]
                    ConcatenatedVersionedXcm,
                    #[codec(index = 1)]
                    ConcatenatedEncodedBlob,
                    #[codec(index = 2)]
                    Signals,
                }
            }
        }
        pub mod polkadot_primitives {
            use super::runtime_types;
            pub mod v1 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct AbridgedHostConfiguration {
                    pub max_code_size: ::core::primitive::u32,
                    pub max_head_data_size: ::core::primitive::u32,
                    pub max_upward_queue_count: ::core::primitive::u32,
                    pub max_upward_queue_size: ::core::primitive::u32,
                    pub max_upward_message_size: ::core::primitive::u32,
                    pub max_upward_message_num_per_candidate: ::core::primitive::u32,
                    pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
                    pub validation_upgrade_cooldown: ::core::primitive::u32,
                    pub validation_upgrade_delay: ::core::primitive::u32,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct AbridgedHrmpChannel {
                    pub max_capacity: ::core::primitive::u32,
                    pub max_total_size: ::core::primitive::u32,
                    pub max_message_size: ::core::primitive::u32,
                    pub msg_count: ::core::primitive::u32,
                    pub total_size: ::core::primitive::u32,
                    pub mqc_head: ::core::option::Option<::subxt::sp_core::H256>,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct PersistedValidationData<_0, _1> {
                    pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    pub relay_parent_number: _1,
                    pub relay_parent_storage_root: _0,
                    pub max_pov_size: _1,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum UpgradeRestriction {
                    #[codec(index = 0)]
                    Present,
                }
            }
        }
        pub mod primitive_types {
            use super::runtime_types;
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub struct H256(pub [::core::primitive::u8; 32usize]);
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
                pub struct Perbill(pub ::core::primitive::u32);
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
        pub mod sp_consensus_aura {
            use super::runtime_types;
            pub mod sr25519 {
                use super::runtime_types;
                pub mod app_sr25519 {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                }
            }
        }
        pub mod sp_consensus_slots {
            use super::runtime_types;
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
                    pub struct Digest {
                        pub logs:
                            ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum DigestItem {
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
                        pub digest: runtime_types::sp_runtime::generic::digest::Digest,
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
                TooManyConsumers,
                #[codec(index = 7)]
                Token(runtime_types::sp_runtime::TokenError),
                #[codec(index = 8)]
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
        pub mod sp_trie {
            use super::runtime_types;
            pub mod storage_proof {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct StorageProof {
                    pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
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
                pub state_version: ::core::primitive::u8,
            }
        }
        pub mod xcm {
            use super::runtime_types;
            pub mod double_encoded {
                use super::runtime_types;
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct DoubleEncoded {
                    pub encoded: ::std::vec::Vec<::core::primitive::u8>,
                }
            }
            pub mod v0 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum BodyId {
                        #[codec(index = 0)]
                        Unit,
                        #[codec(index = 1)]
                        Named(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 2)]
                        Index(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 3)]
                        Executive,
                        #[codec(index = 4)]
                        Technical,
                        #[codec(index = 5)]
                        Legislative,
                        #[codec(index = 6)]
                        Judicial,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum BodyPart {
                        #[codec(index = 0)]
                        Voice,
                        #[codec(index = 1)]
                        Members {
                            #[codec(compact)]
                            count: ::core::primitive::u32,
                        },
                        #[codec(index = 2)]
                        Fraction {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 3)]
                        AtLeastProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                        #[codec(index = 4)]
                        MoreThanProportion {
                            #[codec(compact)]
                            nom: ::core::primitive::u32,
                            #[codec(compact)]
                            denom: ::core::primitive::u32,
                        },
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parent,
                        #[codec(index = 1)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 2)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 3)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 4)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 5)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 6)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 7)]
                        GeneralKey(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 8)]
                        OnlyChild,
                        #[codec(index = 9)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum NetworkId {
                        #[codec(index = 0)]
                        Any,
                        #[codec(index = 1)]
                        Named(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 2)]
                        Polkadot,
                        #[codec(index = 3)]
                        Kusama,
                    }
                }
                pub mod multi_asset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum MultiAsset {
                        #[codec(index = 0)]
                        None,
                        #[codec(index = 1)]
                        All,
                        #[codec(index = 2)]
                        AllFungible,
                        #[codec(index = 3)]
                        AllNonFungible,
                        #[codec(index = 4)]
                        AllAbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 5)]
                        AllAbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                        },
                        #[codec(index = 6)]
                        AllConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 7)]
                        AllConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 8)]
                        AbstractFungible {
                            id: ::std::vec::Vec<::core::primitive::u8>,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 9)]
                        AbstractNonFungible {
                            class: ::std::vec::Vec<::core::primitive::u8>,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                        #[codec(index = 10)]
                        ConcreteFungible {
                            id: runtime_types::xcm::v0::multi_location::MultiLocation,
                            #[codec(compact)]
                            amount: ::core::primitive::u128,
                        },
                        #[codec(index = 11)]
                        ConcreteNonFungible {
                            class: runtime_types::xcm::v0::multi_location::MultiLocation,
                            instance: runtime_types::xcm::v1::multiasset::AssetInstance,
                        },
                    }
                }
                pub mod multi_location {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum MultiLocation {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v0::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                            runtime_types::xcm::v0::junction::Junction,
                        ),
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Order {
                        #[codec(index = 0)]
                        Null,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            receive:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            reserve: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                            assets:
                                ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v0::multi_asset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            xcm: ::std::vec::Vec<runtime_types::xcm::v0::Xcm>,
                        },
                    }
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum OriginKind {
                    #[codec(index = 0)]
                    Native,
                    #[codec(index = 1)]
                    SovereignAccount,
                    #[codec(index = 2)]
                    Superuser,
                    #[codec(index = 3)]
                    Xcm,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposit {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 2)]
                    TeleportAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v0::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: ::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>,
                        dest: runtime_types::xcm::v0::multi_location::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v0::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v0::multi_location::MultiLocation,
                        message: ::std::boxed::Box<runtime_types::xcm::v0::Xcm>,
                    },
                }
            }
            pub mod v1 {
                use super::runtime_types;
                pub mod junction {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Junction {
                        #[codec(index = 0)]
                        Parachain(#[codec(compact)] ::core::primitive::u32),
                        #[codec(index = 1)]
                        AccountId32 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            id: [::core::primitive::u8; 32usize],
                        },
                        #[codec(index = 2)]
                        AccountIndex64 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            #[codec(compact)]
                            index: ::core::primitive::u64,
                        },
                        #[codec(index = 3)]
                        AccountKey20 {
                            network: runtime_types::xcm::v0::junction::NetworkId,
                            key: [::core::primitive::u8; 20usize],
                        },
                        #[codec(index = 4)]
                        PalletInstance(::core::primitive::u8),
                        #[codec(index = 5)]
                        GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 6)]
                        GeneralKey(::std::vec::Vec<::core::primitive::u8>),
                        #[codec(index = 7)]
                        OnlyChild,
                        #[codec(index = 8)]
                        Plurality {
                            id: runtime_types::xcm::v0::junction::BodyId,
                            part: runtime_types::xcm::v0::junction::BodyPart,
                        },
                    }
                }
                pub mod multiasset {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum AssetId {
                        #[codec(index = 0)]
                        Concrete(runtime_types::xcm::v1::multilocation::MultiLocation),
                        #[codec(index = 1)]
                        Abstract(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum AssetInstance {
                        #[codec(index = 0)]
                        Undefined,
                        #[codec(index = 1)]
                        Index(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 2)]
                        Array4([::core::primitive::u8; 4usize]),
                        #[codec(index = 3)]
                        Array8([::core::primitive::u8; 8usize]),
                        #[codec(index = 4)]
                        Array16([::core::primitive::u8; 16usize]),
                        #[codec(index = 5)]
                        Array32([::core::primitive::u8; 32usize]),
                        #[codec(index = 6)]
                        Blob(::std::vec::Vec<::core::primitive::u8>),
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Fungibility {
                        #[codec(index = 0)]
                        Fungible(#[codec(compact)] ::core::primitive::u128),
                        #[codec(index = 1)]
                        NonFungible(runtime_types::xcm::v1::multiasset::AssetInstance),
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct MultiAsset {
                        pub id: runtime_types::xcm::v1::multiasset::AssetId,
                        pub fun: runtime_types::xcm::v1::multiasset::Fungibility,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum MultiAssetFilter {
                        #[codec(index = 0)]
                        Definite(runtime_types::xcm::v1::multiasset::MultiAssets),
                        #[codec(index = 1)]
                        Wild(runtime_types::xcm::v1::multiasset::WildMultiAsset),
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct MultiAssets(
                        pub ::std::vec::Vec<runtime_types::xcm::v1::multiasset::MultiAsset>,
                    );
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum WildFungibility {
                        #[codec(index = 0)]
                        Fungible,
                        #[codec(index = 1)]
                        NonFungible,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum WildMultiAsset {
                        #[codec(index = 0)]
                        All,
                        #[codec(index = 1)]
                        AllOf {
                            id: runtime_types::xcm::v1::multiasset::AssetId,
                            fun: runtime_types::xcm::v1::multiasset::WildFungibility,
                        },
                    }
                }
                pub mod multilocation {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Junctions {
                        #[codec(index = 0)]
                        Here,
                        #[codec(index = 1)]
                        X1(runtime_types::xcm::v1::junction::Junction),
                        #[codec(index = 2)]
                        X2(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 3)]
                        X3(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 4)]
                        X4(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 5)]
                        X5(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 6)]
                        X6(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 7)]
                        X7(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                        #[codec(index = 8)]
                        X8(
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                            runtime_types::xcm::v1::junction::Junction,
                        ),
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub struct MultiLocation {
                        pub parents: ::core::primitive::u8,
                        pub interior: runtime_types::xcm::v1::multilocation::Junctions,
                    }
                }
                pub mod order {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Order {
                        #[codec(index = 0)]
                        Noop,
                        #[codec(index = 1)]
                        DepositAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                        },
                        #[codec(index = 2)]
                        DepositReserveAsset {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            max_assets: ::core::primitive::u32,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 3)]
                        ExchangeAsset {
                            give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                        },
                        #[codec(index = 4)]
                        InitiateReserveWithdraw {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 5)]
                        InitiateTeleport {
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                        },
                        #[codec(index = 6)]
                        QueryHolding {
                            #[codec(compact)]
                            query_id: ::core::primitive::u64,
                            dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                            assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        },
                        #[codec(index = 7)]
                        BuyExecution {
                            fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                            weight: ::core::primitive::u64,
                            debt: ::core::primitive::u64,
                            halt_on_error: ::core::primitive::bool,
                            instructions: ::std::vec::Vec<runtime_types::xcm::v1::Xcm>,
                        },
                    }
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    Version(::core::primitive::u32),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Xcm {
                    #[codec(index = 0)]
                    WithdrawAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 1)]
                    ReserveAssetDeposited {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v1::Response,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        effects: ::std::vec::Vec<runtime_types::xcm::v1::order::Order>,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    RelayedFrom {
                        who: runtime_types::xcm::v1::multilocation::Junctions,
                        message: ::std::boxed::Box<runtime_types::xcm::v1::Xcm>,
                    },
                    #[codec(index = 11)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 12)]
                    UnsubscribeVersion,
                }
            }
            pub mod v2 {
                use super::runtime_types;
                pub mod traits {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Error {
                        #[codec(index = 0)]
                        Overflow,
                        #[codec(index = 1)]
                        Unimplemented,
                        #[codec(index = 2)]
                        UntrustedReserveLocation,
                        #[codec(index = 3)]
                        UntrustedTeleportLocation,
                        #[codec(index = 4)]
                        MultiLocationFull,
                        #[codec(index = 5)]
                        MultiLocationNotInvertible,
                        #[codec(index = 6)]
                        BadOrigin,
                        #[codec(index = 7)]
                        InvalidLocation,
                        #[codec(index = 8)]
                        AssetNotFound,
                        #[codec(index = 9)]
                        FailedToTransactAsset,
                        #[codec(index = 10)]
                        NotWithdrawable,
                        #[codec(index = 11)]
                        LocationCannotHold,
                        #[codec(index = 12)]
                        ExceedsMaxMessageSize,
                        #[codec(index = 13)]
                        DestinationUnsupported,
                        #[codec(index = 14)]
                        Transport,
                        #[codec(index = 15)]
                        Unroutable,
                        #[codec(index = 16)]
                        UnknownClaim,
                        #[codec(index = 17)]
                        FailedToDecode,
                        #[codec(index = 18)]
                        MaxWeightInvalid,
                        #[codec(index = 19)]
                        NotHoldingFees,
                        #[codec(index = 20)]
                        TooExpensive,
                        #[codec(index = 21)]
                        Trap(::core::primitive::u64),
                        #[codec(index = 22)]
                        UnhandledXcmVersion,
                        #[codec(index = 23)]
                        WeightLimitReached(::core::primitive::u64),
                        #[codec(index = 24)]
                        Barrier,
                        #[codec(index = 25)]
                        WeightNotComputable,
                    }
                    #[derive(
                        :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                    )]
                    pub enum Outcome {
                        #[codec(index = 0)]
                        Complete(::core::primitive::u64),
                        #[codec(index = 1)]
                        Incomplete(
                            ::core::primitive::u64,
                            runtime_types::xcm::v2::traits::Error,
                        ),
                        #[codec(index = 2)]
                        Error(runtime_types::xcm::v2::traits::Error),
                    }
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Instruction {
                    #[codec(index = 0)]
                    WithdrawAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    ReserveAssetDeposited(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ReceiveTeleportedAsset(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 3)]
                    QueryResponse {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        response: runtime_types::xcm::v2::Response,
                        #[codec(compact)]
                        max_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 4)]
                    TransferAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 5)]
                    TransferReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 6)]
                    Transact {
                        origin_type: runtime_types::xcm::v0::OriginKind,
                        #[codec(compact)]
                        require_weight_at_most: ::core::primitive::u64,
                        call: runtime_types::xcm::double_encoded::DoubleEncoded,
                    },
                    #[codec(index = 7)]
                    HrmpNewChannelOpenRequest {
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        max_message_size: ::core::primitive::u32,
                        #[codec(compact)]
                        max_capacity: ::core::primitive::u32,
                    },
                    #[codec(index = 8)]
                    HrmpChannelAccepted {
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 9)]
                    HrmpChannelClosing {
                        #[codec(compact)]
                        initiator: ::core::primitive::u32,
                        #[codec(compact)]
                        sender: ::core::primitive::u32,
                        #[codec(compact)]
                        recipient: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    ClearOrigin,
                    #[codec(index = 11)]
                    DescendOrigin(runtime_types::xcm::v1::multilocation::Junctions),
                    #[codec(index = 12)]
                    ReportError {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 13)]
                    DepositAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        beneficiary: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 14)]
                    DepositReserveAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_assets: ::core::primitive::u32,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 15)]
                    ExchangeAsset {
                        give: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        receive: runtime_types::xcm::v1::multiasset::MultiAssets,
                    },
                    #[codec(index = 16)]
                    InitiateReserveWithdraw {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        reserve: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 17)]
                    InitiateTeleport {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        xcm: runtime_types::xcm::v2::Xcm,
                    },
                    #[codec(index = 18)]
                    QueryHolding {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        dest: runtime_types::xcm::v1::multilocation::MultiLocation,
                        assets: runtime_types::xcm::v1::multiasset::MultiAssetFilter,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 19)]
                    BuyExecution {
                        fees: runtime_types::xcm::v1::multiasset::MultiAsset,
                        weight_limit: runtime_types::xcm::v2::WeightLimit,
                    },
                    #[codec(index = 20)]
                    RefundSurplus,
                    #[codec(index = 21)]
                    SetErrorHandler(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 22)]
                    SetAppendix(runtime_types::xcm::v2::Xcm),
                    #[codec(index = 23)]
                    ClearError,
                    #[codec(index = 24)]
                    ClaimAsset {
                        assets: runtime_types::xcm::v1::multiasset::MultiAssets,
                        ticket: runtime_types::xcm::v1::multilocation::MultiLocation,
                    },
                    #[codec(index = 25)]
                    Trap(#[codec(compact)] ::core::primitive::u64),
                    #[codec(index = 26)]
                    SubscribeVersion {
                        #[codec(compact)]
                        query_id: ::core::primitive::u64,
                        #[codec(compact)]
                        max_response_weight: ::core::primitive::u64,
                    },
                    #[codec(index = 27)]
                    UnsubscribeVersion,
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum Response {
                    #[codec(index = 0)]
                    Null,
                    #[codec(index = 1)]
                    Assets(runtime_types::xcm::v1::multiasset::MultiAssets),
                    #[codec(index = 2)]
                    ExecutionResult(
                        ::core::option::Option<(
                            ::core::primitive::u32,
                            runtime_types::xcm::v2::traits::Error,
                        )>,
                    ),
                    #[codec(index = 3)]
                    Version(::core::primitive::u32),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub enum WeightLimit {
                    #[codec(index = 0)]
                    Unlimited,
                    #[codec(index = 1)]
                    Limited(#[codec(compact)] ::core::primitive::u64),
                }
                #[derive(
                    :: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone,
                )]
                pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum VersionedMultiAssets {
                #[codec(index = 0)]
                V0(::std::vec::Vec<runtime_types::xcm::v0::multi_asset::MultiAsset>),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multiasset::MultiAssets),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum VersionedMultiLocation {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::multi_location::MultiLocation),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::multilocation::MultiLocation),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum VersionedResponse {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Response),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Response),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Response),
            }
            #[derive(:: subxt :: codec :: Encode, :: subxt :: codec :: Decode, Debug, Clone)]
            pub enum VersionedXcm {
                #[codec(index = 0)]
                V0(runtime_types::xcm::v0::Xcm),
                #[codec(index = 1)]
                V1(runtime_types::xcm::v1::Xcm),
                #[codec(index = 2)]
                V2(runtime_types::xcm::v2::Xcm),
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
                match (index , error) { (0u8 , 0u8) => Some (ErrorDetails { pallet : "System" , error : "InvalidSpecName" , docs : "The name of specification does not match between the current runtime\nand the new runtime." }) , (0u8 , 1u8) => Some (ErrorDetails { pallet : "System" , error : "SpecVersionNeedsToIncrease" , docs : "The specification version is not allowed to decrease between the current runtime\nand the new runtime." }) , (0u8 , 2u8) => Some (ErrorDetails { pallet : "System" , error : "FailedToExtractRuntimeVersion" , docs : "Failed to extract the runtime version from the new runtime.\n\nEither calling `Core_version` or decoding `RuntimeVersion` failed." }) , (0u8 , 3u8) => Some (ErrorDetails { pallet : "System" , error : "NonDefaultComposite" , docs : "Suicide called when the account has non-default composite data." }) , (0u8 , 4u8) => Some (ErrorDetails { pallet : "System" , error : "NonZeroRefCount" , docs : "There is a non-zero reference count preventing the account from being purged." }) , (0u8 , 5u8) => Some (ErrorDetails { pallet : "System" , error : "CallFiltered" , docs : "The origin filter prevent the call to be dispatched." }) , (1u8 , 0u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "OverlappingUpgrades" , docs : "Attempt to upgrade validation function while existing upgrade pending" }) , (1u8 , 1u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "ProhibitedByPolkadot" , docs : "Polkadot currently prohibits this parachain from upgrading its validation function" }) , (1u8 , 2u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "TooBig" , docs : "The supplied validation function has compiled into a blob larger than Polkadot is\nwilling to run" }) , (1u8 , 3u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "ValidationDataNotAvailable" , docs : "The inherent which supplies the validation data did not run this block" }) , (1u8 , 4u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "HostConfigurationNotAvailable" , docs : "The inherent which supplies the host configuration did not run this block" }) , (1u8 , 5u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "NotScheduled" , docs : "No validation function upgrade is currently scheduled." }) , (1u8 , 6u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "NothingAuthorized" , docs : "No code upgrade has been authorized." }) , (1u8 , 7u8) => Some (ErrorDetails { pallet : "ParachainSystem" , error : "Unauthorized" , docs : "The given code upgrade has not been authorized." }) , (5u8 , 0u8) => Some (ErrorDetails { pallet : "Balances" , error : "VestingBalance" , docs : "Vesting balance too high to send value" }) , (5u8 , 1u8) => Some (ErrorDetails { pallet : "Balances" , error : "LiquidityRestrictions" , docs : "Account liquidity restrictions prevent withdrawal" }) , (5u8 , 2u8) => Some (ErrorDetails { pallet : "Balances" , error : "InsufficientBalance" , docs : "Balance too low to send value" }) , (5u8 , 3u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistentialDeposit" , docs : "Value too low to create account due to existential deposit" }) , (5u8 , 4u8) => Some (ErrorDetails { pallet : "Balances" , error : "KeepAlive" , docs : "Transfer/payment would kill account" }) , (5u8 , 5u8) => Some (ErrorDetails { pallet : "Balances" , error : "ExistingVestingSchedule" , docs : "A vesting schedule already exists for this account" }) , (5u8 , 6u8) => Some (ErrorDetails { pallet : "Balances" , error : "DeadAccount" , docs : "Beneficiary account must pre-exist" }) , (5u8 , 7u8) => Some (ErrorDetails { pallet : "Balances" , error : "TooManyReserves" , docs : "Number of named reserves exceed MaxReserves" }) , (7u8 , 0u8) => Some (ErrorDetails { pallet : "Authorship" , error : "InvalidUncleParent" , docs : "The uncle parent not in the chain." }) , (7u8 , 1u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UnclesAlreadySet" , docs : "Uncles already set in the block." }) , (7u8 , 2u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooManyUncles" , docs : "Too many uncles." }) , (7u8 , 3u8) => Some (ErrorDetails { pallet : "Authorship" , error : "GenesisUncle" , docs : "The uncle is genesis." }) , (7u8 , 4u8) => Some (ErrorDetails { pallet : "Authorship" , error : "TooHighUncle" , docs : "The uncle is too high in chain." }) , (7u8 , 5u8) => Some (ErrorDetails { pallet : "Authorship" , error : "UncleAlreadyIncluded" , docs : "The uncle is already included." }) , (7u8 , 6u8) => Some (ErrorDetails { pallet : "Authorship" , error : "OldUncle" , docs : "The uncle isn't recent enough to be included." }) , (8u8 , 0u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "TooManyCandidates" , docs : "Too many candidates" }) , (8u8 , 1u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "TooFewCandidates" , docs : "Too few candidates" }) , (8u8 , 2u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "Unknown" , docs : "Unknown error" }) , (8u8 , 3u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "Permission" , docs : "Permission issue" }) , (8u8 , 4u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "AlreadyCandidate" , docs : "User is already a candidate" }) , (8u8 , 5u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "NotCandidate" , docs : "User is not a candidate" }) , (8u8 , 6u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "AlreadyInvulnerable" , docs : "User is already an Invulnerable" }) , (8u8 , 7u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "NoAssociatedValidatorId" , docs : "Account has no associated validator ID" }) , (8u8 , 8u8) => Some (ErrorDetails { pallet : "CollatorSelection" , error : "ValidatorNotRegistered" , docs : "Validator ID is not yet registered" }) , (9u8 , 0u8) => Some (ErrorDetails { pallet : "Session" , error : "InvalidProof" , docs : "Invalid ownership proof." }) , (9u8 , 1u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAssociatedValidatorId" , docs : "No associated validator ID for account." }) , (9u8 , 2u8) => Some (ErrorDetails { pallet : "Session" , error : "DuplicatedKey" , docs : "Registered duplicate key." }) , (9u8 , 3u8) => Some (ErrorDetails { pallet : "Session" , error : "NoKeys" , docs : "No keys are associated with this account." }) , (9u8 , 4u8) => Some (ErrorDetails { pallet : "Session" , error : "NoAccount" , docs : "Key setting account is not live, so it's impossible to associate keys." }) , (12u8 , 0u8) => Some (ErrorDetails { pallet : "XcmpQueue" , error : "FailedToSend" , docs : "Failed to send XCM message." }) , (12u8 , 1u8) => Some (ErrorDetails { pallet : "XcmpQueue" , error : "BadXcmOrigin" , docs : "Bad XCM origin." }) , (12u8 , 2u8) => Some (ErrorDetails { pallet : "XcmpQueue" , error : "BadXcm" , docs : "Bad XCM data." }) , (12u8 , 3u8) => Some (ErrorDetails { pallet : "XcmpQueue" , error : "BadOverweightIndex" , docs : "Bad overweight index." }) , (12u8 , 4u8) => Some (ErrorDetails { pallet : "XcmpQueue" , error : "WeightOverLimit" , docs : "Provided weight is possibly not enough to execute the message." }) , (13u8 , 0u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "Unreachable" , docs : "The desired destination was unreachable, generally because there is a no way of routing\nto it." }) , (13u8 , 1u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "SendFailure" , docs : "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps\na lack of space for buffering the message." }) , (13u8 , 2u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "Filtered" , docs : "The message execution fails the filter." }) , (13u8 , 3u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "UnweighableMessage" , docs : "The message's weight could not be determined." }) , (13u8 , 4u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "DestinationNotInvertible" , docs : "The destination `MultiLocation` provided cannot be inverted." }) , (13u8 , 5u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "Empty" , docs : "The assets to be sent are empty." }) , (13u8 , 6u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "CannotReanchor" , docs : "Could not re-anchor the assets to declare the fees for the destination chain." }) , (13u8 , 7u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "TooManyAssets" , docs : "Too many assets have been attempted for transfer." }) , (13u8 , 8u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "InvalidOrigin" , docs : "Origin is invalid for sending." }) , (13u8 , 9u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "BadVersion" , docs : "The version of the `Versioned` value used is not able to be interpreted." }) , (13u8 , 10u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "BadLocation" , docs : "The given location could not be used (e.g. because it cannot be expressed in the\ndesired version of XCM)." }) , (13u8 , 11u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "NoSubscription" , docs : "The referenced subscription could not be found." }) , (13u8 , 12u8) => Some (ErrorDetails { pallet : "PolkadotXcm" , error : "AlreadySubscribed" , docs : "The location is invalid since it already has a subscription from us." }) , (15u8 , 0u8) => Some (ErrorDetails { pallet : "DmpQueue" , error : "Unknown" , docs : "The message index given is unknown." }) , (15u8 , 1u8) => Some (ErrorDetails { pallet : "DmpQueue" , error : "OverLimit" , docs : "The amount of weight given is possibly not enough for executing the message." }) , (16u8 , 0u8) => Some (ErrorDetails { pallet : "Utility" , error : "TooManyCalls" , docs : "Too many calls batched." }) , (17u8 , 0u8) => Some (ErrorDetails { pallet : "Multisig" , error : "MinimumThreshold" , docs : "Threshold must be 2 or greater." }) , (17u8 , 1u8) => Some (ErrorDetails { pallet : "Multisig" , error : "AlreadyApproved" , docs : "Call is already approved by this signatory." }) , (17u8 , 2u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NoApprovalsNeeded" , docs : "Call doesn't need any (more) approvals." }) , (17u8 , 3u8) => Some (ErrorDetails { pallet : "Multisig" , error : "TooFewSignatories" , docs : "There are too few signatories in the list." }) , (17u8 , 4u8) => Some (ErrorDetails { pallet : "Multisig" , error : "TooManySignatories" , docs : "There are too many signatories in the list." }) , (17u8 , 5u8) => Some (ErrorDetails { pallet : "Multisig" , error : "SignatoriesOutOfOrder" , docs : "The signatories were provided out of order; they should be ordered." }) , (17u8 , 6u8) => Some (ErrorDetails { pallet : "Multisig" , error : "SenderInSignatories" , docs : "The sender was contained in the other signatories; it shouldn't be." }) , (17u8 , 7u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NotFound" , docs : "Multisig operation not found when attempting to cancel." }) , (17u8 , 8u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NotOwner" , docs : "Only the account that originally created the multisig is able to cancel it." }) , (17u8 , 9u8) => Some (ErrorDetails { pallet : "Multisig" , error : "NoTimepoint" , docs : "No timepoint was given, yet the multisig operation is already underway." }) , (17u8 , 10u8) => Some (ErrorDetails { pallet : "Multisig" , error : "WrongTimepoint" , docs : "A different timepoint was given to the multisig operation that is underway." }) , (17u8 , 11u8) => Some (ErrorDetails { pallet : "Multisig" , error : "UnexpectedTimepoint" , docs : "A timepoint was given, yet no multisig operation is underway." }) , (17u8 , 12u8) => Some (ErrorDetails { pallet : "Multisig" , error : "MaxWeightTooLow" , docs : "The maximum weight information provided was too low." }) , (17u8 , 13u8) => Some (ErrorDetails { pallet : "Multisig" , error : "AlreadyStored" , docs : "The data to be stored is already stored." }) , (18u8 , 0u8) => Some (ErrorDetails { pallet : "Proxy" , error : "TooMany" , docs : "There are too many proxies registered or too many announcements pending." }) , (18u8 , 1u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NotFound" , docs : "Proxy registration not found." }) , (18u8 , 2u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NotProxy" , docs : "Sender is not a proxy of the account to be proxied." }) , (18u8 , 3u8) => Some (ErrorDetails { pallet : "Proxy" , error : "Unproxyable" , docs : "A call which is incompatible with the proxy type's filter was attempted." }) , (18u8 , 4u8) => Some (ErrorDetails { pallet : "Proxy" , error : "Duplicate" , docs : "Account is already a proxy." }) , (18u8 , 5u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NoPermission" , docs : "Call may not be made by proxy because it may escalate its privileges." }) , (18u8 , 6u8) => Some (ErrorDetails { pallet : "Proxy" , error : "Unannounced" , docs : "Announcement, if made at all, was made too recently." }) , (18u8 , 7u8) => Some (ErrorDetails { pallet : "Proxy" , error : "NoSelfProxy" , docs : "Cannot add self as proxy." }) , (19u8 , 0u8) => Some (ErrorDetails { pallet : "Sudo" , error : "RequireSudo" , docs : "Sender must be the Sudo account" }) , (20u8 , 0u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "InvalidJustification" , docs : "The given justification is invalid for the given header." }) , (20u8 , 1u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "InvalidAuthoritySet" , docs : "The authority set from the underlying header chain is invalid." }) , (20u8 , 2u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "TooManyRequests" , docs : "There are too many requests for the current window to handle." }) , (20u8 , 3u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "OldHeader" , docs : "The header being imported is older than the best finalized header known to the pallet." }) , (20u8 , 4u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "UnknownHeader" , docs : "The header is unknown to the pallet." }) , (20u8 , 5u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "UnsupportedScheduledChange" , docs : "The scheduled authority set change found in the header is unsupported by the pallet.\n\nThis is the case for non-standard (e.g forced) authority set changes." }) , (20u8 , 6u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "NotInitialized" , docs : "The pallet is not yet initialized." }) , (20u8 , 7u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "AlreadyInitialized" , docs : "The pallet has already been initialized." }) , (20u8 , 8u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "Halted" , docs : "All pallet operations are halted." }) , (20u8 , 9u8) => Some (ErrorDetails { pallet : "BridgePangolinGrandpa" , error : "StorageRootMismatch" , docs : "The storage proof doesn't contains storage root. So it is invalid for given header." }) , (21u8 , 0u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "Halted" , docs : "All pallet operations are halted." }) , (21u8 , 1u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageRejectedByChainVerifier" , docs : "Message has been treated as invalid by chain verifier." }) , (21u8 , 2u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageRejectedByLaneVerifier" , docs : "Message has been treated as invalid by lane verifier." }) , (21u8 , 3u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "FailedToWithdrawMessageFee" , docs : "Submitter has failed to pay fee for delivering and dispatching messages." }) , (21u8 , 4u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "TooManyMessagesInTheProof" , docs : "The transaction brings too many messages." }) , (21u8 , 5u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidMessagesProof" , docs : "Invalid messages has been submitted." }) , (21u8 , 6u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidMessagesDeliveryProof" , docs : "Invalid messages delivery proof has been submitted." }) , (21u8 , 7u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidUnrewardedRelayers" , docs : "The bridged chain has invalid `UnrewardedRelayers` in its storage (fatal for the lane)." }) , (21u8 , 8u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "InvalidUnrewardedRelayersState" , docs : "The relayer has declared invalid unrewarded relayers state in the\n`receive_messages_delivery_proof` call." }) , (21u8 , 9u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageIsAlreadyDelivered" , docs : "The message someone is trying to work with (i.e. increase fee) is already-delivered." }) , (21u8 , 10u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "MessageIsNotYetSent" , docs : "The message someone is trying to work with (i.e. increase fee) is not yet sent." }) , (21u8 , 11u8) => Some (ErrorDetails { pallet : "BridgePangolinMessages" , error : "TryingToConfirmMoreMessagesThanExpected" , docs : "The number of actually confirmed messages is going to be larger than the number of\nmessages in the proof. This may mean that this or bridged chain storage is corrupted." }) , (23u8 , 0u8) => Some (ErrorDetails { pallet : "FeeMarket" , error : "InsufficientBalance" , docs : "Insufficient balance." }) , (23u8 , 1u8) => Some (ErrorDetails { pallet : "FeeMarket" , error : "AlreadyEnrolled" , docs : "The relayer has been enrolled." }) , (23u8 , 2u8) => Some (ErrorDetails { pallet : "FeeMarket" , error : "NotEnrolled" , docs : "This relayer doesn't enroll ever." }) , (23u8 , 3u8) => Some (ErrorDetails { pallet : "FeeMarket" , error : "StillHasOrdersNotConfirmed" , docs : "Update locked collateral is not allow since some orders are not confirm." }) , (23u8 , 4u8) => Some (ErrorDetails { pallet : "FeeMarket" , error : "RelayFeeTooLow" , docs : "The fee is lower than MinimumRelayFee." }) , (23u8 , 5u8) => Some (ErrorDetails { pallet : "FeeMarket" , error : "OccupiedRelayer" , docs : "The relayer is occupied, and can't cancel enrollment now." }) , _ => None }
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
        pub fn timestamp(&self) -> timestamp::constants::ConstantsApi {
            timestamp::constants::ConstantsApi
        }
        pub fn balances(&self) -> balances::constants::ConstantsApi {
            balances::constants::ConstantsApi
        }
        pub fn transaction_payment(&self) -> transaction_payment::constants::ConstantsApi {
            transaction_payment::constants::ConstantsApi
        }
        pub fn authorship(&self) -> authorship::constants::ConstantsApi {
            authorship::constants::ConstantsApi
        }
        pub fn utility(&self) -> utility::constants::ConstantsApi {
            utility::constants::ConstantsApi
        }
        pub fn multisig(&self) -> multisig::constants::ConstantsApi {
            multisig::constants::ConstantsApi
        }
        pub fn proxy(&self) -> proxy::constants::ConstantsApi {
            proxy::constants::ConstantsApi
        }
        pub fn bridge_pangolin_grandpa(&self) -> bridge_pangolin_grandpa::constants::ConstantsApi {
            bridge_pangolin_grandpa::constants::ConstantsApi
        }
        pub fn bridge_pangolin_messages(
            &self,
        ) -> bridge_pangolin_messages::constants::ConstantsApi {
            bridge_pangolin_messages::constants::ConstantsApi
        }
        pub fn fee_market(&self) -> fee_market::constants::ConstantsApi {
            fee_market::constants::ConstantsApi
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
        pub fn parachain_system(&self) -> parachain_system::storage::StorageApi<'a, T> {
            parachain_system::storage::StorageApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::storage::StorageApi<'a, T> {
            timestamp::storage::StorageApi::new(self.client)
        }
        pub fn parachain_info(&self) -> parachain_info::storage::StorageApi<'a, T> {
            parachain_info::storage::StorageApi::new(self.client)
        }
        pub fn balances(&self) -> balances::storage::StorageApi<'a, T> {
            balances::storage::StorageApi::new(self.client)
        }
        pub fn transaction_payment(&self) -> transaction_payment::storage::StorageApi<'a, T> {
            transaction_payment::storage::StorageApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::storage::StorageApi<'a, T> {
            authorship::storage::StorageApi::new(self.client)
        }
        pub fn collator_selection(&self) -> collator_selection::storage::StorageApi<'a, T> {
            collator_selection::storage::StorageApi::new(self.client)
        }
        pub fn session(&self) -> session::storage::StorageApi<'a, T> {
            session::storage::StorageApi::new(self.client)
        }
        pub fn aura(&self) -> aura::storage::StorageApi<'a, T> {
            aura::storage::StorageApi::new(self.client)
        }
        pub fn aura_ext(&self) -> aura_ext::storage::StorageApi<'a, T> {
            aura_ext::storage::StorageApi::new(self.client)
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::storage::StorageApi<'a, T> {
            xcmp_queue::storage::StorageApi::new(self.client)
        }
        pub fn polkadot_xcm(&self) -> polkadot_xcm::storage::StorageApi<'a, T> {
            polkadot_xcm::storage::StorageApi::new(self.client)
        }
        pub fn dmp_queue(&self) -> dmp_queue::storage::StorageApi<'a, T> {
            dmp_queue::storage::StorageApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::storage::StorageApi<'a, T> {
            multisig::storage::StorageApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::storage::StorageApi<'a, T> {
            proxy::storage::StorageApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::storage::StorageApi<'a, T> {
            sudo::storage::StorageApi::new(self.client)
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
        pub fn fee_market(&self) -> fee_market::storage::StorageApi<'a, T> {
            fee_market::storage::StorageApi::new(self.client)
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
        pub fn parachain_system(&self) -> parachain_system::calls::TransactionApi<'a, T, X, A> {
            parachain_system::calls::TransactionApi::new(self.client)
        }
        pub fn timestamp(&self) -> timestamp::calls::TransactionApi<'a, T, X, A> {
            timestamp::calls::TransactionApi::new(self.client)
        }
        pub fn balances(&self) -> balances::calls::TransactionApi<'a, T, X, A> {
            balances::calls::TransactionApi::new(self.client)
        }
        pub fn authorship(&self) -> authorship::calls::TransactionApi<'a, T, X, A> {
            authorship::calls::TransactionApi::new(self.client)
        }
        pub fn collator_selection(&self) -> collator_selection::calls::TransactionApi<'a, T, X, A> {
            collator_selection::calls::TransactionApi::new(self.client)
        }
        pub fn session(&self) -> session::calls::TransactionApi<'a, T, X, A> {
            session::calls::TransactionApi::new(self.client)
        }
        pub fn xcmp_queue(&self) -> xcmp_queue::calls::TransactionApi<'a, T, X, A> {
            xcmp_queue::calls::TransactionApi::new(self.client)
        }
        pub fn polkadot_xcm(&self) -> polkadot_xcm::calls::TransactionApi<'a, T, X, A> {
            polkadot_xcm::calls::TransactionApi::new(self.client)
        }
        pub fn dmp_queue(&self) -> dmp_queue::calls::TransactionApi<'a, T, X, A> {
            dmp_queue::calls::TransactionApi::new(self.client)
        }
        pub fn utility(&self) -> utility::calls::TransactionApi<'a, T, X, A> {
            utility::calls::TransactionApi::new(self.client)
        }
        pub fn multisig(&self) -> multisig::calls::TransactionApi<'a, T, X, A> {
            multisig::calls::TransactionApi::new(self.client)
        }
        pub fn proxy(&self) -> proxy::calls::TransactionApi<'a, T, X, A> {
            proxy::calls::TransactionApi::new(self.client)
        }
        pub fn sudo(&self) -> sudo::calls::TransactionApi<'a, T, X, A> {
            sudo::calls::TransactionApi::new(self.client)
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
        pub fn fee_market(&self) -> fee_market::calls::TransactionApi<'a, T, X, A> {
            fee_market::calls::TransactionApi::new(self.client)
        }
    }
}
