#[derive(Debug)]
pub enum BalancerError {
    AllZeroSolution,
    IncorrectBalance,
    MismatchNumberOfCoefficients,
    MultipleIndependentSolutions,
}

impl BalancerError {
    pub fn get_description(&self) -> &'static str {
        match &self {
            BalancerError::AllZeroSolution => "Все коэффициенты равны нулю.",
            BalancerError::IncorrectBalance => "Неверно расставлены коэффициенты.",
            BalancerError::MismatchNumberOfCoefficients =>
                "Несоответствие количества коэффициентов",
            BalancerError::MultipleIndependentSolutions =>
                "Реакцию можно уравнять бесконечным числом способов",
        }
    }
}
