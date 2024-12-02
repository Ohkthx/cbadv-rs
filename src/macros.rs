/// Checks if the agent is authenticated, returns an error if not.
macro_rules! get_auth {
    ($agent:expr, $method_name:expr) => {
        match $agent {
            Some(ref mut agent) => agent,
            None => {
                return Err(CbError::AuthenticationError(format!(
                    "Authentication required for '{}'.",
                    $method_name
                )));
            }
        }
    };
}

/// Checks if the agent is authenticated, returns an error if not.
macro_rules! is_auth {
    ($agent:expr, $method_name:expr) => {
        if $agent.is_none() {
            return Err(CbError::AuthenticationError(format!(
                "Authentication required for '{}'.",
                $method_name
            )));
        }
    };
}

/// Prints out a debug message, wraps `println!` macro.
#[macro_export]
macro_rules! debugln {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        println!(concat!("[DEBUG] ", $fmt), $($($arg)*)?);
    };
}
