#![allow(dead_code)]

pub fn main() {
    // try_cfg();
    // try_feature_detected();
    // try_file_line_column();
    // try_dbg();
    // try_env();
    // try_include();
    // try_stringify();
    try_vec();
    // gen_try_feature_detected();
}

fn try_vec() {
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];
    dbg!(a, b);
}

fn try_stringify() {
    let a = stringify!(2 + 2);
    dbg!(&a);
}

fn try_include() {
    /*
    // This is the code in the included file:
    [1, 2, 3]
        .iter()
        .map(|x| *x)
        .cycle()
        .take(8)
        .collect::<Vec<i32>>();
    */
    dbg!(include!("include_test.txt"));

    let b = include_bytes!("include_test.txt");
    let s = String::from_utf8_lossy(b);
    dbg!(&s);

    let s = include_str!("include_test.txt");
    dbg!(&s);
}

fn try_env() {
    dbg!(env!("PATH"));
    dbg!(option_env!("PATH"));
    dbg!(option_env!("path"));
}

fn try_dbg() {
    let a = dbg!(3 * 4) + 1;
    dbg!(&a);
    assert_eq!(dbg!(a), 13);

    let s = "abc".to_string();
    println!("s = {}", dbg!(s));

    dbg!(vec!(1, 2, 3)
        .iter()
        .map(|x| dbg!(x) * 2)
        .collect::<Vec<i32>>());
}

fn try_file_line_column() {
    dbg!(file!());
    dbg!(line!());
    dbg!(column!());
    dbg!(module_path!());
}

fn try_feature_detected() {
    dbg!(is_x86_feature_detected!("aes"));
    dbg!(is_x86_feature_detected!("pclmulqdq"));
    dbg!(is_x86_feature_detected!("rdrand"));
    dbg!(is_x86_feature_detected!("rdseed"));
    dbg!(is_x86_feature_detected!("tsc"));
    dbg!(is_x86_feature_detected!("mmx"));
    dbg!(is_x86_feature_detected!("sse"));
    dbg!(is_x86_feature_detected!("sse2"));
    dbg!(is_x86_feature_detected!("sse3"));
    dbg!(is_x86_feature_detected!("ssse3"));
    dbg!(is_x86_feature_detected!("sse4.1"));
    dbg!(is_x86_feature_detected!("sse4.2"));
    dbg!(is_x86_feature_detected!("sse4a"));
    dbg!(is_x86_feature_detected!("sha"));
    dbg!(is_x86_feature_detected!("avx"));
    dbg!(is_x86_feature_detected!("avx2"));
    dbg!(is_x86_feature_detected!("avx512f"));
    dbg!(is_x86_feature_detected!("avx512cd"));
    dbg!(is_x86_feature_detected!("avx512er"));
    dbg!(is_x86_feature_detected!("avx512pf"));
    dbg!(is_x86_feature_detected!("avx512bw"));
    dbg!(is_x86_feature_detected!("avx512dq"));
    dbg!(is_x86_feature_detected!("avx512vl"));
    dbg!(is_x86_feature_detected!("avx512ifma"));
    dbg!(is_x86_feature_detected!("avx512vbmi"));
    dbg!(is_x86_feature_detected!("avx512vpopcntdq"));
    dbg!(is_x86_feature_detected!("f16c"));
    dbg!(is_x86_feature_detected!("fma"));
    dbg!(is_x86_feature_detected!("bmi1"));
    dbg!(is_x86_feature_detected!("bmi2"));
    dbg!(is_x86_feature_detected!("abm"));
    dbg!(is_x86_feature_detected!("lzcnt"));
    dbg!(is_x86_feature_detected!("tbm"));
    dbg!(is_x86_feature_detected!("popcnt"));
    dbg!(is_x86_feature_detected!("fxsr"));
    dbg!(is_x86_feature_detected!("xsave"));
    dbg!(is_x86_feature_detected!("xsaveopt"));
    dbg!(is_x86_feature_detected!("xsaves"));
    dbg!(is_x86_feature_detected!("xsavec"));
    dbg!(is_x86_feature_detected!("adx"));
    dbg!(is_x86_feature_detected!("rtm"));
}

fn gen_try_feature_detected() {
    // dbg!(is_x86_feature_detected!("mmx"));
    target_feature_list()
        .iter()
        .for_each(|x| println!("dbg!(is_x86_feature_detected!(\"{}\"));", x));
}

fn try_cfg() {
    dbg!(cfg!(unix));
    dbg!(cfg!(windows));
    dbg!(cfg!(target_family = "unix"));
    dbg!(cfg!(target_family = "windows"));
}

fn target_feature_list() -> Vec<String> {
    vec![
        "aes",
        "pclmulqdq",
        "rdrand",
        "rdseed",
        "tsc",
        "mmx",
        "sse",
        "sse2",
        "sse3",
        "ssse3",
        "sse4.1",
        "sse4.2",
        "sse4a",
        "sha",
        "avx",
        "avx2",
        "avx512f",
        "avx512cd",
        "avx512er",
        "avx512pf",
        "avx512bw",
        "avx512dq",
        "avx512vl",
        "avx512ifma",
        "avx512vbmi",
        "avx512vpopcntdq",
        "f16c",
        "fma",
        "bmi1",
        "bmi2",
        "abm",
        "lzcnt",
        "tbm",
        "popcnt",
        "fxsr",
        "xsave",
        "xsaveopt",
        "xsaves",
        "xsavec",
        "adx",
        "rtm",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}
