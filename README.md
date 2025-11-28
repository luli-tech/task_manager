# Task Manager API

A robust task management REST API built with Rust, Axum, PostgreSQL, featuring JWT authentication, Google OAuth, real-time push notifications, and comprehensive API documentation.

## Features

- **User Authentication**
  - Manual registration and login with JWT
  - Short-lived access tokens (15 min) + long-lived refresh tokens (7 days)
  - Token refresh endpoint for seamless re‑authentication
  - Secure token revocation on logout
  - Google OAuth 2.0 integration
  - Secure password hashing with bcrypt
  - Role‑based authorization (user/admin)

- **Task Management**
  - Full CRUD operations
  - Filtering by status, priority, due date, etc.
  - Due dates and reminder times
  - Status tracking (Pending, InProgress, Completed, Archived)
  - Priority levels (Low, Medium, High, Urgent)

- **Push Notifications**
  - Real‑time notifications via Server‑Sent Events (SSE)
  - Automated cron job checking for due tasks
  - Per‑user notification preferences
  - Mark notifications as read / delete

- **API Documentation**
  - Interactive Swagger UI at `/swagger-ui`
  - OpenAPI 3.0 specification generated with `utoipa`
  - Complete endpoint documentation with examples

## Tech Stack

- **Framework**: Axum 0.7
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT + OAuth2 (Google)
- **Scheduling**: tokio‑cron‑scheduler
- **Documentation**: utoipa + Swagger UI
- **Validation**: validator

## Prerequisites

- Rust 1.70+
- PostgreSQL 14+
- Google Cloud OAuth credentials (for Google login)

## Setup

### 1. Clone the repository

```bash
git clone <repository-url>
cd task-manager
```

### 2. Set up the PostgreSQL database

```bash
# Create database
createdb task_manager
# Or using psql
psql -U postgres -c "CREATE DATABASE task_manager;"
```

### 3. Configure environment variables

```bash
cp .env.example .env
```

Edit `.env` and set the required values:

```env
DATABASE_URL=postgresql://username:password@localhost:5432/task_manager
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production
JWT_EXPIRATION_HOURS=24
GOOGLE_CLIENT_ID=your-google-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-google-client-secret
GOOGLE_REDIRECT_URI=http://localhost:3000/api/auth/google/callback
HOST=127.0.0.1
PORT=3000
RUST_LOG=info,task_manager=debug
```

### 4. Google OAuth setup

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Create/select a project
3. Enable the Google+ API
4. Create **OAuth 2.0 Client ID** credentials
5. Add the authorized redirect URI `http://localhost:3000/api/auth/google/callback`
6. Copy the client ID and secret into `.env`

### 5. Run database migrations

The application runs migrations automatically on startup, or you can run them manually:

```bash
cargo install sqlx-cli --no-default-features --features postgres
sqlx migrate run
```

### 6. Build and run the application

```bash
# Development
cargo run

# Production build
cargo build --release
./target/release/task-manager
```

The server will start on `http://localhost:3000`.

## API Documentation

Access the interactive Swagger UI at:

```
http://localhost:3000/swagger-ui
```

## API Endpoints

### Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Register a new user |
| POST | `/api/auth/login` | Login with email/password |
| POST | `/api/auth/refresh` | Refresh access token |
| POST | `/api/auth/logout` | Logout and revoke refresh token |
| GET | `/api/auth/google` | Initiate Google OAuth |
| GET | `/api/auth/google/callback` | Google OAuth callback |

### Tasks (requires authentication)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/tasks` | List all tasks (supports filters) |
| GET | `/api/tasks/:id` | Retrieve a single task |
| POST | `/api/tasks` | Create a new task |
| PUT | `/api/tasks/:id` | Update an existing task |
| DELETE | `/api/tasks/:id` | Delete a task |
| PATCH | `/api/tasks/:id/status` | Update task status |

### Messages (requires authentication)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/messages/conversations` | List conversations |
| GET | `/api/messages/conversations/{other_user_id}` | Get messages in a conversation |
| POST | `/api/messages` | Send a new message |
| PUT | `/api/messages/{message_id}/read` | Mark a message as read |

### Notifications (requires authentication)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/api/notifications` | List notifications |
| GET | `/api/notifications/stream` | SSE stream for real-time notifications |
| PATCH | `/api/notifications/:id/read` | Mark as read |
| DELETE | `/api/notifications/:id` | Delete notification |
| PUT | `/api/notifications/preferences` | Update preferences |



## Endpoint Use Cases

### Authentication
- **Register** – Create a new account with username, email, and password.
- **Login** – Obtain short‑lived access token and long‑lived refresh token.
- **Refresh** – Exchange a valid refresh token for a new access token without re‑entering credentials.
- **Logout** – Invalidate the refresh token, effectively signing the user out.
- **Google OAuth** – Sign‑in using a Google account, simplifying registration and login.

### Tasks
- **List Tasks** – Retrieve a paginated list; supports filtering by status, priority, due date, etc.
- **Get Task** – Fetch detailed information for a task identified by its UUID.
- **Create Task** – Authenticated users can create tasks with title, description, priority, due date, and optional reminder.
- **Update Task** – Modify mutable fields such as title, description, priority, or due date.
- **Delete Task** – Permanently remove a task.
- **Update Task Status** – Change the status (e.g., from `Pending` to `InProgress`). Used by clients to progress tasks through their lifecycle.

### Notifications
- **List Notifications** – Return all notifications for the authenticated user, optionally filtered by read/unread state.
- **Notification Stream (SSE)** – Open a Server‑Sent Events connection to receive real‑time push notifications when tasks reach their reminder time or other events occur.
- **Mark as Read** – Mark a specific notification as read, allowing UI state updates.
- **Delete Notification** – Remove a notification from the user's inbox.
- **Update Preferences** – Configure notification settings such as enabling/disabling email or push notifications.

## Usage Examples

### Register a user

```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"john_doe","email":"john@example.com","password":"securepassword123"}'
```

### Login

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"john@example.com","password":"securepassword123"}'
```

### Create a task with a reminder

```bash
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{"title":"Team Meeting","description":"Quarterly review meeting","priority":"High","due_date":"2025-11-25T14:00:00Z","reminder_time":"2025-11-25T13:45:00Z"}'
```

### Subscribe to notifications (SSE)

```bash
curl -N http://localhost:3000/api/notifications/stream \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

### Filter tasks

```bash
# High‑priority tasks
curl http://localhost:3000/api/tasks?priority=High \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# Completed tasks
curl http://localhost:3000/api/tasks?status=Completed \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## Project Structure

```text
task-manager/
├── migrations/                    # Database migrations
│   ├── 20251124_001_init.sql
│   └── 20251126_001_add_features.sql
│
├── src/
│   ├── admin/                     # Admin module
│   │   ├── admin.middleware.rs    # Admin authorization middleware
│   │   └── routes.rs              # Module exports
│   │
│   ├── auth/                      # Authentication module
│   │   ├── auth.dto.rs            # DTOs (RegisterRequest, LoginRequest, etc.)
│   │   ├── auth.handlers.rs       # Handlers (register, login, OAuth)
│   │   ├── auth.models.rs         # RefreshToken model
│   │   ├── auth.repository.rs     # RefreshToken repository
│   │   ├── auth.service.rs        # Business logic
│   │   ├── jwt.rs                 # JWT generation/validation
│   │   ├── oauth.rs               # Google OAuth client
│   │   ├── password.rs            # Password hashing/verification
│   │   └── routes.rs              # Module exports
│   │
│   ├── message/                   # Messaging module
│   │   ├── message.dto.rs         # DTOs
│   │   ├── message.handlers.rs    # Handlers
│   │   ├── message.models.rs      # Models
│   │   ├── message.repository.rs  # Repository
│   │   ├── message.service.rs     # Service layer
│   │   └── routes.rs              # Module exports
│   │
│   ├── notification/              # Notification module
│   │   ├── notification.dto.rs    # DTOs
│   │   ├── notification.handlers.rs # Handlers
│   │   ├── notification.models.rs # Models
│   │   ├── notification.repository.rs # Repository
│   │   ├── notification.service.rs # Service (background job)
│   │   └── routes.rs              # Module exports
│   │
│   ├── task/                      # Task module
│   │   ├── task.dto.rs            # DTOs (CreateTaskRequest, UpdateTaskRequest, etc.)
│   │   ├── task.handlers.rs       # Handlers
│   │   ├── task.models.rs         # Models (Task, TaskStatus, TaskPriority)
│   │   ├── task.repository.rs     # Repository
│   │   ├── task.service.rs        # Service layer
│   │   └── routes.rs              # Module exports
│   │
│   ├── user/                      # User module
│   │   ├── user.dto.rs            # DTOs (UpdateProfileRequest, etc.)
│   │   ├── user.handlers.rs       # Handlers
│   │   ├── user.models.rs         # Models (User, UserResponse)
│   │   ├── user.repository.rs     # Repository
│   │   ├── user.service.rs        # Service layer
│   │   └── routes.rs              # Module exports
│   │
│   ├── middleware/                # Middleware
│   │   └── auth.rs                # JWT authentication middleware
│   │
│   ├── db.rs                      # Database connection & migrations
│   ├── error.rs                   # Error handling & AppError type
│   ├── routes.rs                  # API route configuration
│   ├── state.rs                   # AppState & Config
│   └── main.rs                    # Application entry point
│
├── .github/
│   └── workflows/
│       └── ci.yml                 # GitHub Actions CI/CD
│
├── Cargo.toml                     # Rust dependencies
├── .env.example                   # Environment variables template
└── README.md
```

### Architecture Layers

- **Repository Layer** (`*.repository.rs`): Direct DB interactions using SQLx.
- **Service Layer** (`*.service.rs`): Business logic, orchestrates repositories.
- **Handler Layer** (`*.handlers.rs`): HTTP request/response handling, validation, calls services.
- **Models** (`*.models.rs`): Database models and response DTOs.
- **DTOs** (`*.dto.rs`): Request/response data structures with validation rules.

## Development

### Running tests

```bash
cargo test
```

### Code formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## How Notifications Work

1. When creating/updating a task, set a `reminder_time`.
2. A background cron job runs every minute.
3. Tasks with `reminder_time <= now` and `notified = false` trigger notifications.
4. Notifications are saved to the DB, broadcast via SSE, and the task is marked `notified = true`.

## Security Notes

- Use strong JWT secrets in production.
- Enable HTTPS.
- Rotate JWT tokens regularly.
- Keep Google OAuth credentials secure.
- Store sensitive data in environment variables.

## License

MIT

## Contributing

Pull requests are welcome! For major changes, please open an issue first.
