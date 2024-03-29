# Shared Spaces Server Spike

This is a POC I created back in 2020. It's an app that allows users to share files P2P in-browser via WebRTC.

See [Past Rust Experiments: Positivelys & Shared Spaces](https://logankeenan.com/posts/past-rust-experiments-positivelys-and-shared-spaces/) for more details

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
  
  
