/// Package encoding implements message encoding and decoding for Hobbits, a Lightweight,
/// Multiclient Wire Protocol For ETH2.0 Communications.

pub mod message;
pub mod marshal;
pub mod unmarshal;

pub use message::Message;
pub use marshal::marshal;
pub use unmarshal::unmarshal;
