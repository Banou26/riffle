
pub struct Block {
    pub piece_index: u32,
    pub begin: u32,
    pub length: u32,
    pub data: Vec<u8>,
}

pub struct Piece {
    pub index: u32,
    pub blocks: Vec<Block>,
    pub length: u32,
    pub hash: String,
    pub is_complete: bool,
    pub data: Vec<u8>
}

impl Piece {
    pub fn new(index: u32, length: u32, hash: String) -> Self {
        Self {
            index,
            blocks: vec![],
            length,
            hash,
            is_complete: false,
            data: vec![0; length as usize],
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn is_complete(&self) -> bool {
        self.is_complete
    }

    pub fn set_complete(&mut self) {
        self.is_complete = true;
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }
}
