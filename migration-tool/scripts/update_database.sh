#!/bin/bash

# Run the Rust program to update the database
/home/terrhy999/Documents/Code/brawl_hub/migration-tool/target/release/brawl_hub_migration_tool

# Check if the Rust program was successful
if [ $? -eq 0 ]; then
  # Run the top_cards.sh script
  ./top_cards.sh
  # echo "Rust Program running"
else
  echo "Rust program failed. Aborting script."
  exit 1
fi
