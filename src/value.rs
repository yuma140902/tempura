/// [`Value`] is data read from a file by the [`Loader`](crate::loader::Loader).
/// Conceptually, it is the same as JSON.
/// It is hierarchical and has types such as object, string, and numeric.

pub struct Value(pub ron::Value);
