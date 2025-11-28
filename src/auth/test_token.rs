// Simple test for token generation and verification
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use uuid::Uuid;
    use crate::auth::jwt::{create_access_token, create_refresh_token, verify_jwt};
    let user_id = Uuid::new_v4();
    let email = "test@example.com";
    let role = "user";
    let secret = "mysecretkey";
    let access = create_access_token(user_id, email, role, secret)?;
    println!("Access token: {}", access);
    let refresh = create_refresh_token(user_id, email, role, secret)?;
    println!("Refresh token: {}", refresh);
    let claims = verify_jwt(&refresh, secret)?;
    println!("Verified refresh token claims: {{ sub: {}, email: {}, role: {}, exp: {} }}", claims.sub, claims.email, claims.role, claims.exp);
    Ok(())
}
