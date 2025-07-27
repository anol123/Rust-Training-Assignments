// Define custom error types for Age and Income issues
#[derive(Debug)]
enum LoanError {
    AgeError(String),
    IncomeError(String),
}

// Check if the applicant's age is within valid range
fn check_age(age: u32) -> Result<(), LoanError> {
    if age < 21 || age > 60 {
        Err(LoanError::AgeError(format!("Age {} is not eligible", age)))
    } else {
        Ok(())
    }
}

// Check if the applicant's income is above the minimum requirement
fn check_income(income: u32) -> Result<(), LoanError> {
    if income < 30000 {
        Err(LoanError::IncomeError(format!("Income {} is too low", income)))
    } else {
        Ok(())
    }
}

// Handle optional co-applicant income
fn get_coapplicant_income(co_applicant: Option<u32>) -> u32 {
    match co_applicant {
        Some(income) => {
            println!("Co-applicant income considered: {}", income);
            income
        },
        None => 0,
    }
}

// Main eligibility logic
fn is_eligible_for_loan(
    income: u32,
    age: u32,
    loan_amount: u32,
    co_applicant: Option<u32>,
) -> Result<String, LoanError> {
    check_age(age)?;         // Return error if age is invalid
    check_income(income)?;   // Return error if income is too low

    let total_income = income + get_coapplicant_income(co_applicant);

    if total_income >= loan_amount / 2 {
        Ok(format!("✅ Loan Approved! Total income: {}", total_income))
    } else {
        Err(LoanError::IncomeError(format!(
            "❌ Combined income {} is too low for loan {}",
            total_income, loan_amount
        )))
    }
}

fn main() {
    println!("--- Loan Approval System ---");

    // Hardcoded inputs
    let income = 225000;
    let age = 28;
    let loan_amount = 50000;
    let co_applicant_income = Some(10000); // Change to None to test without co-applicant

    // Run eligibility check
    match is_eligible_for_loan(income, age, loan_amount, co_applicant_income) {
        Ok(msg) => println!("{}", msg),
        Err(LoanError::AgeError(e)) => println!("Age Error: {}", e),
        Err(LoanError::IncomeError(e)) => println!("Income Error: {}", e),
    }
}
