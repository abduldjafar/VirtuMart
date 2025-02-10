use service::user::user_service::UserService;
#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
}
