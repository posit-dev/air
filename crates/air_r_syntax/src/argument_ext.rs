use crate::RArgument;

impl RArgument {
    /// Is this argument a "hole"?
    ///
    /// To be a hole, the argument must be missing both its `name =` clause
    /// and its value.
    ///
    /// ```r
    /// # First argument is a hole
    /// fn( , x)
    ///
    /// # First argument is not a hole
    /// fn(x = , x)
    /// ```
    pub fn is_hole(&self) -> bool {
        self.name_clause().is_none() && self.value().is_none()
    }
}
