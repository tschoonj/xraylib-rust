#[test]
fn test_atomicweight_iron() {
    let weight = xraylib::AtomicWeight(26);
    assert!(weight.is_ok());
    assert!((weight.unwrap() - 55.850).abs() < 1E-6);
}

#[test]
fn test_atomicweight_uranium() {
    let weight = xraylib::AtomicWeight(92);
    assert!(weight.is_ok());
    assert!((weight.unwrap() - 238.070).abs() < 1E-6);
}

#[test]
fn test_atomicweight_yet_undiscovered_element() {
    let weight = xraylib::AtomicWeight(185);
    assert!(weight.is_err());
}
