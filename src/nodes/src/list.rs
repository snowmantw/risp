
use node::Node;
use states::State;

pub struct ListBegin {
    pub node: Node
}

impl State for ListBegin {
    fn transfer(&self, node: State) -> State {
        return node;
    }

    fn extract(&self) -> Node {
        return self.node;
    }
}

pub struct ListEnd {
    pub node: Node
}

impl State for ListEnd {
    fn transfer(&self, node: State) -> State {
        return node;
    }

    fn extract(&self) -> Node {
        return self.node;
    }
}
