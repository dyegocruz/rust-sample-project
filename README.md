# Rust Sample Project

This is a sample project using the Rocket framework in Rust to create a simple web server with routes and integration with an opensearch instance.

## Requirements

- Rust (install using [rustup](https://rustup.rs/))
- Cargo (comes with Rust)
- OpenSearch (or Elasticsearch) running locally

## Setup

1. Clone the repository:

    ```sh
    git clone https://github.com/dyegocruz/rust-sample-project.git
    cd rust_sample_project
    ```

2. Install the dependencies:

    ```sh
    cargo build
    ```

3. Configure the environment variables by creating a [.env](http://_vscodecontentref_/0) file in the root of the project:

    ```env
    OPENSEARCH_URL=http://localhost:9200
    ```

## Running the Project

To run the Rocket server with hot reload, use `cargo-watch`:

1. Install `cargo-watch`:

    ```sh
    cargo install cargo-watch
    ```

2. Run the server with hot reload:

    ```sh
    cargo watch -x run
    ```

## Routes

### Main Route

- **GET /**

    Returns a simple message.

    ```sh
    curl http://localhost:8000/
    ```

### Search Route

- **GET /search?term={term}&lang={lang}**

    Performs a search in the catalog service.

    Parameters:
    - `term`: Search term (required)
    - `lang`: Language (optional, possible values: `en`, `pt-br`)

    ```sh
    curl "http://localhost:8000/search?term=example&lang=en"
    ```

## Project Structure

```plaintext
src/
├── main.rs
├── lib.rs
├── catalog_search/
│   ├── mod.rs
│   ├── catalog_search_service.rs
│   └── catalog_search_types.rs
