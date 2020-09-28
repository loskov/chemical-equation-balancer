use num::integer::lcm;
use crate::{
    balancer_error::BalancerError,
    equation::Equation,
    item::Item,
    matrix::Matrix,
    parser::Parser,
    parser_error::ParserError,
};

pub struct Balancer {
    /// Equation
    equation: Equation,
    /// Matrix
    matrix: Matrix,
}

impl Balancer {
    /// Balancer constructor.
    pub fn new(equation: &str) -> Result<Balancer, ParserError> {
        let equation = match Parser::new(equation).parse_equation() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        let matrix = Balancer::get_initial_matrix(&equation);
        Ok(Balancer { equation, matrix })
    }

    /// Returns an initial matrix.
    fn get_initial_matrix(equation: &Equation) -> Matrix {
        let elements_names = equation.get_elements_names();
        let mut matrix = Matrix::new(
            elements_names.len() + 1,
            equation.reactants.len() + equation.products.len() + 1
        );

        for (i, element_name) in elements_names.iter().enumerate() {
            for (j, reactant) in equation.reactants.iter().enumerate() {
                matrix.cells[i][j] = reactant.count_element_by_name(element_name) as i32;
            }

            for (j, product) in equation.products.iter().enumerate() {
                matrix.cells[i][j + equation.reactants.len()]
                    = -(product.count_element_by_name(element_name) as i32);
            }
        }

        matrix
    }

    /// Solves a matrix.
    fn solve_matrix(&mut self) -> Result<(), BalancerError> {
        self.matrix.eliminate();
        let mut row_index = 0;

        for i in 0..self.matrix.rows_count {
            if self.count_nonzero_coefficients_in_row(i) > 1 {
                break;
            }

            row_index += 1
        }

        if row_index == self.matrix.rows_count - 1 {
            return Err(BalancerError::AllZeroSolution);
        }

        self.matrix.cells[self.matrix.rows_count - 1][row_index] = 1;
        self.matrix.cells[self.matrix.rows_count - 1][self.matrix.columns_count - 1] = 1;
        self.matrix.eliminate();
        Ok(())
    }

    /// Counts the non-zero coefficients in row by its index.
    fn count_nonzero_coefficients_in_row(&self, row_index: usize) -> usize {
        self.matrix.cells[row_index].iter().filter(|&x| *x != 0).count()
    }

    /// Extracts the coefficients.
    fn extract_coefficients(&self) -> Result<Vec<i32>, BalancerError> {
        let rows_count = self.matrix.rows_count;
        let columns_count = self.matrix.columns_count;

        if rows_count < columns_count - 1
            || self.matrix.cells[columns_count - 2][columns_count - 2] == 0 {
            return Err(BalancerError::MultipleIndependentSolutions);
        }

        let mut least_common_multiple = 1;

        for i in 0..(columns_count - 1) {
            least_common_multiple = lcm(least_common_multiple, self.matrix.cells[i][i]);
        }

        let mut coefficients = vec![];

        for i in 0..(columns_count - 1) {
            let coefficient = least_common_multiple
                / self.matrix.cells[i][i] * self.matrix.cells[i][columns_count - 1];
            coefficients.push(coefficient);
        }

        Ok(coefficients)
    }

    /// Checks the answer.
    fn check_answer(&self, coefficients: &[i32]) -> Result<(), BalancerError> {
        let reactants = &self.equation.reactants;
        let products = &self.equation.products;

        if reactants.len() + products.len() != coefficients.len() {
            return Err(BalancerError::MismatchNumberOfCoefficients);
        }

        if coefficients.iter().all(|&x| x == 0) {
            return Err(BalancerError::AllZeroSolution);
        }

        let mut sum = 0;
        let reactants_count = reactants.len();

        for x in &self.equation.get_elements_names() {
            for (i, y) in reactants.iter().enumerate() {
                sum += y.count_element_by_name(x) as i32 * coefficients[i];
            }

            for (i, y) in products.iter().enumerate() {
                sum -= y.count_element_by_name(x) as i32 * coefficients[i + reactants_count];
            }

            if sum != 0 {
                return Err(BalancerError::IncorrectBalance);
            }
        }

        Ok(())
    }

    /// Balances an equation.
    pub fn balance_equation(&mut self) -> Result<String, BalancerError> {
        if let Err(e) = self.solve_matrix() {
            return Err(e);
        }

        let coefficients = match self.extract_coefficients() {
            Ok(x) => x,
            Err(e) => return Err(e),
        };

        if let Err(e) = self.check_answer(&coefficients) {
            return Err(e);
        }

        Ok(self.equation.format(&coefficients))
    }
}

#[cfg(test)]
mod tests {
    use crate::balancer::Balancer;

    #[test]
    fn test_new() {
        // TODO
    }

    #[test]
    fn test_fill_matrix() {
        // TODO
    }

    #[test]
    fn test_solve_matrix() {
        // TODO
    }

    #[test]
    fn test_count_nonzero_coefficients_in_row() {
        // TODO
    }

    #[test]
    fn test_extract_coefficients() {
        // TODO
    }

    #[test]
    fn test_check_answer() {
        // TODO
    }

    #[test]
    fn test_balance_equation() {
        let molecular_equations = [
            ["H2 + O2 = H2O", "2\u{a0}H2 + O2 = 2\u{a0}H2O"],
            [
                "Fe + H2SO4 = Fe2(SO4)3 + SO2 + H2O",
                "2\u{a0}Fe + 6\u{a0}H2SO4 = Fe2(SO4)3 + 3\u{a0}SO2 + 6\u{a0}H2O",
            ],
            [
                "H2O + Pb(CH3COO)2 = CH3COOH + Pb(OH)2",
                "2\u{a0}H2O + Pb(CH3COO)2 = 2\u{a0}CH3COOH + Pb(OH)2",
            ],
            [
                "NaOH + Cl2 + Br2 = NaBrO3 + NaCl + H2O",
                "12\u{a0}NaOH + 5\u{a0}Cl2 + Br2 = 2\u{a0}NaBrO3 + 10\u{a0}NaCl + 6\u{a0}H2O",
            ],
            [
                "C6H12O6 + KMnO4 + H2SO4 = CO2 + K2SO4 + MnSO4 + H2O",
                "5\u{a0}C6H12O6 + 24\u{a0}KMnO4 + 36\u{a0}H2SO4 = 30\u{a0}CO2 + 12\u{a0}K2SO4 + 24\u{a0}MnSO4 + 66\u{a0}H2O",
            ],
        ];

        for x in &molecular_equations {
            assert_eq!(Balancer::new(x[0]).unwrap().balance_equation().unwrap(), x[1]);
        }

        let ionic_equations = [
            ["Fe{3+} + e = Fe", "Fe{3+} + 3\u{a0}e− = Fe"],
            ["Cl{5+} + e = Cl{-}", "Cl{5+} + 6\u{a0}e− = Cl"],
            ["CO3{2-} + H{+} = H2O + CO2", "CO3{2−} + 2\u{a0}H = H2O + CO2"],
            ["CaCO3 + H{+} = Ca{2+} + CO2 + H2O", "CaCO3 + 2\u{a0}H = Ca{2+} + CO2 + H2O"],
            [
                "Cr2O7{2-} + H{+} + e = Cr{3+} + H2O",
                "Cr2O7{2−} + 14\u{a0}H + 6\u{a0}e− = 2\u{a0}Cr{3+} + 7\u{a0}H2O",
            ],
        ];

        for x in &ionic_equations {
            assert_eq!(Balancer::new(x[0]).unwrap().balance_equation().unwrap(), x[1]);
        }
    }
}
