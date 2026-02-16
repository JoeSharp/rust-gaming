#[macro_export]

macro_rules! matrix {
    (
        rows: $rows:expr,
        cols: $cols:expr,
        $(
            $( $x:expr ),+ $(,)?
        );+ $(;)?
    ) => {{
        let data = vec![
            $(
                $(
                    $x as f64,
                )+
            )+
        ];

        Matrix::new($rows, $cols, data).unwrap()
    }};
}
