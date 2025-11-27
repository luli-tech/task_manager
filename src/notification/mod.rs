// Declare submodules
pub mod notification_models;
pub mod notification_dto;
pub mod notification_repository;
pub mod notification_handlers;
pub mod notification_service;

// Re-export public items
pub use notification_service::start_notification_service;
