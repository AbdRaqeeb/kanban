create-migration name:
    sqlx migrate add {{name}}

# assumes db container is running
run-migrations:
    sqlx migrate run

take-nap:
    sleep 3

# create a postgres container
postgres:
	docker run --name postgres -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=testPassword -d postgres:15.2-alpine

# create database
createdb:
	docker exec -it postgres createdb --username=root --owner=root kanban

# drop database
dropdb:
	docker exec -it postgres dropdb kanban

# start database container
startdb:
	docker start postgres

# stop database container
stopdb:
	docker stop postgres

build-native:
    RUSTFLAGS="-C target-cpu=native" cargo build --release

run-native:
    RUSTFLAGS="-C target-cpu=native" cargo run --release

get-index:
    curl --include http://localhost:8000/

get-boards:
    curl --header "Authorization: Bearer LET_ME_IN" --include http://localhost:8000/api/boards

get-board-summary:
    curl --header "Authorization: Bearer LET_ME_IN" --include http://localhost:8000/api/boards/1/summary

post-board:
    curl --header "Authorization: Bearer LET_ME_IN" --header "Content-Type: application/json" --request POST --data '{"name": "Created Test Board from CURL"}' --include http://localhost:8000/api/boards

delete-board board-id='1000':
    curl --header "Authorization: Bearer LET_ME_IN" --request DELETE --include http://localhost:8000/api/boards/{{board-id}}

get-cards board-id='1':
    curl --header "Authorization: Bearer LET_ME_IN" --include http://localhost:8000/api/boards/{{board-id}}/cards

post-card board-id='1':
    curl --header "Authorization: Bearer LET_ME_IN" --header "Content-Type: application/json" --request POST --data '{"boardId": 1, "description": "Created Test Card from CURL"}' --include http://localhost:8000/api/cards

patch-card card-id='1':
    curl --header "Authorization: Bearer LET_ME_IN" --header "Content-Type: application/json" --request PATCH --data '{"description": "Updated description from CURL", "status": "doing"}' --include http://localhost:8000/api/cards/{{card-id}}

delete-card card-id='1000':
    curl --header "Authorization: Bearer LET_ME_IN" --request DELETE --include http://localhost:8000/api/cards/{{card-id}}

test-api: get-index get-boards get-cards get-board-summary post-board post-card patch-card delete-board delete-card
