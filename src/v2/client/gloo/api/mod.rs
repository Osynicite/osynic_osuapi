// V2 Gloo API implementations
// Note: These implementations are provided as a foundation but may need adjustments for:
// 1. Interface compatibility - some method signatures may not match the trait definitions
// 2. WASM Send trait requirements - gloo-net doesn't support Send in WASM environments
// 3. Missing model structures that need to be implemented
// 
// To use these implementations, you may need to:
// - Update interface traits to remove Send requirements for WASM targets
// - Implement missing model structures
// - Adjust method signatures to match trait definitions

pub mod beatmaps;
pub mod beatmapsets;
pub mod changelog;
pub mod chat;
pub mod comments;
pub mod events;
pub mod forum;
pub mod matches;
pub mod multiplayer;
pub mod news;
pub mod notifications;
pub mod oauth;
pub mod ranking;
pub mod scores;
pub mod search;
pub mod users;
pub mod wiki;
