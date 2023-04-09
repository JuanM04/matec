use std::f64::MIN_POSITIVE;

/// Dada la naturaleza de los puntos flotantes, esta función compara dos
/// números flotantes para ver si son iguales.
/// Ver https://floating-point-gui.de/errors/comparison/
pub fn nearly_equal(a: f64, b: f64) -> bool {
    const EPSILON: f64 = 1e-12;
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();

    if a == b {
        // shortcut, handles infinities
        true
    } else if a == 0.0 || b == 0.0 || (abs_a + abs_b < MIN_POSITIVE) {
        // a or b is zero or both are extremely close to it
        // relative error is less meaningful here
        diff < (EPSILON * MIN_POSITIVE)
    } else {
        // use relative error
        diff / (abs_a + abs_b) < EPSILON
    }
}

/// Formatea un número flotante para que se parezca a un entero si es
/// posible.
pub fn format_float(n: f64) -> String {
    if nearly_equal(n, 0.0) {
        // Previene el caso "-0"
        return "0".to_string();
    } else {
        let rounded = n.round();
        if nearly_equal(n, rounded) {
            format!("{}", rounded)
        } else {
            format!("{:.4}", n)
        }
    }
}
