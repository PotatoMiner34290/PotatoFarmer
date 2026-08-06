use serde::{Deserialize, Serialize};
#[derive(Clone,Copy,PartialEq)] pub enum CellState{ Grass, Plowed, Planted{growth:f32} }
#[derive(Serialize,Deserialize,Clone,Copy)] pub enum CellStateSave{ Grass, Plowed, Planted{growth:f32} }
impl From<CellState> for CellStateSave{ fn from(s:CellState)->Self{ match s{ CellState::Grass=>Self::Grass, CellState::Plowed=>Self::Plowed, CellState::Planted{growth}=>Self::Planted{growth} } } }
impl From<CellStateSave> for CellState{ fn from(s:CellStateSave)->Self{ match s{ CellStateSave::Grass=>Self::Grass, CellStateSave::Plowed=>Self::Plowed, CellStateSave::Planted{growth}=>Self::Planted{growth} } } }
