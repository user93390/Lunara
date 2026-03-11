dock_init:
	cargo generate-lockfile
	docker build -t lunara .

dock_compose:
	docker-compose up -d

kill_force:
	docker-compose down -v --rmi all --remove-orphans

dock_stop:
	docker-compose down
