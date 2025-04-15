build-server:
	
run-server: run

build-server-docker:
	cd ./server && sudo docker compose up --build

run-server-docker:
	cd ./server && sudo docker compose up

