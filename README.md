# VirtuMart

Welcome to VirtuMart, a simulated e-commerce backend application built to enhance my skills in backend development and Rust programming. This project demonstrates the implementation of modern technologies and best practices to create a robust and scalable e-commerce platform.

Here’s a `README.md` file with detailed information about the Tokopedia-like ERD and the associated table structures.

---
## Entities and Relationships

### **Entities:**
1. **Users**  
   Represents customers, sellers, and admins.
2. **Stores**  
   Represents seller-owned stores listing products.
3. **Products**  
   Represents items available for purchase.
4. **Categories**  
   Represents product classifications.
5. **Cart**  
   Represents the user's temporary storage of items for purchase.
6. **Orders**  
   Represents finalized transactions.
7. **Payments**  
   Represents payment details for orders.
8. **Shipping**  
   Represents shipping information for orders.
9. **Reviews**  
   Represents user feedback for products.

### **Relationships:**
- **Users** can own multiple **Stores**.
- **Stores** list multiple **Products**.
- **Users** can add multiple **Products** to their **Cart**.
- **Users** place **Orders** containing multiple **Products**.
- Each **Order** is associated with a **Payment**.
- Each **Product** belongs to a **Category**.
- Each **Order** is tied to a **Shipping** record.
- **Users** can write **Reviews** for **Products**.

---

## Table Structures

### **1. Users**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| name           | VARCHAR(255)| NOT NULL               |
| email          | VARCHAR(255)| UNIQUE, NOT NULL       |
| password       | VARCHAR(255)| NOT NULL               |
| role           | ENUM('customer', 'seller', 'admin') | DEFAULT 'customer' |
| created_at     | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

### **2. Stores**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| user_id        | INT         | FOREIGN KEY REFERENCES Users(id) |
| name           | VARCHAR(255)| NOT NULL               |
| description    | TEXT        | NULL                   |
| created_at     | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

### **3. Products**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| store_id       | INT         | FOREIGN KEY REFERENCES Stores(id) |
| category_id    | INT         | FOREIGN KEY REFERENCES Categories(id) |
| name           | VARCHAR(255)| NOT NULL               |
| price          | DECIMAL(10,2)| NOT NULL              |
| stock          | INT         | DEFAULT 0             |
| description    | TEXT        | NULL                   |
| created_at     | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

### **4. Categories**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| name           | VARCHAR(255)| NOT NULL               |
| created_at     | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

### **5. Cart**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| user_id        | INT         | FOREIGN KEY REFERENCES Users(id) |
| product_id     | INT         | FOREIGN KEY REFERENCES Products(id) |
| quantity       | INT         | NOT NULL               |
| created_at     | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

### **6. Orders**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| user_id        | INT         | FOREIGN KEY REFERENCES Users(id) |
| total_amount   | DECIMAL(10,2)| NOT NULL              |
| status         | ENUM('pending', 'completed', 'cancelled') | DEFAULT 'pending' |
| created_at     | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

### **7. Payments**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| order_id       | INT         | FOREIGN KEY REFERENCES Orders(id) |
| payment_method | VARCHAR(50) | NOT NULL               |
| payment_date   | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

### **8. Shipping**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| order_id       | INT         | FOREIGN KEY REFERENCES Orders(id) |
| address        | TEXT        | NOT NULL               |
| shipping_date  | TIMESTAMP   | NULL                   |
| status         | ENUM('pending', 'shipped', 'delivered') | DEFAULT 'pending' |

---

### **9. Reviews**
| Column         | Type        | Constraints             |
|-----------------|-------------|-------------------------|
| id             | INT         | PRIMARY KEY, AUTO_INCREMENT |
| user_id        | INT         | FOREIGN KEY REFERENCES Users(id) |
| product_id     | INT         | FOREIGN KEY REFERENCES Products(id) |
| rating         | INT         | CHECK(rating BETWEEN 1 AND 5) |
| comment        | TEXT        | NULL                   |
| created_at     | TIMESTAMP   | DEFAULT CURRENT_TIMESTAMP |

---

## Business Process

### **User Roles**
1. **Customer**: Browses, adds products to the cart, places orders, and writes reviews.
2. **Seller**: Manages a store, lists products, and fulfills orders.
3. **Admin**: Oversees platform operations, manages users, and resolves disputes.

---

### **Detailed Business Process**

#### 1. **User Registration and Authentication**
   - Users (both customers and sellers) create an account with their email, name, and password.
   - Sellers must complete additional verification (e.g., government ID, bank account).
   - Admins can create or disable user accounts if necessary.

#### 2. **Store Management (Seller)**
   - Sellers create stores with a name and description.
   - Sellers add, update, or delete products in their store.
   - Each product must have a name, price, stock, and category.

#### 3. **Product Browsing and Search**
   - Customers browse the platform by searching for products or filtering by category, price, or ratings.
   - Each product displays its name, price, description, seller info, and reviews.

#### 4. **Cart Management (Customer)**
   - Customers add products to their cart.
   - The system ensures that the product stock is available.
   - Customers can update product quantities in the cart or remove items.

#### 5. **Order Placement and Payment**
   - Customers place an order by checking out their cart.
   - The system creates an order and calculates the total cost.
   - Customers choose a payment method (e.g., credit card, e-wallet, bank transfer).
   - Upon successful payment, the order status updates to "paid."

#### 6. **Order Fulfillment (Seller and Shipping)**
   - Sellers receive notifications about new orders.
   - Sellers prepare the order and provide shipping information.
   - The order status updates to "shipped."
   - The shipping company tracks delivery and updates the status to "delivered."

#### 7. **Review and Feedback**
   - After delivery, customers can leave reviews and ratings for the product.
   - Reviews are tied to both the product and the customer who wrote them.
   - Sellers can view and respond to reviews to improve their service.

#### 8. **Admin Oversight**
   - Admins manage user accounts, stores, and products.
   - Admins monitor disputes between customers and sellers.
   - Admins ensure compliance with marketplace policies.

---

## User Flows

### **Customer Journey**
1. Register and log in.
2. Browse and search for products.
3. Add products to the cart.
4. Place an order and complete payment.
5. Receive products and leave a review.

### **Seller Journey**
1. Register and create a store.
2. Add products with detailed information.
3. Fulfill incoming orders and manage shipping.
4. Respond to customer reviews.

### **Admin Workflow**
1. Monitor and manage the platform.
2. Resolve disputes and enforce policies.
3. Oversee platform analytics and user activity.

---

## Additional Notes

- **Scalability**: The schema is designed to support millions of users, products, and transactions.
- **Security**: Sensitive data, like passwords, should be encrypted.
- **Optimization**: Indexes should be created on frequently queried fields, such as `email` in the Users table and `name` in the Products table.

This documentation provides the foundational steps to build and manage a marketplace platform. It can be further customized to include advanced features like promotions, seller dashboards, and recommendation engines.

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
