use grep_lib::utils::*;

#[test]
fn get_env_val_test() {
    assert_eq!(get_main_env_args(vec![1, 2, 3].into_iter()), vec![2, 3]);
    assert_eq!(get_main_env_args(vec![] as Vec<u8>), vec![])
}
