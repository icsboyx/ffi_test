// src/lib.rs
static SOMETHING_TO_CHANGE: &str = "+++-*************-++";

#[unsafe(no_mangle)]
pub extern "C" fn plugin_function(payload: &str) -> String {
    let ret_val = format!(
        "Plugin {}:{} argument passed: {}{}",
        file!(),
        module_path!().to_string(),
        SOMETHING_TO_CHANGE,
        payload
    );
    ret_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_function() {
        let payload = "World";
        let expected = format!("{}{}", SOMETHING_TO_CHANGE, payload);
        let result = plugin_function(payload);
        assert_eq!(
            result
                .split_whitespace()
                .collect::<Vec<_>>()
                .last()
                .unwrap()
                .to_string(),
            expected
        );
    }
}
