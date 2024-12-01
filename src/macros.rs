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
