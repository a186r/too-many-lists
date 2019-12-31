#[derive(Debug)]
struct Node{
    nodeid: usize,
    nodename: String,
}

#[derive(Debug, Clone)]
struct Edge{
    edge: bool,
}

#[derive(Debug)]
struct Graphadj{
    nodenums: usize,
    graphadj: Vec<Vec<Edge>>,
}

impl Node{
    fn new(nodeid: usize, nodename: String) -> Node{
        Node{
            nodeid,
            nodename
        }
    }
}

impl Edge{
    fn new() -> Edge{
        Edge{
            edge: false,
        }
    }

    fn have_edge() -> Edge{
        Edge{
            edge: true
        }
    }
}

impl Graphadj {
    fn new(nums: usize) -> Graphadj{
        Graphadj{
            nodenums: nums,
            graphadj: vec![
                vec![
                    Edge::new();
                    nums
                ];
                nums
            ],
        }
    }

    fn insert_edge(&mut self, v1: Node, v2: Node){
        match v1.nodeid < self.nodenums && v2.nodeid < self.nodenums{
            true => {
                self.graphadj[v1.nodeid][v2.nodeid] = Edge::have_edge();
            }
            false => {
                panic!("your nodeid is bigger than nodenums!");
            }
        }
    }
}