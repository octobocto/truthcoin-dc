use super::{
    Address, AssetId, Authorization, Authorized, Balance, BallotItem,
    BitcoinOutputContent, Block, BlockHash, Body,
    CalculateInitialLiquidityRequest, ClaimedDecisionInfo, ConsensusResults,
    CreateTradeRequest, CreateTradeResponse, DecisionClaimItem,
    DecisionClaimRequest, DecisionClaimResponse, DecisionDetails,
    DecisionFilter, DecisionListItem, DecisionListingFeeInfo,
    DecisionPeriodStatus, DecisionState, DecisionSummary, DecisionType,
    DimensionInput, Dst, EncryptionPubKey, FilledOutputContent, Header,
    InitialLiquidityCalculation, MainchainSyncPhase, MainchainSyncProgress,
    MarketAmplifyBetaRequest, MarketBuyRequest, MarketBuyResponse,
    MarketCreateRequest, MarketCreateResponse, MarketData, MarketOutcome,
    MarketSellRequest, MarketSellResponse, MarketSummary, MerkleRoot, OutPoint,
    Output, OutputContent, ParticipationStats, Peer, PeerConnectionStatus,
    PeriodPricingSummary, PeriodStats, PointedOutput, RpcResult, ScoreChange,
    SharePosition, Signature, SocketAddr, Transaction, TxData, TxIn, TxInfo,
    Txid, UserHoldings, VerifyingKey, VoteFilter, VoteInfo, VoterInfo,
    VoterInfoFull, VotingPeriodFull, WithdrawalBundle, WithdrawalOutputContent,
    open_api, rpc, schema, truthcoin_schema,
};

#[open_api(ref_schemas[
    truthcoin_schema::BitcoinAddr, truthcoin_schema::BitcoinBlockHash,
    truthcoin_schema::BitcoinTransaction, truthcoin_schema::BitcoinOutPoint,
    truthcoin_schema::SocketAddr, Address, AssetId, Authorization,
    BitcoinOutputContent, BlockHash, Body,
    CalculateInitialLiquidityRequest, ClaimedDecisionInfo, DecisionClaimItem,
    DecisionClaimRequest, DecisionClaimResponse, DimensionInput,
    MarketCreateRequest, MarketCreateResponse, PeriodPricingSummary,
    ConsensusResults, DecisionSummary,
    EncryptionPubKey, FilledOutputContent, Header, InitialLiquidityCalculation,
    MainchainSyncPhase, MarketBuyRequest, MarketBuyResponse, MarketData,
    MarketOutcome, MarketSellRequest, MarketSellResponse, MarketSummary,
    MerkleRoot, OutPoint, Output, OutputContent,
    ParticipationStats, PeerConnectionStatus, PeriodStats,
    ScoreChange,
    SharePosition, Signature, DecisionDetails, DecisionFilter, DecisionListItem, DecisionListingFeeInfo, DecisionState, DecisionPeriodStatus, DecisionType,
    Transaction, TxData, Txid, TxIn, UserHoldings,
    BallotItem, VoteFilter, VoteInfo, VoterInfo, VoterInfoFull,
    VotingPeriodFull, WithdrawalOutputContent, VerifyingKey,
])]
#[rpc(client, server)]
pub trait Rpc {
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "bitcoin_balance")]
    async fn bitcoin_balance(&self) -> RpcResult<Balance>;

    #[open_api_method(output_schema(PartialSchema = "schema::BitcoinTxid"))]
    #[method(name = "create_deposit")]
    async fn create_deposit(
        &self,
        address: Address,
        value_sats: u64,
        fee_sats: u64,
    ) -> RpcResult<bitcoin::Txid>;

    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "connect_peer")]
    async fn connect_peer(
        &self,
        #[open_api_method_arg(schema(
            ToSchema = "truthcoin_schema::SocketAddr"
        ))]
        addr: SocketAddr,
    ) -> RpcResult<()>;

    #[method(name = "decrypt_msg")]
    async fn decrypt_msg(
        &self,
        encryption_pubkey: EncryptionPubKey,
        ciphertext: String,
    ) -> RpcResult<String>;

    #[method(name = "encrypt_msg")]
    async fn encrypt_msg(
        &self,
        encryption_pubkey: EncryptionPubKey,
        msg: String,
    ) -> RpcResult<String>;

    /// Delete peer from known_peers DB.
    /// Connections to the peer are not terminated.
    #[method(name = "forget_peer")]
    async fn forget_peer(
        &self,
        #[open_api_method_arg(schema(
            ToSchema = "truthcoin_schema::SocketAddr"
        ))]
        addr: SocketAddr,
    ) -> RpcResult<()>;

    #[method(name = "format_deposit_address")]
    async fn format_deposit_address(
        &self,
        address: Address,
    ) -> RpcResult<String>;

    #[method(name = "generate_mnemonic")]
    async fn generate_mnemonic(&self) -> RpcResult<String>;

    /// Get block data
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "get_block")]
    async fn get_block(&self, block_hash: BlockHash) -> RpcResult<Block>;

    /// Get mainchain blocks that commit to a specified block hash
    #[open_api_method(output_schema(
        PartialSchema = "truthcoin_schema::BitcoinBlockHash"
    ))]
    #[method(name = "get_bmm_inclusions")]
    async fn get_bmm_inclusions(
        &self,
        block_hash: truthcoin_dc::types::BlockHash,
    ) -> RpcResult<Vec<bitcoin::BlockHash>>;

    /// Get the best mainchain block hash known by Thunder
    #[open_api_method(output_schema(
        PartialSchema = "schema::Optional<truthcoin_schema::BitcoinBlockHash>"
    ))]
    #[method(name = "get_best_mainchain_block_hash")]
    async fn get_best_mainchain_block_hash(
        &self,
    ) -> RpcResult<Option<bitcoin::BlockHash>>;

    /// Get the best sidechain block hash known by Truthcoin
    #[open_api_method(output_schema(
        PartialSchema = "schema::Optional<BlockHash>"
    ))]
    #[method(name = "get_best_sidechain_block_hash")]
    async fn get_best_sidechain_block_hash(
        &self,
    ) -> RpcResult<Option<BlockHash>>;

    /// Generate a new address
    #[method(name = "get_new_address")]
    async fn get_new_address(&self) -> RpcResult<Address>;

    /// Get the voter address (index 0), used for reputation
    /// and voting identity
    #[method(name = "get_voter_address")]
    async fn get_voter_address(&self) -> RpcResult<Address>;

    /// Generate new encryption key
    #[method(name = "get_new_encryption_key")]
    async fn get_new_encryption_key(&self) -> RpcResult<EncryptionPubKey>;

    /// Generate new verifying/signing key
    #[method(name = "get_new_verifying_key")]
    async fn get_new_verifying_key(&self) -> RpcResult<VerifyingKey>;

    /// Get transaction by txid
    #[method(name = "get_transaction")]
    async fn get_transaction(
        &self,
        txid: Txid,
    ) -> RpcResult<Option<Transaction>>;

    /// Get information about a transaction in the current chain
    #[method(name = "get_transaction_info")]
    async fn get_transaction_info(
        &self,
        txid: Txid,
    ) -> RpcResult<Option<TxInfo>>;

    /// Get wallet addresses, sorted by base58 encoding
    #[method(name = "get_wallet_addresses")]
    async fn get_wallet_addresses(&self) -> RpcResult<Vec<Address>>;

    /// Get wallet UTXOs
    #[method(name = "get_wallet_utxos")]
    async fn get_wallet_utxos(
        &self,
    ) -> RpcResult<Vec<PointedOutput<FilledOutputContent>>>;

    /// Get the current block count
    #[method(name = "getblockcount")]
    async fn getblockcount(&self) -> RpcResult<u32>;

    /// Get the height of the latest failed withdrawal bundle
    #[method(name = "latest_failed_withdrawal_bundle_height")]
    async fn latest_failed_withdrawal_bundle_height(
        &self,
    ) -> RpcResult<Option<u32>>;

    /// List peers
    #[method(name = "list_peers")]
    async fn list_peers(&self) -> RpcResult<Vec<Peer>>;

    /// List all UTXOs
    #[open_api_method(output_schema(
        ToSchema = "Vec<PointedOutput<FilledOutputContent>>"
    ))]
    #[method(name = "list_utxos")]
    async fn list_utxos(
        &self,
    ) -> RpcResult<Vec<PointedOutput<FilledOutputContent>>>;

    /// Get the progress of the sync with the mainchain
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "mainchain_sync_progress")]
    async fn mainchain_sync_progress(&self)
    -> RpcResult<MainchainSyncProgress>;

    /// Attempt to mine a sidechain block
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "mine")]
    async fn mine(&self, fee: Option<u64>) -> RpcResult<()>;

    /// List unconfirmed owned UTXOs
    #[method(name = "my_unconfirmed_utxos")]
    async fn my_unconfirmed_utxos(&self) -> RpcResult<Vec<PointedOutput>>;

    /// Get pending withdrawal bundle
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "pending_withdrawal_bundle")]
    async fn pending_withdrawal_bundle(
        &self,
    ) -> RpcResult<Option<WithdrawalBundle>>;

    /// Get OpenRPC schema
    #[open_api_method(output_schema(ToSchema = "schema::OpenApi"))]
    #[method(name = "openapi_schema")]
    async fn openapi_schema(&self) -> RpcResult<utoipa::openapi::OpenApi>;

    /// Remove a tx from the mempool
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "remove_from_mempool")]
    async fn remove_from_mempool(&self, txid: Txid) -> RpcResult<()>;

    /// Set the wallet seed from a mnemonic seed phrase
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "set_seed_from_mnemonic")]
    async fn set_seed_from_mnemonic(&self, mnemonic: String) -> RpcResult<()>;

    /// Get total sidechain wealth in sats
    #[method(name = "sidechain_wealth")]
    async fn sidechain_wealth_sats(&self) -> RpcResult<u64>;

    /// Sign an arbitrary message with the specified verifying key
    #[method(name = "sign_arbitrary_msg")]
    async fn sign_arbitrary_msg(
        &self,
        verifying_key: VerifyingKey,
        msg: String,
    ) -> RpcResult<Signature>;

    /// Sign an arbitrary message with the secret key for the specified address
    #[method(name = "sign_arbitrary_msg_as_addr")]
    async fn sign_arbitrary_msg_as_addr(
        &self,
        address: Address,
        msg: String,
    ) -> RpcResult<Authorization>;

    /// Sign a transaction, and optionally broadcast it.
    #[method(name = "sign_transaction")]
    async fn sign_transaction(
        &self,
        transaction: Transaction,
        broadcast: Option<bool>,
    ) -> RpcResult<Authorized<Transaction>>;

    /// Stop the node
    #[method(name = "stop")]
    async fn stop(&self);

    /// Verify and broadcast a transaction
    #[method(name = "submit_transaction")]
    async fn submit_transaction(
        &self,
        transaction: Authorized<Transaction>,
    ) -> RpcResult<Txid>;

    /// Transfer funds to the specified address
    #[method(name = "transfer")]
    async fn transfer(
        &self,
        dest: Address,
        value: u64,
        fee: u64,
        memo: Option<String>,
    ) -> RpcResult<Txid>;

    /// Transfer votecoin to the specified address
    #[method(name = "transfer_votecoin")]
    async fn transfer_votecoin(
        &self,
        dest: Address,
        amount: f64,
        fee_sats: u64,
        memo: Option<String>,
    ) -> RpcResult<Txid>;

    /// Verify a signature on a message against the specified verifying key.
    /// Returns `true` if the signature is valid
    #[method(name = "verify_signature")]
    async fn verify_signature(
        &self,
        signature: Signature,
        verifying_key: VerifyingKey,
        dst: Dst,
        msg: String,
    ) -> RpcResult<bool>;

    /// Initiate a withdrawal to the specified mainchain address
    #[method(name = "withdraw")]
    async fn withdraw(
        &self,
        #[open_api_method_arg(schema(
            PartialSchema = "truthcoin_schema::BitcoinAddr"
        ))]
        mainchain_address: bitcoin::Address<
            bitcoin::address::NetworkUnchecked,
        >,
        amount_sats: u64,
        fee_sats: u64,
        mainchain_fee_sats: u64,
    ) -> RpcResult<Txid>;

    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "refresh_wallet")]
    async fn refresh_wallet(&self) -> RpcResult<()>;

    /// Wait until the node reaches a specific block height (for sync)
    /// Returns the actual height reached (may be higher than requested)
    /// Times out after the specified milliseconds (default 10000ms)
    #[method(name = "await_block_height")]
    async fn await_block_height(
        &self,
        target_height: u32,
        timeout_ms: Option<u64>,
    ) -> RpcResult<u32>;

    /// Trigger a sync to a specific tip block hash.
    /// The block must already exist in our archive (received via P2P).
    /// Returns true if reorg was successful, false if not needed or failed.
    #[method(name = "sync_to_tip")]
    async fn sync_to_tip(&self, block_hash: BlockHash) -> RpcResult<bool>;

    /// Get decision system status and configuration
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "decision_status")]
    async fn decision_status(&self) -> RpcResult<DecisionPeriodStatus>;

    /// List decisions with optional filtering by period and state
    #[open_api_method(output_schema(ToSchema = "Vec<DecisionListItem>"))]
    #[method(name = "decision_list")]
    async fn decision_list(
        &self,
        filter: Option<DecisionFilter>,
    ) -> RpcResult<Vec<DecisionListItem>>;

    /// Get a specific decision by ID (includes is_voting status)
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "decision_get")]
    async fn decision_get(
        &self,
        decision_id: String,
    ) -> RpcResult<Option<DecisionDetails>>;

    /// Claim one or more decisions.
    /// decision_type: "binary", "scaled", or "category"
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "decision_claim")]
    async fn decision_claim(
        &self,
        request: DecisionClaimRequest,
    ) -> RpcResult<DecisionClaimResponse>;

    /// Get listing fee info for a period
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "decision_listing_fee")]
    async fn decision_listing_fee(
        &self,
        period: u32,
    ) -> RpcResult<DecisionListingFeeInfo>;

    /// Compute the listing fee (sats) for claiming a specific decision_id.
    /// The tier (and therefore the price multiplier) is determined by the
    /// id's decision_index field.
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "decision_fee_for_id")]
    async fn decision_fee_for_id(
        &self,
        decision_id_hex: String,
    ) -> RpcResult<u64>;

    /// Create a prediction market, optionally claiming new decisions in
    /// the same tx. Each dimension references either an existing claimed
    /// decision or carries new-claim metadata that will be allocated a
    /// slot and claimed before the market is built.
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "market_create")]
    async fn market_create(
        &self,
        request: MarketCreateRequest,
    ) -> RpcResult<MarketCreateResponse>;

    /// List open voting periods with the price the next claim would pay
    /// for the cheapest available unlocked slot in each. For the GUI's
    /// per-dimension period picker.
    #[open_api_method(output_schema(ToSchema = "Vec<PeriodPricingSummary>"))]
    #[method(name = "list_open_periods_with_pricing")]
    async fn list_open_periods_with_pricing(
        &self,
    ) -> RpcResult<Vec<PeriodPricingSummary>>;

    /// List all markets
    #[open_api_method(output_schema(ToSchema = "Vec<MarketSummary>"))]
    #[method(name = "market_list")]
    async fn market_list(&self) -> RpcResult<Vec<MarketSummary>>;

    /// Get detailed market information
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "market_get")]
    async fn market_get(
        &self,
        market_id: String,
    ) -> RpcResult<Option<MarketData>>;

    /// Buy shares (with dry_run support for cost calculation)
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "market_buy")]
    async fn market_buy(
        &self,
        request: MarketBuyRequest,
    ) -> RpcResult<MarketBuyResponse>;

    /// Sell shares (with dry_run support for proceeds calculation)
    /// Payout is created during block connection from market treasury
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "market_sell")]
    async fn market_sell(
        &self,
        request: MarketSellRequest,
    ) -> RpcResult<MarketSellResponse>;

    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "market_amplify_beta")]
    async fn market_amplify_beta(
        &self,
        request: MarketAmplifyBetaRequest,
    ) -> RpcResult<String>;

    /// Get share positions for an address (optionally filtered by market)
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "market_positions")]
    async fn market_positions(
        &self,
        address: Address,
        market_id: Option<String>,
    ) -> RpcResult<UserHoldings>;

    /// Get full voter information
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "vote_voter")]
    async fn vote_voter(
        &self,
        address: Address,
    ) -> RpcResult<Option<VoterInfoFull>>;

    /// List all registered voters
    #[open_api_method(output_schema(ToSchema = "Vec<VoterInfo>"))]
    #[method(name = "vote_voters")]
    async fn vote_voters(&self) -> RpcResult<Vec<VoterInfo>>;

    /// Submit one or more votes (batch)
    #[open_api_method(output_schema(ToSchema = "String"))]
    #[method(name = "vote_submit")]
    async fn vote_submit(
        &self,
        votes: Vec<BallotItem>,
        fee_sats: u64,
    ) -> RpcResult<String>;

    /// Query votes with filters (by voter, decision, or period)
    #[open_api_method(output_schema(ToSchema = "Vec<VoteInfo>"))]
    #[method(name = "vote_list")]
    async fn vote_list(&self, filter: VoteFilter) -> RpcResult<Vec<VoteInfo>>;

    /// Get full voting period information (null period_id = current)
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "vote_period")]
    async fn vote_period(
        &self,
        period_id: Option<u32>,
    ) -> RpcResult<Option<VotingPeriodFull>>;

    /// Get votecoin balance for an address
    #[open_api_method(output_schema(ToSchema = "f64"))]
    #[method(name = "votecoin_balance")]
    async fn votecoin_balance(&self, address: Address) -> RpcResult<f64>;

    /// Calculate initial liquidity required for market creation
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "calculate_initial_liquidity")]
    async fn calculate_initial_liquidity(
        &self,
        request: CalculateInitialLiquidityRequest,
    ) -> RpcResult<InitialLiquidityCalculation>;

    /// Submit a hex-encoded borsh-serialized `AuthorizedTransaction` directly
    /// to the mempool. Returns the transaction id on success.
    ///
    /// Intended for tests and advanced clients that need to submit a
    /// pre-signed transaction (e.g. to exercise validator paths like
    /// stale `prev_block_hash` rejection).
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "push_tx")]
    async fn push_tx(&self, tx_hex: String) -> RpcResult<Txid>;

    /// Build and sign a Trade transaction with a caller-supplied
    /// `prev_block_hash`, returning the hex-encoded signed
    /// `AuthorizedTransaction` *without* submitting it to the mempool.
    ///
    /// Intended for tests that need to exercise the validator's
    /// chain-binding behavior (e.g. submitting a trade bound to an
    /// out-of-window block hash). The returned tx can be submitted via
    /// [`push_tx`].
    #[open_api_method(output_schema(ToSchema))]
    #[method(name = "create_trade")]
    async fn create_trade(
        &self,
        request: CreateTradeRequest,
    ) -> RpcResult<CreateTradeResponse>;
}
