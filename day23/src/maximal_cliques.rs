/// taken from https://github.com/petgraph/petgraph/blob/cee17df1398adeae9f4696d3f4caba5662b37d63/src/algo/maximal_cliques.rs
/// which is licensend under an MIT+APACHE Licence
///
/// Copyright (c) 2015
///
/// Permission is hereby granted, free of charge, to any
/// person obtaining a copy of this software and associated
/// documentation files (the "Software"), to deal in the
/// Software without restriction, including without
/// limitation the rights to use, copy, modify, merge,
/// publish, distribute, sublicense, and/or sell copies of
/// the Software, and to permit persons to whom the Software
/// is furnished to do so, subject to the following
/// conditions:
///
/// The above copyright notice and this permission notice
/// shall be included in all copies or substantial portions
/// of the Software.
///
/// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
/// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
/// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
/// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
/// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
/// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
/// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
/// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
/// DEALINGS IN THE SOFTWARE.
use petgraph::visit::{GetAdjacencyMatrix, IntoNeighbors, IntoNodeIdentifiers};
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot<G>(
    g: G,
    adj_mat: &G::AdjMatrix,
    r: HashSet<G::NodeId>,
    mut p: HashSet<G::NodeId>,
    mut x: HashSet<G::NodeId>,
) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p.iter().max_by_key(|&v| g.neighbors(*v).count()).unwrap();
    let mut todo = p
        .iter()
        .filter(|&v| *u == *v || !g.is_adjacent(adj_mat, *u, *v) || !g.is_adjacent(adj_mat, *v, *u)) //skip neighbors of pivot
        .cloned()
        .collect::<Vec<G::NodeId>>();
    while let Some(v) = todo.pop() {
        let neighbors = HashSet::from_iter(g.neighbors(v));
        p.remove(&v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let next_p = p
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();
        let next_x = x
            .intersection(&neighbors)
            .cloned()
            .collect::<HashSet<G::NodeId>>();

        cliques.extend(bron_kerbosch_pivot(g, adj_mat, next_r, next_p, next_x));

        x.insert(v);
    }

    cliques
}

/// Find all maximal cliques in a graph using Bron–Kerbosch algorithm
/// with pivoting.
///
/// A clique is a set of nodes such that every node connects to
/// every other. A maximal clique is a clique that cannot be extended
/// by including one more adjacent vertex. A graph may have multiple
/// maximal cliques.
///
/// Example
/// ```
/// use petgraph::algo::maximal_cliques;
/// use petgraph::graph::UnGraph;
/// use std::collections::HashSet;
///
/// let mut g = UnGraph::<i32, ()>::from_edges(&[(0, 1), (0, 2), (1, 2), (2, 3)]);
/// g.add_node(4);
/// // The example graph:
/// //
/// // 0 --- 2 -- 3
/// //  \   /
/// //   \ /
/// //    1       4
/// //
/// // maximal cliques: {4}, {2, 3}, {0, 1, 2}
/// // Output the result
/// let cliques = maximal_cliques(&g);
/// println!("{:?}", cliques);
/// // [
/// //   {NodeIndex(4)},
/// //   {NodeIndex(0), NodeIndex(1), NodeIndex(2)},
/// //   {NodeIndex(2), NodeIndex(3)}
/// // ]
/// ```
pub fn maximal_cliques<G>(g: G) -> Vec<HashSet<G::NodeId>>
where
    G: GetAdjacencyMatrix + IntoNodeIdentifiers + IntoNeighbors,
    G::NodeId: Eq + Hash,
{
    let adj_mat = g.adjacency_matrix();
    let r = HashSet::new();
    let p = g.node_identifiers().collect::<HashSet<G::NodeId>>();
    let x = HashSet::new();
    bron_kerbosch_pivot(g, &adj_mat, r, p, x)
}
