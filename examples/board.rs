use sqlx::{Connection, PgConnection};
use crate::models::{Board, CreateBoard};

// fetch all boards
async fn all_boards(conn: &mut PgConnection) -> Vec<Board> {
    sqlx::query_as("SELECT * FROM boards")
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch all boards in order
async fn all_boards_chronological(conn: &mut PgConnection) -> Vec<Board> {
    sqlx::query_as("SELECT * FROM boards ORDER BY created_at DESC")
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch all boards by primary key
async fn board_by_id(conn: &mut PgConnection, board_id: i64) -> Board {
    sqlx::query_as("SELECT * FROM boards WHERE id = $1")
        .bind(board_id)
        .fetch_one(conn)
        .await
        .unwrap()
}

// fetch all boards with a specific exact name
async fn board_by_name(conn: &mut PgConnection, board_name: &str) -> Vec<Board> {
    sqlx::query_as("SELECT * FROM boards WHERE name = $1")
        .bind(board_name)
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch all board whose name contains some string
async fn board_name_contains(conn: &mut PgConnection, contains: &str) -> Vec<Board> {
    // in LIKE queries "%" means "match zero or more of any character"
    let contains = format!("%{}%", contains);

    sqlx::query_as("SELECT * FROM boards WHERE name ILIKE $1")
        .bind(contains)
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch all boards created in the past 24 hours
async fn recent_boards(conn: &mut PgConnection) -> Vec<Board> {
    sqlx::query_as("SELECT * FROM boards WHERE created_at >= CURRENT_TIMESTAMP - INTERVAL '1 day'")
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch all boards created in the past 24 hours and whose name also contains some string
async fn recent_boards_and_name_contains(conn: &mut PgConnection, contains: &str) -> Vec<Board> {
    let contains = format!("%{}%", contains);
    sqlx::query_as("SELECT * FROM boards WHERE created_at >= CURRENT_TIMESTAMP - INTERVAL '1 day' AND name ILIKE $1")
        .bind(contains)
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch all boards created in the past 24 hours or whose name also contains some string
async fn recent_boards_or_name_contains(conn: &mut PgConnection, contains: &str) -> Vec<Board> {
    let contains = format!("%{}%", contains);
    sqlx::query_as("SELECT * FROM boards WHERE created_at >= CURRENT_TIMESTAMP - INTERVAL '1 day' OR name ILIKE $1")
        .bind(contains)
        .fetch_all(conn)
        .await
        .unwrap()
}

// create board from CreateBoard model
async fn create_board(conn: &mut PgConnection, board: CreateBoard) -> Board {
    sqlx::query_as("INSERT INTO boards (name) VALUES ($1) RETURNING *")
        .bind(board.name)
        .fetch_one(conn)
        .await
        .unwrap()
}

// delete all boards
async fn delete_all_boards(conn: &mut PgConnection) {
    sqlx::query("DELETE FROM boards")
        .execute(conn)
        .await
        .unwrap();
}

// delete a board by its id
async fn delete_board_by_id(conn: &mut PgConnection, board_id: i64) {
    sqlx::query("DELETE FROM boards WHERE id = $1")
        .execute(board_id)
        .execute(conn)
        .await
        .unwrap();
}

fn main() {}