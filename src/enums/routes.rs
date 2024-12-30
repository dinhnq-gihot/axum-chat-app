const AUTH_PATH: &str = "/auth";
const USERS_PATH: &str = "/users";
const GROUPS_PATH: &str = "/groups";
const CHAT_PATH: &str = "/chat";

pub enum RoutePath {
    AUTH,
    USERS,
    GROUPS,
    CHAT,
}

impl RoutePath {
    pub fn get_path(&self) -> &'static str {
        match self {
            RoutePath::AUTH => AUTH_PATH,
            RoutePath::USERS => USERS_PATH,
            RoutePath::GROUPS => GROUPS_PATH,
            RoutePath::CHAT => CHAT_PATH,
        }
    }
}
