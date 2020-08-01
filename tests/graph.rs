use rmu::combinatorics::graph::*;

#[test]
fn graph() {
    let mut g = Graph::<()>::new();

    assert_eq!(g.add_node("a".into(),()),None);
    assert_eq!(g.add_node("b".into(),()),None);
    assert_eq!(g.add_node("c".into(),()),None);
    assert_eq!(g.add_node("c".into(),()),Some(()));

    assert_eq!(g.connect("i".into(),"a".into(), "b".into()),Ok(None));
    assert_eq!(g.connect("j".into(),"a".into(), "c".into()),Ok(None));
    assert_eq!(g.connect("k".into(),"a".into(), "d".into()),Err(GraphConnectError::NodeNotExit));
}


#[test]
fn direct_graph() {
    let mut dg = DGraph::<()>::new();

    assert_eq!(dg.add_node("a".into(),()),None);
    assert_eq!(dg.add_node("b".into(),()),None);
    assert_eq!(dg.add_node("c".into(),()),None);
    assert_eq!(dg.add_node("c".into(),()),Some(()));

    assert_eq!(dg.connect("a".into(), "b".into()),Ok(true));
    assert_eq!(dg.connect("a".into(), "c".into()),Ok(true));
    assert_eq!(dg.connect("a".into(), "d".into()),Err(GraphConnectError::NodeNotExit));

    assert_eq!(dg.acyclic_connect("b".into(),"c".into()), Ok(true));
    assert_eq!(dg.acyclic_connect("b".into(),"a".into()), Err(GraphConnectError::CyclicConnect));
}