use std::mem::variant_count;
use std::ops::Index;

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use petgraph::graph::Node;
use petgraph::prelude::*;

use crate::building::Building;
use crate::resources::ResourceKind;
use crate::Game;

pub const DEFAULT_TILE_COUNT: usize = 19;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HarborKind {
    Generic,
    Special(ResourceKind),
}

impl HarborKind {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..=variant_count::<HarborKind>() - 1) {
            0 => HarborKind::Generic,
            1 => HarborKind::Special(ResourceKind::random()),
            n => panic!("Invalid index, i: {}", n),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TileKind {
    Resource(ResourceKind),
    Desert,
    ResourceWithHarbor(HarborKind, ResourceKind),
}

use TileKind::*;

impl TileKind {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        match rng.gen_range(0..=variant_count::<TileKind>() - 1) {
            0 => Resource(ResourceKind::random()),
            1 => Desert,
            2 => ResourceWithHarbor(HarborKind::random(), ResourceKind::random()),
            n => panic!("Invalid index, i: {}", n),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    kind: TileKind,
    #[serde(with = "uuid::serde::compact")]
    id: Uuid,
    token: usize,
    intersections: [Option<Building>; 6],
}

impl Tile {
    pub fn new(kind: TileKind, token: usize) -> Self {
        Self {
            kind,
            id: Uuid::new_v4(),
            token,
            intersections: [None; 6],
        }
    }

    pub fn random() -> Self {
        let (d1, d2) = Game::roll_dice();
        let token = (d1 + d2) as usize;
        Self {
            kind: TileKind::random(),
            id: Uuid::new_v4(),
            token,
            intersections: [None; 6],
        }
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn kind(&self) -> &TileKind {
        &self.kind
    }

    pub fn token(&self) -> &usize {
        &self.token
    }

    pub fn intersections(&self) -> &[Option<Building>] {
        &self.intersections
    }
}

impl Default for Tile {
    fn default() -> Self {
        let roll = Game::roll_dice();
        let roll = roll.0 + roll.1;
        Self {
            kind: TileKind::random(),
            id: Uuid::new_v4(),
            token: roll as usize,
            intersections: [None; 6],
        }
    }
}

/// Helper macro to make generating graphs with connections between nodes easier
macro_rules! graph {
    ($graph:ident, $node_refs:ident, [$([$from:expr => [$($to:expr),*]]),*]) => {{
        $graph.extend_with_edges([$($(($node_refs[$from - 1], $node_refs[$to - 1]),)*)*]);
    }}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board(UnGraph<Tile, Option<Building>>);

impl Board {
    pub fn new() -> Self {
        let mut graph: UnGraph<Tile, Option<Building>> = UnGraph::new_undirected();
        let mut ids: Vec<_> = Vec::new();
        for _ in 0..DEFAULT_TILE_COUNT {
            ids.push(graph.add_node(Tile::random()));
        }

        // FIXME: There's probably a good way to extend this to game boards
        // with >= 7 tiles in diameter, but this works fine for now
        graph!(graph, ids, [
               [1 => [2, 4, 5]],
               [2 => [1, 5, 6, 3]],
               [3 => [2, 6, 7]],
               [4 => [1, 5, 9, 8]],
               [5 => [1, 2, 6, 10, 9, 4]],
               [6 => [2, 3, 7, 11, 10, 5]],
               [7 => [3, 6, 11, 12]],
               [8 => [4, 9, 13]],
               [9 => [4, 5, 10, 14, 13, 8]],
               [10 => [5, 6, 11, 15, 14, 9]],
               [11 => [6, 7, 12, 16, 15, 10]],
               [12 => [7, 11, 16]],
               [13 => [8, 9, 14, 17]],
               [14 => [9, 10, 11, 16, 19, 18, 14]],
               [15 => [10, 11, 16, 19, 18, 14]],
               [16 => [12, 11, 15, 19]],
               [17 => [13, 14, 18]],
               [18 => [17, 14, 15, 19]],
               [19 => [18, 15, 16]]
        ]);

        Board(graph)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self(UnGraph::new_undirected())
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        let nodes_match = self
            .0
            .node_indices()
            .zip(other.0.node_indices())
            .all(|(lhs_i, rhs_i)| self.0[lhs_i] == other.0[rhs_i]);

        let edges_match = self.0.edge_indices().all(|idx| self.0[idx] == other.0[idx]);

        nodes_match && edges_match
    }
}

impl Index<usize> for Board {
    type Output = Node<Tile>;
    fn index(&self, target: usize) -> &Self::Output {
        if target > DEFAULT_TILE_COUNT {
            panic!("Index out of bounds");
        }
        &self.0.raw_nodes()[target]
    }
}

impl Eq for Board {}
#[cfg(test)]
mod test {
    use std::panic::catch_unwind;

    use uuid::Uuid;

    use super::{Board, Tile};

    #[test]
    fn test_random() {
        let res = catch_unwind(|| {
            (0..10).for_each(|_| {
                Tile::random();
            })
        });
        assert!(res.is_ok());
    }

    #[test]
    fn test_init() {
        let b = Board::new();

        for node_idx in b.0.node_indices() {
            let node = b.0[node_idx];
            assert!(Uuid::parse_str(&node.id().to_string()).is_ok());
            assert!(2 <= *node.token() && *node.token() <= 12)
        }

        assert_eq!(b.0.node_count(), 19);
        assert_eq!(b.0.edge_count(), 85);
    }

    #[test]
    fn test_serde() {
        let b = Board::new();

        let ser = serde_json::to_string(&b).unwrap();
        let de: Board = serde_json::from_str(&ser).unwrap();
        assert_eq!(b, de);
    }
}
