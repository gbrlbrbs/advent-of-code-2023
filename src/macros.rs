macro_rules! max_by_field {
    ($var:expr, $field:ident) => {
        $var.iter().map(|x| x.$field).max().unwrap()
    }
}