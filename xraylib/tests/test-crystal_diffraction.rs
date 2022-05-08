#![allow(non_snake_case)]

#[test]
fn test_Crystal_GetCrystalsList_cross_validation() {
    let crystal_names = xraylib::Crystal_GetCrystalsList().unwrap();
    assert_eq!(crystal_names.len(), 38);

    for name in crystal_names.iter() {
        let cs = xraylib::Crystal_GetCrystal(name).unwrap();
        assert_eq!(cs.name, *name);
    }
}

#[test]
fn test_Crystal_GetCrystal() {
    assert!(xraylib::Crystal_GetCrystal("Diamond").is_ok());
    assert!(xraylib::Crystal_GetCrystal("No such crystal").is_err());
}

#[test]
fn test_Bragg_angle() {
    let cs = xraylib::Crystal_GetCrystal("Diamond").unwrap();
    println!("first dump {:#?}", cs);
    let angle = xraylib::Bragg_angle(&cs, 10.0, 1, 1, 1).unwrap();
    println!("second dump {:#?}", cs);
    assert_eq!(angle, 0.3057795845795849);

    // invalid energy
    assert!(xraylib::Bragg_angle(&cs, -10.0, 1, 1, 1).is_err());
}
