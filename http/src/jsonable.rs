/// Serialization trait.
/// 
/// 
/// Types which implement `Jsonable` can be converted into a valid json string.
pub trait Jsonable {

    /// Converts the struct into a valid json string.
    fn into_json(&self) -> String;


    /// Converts a valid json string into the given type.
    fn from_json(json_string: &str) -> Self;
}