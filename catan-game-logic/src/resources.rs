use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign};
use std::ops::{Index, IndexMut};
use std::ops::{Mul, MulAssign};
use std::ops::{Sub, SubAssign};

use crate::building::Building;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Ore,
    Grain,
    Wool,
    Brick,
    Lumber,
}

use ResourceKind::*;

impl<S> From<S> for ResourceKind
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let value = value.as_ref().to_lowercase();
        match value.as_ref() {
            "ore" => Self::Ore,
            "grain" => Self::Grain,
            "wool" => Self::Wool,
            "brick" => Self::Brick,
            "lumber" => Self::Lumber,
            _ => panic!("Unrecognized resource"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Resources {
    ore: usize,
    grain: usize,
    lumber: usize,
    brick: usize,
    wool: usize,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            ore: 0,
            grain: 0,
            wool: 0,
            brick: 0,
            lumber: 0,
        }
    }

    pub fn new_explicit(
        ore: usize,
        grain: usize,
        wool: usize,
        brick: usize,
        lumber: usize,
    ) -> Self {
        Self {
            ore,
            grain,
            wool,
            brick,
            lumber,
        }
    }

    pub fn new_with_amount(amount: usize) -> Self {
        Self {
            ore: amount,
            grain: amount,
            wool: amount,
            brick: amount,
            lumber: amount,
        }
    }

    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn can_build(&self, infrastructure: Building) -> bool {
        let resource_requirements = infrastructure.get_resource_cost();
        resource_requirements
            .into_iter()
            .filter(|(_, count)| *count == 0)
            .all(|(kind, count)| self[kind] >= count)
    }
}

// Indexing using `ResourceKind` as a key
impl Index<ResourceKind> for Resources {
    type Output = usize;
    fn index(&self, index: ResourceKind) -> &Self::Output {
        match index {
            Ore => &self.ore,
            Grain => &self.grain,
            Wool => &self.wool,
            Brick => &self.brick,
            Lumber => &self.lumber,
        }
    }
}

impl IndexMut<ResourceKind> for Resources {
    fn index_mut(&mut self, index: ResourceKind) -> &mut Self::Output {
        match index {
            Ore => &mut self.ore,
            Grain => &mut self.grain,
            Wool => &mut self.wool,
            Brick => &mut self.brick,
            Lumber => &mut self.lumber,
        }
    }
}

impl Add<Resources> for Resources {
    type Output = Resources;
    fn add(self, rhs: Resources) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            grain: self.grain + rhs.grain,
            wool: self.wool + rhs.wool,
            brick: self.brick + rhs.brick,
            lumber: self.ore + rhs.ore,
        }
    }
}

impl AddAssign<Resources> for Resources {
    fn add_assign(&mut self, rhs: Resources) {
        self.ore += rhs.ore;
        self.grain += rhs.grain;
        self.wool += rhs.wool;
        self.brick += rhs.brick;
        self.lumber += rhs.lumber;
    }
}

impl Sub<Resources> for Resources {
    type Output = Resources;
    fn sub(self, rhs: Resources) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            grain: self.grain - rhs.grain,
            wool: self.wool - rhs.wool,
            brick: self.brick - rhs.brick,
            lumber: self.ore - rhs.ore,
        }
    }
}

impl SubAssign<Resources> for Resources {
    fn sub_assign(&mut self, rhs: Resources) {
        self.ore -= rhs.ore;
        self.grain -= rhs.grain;
        self.wool -= rhs.wool;
        self.brick -= rhs.brick;
        self.lumber -= rhs.lumber;
    }
}

impl Mul<usize> for Resources {
    type Output = Resources;

    fn mul(self, scalar: usize) -> Self::Output {
        Resources {
            ore: self.ore * scalar,
            grain: self.grain * scalar,
            wool: self.wool * scalar,
            brick: self.brick * scalar,
            lumber: self.lumber * scalar,
        }
    }
}

impl MulAssign<usize> for Resources {
    fn mul_assign(&mut self, scalar: usize) {
        self.ore *= scalar;
        self.grain *= scalar;
        self.wool *= scalar;
        self.brick *= scalar;
        self.lumber *= scalar;
    }
}

impl IntoIterator for Resources {
    type Item = (ResourceKind, usize);
    type IntoIter = std::array::IntoIter<Self::Item, 5>;

    fn into_iter(self) -> Self::IntoIter {
        [
            (Ore, self.ore),
            (Grain, self.grain),
            (Wool, self.wool),
            (Brick, self.brick),
            (Lumber, self.lumber),
        ]
        .into_iter()
    }
}

impl Default for Resources {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let r = Resources::new();
        assert_eq!(
            r,
            Resources {
                ore: 0,
                grain: 0,
                wool: 0,
                brick: 0,
                lumber: 0,
            }
        );

        let r = Resources::new_with_amount(20);
        assert_eq!(
            r,
            Resources {
                ore: 20,
                grain: 20,
                wool: 20,
                brick: 20,
                lumber: 20
            }
        );

        let r = Resources::new_explicit(5, 3, 2, 6, 2);
        assert_eq!(
            r,
            Resources {
                ore: 5,
                grain: 3,
                wool: 2,
                brick: 6,
                lumber: 2
            }
        );
    }

    #[test]
    fn test_index() {
        let r = Resources::new_with_amount(20);
        assert_eq!(r[Ore], 20);
        // checks that indexing with an invalid key panics
        let result = std::panic::catch_unwind(|| ResourceKind::from("foo"));
        assert!(result.is_err());
    }

    #[test]
    fn test_can_build() {
        let r = Building::Road.get_resource_cost();
        assert!(r.can_build(Building::Road));

        let r = Building::Settlement.get_resource_cost();
        assert!(r.can_build(Building::Settlement));

        let r = Building::City.get_resource_cost();
        assert!(r.can_build(Building::City));
    }
}
