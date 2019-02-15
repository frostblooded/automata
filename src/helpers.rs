// Build sets easily for easy testing and comparing
#[macro_export]
macro_rules! set {
    [$($x:expr),+] => {
        [$($x,)+].iter().map(|x| x.clone()).collect()
    }
}
