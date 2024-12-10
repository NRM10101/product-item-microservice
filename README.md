# Product and Item Management Service

This project is an API service developed in Rust, utilizing Axum for the web framework and SeaORM for object-relational mapping. It's designed to manage products and items in a database, offering a suite of RESTful endpoints.

## Features

- **List all products**: Retrieve a list of all products stored in the database.
- **Get a product by ID**: Fetch a detailed view of a specific product by its unique identifier.
- **Delete a product by ID**: Remove a product from the database using its ID.
- **List all items**: Retrieve a list of all items related to products.
- **Get an item by ID**: Fetch details of a specific item by its unique identifier.
- **Update item details**: Modify existing item details like name and price.
- **Delete an item by ID**: Remove an item from the database using its ID.

## Technologies Used

- **Rust**: A language empowering everyone to build reliable and efficient software.
- **Axum**: A web application framework that is particularly suited for building scalable web services.
- **SeaORM**: An async & dynamic ORM for Rust.
- **PostgreSQL**: A powerful, open source object-relational database system that uses and extends the SQL language combined with many features that safely store and scale the most complicated data workloads.

## Getting Started

### Prerequisites

- Rust and Cargo (latest stable release)
- PostgreSQL server running on localhost (default settings)

### Setup

1. **Clone the repository**

    ```bash
    git clone https://github.com/NRM10101/product-item-microservice.git
    cd product-item-service
    ```

2. **Configure the database connection**

    Edit the `.env` file to include your database connection string:

    ```
    DATABASE_URL=postgres://username:password@localhost:port
    DB_NAME=dbname
    
    ```

3. **Run the application**

    ```bash
    cargo run
    ```

    This will start the server on `http://localhost:3000/items` ,`http://localhost:3000/products`.

## API Endpoints

- `GET /products`: Lists all products.
- `GET /products/{id}`: Retrieves a product by ID.
- `DELETE /products/{id}`: Deletes a product by ID.
- `GET /items`: Lists all items.
- `GET /items/{id}`: Retrieves an item by ID.
- `DELETE /items/{id}`: Deletes an item by ID.
- `PUT /items/{id}`: Updates an item by ID.

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

## Contact

Project Link: [https://github.com/NRM10101/product-item-service](https://github.com/NRM10101/product-item-microservice/tree/main)

