use sqlx::{Connection, PgConnection};
use crate::models::{Card, CreateCard, Status, UpdateCard};

// fetch all cards
async fn all_cards(conn: &mut PgConnection) -> Vec<Card> {
    sqlx::query_as("SELECT * FROM cards")
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch cards by board
async fn cards_by_board(conn: &mut PgConnection, board_id: i64) -> Vec<Card> {
    sqlx::query_as("SELECT * FROM cards WHERE board_id = $1")
        .bind(board_id)
        .fetch_all(conn)
        .await
        .unwrap()
}

// fetch cards by status
async fn cards_by_status(conn: &mut PgConnection, status: Status) -> Vec<Card> {
    sqlx::query_as("SELECT * FROM cards WHERE status = $1")
        .bind(status)
        .fetch_all(conn)
        .await
        .unwrap()
}

// create card from CreateCard model
async fn create_card(conn: &mut PgConnection, card: CreateCard) -> Card {
    sqlx::query_as("INSERT INTO cards (board_id, description)  VALUES ($1, $2) RETURNING *")
        .bind(card.board_id)
        .bind(card.description)
        .fetch_one(conn)
        .await
        .unwrap()
}

// update c ard from UpdateCard model
async fn update_card(conn: &mut PgConnection, card_id: i64, update_card: UpdateCard) -> Card {
    sqlx::query_as("UPDATE cards SET description = $1, status = $2 WHERE id = $3 RETURNING *")
        .bind(update_card.description)
        .bind(update_card.status)
        .bind(card_id)
        .fetch_one(conn)
        .await
        .unwrap()
}

// delete all cards
async fn delete_cards(conn: &mut PgConnection) {
    sqlx::query("DELETE FROM cards")
        .execute(conn)
        .await
        .unwrap();
}

// delete a card  by id
async fn delete_card_by_id(conn: &mut PgConnection, card_id: i64) {
    sqlx::query("DELETE FROM cards WHERE id = $1")
        .bind(card_id)
        .execute(conn)
        .await
        .unwrap();
}

// delete all of the cards on a board
async fn delete_cards_by_board(conn: &mut PgConnection, board_id: i64) {
    sqlx::query("DELETE FROM cards WHERE board_id = $1")
        .bind(board_id)
        .execute(conn)
        .await
        .unwrap();
}

// delete all of the done cards on a board
async fn delete_done_cards_by_board(conn: &mut PgConnection, board_id: i64) {
    sqlx::query("DELETE FROM cards WHERE board_id = $1 AND status = 'done'")
        .bind(board_id)
        .execute(conn)
        .await
        .unwrap();
}

fn main() {}