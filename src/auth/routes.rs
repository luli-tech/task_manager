// Re-export all auth module items
pub mod jwt;
pub mod oauth;
pub mod password;

pub use jwt::{create_jwt, create_access_token, create_refresh_token, verify_jwt, Claims};
pub use oauth::create_oauth_client;
pub use password::{hash_password, verify_password};
pub use crate::auth::auth_models::RefreshToken;
pub use crate::auth::auth_dto::{AuthResponse, LoginRequest, RegisterRequest, RefreshTokenRequest, RefreshTokenResponse};
pub use crate::auth::auth_repository::RefreshTokenRepository;
pub use crate::auth::auth_handlers::{register, login, google_login, google_callback, refresh_token, logout};
pub use crate::auth::auth_service::AuthService;
