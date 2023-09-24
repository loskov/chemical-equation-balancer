#[derive(Debug)]
pub enum BalancerError {
    AllCoefficientsAreZero,
    CoefficientsAreIncorrectlyPlaced,
    MismatchInNumberOfCoefficients,
    ReactionCanBeEqualizedInInfiniteNumberOfWays,
}

impl BalancerError {
    /// Returns the description.
    pub fn get_description(&self) -> &str {
        match self {
            Self::AllCoefficientsAreZero => "All coefficients are zero.",
            Self::CoefficientsAreIncorrectlyPlaced => "The coefficients are incorrectly placed.",
            Self::MismatchInNumberOfCoefficients => "Mismatch in the number of coefficients.",
            Self::ReactionCanBeEqualizedInInfiniteNumberOfWays =>
                "The reaction can be equalized in an infinite number of ways.",
        }
    }
}
