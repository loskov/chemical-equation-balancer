#[derive(Debug)]
pub enum BalancerError {
    AllZeroSolution,
    IncorrectBalance,
    MismatchNumberOfCoefficients,
    MultipleIndependentSolutions,
}

impl BalancerError {
    /// Returns the description.
    pub fn get_description(&self) -> &str {
        match self {
            Self::AllZeroSolution => "Все коэффициенты равны нулю.",
            Self::IncorrectBalance => "Неверно расставлены коэффициенты.",
            Self::MismatchNumberOfCoefficients => "Несоответствие количества коэффициентов.",
            Self::MultipleIndependentSolutions =>
                "Реакцию можно уравнять бесконечным числом способов.",
        }
    }
}
