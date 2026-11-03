use spinon_style::ComputedCssMath;

pub(super) fn supported_auto_column_calc(expression: &ComputedCssMath) -> bool {
    let ComputedCssMath::Sum(terms) = expression else {
        return false;
    };
    if terms.len() != 2 {
        return false;
    }
    let mut has_length = false;
    let mut has_percentage = false;
    for term in terms {
        match signed_term(term) {
            Some(SignedTerm::Length(value)) if value.is_finite() && !has_length => {
                has_length = true;
            }
            Some(SignedTerm::Percentage(value)) if value.is_finite() && !has_percentage => {
                has_percentage = true;
            }
            _ => return false,
        }
    }
    has_length && has_percentage
}

enum SignedTerm {
    Length(f32),
    Percentage(f32),
}

fn signed_term(term: &ComputedCssMath) -> Option<SignedTerm> {
    match term {
        ComputedCssMath::LengthPx(value) => Some(SignedTerm::Length(*value)),
        ComputedCssMath::Percentage(value) => Some(SignedTerm::Percentage(*value)),
        ComputedCssMath::Negate(value) => match value.as_ref() {
            ComputedCssMath::LengthPx(value) => Some(SignedTerm::Length(-value)),
            ComputedCssMath::Percentage(value) => Some(SignedTerm::Percentage(-value)),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_column_calc_requires_one_finite_length_and_percentage_sum() {
        for value in [
            ComputedCssMath::Sum(vec![
                ComputedCssMath::Percentage(0.1),
                ComputedCssMath::LengthPx(5.0),
            ]),
            ComputedCssMath::Sum(vec![
                ComputedCssMath::Negate(Box::new(ComputedCssMath::LengthPx(1.0))),
                ComputedCssMath::Percentage(0.1),
            ]),
            ComputedCssMath::Sum(vec![
                ComputedCssMath::LengthPx(1.0),
                ComputedCssMath::Negate(Box::new(ComputedCssMath::Percentage(0.1))),
            ]),
        ] {
            assert!(supported_auto_column_calc(&value));
        }
        for value in [
            ComputedCssMath::LengthPx(5.0),
            ComputedCssMath::Sum(vec![ComputedCssMath::LengthPx(1.0); 2]),
            ComputedCssMath::Sum(vec![
                ComputedCssMath::Percentage(0.1),
                ComputedCssMath::LengthPx(2.0),
                ComputedCssMath::LengthPx(3.0),
            ]),
            ComputedCssMath::Sum(vec![
                ComputedCssMath::Product(vec![ComputedCssMath::LengthPx(2.0)]),
                ComputedCssMath::Percentage(0.1),
            ]),
            ComputedCssMath::Sum(vec![
                ComputedCssMath::LengthPx(f32::NAN),
                ComputedCssMath::Percentage(0.1),
            ]),
        ] {
            assert!(!supported_auto_column_calc(&value));
        }
    }
}
