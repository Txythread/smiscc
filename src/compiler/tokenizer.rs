    use crate::compiler::line_map::LineMap;


/// Turn the split string into tokens ("classify" them)
///
/// A simple `"10"` would be turned into ImmediateValue(Signed32BitNumber(10)) or whatever the default number system is set to.
pub fn tokenize(separated: Vec<Vec<String>>, line_map: &mut LineMap) {

}