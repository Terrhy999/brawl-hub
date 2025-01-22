.PHONY: migration-tool deploy weekly-update build-frontend restart-backend wait-for-backend
LOG_FILE = ./logs/deploy.log
# TIMESTAMP = $(shell date '+%Y-%m-%d %H:%M:%S')

# # Log a message with a timestamp
# log = @echo "[$(TIMESTAMP)] $$1" >> $(LOG_FILE)

# Helper to log with timestamp
log = @echo "[`date '+%Y-%m-%d %H:%M:%S'`] [$1] $2" | tee -a $(LOG_FILE)

# Run the Rust migration tool program to update the database
migration-tool:
	$(call log,"START","Starting database update")
	./migration-tool/target/release/brawl_hub_migration_tool >> $(LOG_FILE) 2>&1 || \
		{ $(call log,"ERROR","Database update failed"); exit 1; }
	$(call log,"SUCCESS","Database update completed")

# Build the frontend with no cache
build-frontend:
	$(call log,"START","Building frontend")
	docker compose build --no-cache frontend >> $(LOG_FILE) 2>&1 || \
		{ $(call log,"ERROR","Frontend build failed"); exit 1; }
	docker compose up -d frontend >> $(LOG_FILE) 2>&1 || \
		{ $(call log,"ERROR","Frontend container startup failed"); exit 1; }
	$(call log,"SUCCESS","Frontend build and startup completed")

# Restart the backend (db and server)
restart-backend:
	$(call log,"START","Restarting backend services")
	docker compose up -d db server >> $(LOG_FILE) 2>&1 || \
		{ $(call log,"ERROR", "Backend restart failed"); exit 1; }
	$(call log,"SUCCESS","Backend services restarted")

# Wait for the backend to be ready
wait-for-backend:
	$(call log,"Waiting for backend to be healthy")
	until curl -sf http://localhost:3030/health; do \
		echo "[$(TIMESTAMP)] Waiting for backend..."; \
		sleep 5; \
	done
	$(call log,"SUCCESS","Backend is healthy")

# Deploy the entire stack
deploy: restart-backend wait-for-backend build-frontend
	$(call log,"SUCCESS","Deployment completed successfully")

# Weekly update process
weekly-update: migration-tool deploy
	$(call log,"SUCCESS","Weekly update completed successfully")
