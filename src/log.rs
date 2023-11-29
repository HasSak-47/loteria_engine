#[derive(Default, Debug, Clone)]
pub struct Logger<T>(T, Vec<String>);

