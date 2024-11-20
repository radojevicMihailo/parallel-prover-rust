use ark_ff::Field;

// pub fn add<E:Field>(p1: &[E], p2: &[E]) -> Vec<E> {}

// pub fn mul<E:Field>(p1: &[E], p2: &[E]) -> Vec<E> {}

pub fn div<E:Field>(p1: &[E], p2: &[E]) -> Result<Vec<E>, &'static str> {
    if p2.is_empty() || p2.iter().all(|&x| x == E::ZERO) {
        return Err("Cannot divide by zero polynomial");
    }

    if p1.len() < p2.len() {
        return Ok(vec![E::ZERO]);
    }

    let mut quotient = vec![E::ZERO; p1.len() - p2.len() + 1];
    let mut remainder: Vec<E> = p1.to_vec();

    while remainder.len() >= p2.len() {
        let coeff = *remainder.last().unwrap() / *p2.last().unwrap();
        let pos = remainder.len() - p2.len();

        quotient[pos] = coeff;

        for (i, &factor) in p2.iter().enumerate() {
            remainder[pos + i] -= factor * coeff;
        }

        while let Some(true) = remainder.last().map(|x| *x == E::ZERO) {
            remainder.pop();
        }
    }

    Ok(quotient)
}

pub fn evaluate<E:Field>(poly: &[E], point: E) -> E {
    let mut value = E::ZERO;

    for i in 0..poly.len() {
        value += poly[i] * point.pow(&[i as u64]);
    }

    value
}

// pub fn interpolate<E:Field>(points: &[E], values: &[E]) -> Result<Vec<E>, &'static str> {}

// pub fn get_omega<E:PrimeField>(coefficients: &[E]) -> E {}

// pub fn scalar_mul<E:Field>(poly: &[E], point: E) -> Vec<E> {}