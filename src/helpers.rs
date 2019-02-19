// Build sets easily for easy testing and comparing
#[macro_export]
macro_rules! set {
    [$($x:expr),+] => {
        vec![$($x,)+].into_iter().collect()
    }
}
