use dialoguer::{Input, Password};
use log::debug;

pub fn get_connection_info(
    host: Option<String>,
    user: Option<String>,
    password: Option<String>,
) -> (String, String, String) {
    debug!("Getting connection info");

    debug!("Host: {:?}", host);
    debug!("User: {:?}", user);
    let password_is_set = password.is_some();
    debug!("Password is set: {:?}", password_is_set);

    // Initialize Host, User, and Password
    let host: String = host.unwrap_or_else(|| {
        let input: String = Input::new()
            .with_prompt("Enter ilo host")
            .interact_text()
            .unwrap();
        input
    });

    let user: String = user.unwrap_or_else(|| {
        let input: String = Input::new()
            .with_prompt("Enter ilo username")
            .interact_text()
            .unwrap();
        input
    });

    let password: String = password.unwrap_or_else(|| {
        let input: String = Password::new()
            .with_prompt("Enter ilo password")
            .interact()
            .unwrap();
        input
    });
    debug!("Host: {}", host);
    debug!("User: {}", user);

    let password_is_set = !password.is_empty();

    debug!("Password is set: {}", password_is_set);

    (host, user, password)
}
