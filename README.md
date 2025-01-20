# VirtuMart

Welcome to VirtuMart, a simulated e-commerce backend application built to enhance my skills in backend development and Rust programming. This project demonstrates the implementation of modern technologies and best practices to create a robust and scalable e-commerce platform.

## Features

- User authentication and authorization
- Product catalog management
- Shopping cart functionality
- Order processing
- Database integration using SurrealDB
- Caching and session management using Redis

## Technologies Used

- **Programming Language**: Rust
- **Database**: SurrealDB
- **Cache**: Redis
- **Framework**: Axum
- **Containerization**: Docker
- **Version Control**: Git

## Getting Started

### Prerequisites

To run this project locally, you need to have the following installed:

- Rust (latest stable version)
- Docker
- Redis
- SurrealDB

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/VirtuMart.git
   cd VirtuMart
   ```

2. Set up the environment variables in a `.env` file:

   ```env
   REFRESH_TOKEN_PRIVATE_KEY=xxxxxxx
   REFRESH_TOKEN_PUBLIC_KEY=xxxxxxxt
   ACCESS_TOKEN_PRIVATE_KEY=xxxxxxx
   ACCESS_TOKEN_PUBLIC_KEY=xxxxxxx
   
   # Database configuration
   DB_HOST="127.0.0.1"  # Database host
   DB_PORT="5432"        # Database port (changed to a common default)
   DB_USER="user_$(date +%s)"  # Randomized username based on current timestamp
   DB_PASS="pass_$(date +%s)"  # Randomized password based on current timestamp
   DB_NAME="development_db_$(date +%s)" # Randomized database name
   DB_NAMESPACE="gymconnect" # Database namespace
   
   # Redis configuration
   REDIS_HOST="redis-$(shuf -i 10000-99999 -n 1).ec2.redns.redis-cloud.com" # Randomized Redis host
   REDIS_USERNAME="default" # Redis username
   REDIS_PASSWORD="password_$(date +%s)" # Randomized Redis password based on current timestamp
   REDIS_PORT="6379" # Common default Redis port
   
   # Mailjet configuration
   MAILJET_API_KEY="api_key_$(shuf -i 1000000000000000-9999999999999999 -n 1)" # Randomized Mailjet API key
   HOST_NAME="http://localhost:3000/api/v1/verify/" # Host name for verification API
   
   # Google Cloud Platform configuration
   GCP_CREDENTIALS_PATH="/path/to/gcp/credentials/$(date +%s)-gcp.json" # Randomized path for GCP credentials
   RUNNING_ENVIRONMENT="development" # Current running environment
   STORAGE_BUCKET="bucket_$(shuf -i 1000-9999 -n 1)" # Randomized Google Cloud Storage bucket name
   GOOGLE_STORAGE_API_HOST="https://storage.googleapis.com" # Google Storage API host
   ```

3. Build and run the application:

   ```bash
   cargo build
   cargo run
   ```

4. (Optional) Use Docker to run the application:

   ```bash
   docker-compose up --build
   ```

## Roadmap

- [ ] Add support for WebSocket notifications
- [ ] Implement payment gateway integration
- [ ] Enhance search functionality with filters and sorting
- [ ] Write comprehensive tests

## Contributing

Contributions are welcome! Please fork this repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Acknowledgments

- Inspiration for the project
- Resources and libraries used

---
Thank you for checking out VirtuMart. Happy coding!
