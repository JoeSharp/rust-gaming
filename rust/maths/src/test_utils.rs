#[macro_export]
macro_rules! approx_eq {
    ($a:expr, $b:expr) => {
        approx_eq!($a, $b, 1e-6)
    };
    ($a:expr, $b:expr, $eps:expr) => {
        if ($a - $b).abs() > $eps {
            panic!(
                "assertion failed: |{} - {}| = {} > {}",
                $a,
                $b,
                ($a - $b).abs(),
                $eps
            );
        }
    };
}
#[macro_export]
macro_rules! approx_vec2 {
    ($a:expr, $b:expr) => {
        approx_vec2!($a, $b, 1e-6)
    };
    ($a:expr, $b:expr, $eps:expr) => {{
        let da = ($a.x - $b.x).abs();
        let db = ($a.y - $b.y).abs();
        if da > $eps || db > $eps {
            panic!(
                "assertion failed: Vector2 not approx equal\n\
                     left:  ({}, {})\n\
                     right: ({}, {})\n\
                     diffs: ({}, {}) > {}",
                $a.x, $a.y, $b.x, $b.y, da, db, $eps
            );
        }
    }};
}

#[macro_export]
macro_rules! approx_vec3 {
    ($a:expr, $b:expr) => {
        approx_vec3!($a, $b, 1e-6)
    };
    ($a:expr, $b:expr, $eps:expr) => {{
        let da = ($a.x - $b.x).abs();
        let db = ($a.y - $b.y).abs();
        let dc = ($a.z - $b.z).abs();
        if da > $eps || db > $eps || dc > $eps {
            panic!(
                "assertion failed: Vector3 not approx equal\n\
                     left:  ({}, {}, {})\n\
                     right: ({}, {}, {})\n\
                     diffs: ({}, {}, {}) > {}",
                $a.x, $a.y, $a.z, $b.x, $b.y, $b.z, da, db, dc, $eps
            );
        }
    }};
}
