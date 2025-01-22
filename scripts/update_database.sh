#!/bin/bash

# Define log file
LOG_FILE="/home/terrhy999/brawl-hub/logs/update_database.log"

# Make sure to add Vercel Project ID and Vercel Token to your bashrc and crontab as environment variables
# Crontab entry to run every Monday at 2am:
# 0 2 * * 1 /root/brawl-hub/migration-tool/scripts/update_database.sh >> /root/brawl-hub/migration-tool/scripts/update_database.log 2>&1

echo "Script started at $(date)" >> $LOG_FILE

# Source the environment variables
set -o allexport
source /home/terrhy999/brawl-hub/.env
set +o allexport

# Run the Rust program to update the database
echo "Running Rust program..." >> $LOG_FILE
/home/terrhy999/brawl-hub/migration-tool/target/release/brawl_hub_migration_tool >> $LOG_FILE 2>&1
RUST_EXIT_CODE=$?

# Check if the Rust program was successful
if [ $RUST_EXIT_CODE -eq 0 ]; then
  # Run the top_cards.sh script

  echo "Rust program completed successfully." >> $LOG_FILE
  
  echo "Running top_cards.sh script..." >> $LOG_FILE
  
  sudo /home/terrhy999/brawl-hub/scripts/top_cards.sh >> $LOG_FILE 2>&1
  SHELL_EXIT_CODE=$?

  if [ $SHELL_EXIT_CODE -eq 0 ]; then
    echo "top_cards.sh script completed successfully." >> $LOG_FILE

    # Rebuild the NextJs frotend
    echo "Rebuilding Next.js frontend..." >> $LOG_FILE
    cd /home/terrhy999/brawl-hub/frontend
    rm -rf /home/terrhy999/brawl-hub/frontend/.next
    npm install >> $LOG_FILE 2>&1  # Install any new dependencies
    npm run build >> $LOG_FILE 2>&1  # Build the project
    BUILD_EXIT_CODE=$?

    if [ $BUILD_EXIT_CODE -eq 0 ]; then
      echo "Next.js frontend built successfully." >> $LOG_FILE
      
      # Restart the frontend service
      echo "Restarting Next.js frontend service..." >> $LOG_FILE
      sudo systemctl restart brawlhub-frontend >> $LOG_FILE 2>&1

      if [ $? -eq 0 ]; then
        echo "Next.js frontend restarted successfully." >> $LOG_FILE
      else
        echo "Failed to restart Next.js frontend." >> $LOG_FILE
      fi

    else
      echo "Next.js frontend build failed with exit code $BUILD_EXIT_CODE" >> $LOG_FILE
    fi

  else
    echo "top_cards.sh script failed with exit code $SHELL_EXIT_CODE" >> $LOG_FILE
  fi

else
  echo "Rust program failed with exit code $RUST_EXIT_CODE" >> $LOG_FILE
fi

echo "Script ended at $(date)" >> $LOG_FILE
