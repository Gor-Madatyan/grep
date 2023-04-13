#[cfg(test)]
#[path = "../tests/unit_tests/utils_tests.rs"]
mod utils_tests;

#[allow(unused)]
pub fn get_main_env_args<R, T: IntoIterator<Item=R>>(args: T) -> Vec<R> {
    args.into_iter().skip(1).collect()
}
