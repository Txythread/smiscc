use crate::compiler::data_types::object::{Object, ObjectType, Trait};
use crate::compiler::tokenizer::Token;
use crate::config::target::*;
use crate::util::math::*;
use uuid::Uuid;


/// ### Generates data types via tokens.
/// A struct that makes a data type able to generate
/// instances of itself from code in the form of an
/// [object](Object).
/// This also means that it has to be able to build
/// an [object type](ObjectType) that describes what
/// it can do and is referenced by all its children
/// (the objects) by its uuid.
pub trait Buildable {
    /// ### Create an object with tokens
    ///
    /// Try to create an [object](Object) given some tokens,
    /// look if this data type was explicitly requested by the
    /// user (i.g. `25u8`) or not (i.g. `25`). And generate an
    /// error if it fails regardless of it should've been built
    /// in the first place. The message should only be shown if
    /// `ambiguous` (in the [result](BuildResult)) is set to
    /// false.
    ///
    /// **Note:** The originally generated object type needs to
    /// be re-supplied as the uuid is required again. Don't
    /// re-generate it for this purpose, as the uuid might differ.
    fn build(&self, tokens: Vec<Token>, parent_type: ObjectType) -> BuildResult;

    /// ### Create an object type
    ///
    /// Create an [object type](ObjectType), which is necessary
    /// for building instances of the type.
    fn build_type(&self) -> ObjectType;
}


/// The result after trying to generate a [buildable](Buildable) object.
#[derive(Clone)]
pub struct BuildResult {
    /// ### The Resulting object or an error
    ///
    /// An error is not necessarily negative.
    /// If the result is ambiguous, an error might not
    /// be displayed at all.
    pub result: Result<Object, ObjectBuildingError>,


    /// ### If it is this data type for sure.
    ///
    /// If there is no other way to way of building an object,
    /// it's clear from the code that that's the correct
    /// interpretation, it's unambiguous, so to say, this is
    /// set to false. If building that object from that builder
    /// is possible, but not clearly specified, this is true.
    ///
    /// This might be set to unambiguous although no result
    /// exists when it should be that data type for sure,
    /// but is malformed.
    pub ambiguous: bool,
}


impl BuildResult {
    pub fn new(result: Result<Object, ObjectBuildingError>, ambiguous: bool) -> Self {
        BuildResult { result, ambiguous }
    }
}


/// Contains information about an error that arose from trying
/// to build an object via the [buildable trait](Buildable)
#[derive(Debug, Clone)]
pub struct ObjectBuildingError {
    /// The name of the object that should've been built.
    pub expected_object: String,

    /// The message displayed as an error
    pub message: String,
}

impl ObjectBuildingError {
    pub fn new(expected_object: String, message: String) -> Self {
        ObjectBuildingError { expected_object, message }
    }
}





