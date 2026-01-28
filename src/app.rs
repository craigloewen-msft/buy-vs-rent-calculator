use leptos::*;
use wasm_bindgen::prelude::*;
use crate::calculations::{self, Inputs, CalculationResult, generate_sensitivity_data};

fn call_create_or_update_chart(canvas_id: &str, labels: &[String], buy_data: &[f64], rent_data: &[f64]) {
    let window = web_sys::window().unwrap();
    let func = js_sys::Reflect::get(&window, &JsValue::from_str("createOrUpdateChart"))
        .unwrap()
        .dyn_into::<js_sys::Function>()
        .unwrap();

    let labels_array = js_sys::Array::new();
    for label in labels {
        labels_array.push(&JsValue::from_str(label));
    }

    let buy_array = js_sys::Array::new();
    for &val in buy_data {
        buy_array.push(&JsValue::from_f64(val));
    }

    let rent_array = js_sys::Array::new();
    for &val in rent_data {
        rent_array.push(&JsValue::from_f64(val));
    }

    let _ = func.call4(
        &JsValue::NULL,
        &JsValue::from_str(canvas_id),
        &labels_array,
        &buy_array,
        &rent_array,
    );
}

#[component]
pub fn App() -> impl IntoView {
    // Create signals for all inputs
    let (home_price, set_home_price) = create_signal(400_000.0);
    let (down_payment_percent, set_down_payment_percent) = create_signal(20.0);
    let (mortgage_rate, set_mortgage_rate) = create_signal(6.5);
    let (loan_term_years, set_loan_term_years) = create_signal(30.0);
    let (property_tax_rate, set_property_tax_rate) = create_signal(1.2);
    let (home_insurance, set_home_insurance) = create_signal(1_500.0);
    let (hoa_monthly, set_hoa_monthly) = create_signal(0.0);
    let (maintenance_percent, set_maintenance_percent) = create_signal(1.0);
    let (home_appreciation, set_home_appreciation) = create_signal(3.0);
    let (closing_cost_percent, set_closing_cost_percent) = create_signal(3.0);
    let (selling_cost_percent, set_selling_cost_percent) = create_signal(6.0);
    let (monthly_rent, set_monthly_rent) = create_signal(2_000.0);
    let (rent_increase_rate, set_rent_increase_rate) = create_signal(3.0);
    let (renters_insurance, set_renters_insurance) = create_signal(200.0);
    let (investment_return, set_investment_return) = create_signal(7.0);
    let (time_horizon_years, set_time_horizon_years) = create_signal(10.0);

    // Derived signal that creates Inputs struct
    let inputs = create_memo(move |_| Inputs {
        home_price: home_price.get(),
        down_payment_percent: down_payment_percent.get(),
        mortgage_rate: mortgage_rate.get(),
        loan_term_years: loan_term_years.get() as u32,
        property_tax_rate: property_tax_rate.get(),
        home_insurance: home_insurance.get(),
        hoa_monthly: hoa_monthly.get(),
        maintenance_percent: maintenance_percent.get(),
        home_appreciation: home_appreciation.get(),
        closing_cost_percent: closing_cost_percent.get(),
        selling_cost_percent: selling_cost_percent.get(),
        monthly_rent: monthly_rent.get(),
        rent_increase_rate: rent_increase_rate.get(),
        renters_insurance: renters_insurance.get(),
        investment_return: investment_return.get(),
        time_horizon_years: time_horizon_years.get() as u32,
    });

    // Calculate results
    let result = create_memo(move |_| calculations::calculate(&inputs.get()));

    view! {
        <div class="container">
            <h1>"Buy vs Rent Calculator"</h1>
            <p class="subtitle">"Compare the true cost of buying a home versus renting"</p>

            <ResultBanner result=result />

            <div class="inputs-section">
                <div class="section-title">"Time Horizon"</div>
                <SliderInput
                    label="How long do you plan to stay?"
                    value=time_horizon_years
                    set_value=set_time_horizon_years
                    min=1.0
                    max=30.0
                    step=1.0
                    format_value=|v| format!("{} years", v as u32)
                    field="time_horizon_years"
                    inputs=inputs
                />
            </div>

            <div class="inputs-section">
                <div class="section-title">"Home Purchase Details"</div>

                <SliderInput
                    label="Home Price"
                    value=home_price
                    set_value=set_home_price
                    min=100_000.0
                    max=2_000_000.0
                    step=10_000.0
                    format_value=format_currency
                    field="home_price"
                    inputs=inputs
                />

                <div class="input-row">
                    <SliderInput
                        label="Down Payment"
                        value=down_payment_percent
                        set_value=set_down_payment_percent
                        min=0.0
                        max=100.0
                        step=1.0
                        format_value=|v| format!("{}%", v as u32)
                        field="down_payment_percent"
                        inputs=inputs
                    />

                    <SliderInput
                        label="Mortgage Interest Rate"
                        value=mortgage_rate
                        set_value=set_mortgage_rate
                        min=0.0
                        max=15.0
                        step=0.125
                        format_value=|v| format!("{:.2}%", v)
                        field="mortgage_rate"
                        inputs=inputs
                    />
                </div>

                <div class="input-row">
                    <SliderInput
                        label="Loan Term"
                        value=loan_term_years
                        set_value=set_loan_term_years
                        min=10.0
                        max=30.0
                        step=5.0
                        format_value=|v| format!("{} years", v as u32)
                        field="loan_term_years"
                        inputs=inputs
                    />

                    <SliderInput
                        label="Home Appreciation Rate"
                        value=home_appreciation
                        set_value=set_home_appreciation
                        min=-5.0
                        max=10.0
                        step=0.5
                        format_value=|v| format!("{:.1}%/year", v)
                        field="home_appreciation"
                        inputs=inputs
                    />
                </div>
            </div>

            <div class="inputs-section">
                <div class="section-title">"Ongoing Home Costs"</div>

                <div class="input-row">
                    <SliderInput
                        label="Property Tax Rate"
                        value=property_tax_rate
                        set_value=set_property_tax_rate
                        min=0.0
                        max=4.0
                        step=0.1
                        format_value=|v| format!("{:.1}%/year", v)
                        field="property_tax_rate"
                        inputs=inputs
                    />

                    <SliderInput
                        label="Home Insurance"
                        value=home_insurance
                        set_value=set_home_insurance
                        min=0.0
                        max=5_000.0
                        step=100.0
                        format_value=|v| format!("{}/year", format_currency(v))
                        field="home_insurance"
                        inputs=inputs
                    />
                </div>

                <div class="input-row">
                    <SliderInput
                        label="HOA Fees"
                        value=hoa_monthly
                        set_value=set_hoa_monthly
                        min=0.0
                        max=1_000.0
                        step=25.0
                        format_value=|v| format!("{}/month", format_currency(v))
                        field="hoa_monthly"
                        inputs=inputs
                    />

                    <SliderInput
                        label="Maintenance"
                        value=maintenance_percent
                        set_value=set_maintenance_percent
                        min=0.0
                        max=3.0
                        step=0.25
                        format_value=|v| format!("{:.1}% of home/year", v)
                        field="maintenance_percent"
                        inputs=inputs
                    />
                </div>
            </div>

            <div class="inputs-section">
                <div class="section-title">"Transaction Costs"</div>

                <div class="input-row">
                    <SliderInput
                        label="Closing Costs"
                        value=closing_cost_percent
                        set_value=set_closing_cost_percent
                        min=0.0
                        max=6.0
                        step=0.5
                        format_value=|v| format!("{}% of price", v)
                        field="closing_cost_percent"
                        inputs=inputs
                    />

                    <SliderInput
                        label="Selling Costs (Realtor, etc.)"
                        value=selling_cost_percent
                        set_value=set_selling_cost_percent
                        min=0.0
                        max=10.0
                        step=0.5
                        format_value=|v| format!("{}% of sale", v)
                        field="selling_cost_percent"
                        inputs=inputs
                    />
                </div>
            </div>

            <div class="inputs-section">
                <div class="section-title">"Rental Details"</div>

                <SliderInput
                    label="Monthly Rent"
                    value=monthly_rent
                    set_value=set_monthly_rent
                    min=500.0
                    max=10_000.0
                    step=100.0
                    format_value=|v| format!("{}/month", format_currency(v))
                    field="monthly_rent"
                    inputs=inputs
                />

                <div class="input-row">
                    <SliderInput
                        label="Annual Rent Increase"
                        value=rent_increase_rate
                        set_value=set_rent_increase_rate
                        min=0.0
                        max=10.0
                        step=0.5
                        format_value=|v| format!("{:.1}%/year", v)
                        field="rent_increase_rate"
                        inputs=inputs
                    />

                    <SliderInput
                        label="Renter's Insurance"
                        value=renters_insurance
                        set_value=set_renters_insurance
                        min=0.0
                        max=1_000.0
                        step=25.0
                        format_value=|v| format!("{}/year", format_currency(v))
                        field="renters_insurance"
                        inputs=inputs
                    />
                </div>
            </div>

            <div class="inputs-section">
                <div class="section-title">"Investment Assumptions"</div>

                <SliderInput
                    label="Investment Return Rate"
                    value=investment_return
                    set_value=set_investment_return
                    min=0.0
                    max=15.0
                    step=0.5
                    format_value=|v| format!("{:.1}%/year", v)
                    field="investment_return"
                    inputs=inputs
                />
            </div>

            <NetWorthChart result=result time_horizon=time_horizon_years />

            <BreakdownSection result=result />
        </div>
    }
}

fn format_currency(value: f64) -> String {
    let abs_value = value.abs();
    let sign = if value < 0.0 { "-" } else { "" };
    if abs_value >= 1_000_000.0 {
        format!("{}${:.2}M", sign, abs_value / 1_000_000.0)
    } else if abs_value >= 1_000.0 {
        format!("{}${:.0}K", sign, abs_value / 1_000.0)
    } else {
        format!("{}${:.0}", sign, abs_value)
    }
}

/// More precise currency format for sensitivity labels
fn format_currency_precise(value: f64) -> String {
    let abs_value = value.abs().round() as i64;
    let sign = if value < 0.0 { "-" } else { "" };
    if abs_value >= 1_000_000 {
        let millions = value.abs() / 1_000_000.0;
        format!("{}${:.2}M", sign, millions)
    } else {
        let formatted = abs_value
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(|chunk| std::str::from_utf8(chunk).unwrap())
            .collect::<Vec<_>>()
            .join(",");
        format!("{}${}", sign, formatted)
    }
}

/// Format bound value for editing (raw number)
fn format_bound_value(value: f64, step: f64) -> String {
    if step >= 1.0 {
        format!("{}", value as i64)
    } else if step >= 0.1 {
        format!("{:.1}", value)
    } else {
        format!("{:.3}", value)
    }
}

/// Format bound value for display (abbreviated)
fn format_bound_display(value: f64) -> String {
    if value.abs() >= 1_000_000.0 {
        format!("{:.1}M", value / 1_000_000.0)
    } else if value.abs() >= 1_000.0 {
        format!("{:.0}K", value / 1_000.0)
    } else if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.1}", value)
    }
}

/// Parse a bound value from user input (handles K, M suffixes)
fn parse_bound_value(input: &str) -> Result<f64, ()> {
    let trimmed = input.trim().to_uppercase();
    let trimmed = trimmed.replace(",", "").replace("$", "");

    if trimmed.ends_with('K') {
        let num_str = trimmed.trim_end_matches('K').trim();
        num_str.parse::<f64>().map(|n| n * 1_000.0).map_err(|_| ())
    } else if trimmed.ends_with('M') {
        let num_str = trimmed.trim_end_matches('M').trim();
        num_str.parse::<f64>().map(|n| n * 1_000_000.0).map_err(|_| ())
    } else {
        trimmed.parse::<f64>().map_err(|_| ())
    }
}

fn format_currency_full(value: f64) -> String {
    let abs_value = value.abs().round() as i64;
    let sign = if value < 0.0 { "-" } else { "" };
    let formatted = abs_value
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(",");
    format!("{}${}", sign, formatted)
}

#[component]
fn ResultBanner(result: Memo<CalculationResult>) -> impl IntoView {
    let banner_class = move || {
        if result.get().difference > 0.0 {
            "result-banner buy-wins"
        } else {
            "result-banner rent-wins"
        }
    };

    let title_class = move || {
        if result.get().difference > 0.0 {
            "result-title buy"
        } else {
            "result-title rent"
        }
    };

    view! {
        <div class=banner_class>
            <div class=title_class>
                {move || {
                    let r = result.get();
                    if r.difference > 0.0 {
                        format!("Buying wins by {}", format_currency_full(r.difference))
                    } else {
                        format!("Renting wins by {}", format_currency_full(-r.difference))
                    }
                }}
            </div>
            <div class="result-detail">
                {move || {
                    let r = result.get();
                    format!(
                        "Buy net worth: {} | Rent net worth: {}",
                        format_currency_full(r.buy_breakdown.net_worth),
                        format_currency_full(r.rent_breakdown.net_worth)
                    )
                }}
            </div>
        </div>
    }
}

#[component]
fn SliderInput<F>(
    label: &'static str,
    value: ReadSignal<f64>,
    set_value: WriteSignal<f64>,
    min: f64,
    max: f64,
    step: f64,
    format_value: F,
    field: &'static str,
    inputs: Memo<Inputs>,
) -> impl IntoView
where
    F: Fn(f64) -> String + Copy + 'static,
{
    // Editable bounds - start with the default values
    let (current_min, set_current_min) = create_signal(min);
    let (current_max, set_current_max) = create_signal(max);
    let (editing_min, set_editing_min) = create_signal(false);
    let (editing_max, set_editing_max) = create_signal(false);
    let (min_input_value, set_min_input_value) = create_signal(format_bound_value(min, step));
    let (max_input_value, set_max_input_value) = create_signal(format_bound_value(max, step));

    let sensitivity_data = create_memo(move |_| {
        generate_sensitivity_data(&inputs.get(), field, current_min.get(), current_max.get(), 50)
    });

    // Clamp value when bounds change
    create_effect(move |_| {
        let v = value.get();
        let min_v = current_min.get();
        let max_v = current_max.get();
        if v < min_v {
            set_value.set(min_v);
        } else if v > max_v {
            set_value.set(max_v);
        }
    });

    view! {
        <div class="input-group">
            <div class="input-header">
                <span class="input-label">{label}</span>
                <span class="input-value">{move || format_value(value.get())}</span>
            </div>
            <div class="slider-with-bounds">
                <div class="bound-input">
                    {move || {
                        if editing_min.get() {
                            view! {
                                <input
                                    type="text"
                                    class="bound-text-input"
                                    prop:value=min_input_value
                                    on:input=move |ev| {
                                        set_min_input_value.set(event_target_value(&ev));
                                    }
                                    on:blur=move |_| {
                                        if let Ok(v) = parse_bound_value(&min_input_value.get()) {
                                            if v < current_max.get() {
                                                set_current_min.set(v);
                                            }
                                        }
                                        set_min_input_value.set(format_bound_value(current_min.get(), step));
                                        set_editing_min.set(false);
                                    }
                                    on:keydown=move |ev| {
                                        if ev.key() == "Enter" {
                                            if let Ok(v) = parse_bound_value(&min_input_value.get()) {
                                                if v < current_max.get() {
                                                    set_current_min.set(v);
                                                }
                                            }
                                            set_min_input_value.set(format_bound_value(current_min.get(), step));
                                            set_editing_min.set(false);
                                        } else if ev.key() == "Escape" {
                                            set_min_input_value.set(format_bound_value(current_min.get(), step));
                                            set_editing_min.set(false);
                                        }
                                    }
                                    autofocus=true
                                />
                            }.into_view()
                        } else {
                            view! {
                                <span
                                    class="bound-label clickable"
                                    on:click=move |_| {
                                        set_min_input_value.set(format_bound_value(current_min.get(), step));
                                        set_editing_min.set(true);
                                    }
                                    title="Click to edit minimum"
                                >
                                    {move || format_bound_display(current_min.get())}
                                </span>
                            }.into_view()
                        }
                    }}
                </div>
                <div class="slider-container">
                    <input
                        type="range"
                        min=move || current_min.get()
                        max=move || current_max.get()
                        step=step
                        prop:value=move || value.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<f64>().unwrap_or(current_min.get());
                            set_value.set(val);
                        }
                    />
                </div>
                <div class="bound-input">
                    {move || {
                        if editing_max.get() {
                            view! {
                                <input
                                    type="text"
                                    class="bound-text-input"
                                    prop:value=max_input_value
                                    on:input=move |ev| {
                                        set_max_input_value.set(event_target_value(&ev));
                                    }
                                    on:blur=move |_| {
                                        if let Ok(v) = parse_bound_value(&max_input_value.get()) {
                                            if v > current_min.get() {
                                                set_current_max.set(v);
                                            }
                                        }
                                        set_max_input_value.set(format_bound_value(current_max.get(), step));
                                        set_editing_max.set(false);
                                    }
                                    on:keydown=move |ev| {
                                        if ev.key() == "Enter" {
                                            if let Ok(v) = parse_bound_value(&max_input_value.get()) {
                                                if v > current_min.get() {
                                                    set_current_max.set(v);
                                                }
                                            }
                                            set_max_input_value.set(format_bound_value(current_max.get(), step));
                                            set_editing_max.set(false);
                                        } else if ev.key() == "Escape" {
                                            set_max_input_value.set(format_bound_value(current_max.get(), step));
                                            set_editing_max.set(false);
                                        }
                                    }
                                    autofocus=true
                                />
                            }.into_view()
                        } else {
                            view! {
                                <span
                                    class="bound-label clickable"
                                    on:click=move |_| {
                                        set_max_input_value.set(format_bound_value(current_max.get(), step));
                                        set_editing_max.set(true);
                                    }
                                    title="Click to edit maximum"
                                >
                                    {move || format_bound_display(current_max.get())}
                                </span>
                            }.into_view()
                        }
                    }}
                </div>
            </div>
            <SensitivityGraph
                data=sensitivity_data
                current_value=value
                min=current_min
                max=current_max
            />
        </div>
    }
}

#[component]
fn SensitivityGraph(
    data: Memo<Vec<(f64, f64)>>,
    current_value: ReadSignal<f64>,
    min: ReadSignal<f64>,
    max: ReadSignal<f64>,
) -> impl IntoView {
    let segments = move || {
        let d = data.get();
        if d.is_empty() {
            return vec![];
        }

        // Find the range of differences for color scaling
        let max_diff = d.iter().map(|(_, diff)| diff.abs()).fold(0.0_f64, f64::max);

        d.iter()
            .map(|(_, diff)| {
                let intensity = if max_diff > 0.0 { diff.abs() / max_diff } else { 0.0 };
                let is_buy_better = *diff > 0.0;
                (is_buy_better, intensity)
            })
            .collect::<Vec<_>>()
    };

    let marker_position = move || {
        let val = current_value.get();
        let min_v = min.get();
        let max_v = max.get();
        let range = max_v - min_v;
        let pct = if range > 0.0 { (val - min_v) / range * 100.0 } else { 50.0 };
        format!("calc({}% - 1px)", pct)
    };

    let min_label = move || {
        let d = data.get();
        if let Some((_, diff)) = d.first() {
            if *diff > 0.0 {
                format!("Buy +{}", format_currency_precise(*diff))
            } else {
                format!("Rent +{}", format_currency_precise(-*diff))
            }
        } else {
            String::new()
        }
    };

    let max_label = move || {
        let d = data.get();
        if let Some((_, diff)) = d.last() {
            if *diff > 0.0 {
                format!("Buy +{}", format_currency_precise(*diff))
            } else {
                format!("Rent +{}", format_currency_precise(-*diff))
            }
        } else {
            String::new()
        }
    };

    view! {
        <div class="sensitivity-graph">
            <div class="sensitivity-bar">
                {move || {
                    segments()
                        .into_iter()
                        .enumerate()
                        .map(|(_i, (is_buy, intensity))| {
                            let color = if is_buy {
                                format!("rgba(37, 99, 235, {})", 0.2 + intensity * 0.8)
                            } else {
                                format!("rgba(220, 38, 38, {})", 0.2 + intensity * 0.8)
                            };
                            view! {
                                <div
                                    class="sensitivity-segment"
                                    style=format!("flex: 1; background-color: {}", color)
                                />
                            }
                        })
                        .collect_view()
                }}
            </div>
            <div class="current-marker" style:left=marker_position></div>
        </div>
        <div class="sensitivity-labels">
            <span>{min_label}</span>
            <span>{max_label}</span>
        </div>
    }
}

#[component]
fn NetWorthChart(result: Memo<CalculationResult>, time_horizon: ReadSignal<f64>) -> impl IntoView {
    let canvas_id = "net-worth-chart";

    create_effect(move |_| {
        let r = result.get();
        let years = time_horizon.get() as usize;
        let snapshots = &r.yearly_snapshots;

        if snapshots.is_empty() {
            return;
        }

        // Start at year 1 (no year 0)
        let labels: Vec<String> = (1..=years)
            .map(|y| format!("Year {}", y))
            .collect();

        let buy_data: Vec<f64> = snapshots
            .iter()
            .take(years)
            .map(|s| s.buy_net_worth)
            .collect();

        let rent_data: Vec<f64> = snapshots
            .iter()
            .take(years)
            .map(|s| s.rent_net_worth)
            .collect();

        call_create_or_update_chart(canvas_id, &labels, &buy_data, &rent_data);
    });

    view! {
        <div class="chart-section">
            <div class="section-title">"Net Worth Over Time"</div>
            <div class="chart-container">
                <canvas id=canvas_id></canvas>
            </div>
        </div>
    }
}

#[component]
fn BreakdownSection(result: Memo<CalculationResult>) -> impl IntoView {
    view! {
        <div class="breakdown-section">
            <div class="section-title">"Monthly Cost Comparison"</div>
            <div class="monthly-comparison">
                <div class="monthly-item">
                    <span class="monthly-label">"Avg. Monthly Cost to Buy"</span>
                    <span class="monthly-value buy">
                        {move || format!("${:.0}/mo", result.get().monthly_comparison.avg_buy_monthly)}
                    </span>
                </div>
                <div class="monthly-item">
                    <span class="monthly-label">"Avg. Monthly Cost to Rent"</span>
                    <span class="monthly-value rent">
                        {move || format!("${:.0}/mo", result.get().monthly_comparison.avg_rent_monthly)}
                    </span>
                </div>
                <div class="monthly-item highlight">
                    <span class="monthly-label">"Monthly Difference"</span>
                    <span class="monthly-value">
                        {move || {
                            let diff = result.get().monthly_comparison.avg_monthly_difference;
                            if diff > 0.0 {
                                format!("Renting saves ${:.0}/mo", diff)
                            } else {
                                format!("Buying saves ${:.0}/mo", -diff)
                            }
                        }}
                    </span>
                </div>
            </div>
        </div>

        <div class="breakdown-section">
            <div class="section-title">"Full Breakdown"</div>
            <div class="breakdown-grid">
                <div class="breakdown-column buy">
                    <h3>"Buying"</h3>

                    <div class="breakdown-item">
                        <span class="label">"Down Payment"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.down_payment)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Closing Costs"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.closing_costs)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Total Mortgage Payments"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.total_mortgage_payments)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"  └ Interest Paid"</span>
                        <span class="value">{move || format_currency_full(result.get().buy_breakdown.total_interest_paid)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"  └ Principal Paid"</span>
                        <span class="value">{move || format_currency_full(result.get().buy_breakdown.total_principal_paid)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Property Taxes"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.total_property_tax)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Home Insurance"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.total_insurance)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"HOA Fees"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.total_hoa)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Maintenance"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.total_maintenance)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Selling Costs"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.selling_costs)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Final Home Value"</span>
                        <span class="value positive">{move || format_currency_full(result.get().buy_breakdown.final_home_value)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Remaining Mortgage"</span>
                        <span class="value negative">{move || format_currency_full(result.get().buy_breakdown.remaining_mortgage)}</span>
                    </div>

                    {move || {
                        let savings = result.get().buy_breakdown.monthly_savings_invested;
                        if savings > 0.0 {
                            view! {
                                <div class="breakdown-item">
                                    <span class="label">"Monthly Savings Invested"</span>
                                    <span class="value positive">{format_currency_full(savings)}</span>
                                </div>
                                <div class="breakdown-item">
                                    <span class="label">"  (Because buying cost less)"</span>
                                    <span class="value"></span>
                                </div>
                                <div class="breakdown-item">
                                    <span class="label">"Investment Returns"</span>
                                    <span class="value positive">{format_currency_full(result.get().buy_breakdown.investment_returns)}</span>
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}

                    <div class="breakdown-item total">
                        <span class="label">"Net Worth (Home + Investments)"</span>
                        <span class="value">{move || format_currency_full(result.get().buy_breakdown.net_worth)}</span>
                    </div>
                </div>

                <div class="breakdown-column rent">
                    <h3>"Renting"</h3>

                    <div class="breakdown-item">
                        <span class="label">"Initial Investment"</span>
                        <span class="value">{move || format_currency_full(result.get().rent_breakdown.initial_investment)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"  (Down payment + closing costs)"</span>
                        <span class="value"></span>
                    </div>

                    {move || {
                        let savings = result.get().rent_breakdown.monthly_cost_savings;
                        if savings > 0.0 {
                            view! {
                                <div class="breakdown-item">
                                    <span class="label">"Monthly Savings Invested"</span>
                                    <span class="value positive">{format!("+{}", format_currency_full(savings))}</span>
                                </div>
                                <div class="breakdown-item">
                                    <span class="label">"  (Because renting cost less)"</span>
                                    <span class="value"></span>
                                </div>
                            }.into_view()
                        } else {
                            view! {}.into_view()
                        }
                    }}

                    <div class="breakdown-item">
                        <span class="label">"Investment Returns"</span>
                        <span class="value positive">{move || format_currency_full(result.get().rent_breakdown.investment_returns)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Total Rent Paid"</span>
                        <span class="value negative">{move || format_currency_full(result.get().rent_breakdown.total_rent_paid)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Renter's Insurance"</span>
                        <span class="value negative">{move || format_currency_full(result.get().rent_breakdown.total_renters_insurance)}</span>
                    </div>

                    <div class="breakdown-item total">
                        <span class="label">"Net Worth (Investments)"</span>
                        <span class="value">{move || format_currency_full(result.get().rent_breakdown.net_worth)}</span>
                    </div>
                </div>
            </div>
        </div>

        <div class="breakdown-section">
            <div class="section-title">"Average Monthly Costs"</div>
            <div class="breakdown-grid">
                <div class="breakdown-column buy">
                    <h3>"Buying (per month)"</h3>

                    <div class="breakdown-item">
                        <span class="label">"Mortgage Payment"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.buy_mortgage)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Property Tax"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.buy_property_tax)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Home Insurance"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.buy_insurance)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"HOA Fees"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.buy_hoa)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Maintenance"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.buy_maintenance)}</span>
                    </div>

                    <div class="breakdown-item total">
                        <span class="label">"Total Monthly"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.buy_total)}</span>
                    </div>
                </div>

                <div class="breakdown-column rent">
                    <h3>"Renting (per month)"</h3>

                    <div class="breakdown-item">
                        <span class="label">"Rent Payment"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.rent_payment)}</span>
                    </div>
                    <div class="breakdown-item">
                        <span class="label">"Renter's Insurance"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.rent_insurance)}</span>
                    </div>

                    <div class="breakdown-item total">
                        <span class="label">"Total Monthly"</span>
                        <span class="value">{move || format!("${:.0}", result.get().monthly_breakdown.rent_total)}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
