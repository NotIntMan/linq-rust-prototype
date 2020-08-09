use linq_first_step_proc_macro::expr;

#[test]
fn it_works() {
    let expr = expr!(|x| x * 2);
    assert_eq!(expr(2), 4);
}
