//! The trait definition for the weights of extrinsics.

use frame_support::weights::Weight;

pub trait WeightInfo {
    fn add_item() -> Weight;
    fn update_item() -> Weight;
    fn get_item() -> Weight;
}
