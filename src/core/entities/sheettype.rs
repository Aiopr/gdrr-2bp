use std::hash::{Hash, Hasher};
use std::sync::atomic::AtomicUsize;

#[derive(Debug, PartialEq, Eq)]
pub struct SheetType{
    id: usize,
    width: u64,
    height: u64,
    value : u64,
}

impl SheetType{
    pub fn new (width: u64, height: u64, value: u64) -> SheetType{
        SheetType{
            id: 0,
            width,
            height,
            value
        }
    }

    pub fn id(&self) -> usize{
        self.id
    }



    pub fn width(&self) -> u64{
        self.width
    }

    pub fn height(&self) -> u64{
        self.height
    }

    pub fn value(&self) -> u64{
        self.value
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}

impl Hash for SheetType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

