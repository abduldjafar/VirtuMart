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
   DATABASE_URL=surrealdb://localhost:8000
   REDIS_URL=redis://localhost:6379
   SECRET_KEY=your_secret_key
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
