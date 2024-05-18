use crate::peer::PeerWire;


#[derive(Debug, Clone)]
pub struct PiecePicker<'a> {
    peers: &'a Vec<PeerWire>,
}

impl PiecePicker<'_> {
    pub fn new(peers: &Vec<PeerWire>) -> PiecePicker {
        PiecePicker {
            peers,
        }
    }
}
