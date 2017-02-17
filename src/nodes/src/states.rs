
// Do something like to store the position of a string fragment,
// then give the next state to receive the next text segment.
//
// Compound state can be generated via the latest transferring:
//
// ListBegin -> AtomSymbol -> AtomSymbol -> ... -> ListEnd -> List
//
// The latest List get created because the chain successfully reach ListEnd. 
//

pub trait State {
    fn transfer(&self, node: State) -> State;
    fn extract(&self) -> Node;      // TODO: a ref? a copy-instance?
}
