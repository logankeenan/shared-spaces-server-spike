# Shared Spaces Server Spike

An app I created in 2020 to help me learn Rust and experiment with a few ideas. See [shared-spaces-app-spike](https://github.com/logankeenan/shared-spaces-app-spike) for details

## Dependencies
* rust
* node v10.16.3 or greater
* postgresql 11

## Auto-Reloading the Development Server
1. Install Dependencies
    ```
    cargo install systemfd cargo-watch
   ```
2. Running the server
    ```
    RUST_ENV=development systemfd --no-pid -s http::3000 -- cargo watch -x run
    ```
3. open [localhost:3000](localhost:3000)

## Tests
The app needs to be running in test mode before the tests can be run. 
1. `npm run test:app` This will then start the app in the test environment.  
     
2. `npm run test` This will create a test database w/ the schema and seed that database with test data. This runs the integration suite against the test app and database.


 
## Diesel

* Create new migration
    ```
    diesel migration generate user_add_security
    ```
* run migrations
    ```
    diesel migration run
    ```
* redo migration
    ```
    diesel migration redo
    ```
* re-create database
    ```
    diesel database --reset
    ```
  
  