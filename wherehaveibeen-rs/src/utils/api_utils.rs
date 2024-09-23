use std::{collections::HashMap, str::FromStr};

pub fn get_query_parameter<T>(params: &HashMap<String, String>, value: &str) -> T
where
    T: FromStr + Default,
{
    return params
        .get(value)
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or_default();
}
