#![allow(non_snake_case)]
use std::vec;

#[test]
fn test_GetCompoundDataNISTList_cross_validation() {
    let radionuclides = xraylib::GetRadioNuclideDataList().unwrap();
    assert_eq!(radionuclides.len(), 10);

    for (i, v) in radionuclides.iter().enumerate() {
        let rnd = xraylib::GetRadioNuclideDataByIndex(i as i32).unwrap();
        assert_eq!(rnd.name, *v);
        let rnd = xraylib::GetRadioNuclideDataByName(v).unwrap();
        assert_eq!(rnd.name, *v);
    }
}

fn check_radionuclide(rnd: xraylib::radioNuclideData) {
    assert_eq!(rnd.A, 125);
    assert_eq!(rnd.N, 72);
    assert_eq!(rnd.Z, 53);
    assert_eq!(rnd.Z_xray, 52);
    assert_eq!(rnd.name, "125I");
    assert_eq!(rnd.GammaEnergies, vec![35.4919]);
    assert_eq!(rnd.GammaIntensities, vec![0.0668]);
    assert_eq!(
        rnd.XrayLines,
        vec![
            -86, -60, -89, -90, -63, -33, -34, -91, -95, -68, -38, -39, -1, -2, -3, -5, -6, -8,
            -11, -13
        ]
    );
    assert_eq!(
        rnd.XrayIntensities,
        vec![
            0.0023, 0.00112, 0.0063, 0.056, 0.035, 0.0042, 0.007, 0.00043, 0.0101, 0.0045, 0.00103,
            0.0016, 3.24e-05, 0.406, 0.757, 0.0683, 0.132, 0.00121, 0.0381, 0.0058
        ]
    );
}

#[test]
fn test_GetRadioNuclideDataByIndex_3() {
    let rnd = xraylib::GetRadioNuclideDataByIndex(xraylib::RADIO_NUCLIDE_125I).unwrap();
    check_radionuclide(rnd);
}

#[test]
fn test_GetRadioNuclideDataByName_125I() {
    let rnd = xraylib::GetRadioNuclideDataByName("125I").unwrap();
    check_radionuclide(rnd);
}

#[test]
fn test_GetRadioNuclideDataByIndex_invalid_input() {
    let rnd = xraylib::GetRadioNuclideDataByIndex(-1);
    assert!(rnd.is_err());
    let rnd = xraylib::GetRadioNuclideDataByIndex(10);
    assert!(rnd.is_err());
    let rnd = xraylib::GetRadioNuclideDataByIndex(9);
    assert!(rnd.is_ok());
}

#[test]
fn test_GetRadioNuclideDataByName_invalid_input() {
    let rnd = xraylib::GetRadioNuclideDataByName("howeqhfofo");
    assert!(rnd.is_err());
    let rnd = xraylib::GetRadioNuclideDataByName("");
    assert!(rnd.is_err());
}
