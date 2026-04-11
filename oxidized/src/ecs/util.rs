macro_rules! query {
    ( ($( $name:ident ),*) in ($( $vec:expr ),*) ) => {
        izip!( $( $vec ),* ).filter_map(|tuple| {
            if let ( $( Some($name), )* ) = tuple {
                Some(($( $name ),*))
            } else {
                None
            }
        })
    };
}
