# Chemical Equation Balancer

## Algorithm

1. Parse the equation of a chemical reaction using recursive descent.
2. Create the system of linear algebraic equations in matrix form.
3. Compute the reduced row echelon form of the matrix.
4. Extract the calculated coefficients from the transformed matrix.

## Rules

* The reactants and products of a chemical reaction must be entered without coefficients.
* Every chemical symbol must begin with a capital letter.
* The ion charge must be enclosed in curly brackets, the sign comes after the number: `{3+}`, `{2-}`.
* An electron must be written as `e` or `e{-}`.

## Examples

Molecular equations:
* `H2 + O2 = H2O`.
* `Fe + H2SO4 = Fe2(SO4)3 + SO2 + H2O`.
* `H2O + Pb(CH3COO)2 = CH3COOH + Pb(OH)2`.
* `NaOH + Cl2 + Br2 = NaBrO3 + NaCl + H2O`.
* `C6H12O6 + KMnO4 + H2SO4 = CO2 + K2SO4 + MnSO4 + H2O`.

Ionic equations:
* `Fe{3+} + e = Fe`.
* `Cl{5+} + e = Cl{-}`.
* `CO3{2-} + H{+} = H2O + CO2`.
* `CaCO3 + H{+} = Ca{2+} + CO2 + H2O`.
* `Cr2O7{2-} + H{+} + e = Cr{3+} + H2O`.

## Usage

```shell script
# Compile and run by passing the equation.
cargo run "H2 + O2 = H2O"

# Compile the release version.
cargo build --release

# Run the tests.
cargo test
```
