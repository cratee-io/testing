#[test]
fn figure_out_should_panic_and_ignored() {
    let test_vector = vec![
        (r"", (None, false)),
        (r" ", (None, false)),
        (r"#[should_panic]", (Some(""), false)),
        (
            r#"#[should_panic(expected = "hello")]"#,
            (Some("hello"), false),
        ),
        // below isn't ok
        //(
        //    r#"#[should_panic(expected="hello")]"#,
        //    (Some("hello"), false),
        //),
        //        (r#"#[should_panic(expected ="hello")]"#, "hello", false),
        //        (r#"#[should_panic(expected =  "hello")]"#, "hello", false),
        //(
        //    r#"#[should_panic(expected =
        //            "hello")]"#,
        //    (Some("hello"), false),
        //),
        (
            r#"#[should_panic(expected = "hello
world")]"#,
            (Some("hello\nworld"), false),
        ),
        (r"#[should_panic]#[ignore]", (Some(""), true)),
        (r"#[should_panic] #[ignore]", (Some(""), true)),
        (
            r"#[should_panic]
#[ignore]",
            (Some(""), true),
        ),
        ("#[ignore]", (None, true)),
        (r"#[ignore]#[should_panic]", (Some(""), true)),
        (
            r"#[ignore]
#[should_panic]",
            (Some(""), true),
        ),
        (
            r#"#[ignore]
        #[should_panic(expected = "hello")]"#,
            (Some("hello"), true),
        ),
    ];

    for (i, c) in test_vector.iter().enumerate() {
        let got = crate::figure_out_should_panic_and_ignored(c.0);
        assert_eq!(c.1, got, "#{}: fail at {}", i, c.0);
    }
}
