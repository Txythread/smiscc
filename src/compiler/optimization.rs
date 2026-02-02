use strum_macros::{AsRefStr, EnumIter};

pub struct OptimizationFlags {
    /// The optimization level specifies which optimizations should take
    /// place in a broad sense. 0 means (close to) none and 3 means all.
    pub opt_level: u8,

    /// The user can also specify which optimizations take place, overwriting
    /// the optimization level.
    /// The second parameter in the tuple refers to whether the feature has been
    /// enabled *(true)* or disabled *(false)*.
    pub specified_optimizations: Vec<(OptimizationKind, bool)>,
}



/// Specifies all kinds of optimizations that exist for all architectures and specifies
/// things such as the minimum opt level for the optimization to be enabled at default.
///
/// *Note:*: Names are always camel case; opt level 0 is the bare minimum, level 3 means all
/// optimizations apply if applicable.
#[derive(AsRefStr, EnumIter)]
pub enum OptimizationKind {
    /// Operations with the identity element (e.g. 1 for multiplication, 0 for addition
    /// or subtraction) will be skipped.
    #[strum(serialize = "noIdentityOperations")]
    NoIdentityOperations,
}