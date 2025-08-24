2. System Architecture
2.1 High-Level Components

Frontend (TypeScript): Browser-based UI using Polkadot.js for contract interactions, chessboard.js for chess rendering, and reactive-dot for event-driven state. Handles user inputs, local validation, and real-time sync.
Smart Contracts (Rust/ink! v6): Deployed on Rococo. Modular contracts for profiles, queuing, matches, validation, and admin functions. Uses SCALE encoding for efficiency.
Off-Chain Helpers: Polkadot.js for tx signing; optional WebRTC for peer sync (MVP: direct chain submits).
Data Flow: User inputs → Frontend (local check) → Signed tx to contract → On-chain validation/storage → Events emitted → Frontend subscribes (reactive-dot) → UI updates.

2.2 Entities/Models















































Entity/ModelDescriptionKey Fields (Rust Struct Example)Where Stored/UsedUserProfileUser metadata for rankings and identity.struct UserProfile { username: String, ranking: u32, wins: u32, losses: u32 }Mapping<AccountId, UserProfile>MatchActive game session with stakes and moves.struct Match { players: (AccountId, AccountId), game_type: String, stake: Balance, moves: Vec<String>, start_time: Timestamp, active: bool }Mapping<u64, Match>QueueEntryPending matchmaking request.struct QueueEntry { player: AccountId, game_type: String, min_stake: Balance, elo_range: u32 }Vec<queueentry> or Mapping</queueentry>GameStateGame-specific state (chess first; stubs for others).enum GameSpecific { Chess { fen: String, turn: bool }, Tetris { scores: (u32, u32) }, Snake { positions: Vec<(u8, u8)> } }Embedded in MatchEvent LogsImmutable records for sync.E.g., #[ink(event)] pub struct MoveMade { match_id: u64, move_notation: String }Emitted on-chainPlatformConfigGlobal settings.struct PlatformConfig { fee_percent: u8, timeout_blocks: u32 }Contract storage
3. Actors and Usecases
3.1 Actors

Player: Core user; interacts with gameplay. Can: Register, queue, confirm/stake, submit moves, end games. Cannot: Alter states invalidly or access admin.
Platform Owner: Admin; manages fees. Can: Withdraw fees, update config. Cannot: Interfere in matches.
System/Contract: Passive; enforces rules automatically.

3.2 Usecases

Player Registers Profile.
Player Queues for Game.
Player Confirms Match and Stakes.
Player Submits Move.
End Game and Payout.
Platform Owner Withdraws Fees.

4. Flow Charts (Sequence Diagrams)
Textual representations for each usecase:

Player Registers Profile:

Player → Frontend: Connect wallet, input username.
Frontend → Contract: register(username).
Contract → Storage: Insert UserProfile.
Contract → Frontend: Emit ProfileRegistered.
Frontend → Player: Display profile.


Player Queues for Game:

Player → Frontend: Select game/stake/elo.
Frontend → Contract: queue_up(...).
Contract → Storage: Add QueueEntry.
System → Contract: Pair if match found, emit MatchProposed.
Frontend → Player: Notify.


Player Confirms Match and Stakes:

Player → Frontend: Approve stake.
Frontend → Contract: confirm_match(match_id) (payable).
Contract → Storage: Update Match, escrow DOT.
Contract → Frontend: Emit MatchStarted.
Frontend → Player: Load game.


Player Submits Move:

Player → Frontend: Input move.
Frontend → Contract: submit_move(match_id, notation).
Contract → Storage: Append/validate moves.
Contract → Frontend: Emit MoveMade.
Frontend → Player: Update UI.


End Game and Payout:

Player/System → Frontend: Claim/timeout.
Frontend → Contract: end_game(match_id, action).
Contract → Storage: Replay, payout, update profiles.
Contract → Frontend: Emit GameEnded.
Frontend → Player: Show results.


Platform Owner Withdraws Fees:

Owner → Frontend: Request.
Frontend → Contract: withdraw_fees().
Contract → Balances: Transfer fees.
Contract → Frontend: Emit FeesWithdrawn.



5. Modules and Specifications
5.1 Rust Modules (ink! Contracts)

ProfileModule: Specs: Register/query profiles.
QueueModule: Specs: Queue management, FIFO pairing.
MatchModule: Specs: Match creation, moves, cross-calls to validator.
ValidatorModule: Specs: On-chain replay for chess (stubs for others).
AdminModule: Specs: Fee withdrawal, config updates.

5.2 TypeScript Frontend Pages

Home/Dashboard: Query profiles/leaderboards.
Register: Username form.
Queue: Game selection.
Match Confirm: Stake approval.
Game Play: chessboard.js integration.
Results: Post-game UI.
Admin: Fee tools.

6. Optimization and Tools

Data Encoding: Compact moves (e.g., bytes for chess notations), lazy storage, post-game cleanup.
Hydration Tool: "DotDecoder" (TS function): Decodes on-chain bytes to readable formats for UI. E.g., decodeMoves(encoded: Uint8Array) → string[].
