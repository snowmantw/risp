
// Do something like to store the position of a string fragment,
// then give the next state to receive the next text segment.
//
// Compound state can be generated via the latest transferring:
//
// ListBegin -> AtomSymbol -> AtomSymbol -> ... -> ListEnd -> List
//
// The latest List get created because the chain successfully reach ListEnd. 
//

use node::Node;

// The problem is State cannot be a return value.
// The size is undefined, unlike a structure (trait is a static thing).
//
// Also, Rust is designed to know the concrete item after a function call,
// during compilation time:
//
//      let x = instance.foo();
//
// So if struct instance a, b both implemenet the trait T and the foo() is a T,
// Rust cannot know which concrete item it is.
//
// http://www.ncameron.org/blog/abstract-return-types-aka-%60impl-trait%60/
//
pub trait State {
    fn transfer(&self, node: State) -> State;
    fn extract(&self) -> Node;      // TODO: a ref? a copy-instance?
}
