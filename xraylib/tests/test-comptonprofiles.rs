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

//     def test_pz_100(self):
//         profile = xraylib.ComptonProfile(26, 100.0)
//         self.assertAlmostEqual(profile, 1.8E-5, delta=1E-8)

//         profile = xraylib.ComptonProfile_Partial(26, xraylib.N1_SHELL, 100.0)
//         self.assertAlmostEqual(profile, 5.1E-9, delta=1E-12)

//         profile1 = xraylib.ComptonProfile_Partial(26, xraylib.L2_SHELL, 100.0)
//         profile2 = xraylib.ComptonProfile_Partial(26, xraylib.L3_SHELL, 100.0)
//         self.assertAlmostEqual(profile1, profile2, delta=1E-10)
//         self.assertAlmostEqual(profile1, 1.1E-8, delta=1E-10)

//     def test_pz_50(self):
//         profile = xraylib.ComptonProfile(26, 50.0)
//         self.assertAlmostEqual(profile, 0.0006843950273082384, delta=1E-8)

//         profile = xraylib.ComptonProfile_Partial(26, xraylib.N1_SHELL, 50.0)
//         self.assertAlmostEqual(profile, 2.4322755767709126e-07, delta=1E-10)

//         profile1 = xraylib.ComptonProfile_Partial(26, xraylib.L2_SHELL, 50.0)
//         profile2 = xraylib.ComptonProfile_Partial(26, xraylib.L3_SHELL, 50.0)
//         self.assertAlmostEqual(profile1, profile2, delta=1E-10)
//         self.assertAlmostEqual(profile1, 2.026953933016568e-06, delta=1E-10)

//     def test_bad_input(self):
//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile(0, 0.0)
//         xraylib.ComptonProfile(102, 0.0)
//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile(103, 0.0)
//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile(26, -1.0)

//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile_Partial(0, xraylib.K_SHELL, 0.0)
//         xraylib.ComptonProfile_Partial(102, xraylib.K_SHELL, 0.0)
//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile_Partial(103, xraylib.K_SHELL, 0.0)
//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile_Partial(26, xraylib.K_SHELL, -1.0)
//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile_Partial(26, -1, 0.0)
//         with self.assertRaises(ValueError):
//             xraylib.ComptonProfile_Partial(26, xraylib.N2_SHELL, 0.0)
//         with self.assertRaises(TypeError):
//             xraylib.ComptonProfile_Partial()
//         with self.assertRaises(TypeError):
//             xraylib.ComptonProfile_Partial("26", xraylib.N2_SHELL, 0.0)

// if __name__ == '__main__':
//     unittest.main(verbosity=2)
