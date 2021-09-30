use crate::room;
use crate::hand;
use crate::simple_serde::SimpleSerde;

#[repr(C)]
#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(clippy::large_enum_variant)]
pub enum State {
    /// State is not initialized yet
    Uninitialized,
    /// State holds room state
    Room(room::Room),
    /// State holds hand state
    Hand(hand::Hand),
}
impl SimpleSerde for State {}
