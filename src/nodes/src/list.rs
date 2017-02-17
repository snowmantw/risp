
use node::Node;
use states::State;

pub struct ListBegin {
    pub node: Node
}

impl State for ListBegin {
    fn extract(&self) -> Node {
        return self.node;
    }
}

pub struct ListEnd {
    pub node: Node
}

impl State for ListEnd {
    fn extract(&self) -> Node {
        return self.node;
    }
}
