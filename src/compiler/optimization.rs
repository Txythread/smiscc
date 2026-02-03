use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};
use crate::ArgumentList;

#[derive(Debug)]
pub struct OptimizationFlags {
    /// The optimization level specifies which optimizations should take
    /// place in a broad sense. 0 means (close to) none and 3 means all.
    opt_level: u8,

    /// The user can also specify which optimizations take place, overwriting
    /// the optimization level, which is why the level alone is not enough.
    /// This value always contains **all optimizations, no matter which were specified
    /// explicitly**.
    /// The second parameter in the tuple refers to whether the feature has been
    /// enabled *(true)* or disabled *(false)*.
    pub optimizations: HashMap<OptimizationKind, bool>,
}


impl OptimizationFlags {
    /// Generates [OptimizationFlags] given the general opt level
    /// and a string specifying specific operations either being turned on (no prefix)
    /// or off ('!' as a prefix).
    fn new_opt(opt_level: u8, flags: &Vec<String>) -> Self {
        let mut optimizations = HashMap::new();

        let mut flags = flags.clone();

        // TODO: This can be optimized by going over the flags and assigning by
        // TODO: optimization level and then going through all arguments.
        // TODO: Probably negligible though.

        for optimization in OptimizationKind::iter() {
            let mut is_activated = optimization.standard_optimization_level() <= opt_level;

            for flag in flags.clone().iter().enumerate() {
                if (&flag.1).starts_with("!") && &flag.1[1..] == optimization.as_ref() {
                    is_activated = false;
                } else if flag.1 == optimization.as_ref() {
                    is_activated = true;
                } else { continue; }

                flags.remove(flag.0);
            }

            optimizations.insert(optimization, is_activated);
        }


        OptimizationFlags { opt_level, optimizations }
    }

    pub fn new(args: &ArgumentList) -> Self {
        Self::new_opt(args.optimization_level, &args.optimizations)
    }
}



/// Specifies all kinds of optimizations that exist for all architectures and specifies
/// things such as the minimum opt level for the optimization to be enabled at default.
///
/// *Note:*: Names are always camel case; opt level 0 is the bare minimum, level 3 means all
/// optimizations apply if applicable.
#[derive(AsRefStr, Clone, Copy, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum OptimizationKind {
    /// Operations with the identity element (e.g. 1 for multiplication, 0 for addition
    /// or subtraction) will be skipped.
    #[strum(serialize = "removeIdentityOperations")]
    RemoveIdentityOperations,
}


impl OptimizationKind {
    /// Gets the minimum optimization level at which the optimization
    /// should be activated if not specified otherwise.
    pub fn standard_optimization_level(&self) -> u8 {
        use OptimizationKind as OK;
        match self {
            OK::RemoveIdentityOperations => 1,
        }
    }
}


#[cfg(test)]
mod tests{
    use std::rc::Rc;
    use strum::IntoEnumIterator;
    use crate::compiler::optimization::OptimizationKind;

    #[test]
    fn no_duplicate_optimization_names() {
        let mut names: Vec<String> = vec![];

        for optimization in OptimizationKind::iter() {
            if names.contains(&optimization.as_ref().to_string()){
                panic!("Duplicate optimization name: {}", optimization.as_ref());
            }

            names.push(optimization.as_ref().to_string());
        }
    }
}