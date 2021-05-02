pub mod mt19937;

#[cfg(test)]
mod tests {
    use crate::mt19937::{MT19937, MT19937_64};

    #[test]
    fn test_mt19937() {
        let mut mt = MT19937::new(0);
        assert_eq!(mt.next(), 2357136044);
        assert_eq!(mt.next(), 2546248239);
        assert_eq!(mt.next(), 3071714933);
        mt.seed(42);
        assert_eq!(mt.next(), 1608637542);
    }

    #[test]
    fn test_mt19937_64() {
        let mut mt64 = MT19937_64::new(0);
        assert_eq!(mt64.next(), 2947667278772165694);
        mt64.seed(42);
        assert_eq!(mt64.next(), 13930160852258120406);
        assert_eq!(mt64.next(), 11788048577503494824);
        assert_eq!(mt64.next(), 13874630024467741450);
    }
}
