#[test]
fn test_pz_0() {
    let profile = xraylib::ComptonProfile(26, 0.0);
    assert!(profile.is_ok());
    assert!((profile.unwrap() - 7.060).abs() < 1E-6);

    let profile = xraylib::ComptonProfile_Partial(26, xraylib::N1_SHELL, 0.0);
    assert!(profile.is_ok());
    assert!((profile.unwrap() - 1.550).abs() < 1E-6);

    let profile1 = xraylib::ComptonProfile_Partial(26, xraylib::L2_SHELL, 0.0);
    assert!(profile1.is_ok());
    let profile2 = xraylib::ComptonProfile_Partial(26, xraylib::L3_SHELL, 0.0);
    assert!(profile2.is_ok());
    assert!((profile1.as_ref().unwrap() - profile2.unwrap()).abs() < 1E-6);
    assert!((profile1.unwrap() - 0.065).abs() < 1E-6);
}

#[test]
fn test_pz_100() {
    let profile = xraylib::ComptonProfile(26, 100.0);
    assert!(profile.is_ok());
    assert!((profile.unwrap() - 1.8E-5).abs() < 1E-6);

    let profile = xraylib::ComptonProfile_Partial(26, xraylib::N1_SHELL, 100.0);
    assert!(profile.is_ok());
    assert!((profile.unwrap() - 5.1E-9).abs() < 1E-6);

    let profile1 = xraylib::ComptonProfile_Partial(26, xraylib::L2_SHELL, 100.0);
    assert!(profile1.is_ok());
    let profile2 = xraylib::ComptonProfile_Partial(26, xraylib::L3_SHELL, 100.0);
    assert!(profile2.is_ok());
    assert!((profile1.as_ref().unwrap() - profile2.unwrap()).abs() < 1E-6);
    assert!((profile1.unwrap() - 1.1E-8).abs() < 1E-6);
}

#[test]
fn test_pz_50() {
    let profile = xraylib::ComptonProfile(26, 50.0);
    assert!(profile.is_ok());
    assert!((profile.unwrap() - 0.0006843950273082384).abs() < 1E-6);

    let profile = xraylib::ComptonProfile_Partial(26, xraylib::N1_SHELL, 50.0);
    assert!(profile.is_ok());
    assert!((profile.unwrap() - 2.4322755767709126e-07).abs() < 1E-6);

    let profile1 = xraylib::ComptonProfile_Partial(26, xraylib::L2_SHELL, 50.0);
    assert!(profile1.is_ok());
    let profile2 = xraylib::ComptonProfile_Partial(26, xraylib::L3_SHELL, 50.0);
    assert!(profile2.is_ok());
    assert!((profile1.as_ref().unwrap() - profile2.unwrap()).abs() < 1E-6);
    assert!((profile1.unwrap() - 2.026953933016568e-06).abs() < 1E-6);
}

#[test]
fn test_bad_input() {
    assert!(xraylib::ComptonProfile(0, 0.0).is_err());
    assert!(xraylib::ComptonProfile(102, 0.0).is_ok());
    assert!(xraylib::ComptonProfile(103, 0.0).is_err());
    assert!(xraylib::ComptonProfile(26, -1.0).is_err());

    assert!(xraylib::ComptonProfile_Partial(0, xraylib::K_SHELL, 0.0).is_err());
    assert!(xraylib::ComptonProfile_Partial(102, xraylib::K_SHELL, 0.0).is_ok());
    assert!(xraylib::ComptonProfile_Partial(103, xraylib::K_SHELL, 0.0).is_err());
    assert!(xraylib::ComptonProfile_Partial(26, xraylib::K_SHELL, -1.0).is_err());
    assert!(xraylib::ComptonProfile_Partial(26, -1, 0.0).is_err());
}
