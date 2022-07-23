
mod pow;
mod pos;


#[derive(Debug,Default)]
pub enum ConsensusType {
    Pow,
    Pos,
    Dpos,
    Poh,
    Poa
}
