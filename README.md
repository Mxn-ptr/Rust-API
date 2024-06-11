# Simple Rust API with ScyllaDB, Actix-web, and bcrypt

This project is a Rust-based RESTful API using Actix-web, ScyllaDB as the database, and bcrypt for password hashing. The API allows users to create accounts, retrieve user information, update and delete users by ID. 

I created this first project in Rust to put into practice what I had learned. While it's a simple API, it has helped me gain a better understanding of the language and introduced me to Rust-specific concepts and practices. As I become more proficient in Rust, I will continue to improve this project.

## **Table of Contents**
- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running the Project](#running-the-project)
- [API Endpoints](#api-endpoints)
- [Project Structure](#project-structure)

## **Getting Started**

Follow these instructions to set up and run the project on your local machine for development and testing purposes.

### **Prerequisites**

Ensure you have the following installed on your machine:
- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/products/docker-desktop)
- [Docker Compose](https://docs.docker.com/compose/install/)

### **Installation**

Clone the repository:
    ```sh
    git clone https://github.com/mxn-ptr/rust-api.git
    cd rust-api
    ```

### **Running the Project**

1. Start the Docker container:
    ```sh
    docker-compose up -d
    ```

2. Run the application:
    ```sh
    cargo run
    ```

**The API will be available at `http://localhost:8080`.**

## API Endpoints

- ### **Create User**
    - **URL:** `/users`
    - **Method:** `POST`
    - **Request Body:**
      ```json
      {
          "email": "user@example.com",
          "password": "securepassword"
      }
      ```

- ### **Get All Users**
    - **URL:** `/users`
    - **Method:** `GET`


- ### **Get User by ID**
    - **URL:** `/users/{id}`
    - **Method:** `GET`

- ### **Login**
	- **URL:** `/users/login`
	- **Method:** `POST`
	- **Request Body:**
      ```json
      {
          "email": "user@example.com",
          "password": "securepassword"
      }
      ```

- ### **Update Email**
	- **URL:** `/users/{id}`
	- **Method:** `PUT`
	- **Request Body:**
		```json
		{
			"email": "user@example.com"
		}
		```
	
- ### **Reset Password**
	- **URL:** `/users/reset_password/{id}`
	- **Method:** `PUT`
	- **Request Body:**
		```json
		{
			"password": "new_password"
		}
		```

- ### **Delete User**
    - **URL:** `/users/{id}`
    - **Method:** `DELETE`

**Feel free to use import the following collection to test endpoints with Postman : [Rust API Collection](Rust-API.postman_collection.json)**

## **Author**

Maxence Potier

<a href="https://linkedin.com/in/maxence-potier" target="_blank">
<img src=https://img.shields.io/badge/linkedin-%231E77B5.svg?&style=for-the-badge&logo=linkedin&logoColor=white alt=linkedin style="margin-bottom: 5px;" />
</a>
<a href="https://github.com/Mxn-ptr" target="_blank">
<img src=https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white alt=linkedin style="margin-bottom: 5px;" />
</a>
