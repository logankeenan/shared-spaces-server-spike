pub type ConnectionManager = diesel::r2d2::ConnectionManager<diesel::pg::PgConnection>;
pub type Pool = diesel::r2d2::Pool<ConnectionManager>;
pub type PooledConnection = diesel::r2d2::PooledConnection<ConnectionManager>;

pub fn establish_connection_from(pool: &Pool) -> Option<PooledConnection> {
    let result = pool.get();

    match result {
        Ok(pool) => {
            Some(pool)
        },
        Err(_) => None,
    }
}

pub fn create_database_pool(database_url: String) -> Pool {
    let manager = ConnectionManager::new(database_url);
    let pool = Pool::builder()
        .build(manager);

    pool.expect("Failed to create connection pool")
}