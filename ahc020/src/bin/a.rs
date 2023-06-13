// 住居とそこから最も近い砲塔塔を見つける
// その際、最も砲塔等から離れている家の座標と、放送等がアクセスできる住居の数を覚えておく
// アイディア1
// 放送等から最も遠い家に電波が届く強度で電波を飛ばす
use std::cmp::Ordering;
use std::collections::HashMap;

use num_integer::Roots;
use proconio::input;

struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Node {
    index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Edge {
    source: Node,
    destination: Node,
    weight: usize,
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn find_min_spanning_tree(edges: &[(usize, usize, usize)]) -> HashMap<(usize, usize), usize> {
    let mut nodes: HashMap<usize, Node> = HashMap::new();
    let mut graph: Vec<Edge> = Vec::new();

    for (source_index, destination_index, weight) in edges {
        let source = nodes
            .entry(*source_index)
            .or_insert(Node {
                index: *source_index,
            })
            .clone();
        let destination = nodes
            .entry(*destination_index)
            .or_insert(Node {
                index: *destination_index,
            })
            .clone();

        graph.push(Edge {
            source,
            destination,
            weight: *weight,
        });
    }

    graph.sort();

    let mut parent: HashMap<Node, Node> = HashMap::new();
    for (_, node) in nodes {
        parent.insert(node.clone(), node.clone());
    }

    let mut min_spanning_tree: HashMap<(usize, usize), usize> = HashMap::new();

    for edge in graph {
        let source_parent = find_parent(&parent, &edge.source);
        let destination_parent = find_parent(&parent, &edge.destination);

        if source_parent != destination_parent {
            min_spanning_tree.insert((edge.source.index, edge.destination.index), edge.weight);
            parent.insert(source_parent, destination_parent);
        }
    }

    min_spanning_tree
}

fn find_parent(parent: &HashMap<Node, Node>, node: &Node) -> Node {
    let mut current = node.clone();

    while parent[&current] != current {
        current = parent[&current].clone();
    }

    current
}

fn find_nearest_point(points: &[Point], target: Point) -> Option<(&Point, isize)> {
    if points.is_empty() {
        return None;
    }

    let mut nearest_point = &points[0];
    let mut min_distance = calculate_distance(nearest_point, &target);

    let mut distance = 0;
    for point in points {
        distance = calculate_distance(point, &target);
        if distance < min_distance {
            nearest_point = point;
            min_distance = distance;
        }
    }

    Some((nearest_point, distance))
}

fn calculate_distance(p1: &Point, p2: &Point) -> isize {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    ((dx * dx) + (dy * dy) as isize).sqrt() as isize
}

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        // 放送塔の座標
        _xy: [(isize, isize); n],
        // 放送塔と放送塔を結ぶ辺の重み
        edges: [(usize, usize, usize); m],
        // 住民の座標
        _ab: [(isize, isize); k],
    };

    // 放送塔の座標：放送塔から最も遠い家の距離、放送塔が最寄りの家の数、放送塔から最も遠い家の座標
    let mut value_of_plant: HashMap<(isize, isize), (isize, usize, (isize, isize))> =
        HashMap::new();
    let points: Vec<_> = _xy.iter().map(|(x, y)| Point { x: *x, y: *y }).collect();

    for (a, b) in &_ab {
        let target = Point { x: *a, y: *b };

        let (nearest_point, distance) = find_nearest_point(&points, target).unwrap();

        if let Some((dist, num, pos)) = value_of_plant.get_mut(&(nearest_point.x, nearest_point.y))
        {
            *num += 1;
            if distance > *dist {
                *dist = distance;
                *pos = (*a, *b);
            }
        } else {
            value_of_plant.insert((nearest_point.x, nearest_point.y), (distance, 0, (*a, *b)));
        }
    }

    for i in 0..n {
        if let Some(val) = value_of_plant.get(&_xy[i]) {
            let power = val.0;
            if power > 5000 {
                print!("2500 ");
            } else {
                print!("{} ", power);
            }
        } else {
            print!("0 ");
        }
    }

    let min_spanning_tree = find_min_spanning_tree(&edges);
    for i in 0..m {
        let (start, end, _) = edges[i];
        if let Some(_) = min_spanning_tree.get(&(start, end)) {
            print!("1 ");
        } else {
            print!("0 ");
        }
    }
}
