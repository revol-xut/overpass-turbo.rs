/// 
/// Overpass Turbo is website which let you query features of OpenStreetMap
///
/// This format has a couple of types:
/// 
/// - Node:
///     A point on the map
/// - Relation:
///     This contains relation members 
/// - Way:
///     It's a path on path defined by a list of nodes 
///
/// This crate also provides overpass turbo simplified 
/// where all the ids are resolved into actuall objects 
/// which makes way easier to work with. If you intend to 
/// work with the simplified version add "simple" into the features list.
///

pub mod model;
pub use model::*;

#[cfg(feature = "simple")]
pub mod simplified;
