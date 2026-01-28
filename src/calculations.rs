/// All the financial calculations for buy vs rent comparison
///
/// Fair comparison assuming same income/budget:
/// - Both scenarios start with the same money (down payment + closing costs worth)
/// - Both scenarios have the same monthly budget for housing
/// - Whoever spends less invests the difference

#[derive(Clone, Debug, PartialEq)]
pub struct Inputs {
    pub home_price: f64,
    pub down_payment_percent: f64,
    pub mortgage_rate: f64,
    pub loan_term_years: u32,
    pub property_tax_rate: f64,
    pub home_insurance: f64,
    pub hoa_monthly: f64,
    pub maintenance_percent: f64,
    pub home_appreciation: f64,
    pub closing_cost_percent: f64,
    pub selling_cost_percent: f64,
    pub monthly_rent: f64,
    pub rent_increase_rate: f64,
    pub renters_insurance: f64,
    pub investment_return: f64,
    pub time_horizon_years: u32,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            home_price: 400_000.0,
            down_payment_percent: 20.0,
            mortgage_rate: 6.5,
            loan_term_years: 30,
            property_tax_rate: 1.2,
            home_insurance: 1_500.0,
            hoa_monthly: 0.0,
            maintenance_percent: 1.0,
            home_appreciation: 3.0,
            closing_cost_percent: 3.0,
            selling_cost_percent: 6.0,
            monthly_rent: 2_000.0,
            rent_increase_rate: 3.0,
            renters_insurance: 200.0,
            investment_return: 7.0,
            time_horizon_years: 10,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct YearlySnapshot {
    pub year: u32,
    pub buy_net_worth: f64,
    pub rent_net_worth: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BuyBreakdown {
    pub down_payment: f64,
    pub closing_costs: f64,
    pub total_mortgage_payments: f64,
    pub total_interest_paid: f64,
    pub total_principal_paid: f64,
    pub total_property_tax: f64,
    pub total_insurance: f64,
    pub total_hoa: f64,
    pub total_maintenance: f64,
    pub selling_costs: f64,
    pub final_home_value: f64,
    pub remaining_mortgage: f64,
    // Buyer's investments (when buying is cheaper than renting)
    pub monthly_savings_invested: f64,
    pub investment_returns: f64,
    pub investment_balance: f64,
    pub net_worth: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct RentBreakdown {
    pub initial_investment: f64,         // Down payment + closing costs invested
    pub total_rent_paid: f64,
    pub total_renters_insurance: f64,
    pub monthly_cost_savings: f64,       // Total saved because rent < buy (can be negative)
    pub investment_returns: f64,         // Market gains on all invested money
    pub final_investment_value: f64,     // Total portfolio value
    pub net_worth: f64,
}

/// For displaying monthly cost comparison
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MonthlyCostComparison {
    pub avg_buy_monthly: f64,
    pub avg_rent_monthly: f64,
    pub avg_monthly_difference: f64,  // Positive = renting is cheaper
}

/// Monthly breakdown of where money goes
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MonthlyBreakdown {
    // Buy costs (monthly averages)
    pub buy_mortgage: f64,
    pub buy_property_tax: f64,
    pub buy_insurance: f64,
    pub buy_hoa: f64,
    pub buy_maintenance: f64,
    pub buy_total: f64,
    // Rent costs (monthly averages)
    pub rent_payment: f64,
    pub rent_insurance: f64,
    pub rent_total: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CalculationResult {
    pub buy_breakdown: BuyBreakdown,
    pub rent_breakdown: RentBreakdown,
    pub monthly_comparison: MonthlyCostComparison,
    pub monthly_breakdown: MonthlyBreakdown,
    pub difference: f64, // Positive means buying is better
    pub yearly_snapshots: Vec<YearlySnapshot>,
}

/// Calculate monthly mortgage payment using standard amortization formula
pub fn calculate_monthly_payment(principal: f64, annual_rate: f64, years: u32) -> f64 {
    if annual_rate == 0.0 {
        return principal / (years as f64 * 12.0);
    }
    let monthly_rate = annual_rate / 100.0 / 12.0;
    let n = years as f64 * 12.0;
    principal * (monthly_rate * (1.0 + monthly_rate).powf(n)) / ((1.0 + monthly_rate).powf(n) - 1.0)
}

/// Calculate remaining mortgage balance after a certain number of months
pub fn remaining_balance(principal: f64, annual_rate: f64, years: u32, months_paid: u32) -> f64 {
    if annual_rate == 0.0 {
        let monthly_payment = principal / (years as f64 * 12.0);
        return (principal - monthly_payment * months_paid as f64).max(0.0);
    }
    let monthly_rate = annual_rate / 100.0 / 12.0;
    let n = years as f64 * 12.0;
    let monthly_payment = calculate_monthly_payment(principal, annual_rate, years);

    if months_paid as f64 >= n {
        return 0.0;
    }

    principal * (1.0 + monthly_rate).powf(months_paid as f64)
        - monthly_payment * ((1.0 + monthly_rate).powf(months_paid as f64) - 1.0) / monthly_rate
}

pub fn calculate(inputs: &Inputs) -> CalculationResult {
    let down_payment = inputs.home_price * inputs.down_payment_percent / 100.0;
    let loan_amount = inputs.home_price - down_payment;
    let closing_costs = inputs.home_price * inputs.closing_cost_percent / 100.0;
    let initial_investment = down_payment + closing_costs;

    let monthly_mortgage = calculate_monthly_payment(loan_amount, inputs.mortgage_rate, inputs.loan_term_years);
    let monthly_home_insurance = inputs.home_insurance / 12.0;
    let monthly_renters_insurance = inputs.renters_insurance / 12.0;

    let monthly_investment_return = inputs.investment_return / 100.0 / 12.0;
    let monthly_appreciation = inputs.home_appreciation / 100.0 / 12.0;

    let total_months = inputs.time_horizon_years * 12;

    // === BUY SCENARIO TRACKING ===
    let mut total_mortgage_payments = 0.0;
    let mut total_property_tax = 0.0;
    let mut total_home_insurance = 0.0;
    let mut total_hoa = 0.0;
    let mut total_maintenance = 0.0;
    let mut current_home_value = inputs.home_price;
    let mut total_buy_monthly_costs = 0.0;

    // Buyer's investment account (for when buying is cheaper than renting)
    let mut buyer_investment_balance = 0.0;
    let mut buyer_total_contributions = 0.0;

    // === RENT SCENARIO TRACKING ===
    let mut total_rent_paid = 0.0;
    let mut total_renters_insurance = 0.0;
    let mut current_rent = inputs.monthly_rent;
    let mut total_rent_monthly_costs = 0.0;

    // Renter invests the down payment + closing costs
    // PLUS any monthly savings when renting is cheaper
    let mut renter_investment_balance = initial_investment;
    let mut renter_monthly_contributions = 0.0;

    let mut yearly_snapshots = Vec::new();

    for month in 1..=total_months {
        // === CALCULATE MONTHLY COSTS ===

        // Buy: mortgage (if still paying) + taxes + insurance + HOA + maintenance
        let paying_mortgage = month <= inputs.loan_term_years * 12;
        let mortgage_this_month = if paying_mortgage { monthly_mortgage } else { 0.0 };

        let property_tax_this_month = current_home_value * inputs.property_tax_rate / 100.0 / 12.0;
        let maintenance_this_month = current_home_value * inputs.maintenance_percent / 100.0 / 12.0;

        let buy_monthly_cost = mortgage_this_month
            + property_tax_this_month
            + monthly_home_insurance
            + inputs.hoa_monthly
            + maintenance_this_month;

        // Rent: rent + renter's insurance
        let rent_monthly_cost = current_rent + monthly_renters_insurance;

        // Track totals
        total_buy_monthly_costs += buy_monthly_cost;
        total_rent_monthly_costs += rent_monthly_cost;

        // === UPDATE BUY SCENARIO ===
        total_mortgage_payments += mortgage_this_month;
        total_property_tax += property_tax_this_month;
        total_home_insurance += monthly_home_insurance;
        total_hoa += inputs.hoa_monthly;
        total_maintenance += maintenance_this_month;
        current_home_value *= 1.0 + monthly_appreciation;

        // === UPDATE RENT SCENARIO ===
        total_rent_paid += current_rent;
        total_renters_insurance += monthly_renters_insurance;

        // === INVESTMENT LOGIC ===
        // Whoever spends less on housing invests the difference

        // Buyer's investments grow
        buyer_investment_balance *= 1.0 + monthly_investment_return;

        // Renter's investments grow
        renter_investment_balance *= 1.0 + monthly_investment_return;

        if buy_monthly_cost < rent_monthly_cost {
            // Buying is cheaper - BUYER invests the difference
            let savings = rent_monthly_cost - buy_monthly_cost;
            buyer_investment_balance += savings;
            buyer_total_contributions += savings;
        } else {
            // Renting is cheaper - RENTER invests the difference
            let savings = buy_monthly_cost - rent_monthly_cost;
            renter_investment_balance += savings;
            renter_monthly_contributions += savings;
        }

        // Rent increases annually
        if month % 12 == 0 {
            current_rent *= 1.0 + inputs.rent_increase_rate / 100.0;

            // Record yearly snapshot
            let year = month / 12;
            let months_paid = month.min(inputs.loan_term_years * 12);
            let remaining_mort = remaining_balance(loan_amount, inputs.mortgage_rate, inputs.loan_term_years, months_paid);

            let selling_costs_now = current_home_value * inputs.selling_cost_percent / 100.0;
            let buy_net_worth = current_home_value - remaining_mort - selling_costs_now + buyer_investment_balance;
            let rent_net_worth = renter_investment_balance;

            yearly_snapshots.push(YearlySnapshot {
                year,
                buy_net_worth,
                rent_net_worth,
            });
        }
    }

    // === FINAL CALCULATIONS ===

    let remaining_mortgage = remaining_balance(
        loan_amount,
        inputs.mortgage_rate,
        inputs.loan_term_years,
        total_months.min(inputs.loan_term_years * 12)
    );

    let selling_costs = current_home_value * inputs.selling_cost_percent / 100.0;

    // Buyer's net worth = home equity + any investments from monthly savings
    let buy_net_worth = current_home_value - remaining_mortgage - selling_costs + buyer_investment_balance;

    let total_principal_paid = loan_amount - remaining_mortgage;
    let total_interest_paid = total_mortgage_payments - total_principal_paid;

    // Renter's investment returns = final value - initial investment - monthly contributions
    let renter_investment_returns = renter_investment_balance - initial_investment - renter_monthly_contributions;

    // Buyer's investment returns (if any)
    let buyer_investment_returns = buyer_investment_balance - buyer_total_contributions;

    // Average monthly costs for display
    let avg_buy_monthly = total_buy_monthly_costs / total_months as f64;
    let avg_rent_monthly = total_rent_monthly_costs / total_months as f64;

    let buy_breakdown = BuyBreakdown {
        down_payment,
        closing_costs,
        total_mortgage_payments,
        total_interest_paid,
        total_principal_paid,
        total_property_tax,
        total_insurance: total_home_insurance,
        total_hoa,
        total_maintenance,
        selling_costs,
        final_home_value: current_home_value,
        remaining_mortgage,
        net_worth: buy_net_worth,
        // New fields for buyer's investments
        monthly_savings_invested: buyer_total_contributions,
        investment_returns: buyer_investment_returns,
        investment_balance: buyer_investment_balance,
    };

    let rent_breakdown = RentBreakdown {
        initial_investment,
        total_rent_paid,
        total_renters_insurance,
        monthly_cost_savings: renter_monthly_contributions,
        investment_returns: renter_investment_returns,
        final_investment_value: renter_investment_balance,
        net_worth: renter_investment_balance,
    };

    let monthly_comparison = MonthlyCostComparison {
        avg_buy_monthly,
        avg_rent_monthly,
        avg_monthly_difference: avg_buy_monthly - avg_rent_monthly,
    };

    // Monthly breakdown of where money goes
    let months = total_months as f64;
    let monthly_breakdown = MonthlyBreakdown {
        buy_mortgage: total_mortgage_payments / months,
        buy_property_tax: total_property_tax / months,
        buy_insurance: total_home_insurance / months,
        buy_hoa: total_hoa / months,
        buy_maintenance: total_maintenance / months,
        buy_total: avg_buy_monthly,
        rent_payment: total_rent_paid / months,
        rent_insurance: total_renters_insurance / months,
        rent_total: avg_rent_monthly,
    };

    let difference = buy_breakdown.net_worth - rent_breakdown.net_worth;

    CalculationResult {
        buy_breakdown,
        rent_breakdown,
        monthly_comparison,
        monthly_breakdown,
        difference,
        yearly_snapshots,
    }
}

/// Calculate the difference (buy net worth - rent net worth) for a given input value
pub fn calculate_difference_for_value(inputs: &Inputs, field: &str, value: f64) -> f64 {
    let mut modified = inputs.clone();
    match field {
        "home_price" => modified.home_price = value,
        "down_payment_percent" => modified.down_payment_percent = value,
        "mortgage_rate" => modified.mortgage_rate = value,
        "loan_term_years" => modified.loan_term_years = value as u32,
        "property_tax_rate" => modified.property_tax_rate = value,
        "home_insurance" => modified.home_insurance = value,
        "hoa_monthly" => modified.hoa_monthly = value,
        "maintenance_percent" => modified.maintenance_percent = value,
        "home_appreciation" => modified.home_appreciation = value,
        "closing_cost_percent" => modified.closing_cost_percent = value,
        "selling_cost_percent" => modified.selling_cost_percent = value,
        "monthly_rent" => modified.monthly_rent = value,
        "rent_increase_rate" => modified.rent_increase_rate = value,
        "renters_insurance" => modified.renters_insurance = value,
        "investment_return" => modified.investment_return = value,
        "time_horizon_years" => modified.time_horizon_years = value as u32,
        _ => {}
    }
    let result = calculate(&modified);
    result.difference
}

/// Generate sensitivity data for a slider
pub fn generate_sensitivity_data(inputs: &Inputs, field: &str, min: f64, max: f64, steps: usize) -> Vec<(f64, f64)> {
    let step_size = (max - min) / steps as f64;
    (0..=steps)
        .map(|i| {
            let value = min + step_size * i as f64;
            let diff = calculate_difference_for_value(inputs, field, value);
            (value, diff)
        })
        .collect()
}
