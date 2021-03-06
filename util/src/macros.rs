#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr), +) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(map.insert($key, $value);)+
            map
        }
    }
}

#[macro_export]
macro_rules! set {
    ($($elem:expr), *) => {
        {
            let mut set = ::std::collections::HashSet::new();
            $(set.insert($elem);)*
            set
        }
    }
}
