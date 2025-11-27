// Declare existing modules
pub mod jwt;
pub mod oauth;
pub mod password;

// Declare submodules
pub mod auth_models;
pub mod auth_dto;
pub mod auth_repository;
pub mod auth_handlers;
pub mod auth_service;

// Re-export public items
pub use jwt::{create_access_token, create_refresh_token, verify_jwt};
pub use oauth::create_oauth_client;
pub use password::{hash_password, verify_password};
