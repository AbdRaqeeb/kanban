// for GET requests

#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub id: i64,
    pub board_id: i64,
    pub description: String,
    pub status: Status,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Todo,
    Doing,
    Done,
}

#[derive(Default, serde::Serialize)]
pub struct BoardSummary {
    pub todo: i64,
    pub doing: i64,
    pub done: i64,
}

// convert list of status counts into a board summary
impl From<Vec<(i64, Status)>> for BoardSummary {
    fn from(counts: Vec<(i64, Status)>) -> BoardSummary {
        let mut summary = BoardSummary::default();

        for (count, status) in counts {
            match status {
                Status::Todo => summary.todo += count,
                Status::Doing => summary.doing += count,
                Status::Done => summary.done += count,
            }
        }

        summary
    }
}

// for POST requests

#[derive(serde::Deserialize)]
pub struct CreateBoard {
    pub name: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCard {
    pub board_id: i64,
    pub description: String,
}

// for PATCH requests

#[derive(serde::Deserialize)]
pub struct UpdateCard {
    pub description: String,
    pub status: Status,
}

// for authentication
#[derive(sqlx::FromRow)]
pub struct Token {
    pub id: String,
    pub expired_at: chrono::DateTime<chrono::Utc>,
}
