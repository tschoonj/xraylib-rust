#![allow(non_snake_case)]
use std::vec;

#[test]
fn test_GetCompoundDataNISTList_cross_validation() {
    let compound_names = xraylib::GetCompoundDataNISTList().unwrap();
    assert_eq!(compound_names.len(), 180);

    for (i, v) in compound_names.iter().enumerate() {
        let cdn = xraylib::GetCompoundDataNISTByIndex(i as i32).unwrap();
        assert_eq!(cdn.name, *v);
        let cdn = xraylib::GetCompoundDataNISTByName(v).unwrap();
        assert_eq!(cdn.name, *v);
    }
}

fn check_nist_compound(cdn: xraylib::compoundDataNIST) {
    assert_eq!(cdn.nElements, 4);
    assert_eq!(cdn.density, 0.001205);
    assert_eq!(cdn.Elements, vec![6, 7, 8, 18]);
    assert_eq!(
        cdn.massFractions,
        vec![0.000124, 0.755267, 0.231781, 0.012827]
    );
    assert_eq!(cdn.name, "Air, Dry (near sea level)");
}

#[test]
fn test_GetCompoundDataNISTByIndex_5() {
    let cdn =
        xraylib::GetCompoundDataNISTByIndex(xraylib::NIST_COMPOUND_AIR_DRY_NEAR_SEA_LEVEL).unwrap();
    check_nist_compound(cdn);
}

#[test]
fn test_GetCompoundDataNISTByName_Air() {
    let cdn = xraylib::GetCompoundDataNISTByName("Air, Dry (near sea level)").unwrap();
    check_nist_compound(cdn);
}

#[test]
fn test_GetCompoundDataNISTByIndex_invalid_input() {
    let cdn = xraylib::GetCompoundDataNISTByIndex(-1);
    assert!(cdn.is_err());
    let cdn = xraylib::GetCompoundDataNISTByIndex(180);
    assert!(cdn.is_err());
    let cdn = xraylib::GetCompoundDataNISTByIndex(179);
    assert!(cdn.is_ok());
}

#[test]
fn test_GetCompoundDataNISTByName_invalid_input() {
    let cdn = xraylib::GetCompoundDataNISTByName("howeqhfofo");
    assert!(cdn.is_err());
    let cdn = xraylib::GetCompoundDataNISTByName("");
    assert!(cdn.is_err());
}
