/// https://en.wikipedia.org/wiki/Disjoint-set_data_structure
pub struct DisjointSet {
    pub parents: Vec<usize>,
    pub ranks: Vec<usize>,
}

impl DisjointSet {
    pub fn new(n: usize) -> Self {
        let mut parents = vec![0; n + 1];
        for i in 1..=n {
            parents[i] = i;
        }
        let ranks = vec![0; n + 1];
        Self { parents, ranks }
    }

    // ≈ O(1)
    // Achieved with path compression:
    // when finding x, point itself and all its ancestor to root node
    pub fn find(&mut self, u: usize) -> usize {
        if self.parents[u] != u {
            self.parents[u] = self.find(self.parents[u]);
        }
        return self.parents[u];
    }

    // ≈ O(1)
    // Achieved with union by rank (merge low rank tree to high rank ones)
    pub fn union(&mut self, u: usize, v: usize) -> bool {
        let pu = self.find(u);
        let pv = self.find(v);
        if pu == pv {
            return false;
        }
        // Meger low rank tree into high rank tree
        if self.ranks[pu] > self.ranks[pv] {
            self.parents[pv] = pu;
        } else if self.ranks[pu] < self.ranks[pv] {
            self.parents[pu] = pv;
        } else {
            self.parents[pv] = pu;
            self.ranks[pu] += 1;
        }
        return true;
    }
}
