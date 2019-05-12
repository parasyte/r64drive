#[cfg(test)]
mod tests {
    use r64drive::*;
    #[test]
    fn test_get_version() {
        let driver = test::R64DriverTest::new();
        let r64d = R64Drive::new(&driver);
        let (variant, version) = r64d.get_version().unwrap();
        assert!(variant != HardwareVariant::Unexpected);
        assert!(version.into_inner() == 205);
    }
}
