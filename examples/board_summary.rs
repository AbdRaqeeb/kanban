use sqlx::{Connection, PgConnection};
use crate::models::BoardSummary;

// fetch board summary
async fn board_summary(conn: &mut PgConnection, board_id: i64) -> BoardSummary {
    sqlx::query_as(
        "SELECT COUNT(*), status FROM cards WHERE board_id = $1 GROUP BY status",
    )
        .bind(board_id)
        .fetch_all(conn)
        .await
        .unwrap()
        .into()
}

fn main() {}