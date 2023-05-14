use crate::quadtree::{coord::Coord, region::Region, tree::QuadTree};

use super::slot_map::SlotId;

macro_rules! region {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        Region::new(Coord::new($x, $y), Coord::new($w, $h))
    };
}

#[test]
fn insert() {
    let mut graph = QuadTree::<usize>::new(region!(0, 0, 10, 10));

    let reg1 = region!(0, 0, 5, 5);
    let reg2 = region!(5, 5, 10, 10);

    graph.insert(reg1, 1);
    graph.insert(reg2, 2);

    assert_eq!(graph.value_count(), 2);

    let exclude: Vec<SlotId> = vec![];
    let query_region = region!(0, 0, 10, 10);
    let result = graph.query(&query_region, &exclude);

    assert_eq!(result.len(), 2);

    let query_region = region!(0, 0, 4, 5);
    let result = graph.query(&query_region, &exclude);

    assert_eq!(result.len(), 1);
}

#[test]
fn query() {
    let mut graph = QuadTree::<usize>::new(region!(0, 0, 10, 10));

    let reg1 = region!(0, 0, 5, 5);
    let reg2 = region!(5, 5, 10, 10);
    let reg3 = region!(4, 4, 6, 6);

    let reg1_id = graph.insert(reg1, 1);
    let reg2_id = graph.insert(reg2, 2);
    let reg3_id = graph.insert(reg3, 3);

    // test query
    let exclude: Vec<SlotId> = vec![reg2_id];
    let query_region = region!(0, 0, 10, 10);
    let result = graph.query(&query_region, &exclude);
    assert_eq!(result.len(), 2);

    // test exclude
    let query_region = region!(6, 6, 7, 7);
    let exclude: Vec<SlotId> = vec![];
    let result = graph.query(&query_region, &exclude);
    assert_eq!(result.len(), 2);

    // test value result
    let query_region = region!(7, 7, 8, 8);
    let exclude: Vec<SlotId> = vec![];
    let result = graph.query(&query_region, &exclude);
    assert_eq!(result.len(), 1);

    let val = result[0].clone();
    assert_eq!(val, 2)

}
