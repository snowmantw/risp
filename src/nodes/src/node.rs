use std::string::String;

pub struct Node {
    // TODO: Use string is just for convenient. Replace it for saving the 24 byte as much as possible.
    // Maybe just a position of Lineno to the main text,
    // Since a size fixed "string" cannot be adjusted according to the input length.
    pub text: String
}
