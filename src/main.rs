use simplelog::{Config, LevelFilter, SimpleLogger};
use sqlx::{sqlite::{SqliteConnectOptions}, ConnectOptions, Executor, Connection};

#[actix_rt::main]
async fn main() {
    SimpleLogger::init(LevelFilter::Trace, Config::default()).unwrap();
    let mut conn = SqliteConnectOptions::new()
        .pragma("vdbe_debug", "TRUE")
        .filename(":memory:")
        .connect()
        .await
        .unwrap();

    conn.execute("BEGIN").await.unwrap();

    // let mut tx = conn.begin().await.unwrap();

    // tx.execute(
    conn.execute(
        r#"
CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);
"#,
    )
    .await
    .unwrap();

    // let q1 = tx
    let q1 = conn
        .execute(
            r#"
INSERT INTO todos ( description )
VALUES ("foo"), ("bar"), ("baz")
"#,
        )
        .await
        .unwrap()
        .rows_affected();
    println!("rows_affected: {:?}", q1);

    // let q2 = tx
    let q2 = conn
        .execute(
            r#"
UPDATE todos
SET done = TRUE
WHERE description = "foo"
"#,
        )
        .await
        .unwrap()
        .rows_affected();
    println!("rows_affected: {:?}", q2);

    let q3 = conn
    // let q3 = tx
        .execute(
            r#"
DELETE FROM todos
WHERE description LIKE "ba%"
"#,
        )
        .await
        .unwrap()
        .rows_affected();
    println!("rows_affected: {:?}", q3);

    let q4 = conn.execute("COMMIT").await.unwrap().rows_affected();
    println!("rows_affected: {:?}", q4);

    // tx.commit().await.unwrap();


}
