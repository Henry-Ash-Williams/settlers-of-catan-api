use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ResourceKind {
    Ore,
    Grain, 
    Wool, 
    Brick,
    Lumber
}

impl From<&str> for ResourceKind {
    fn from(value: &str) -> Result<Self> {
        match value {
            "ore" => Ok(Self::Ore), 
            "grain" => Ok(Self::Grain), 
            "wool" => Ok(Self::Wool), 
            "brick" => Ok(Self::Brick), 
            "lumber" => Ok(Self::Lumber), 
            _ => Err(anyhow!("Key not found in"))
        }
    }
}

pub struct Resources (HashMap<ResourceKind, usize>);

impl Resources {
    pub fn new() -> Self {
        Self ( HashMap::from([
                             (ResourceKind::Ore, 0),
                             (ResourceKind::Grain, 0),
                             (ResourceKind::Wool, 0),
                             (ResourceKind::Brick, 0),
                             (ResourceKind::Lumber, 0)
        ]) )
    }

    pub fn new_with_amount(amount: usize) -> Self {
        Self ( HashMap::from([
                             (ResourceKind::Ore, amount),
                             (ResourceKind::Grain, amount),
                             (ResourceKind::Wool, amount),
                             (ResourceKind::Brick, amount),
                             (ResourceKind::Lumber, amount)
        ]) )
    }
}

// Indexing using `ResourceKind` as a key
impl Index<ResourceKind> for Resources {
    type Output = usize;
    fn index(&self, index: ResourceKind) -> &Self::Output {
        self.0.get(&index).unwrap()
    }
}

impl IndexMut<ResourceKind> for Resources {
    fn index_mut(&mut self, index: ResourceKind) -> &mut Self::Output {
        self.0.get_mut(&index).unwrap()
    }
}

// Indexing using `&str` as a key
impl Index<&str> for Resources {
    type Output = usize;
    fn index(&self, index: &str) -> &Self::Output {
        let index = index.to_lowercase();
        &self[ResourceKind::from(index.())]

        
    }
}
