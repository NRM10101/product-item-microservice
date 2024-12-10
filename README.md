# Product and Item Management Service

This Rust-based API service uses Axum for the web framework and SeaORM for object-relational mapping to manage products and items within a database. The service provides a suite of RESTful endpoints to handle CRUD operations for products and items.

## Features 🚀

- **List all products**: Retrieve a list of all products stored in the database.
- **Get a product by ID**: Fetch a detailed view of a specific product by its unique identifier.
- **Create a new product**: Add a new product to the database.
- **Update a product by ID**: Modify details of an existing product.
- **Delete a product by ID**: Remove a product from the database using its ID.
- **List all items**: Retrieve a list of all items related to products.
- **Get an item by ID**: Fetch details of a specific item by its unique identifier.
- **Create a new item**: Add a new item to the database.
- **Update an item by ID**: Modify existing item details like name and price.
- **Delete an item by ID**: Remove an item from the database using its ID.

## Technologies Used 🛠️

- **Rust**: Empowering everyone to build reliable and efficient software.
- **Axum**: A web application framework suited for building scalable web services.
- **SeaORM**: An async & dynamic ORM for Rust.
- **PostgreSQL**: A powerful, open-source object-relational database system with a strong reputation for reliability and feature robustness.

## Getting Started 🏁

### Prerequisites

- Rust and Cargo (latest stable release)
- PostgreSQL server running on localhost (default settings)
- Docker (optional, for running inside a container)

### Setup

1. **Clone the repository**
    ```bash
    git clone https://github.com/NRM10101/product-item-microservice.git
    cd product-item-microservice
    ```

2. **Configure the database connection**
    Edit the `.env` file to include your database connection string:
    ```
    DATABASE_URL=postgres://username:password@localhost:port
    DB_NAME=dbname
    ```

### Running the Application 🚀

#### Locally

```bash
cargo run
```
This will start the server on `http://localhost:3000/`.

#### Using Docker

1. **Build the Docker image**
    ```bash
    docker build -t product-item-microservice .
    ```

2. **Run the Docker container**
    ```bash
    docker run -p 3000:3000 --name myapp product-item-microservice
    ```
    This will start the application and make it accessible at `http://localhost:3000/`.

## API Endpoints 📡

- `GET /products`: Lists all products.
- `POST /products`: Creates a new product.
- `GET /products/{id}`: Retrieves a product by ID.
- `PUT /products/{id}`: Updates a product by ID.
- `DELETE /products/{id}`: Deletes a product by ID.
- `GET /items`: Lists all items.
- `POST /items`: Creates a new item.
- `GET /items/{id}`: Retrieves an item by ID.
- `PUT /items/{id}`: Updates an item by ID.
- `DELETE /items/{id}`: Deletes an item by ID.

## Contributing 🤝

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

## Contact 📫

Project Link: [https://github.com/NRM10101/product-item-microservice](https://github.com/NRM10101/product-item-microservice/tree/main)
