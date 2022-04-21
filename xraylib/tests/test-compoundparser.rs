#![allow(non_snake_case)]

use std::vec;

#[test]
fn test_SymbolToAtomicNumber_iron() {
    assert_eq!(xraylib::SymbolToAtomicNumber("Fe").unwrap(), 26);
}

#[test]
fn test_SymbolToAtomicNumber_non_existent_element() {
    assert!(xraylib::SymbolToAtomicNumber("Uu").is_err());
}

#[test]
fn test_AtomicNumberToSymbol_iron() {
    assert_eq!(xraylib::AtomicNumberToSymbol(26).unwrap(), "Fe");
}

#[test]
fn test_AtomicNumberToSymbol_non_existent_element() {
    assert!(xraylib::AtomicNumberToSymbol(0).is_err());
    assert!(xraylib::AtomicNumberToSymbol(107).is_ok());
    assert!(xraylib::AtomicNumberToSymbol(108).is_err());
}

#[test]
fn test_cross_validation() {
    for Z in 1..108 {
        let symbol = xraylib::AtomicNumberToSymbol(Z).unwrap();
        assert_eq!(xraylib::SymbolToAtomicNumber(&symbol).unwrap(), Z);
    }
}

#[test]
fn test_compound_parser_good() {
    let compounds: Vec<&str> = vec![
        "C19H29COOH",
        "C12H10",
        "C12H6O2",
        "C6H5Br",
        "C3H4OH(COOH)3",
        "HOCH2CH2OH",
        "C5H11NO2",
        "CH3CH(CH3)CH3",
        "NH2CH(C4H5N2)COOH",
        "H2O",
        "Ca5(PO4)3F",
        "Ca5(PO4)3OH",
        "Ca5.522(PO4.48)3OH",
        "Ca5.522(PO.448)3OH",
    ];
    for compound in compounds {
        assert!(xraylib::CompoundParser(compound).is_ok());
    }
}

#[test]
fn test_compound_parser_bad() {
    let compounds: Vec<&str> = vec![
        "CuI2ww",
        "0C",
        "2O",
        "13Li",
        "2(NO3)",
        "H(2)",
        "Ba(12)",
        "Cr(5)3",
        "Pb(13)2",
        "Au(22)11",
        "Au11(H3PO4)2)",
        "Au11(H3PO4))2",
        "Au(11(H3PO4))2",
        "Ca5.522(PO.44.8)3OH",
        "Ba[12]",
        "Auu1",
        "AuL1",
        "  ",
        "\t",
        "\n",
        "Au L1",
        "Au\tFe",
    ];
    for compound in compounds {
        assert!(xraylib::CompoundParser(compound).is_err());
    }
}

#[test]
fn test_compound_parser_H2SO4() {
    let cd = xraylib::CompoundParser("H2SO4").unwrap();
    assert_eq!(cd.nElements, 3);
    assert_eq!(cd.molarMass, 98.09);
    assert_eq!(cd.nAtomsAll, 7.0);
    assert_eq!(cd.Elements, vec![1, 8, 16,]);
    assert_eq!(
        cd.massFractions,
        vec![0.02059333265368539, 0.6524620246712203, 0.32694464267509427]
    );
    assert_eq!(cd.nAtoms, vec![2.0, 4.0, 1.0]);
}
