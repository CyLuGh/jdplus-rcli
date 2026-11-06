library(rextendr)

rust_function("fn add(a:f64, b:f64) -> f64 { a + b }")

test <- add(2.5, 4.7)
