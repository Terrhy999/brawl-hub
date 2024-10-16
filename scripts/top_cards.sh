#!/usr/bin/env bash

cd "$(dirname "$0")"
set -o allexport
source /home/terrhy999/brawl-hub/.env
set +o allexport

sudo -i PGPASSWORD=$PGPASSWORD psql -U $PGUSER -h $PGHOST -p $PGPORT -d $PGDATABASE --set=sslmode=require << EOF
CREATE TABLE IF NOT EXISTS top_cards (
    oracle_id uuid PRIMARY KEY,
    name_full text NOT NULL,
    color_identity char(1)[] NOT NULL,
    total_decks_could_play integer NOT NULL,
    total_decks_with_card integer NOT NULL,
    rank real NOT NULL -- Change the data type to real or numeric as needed
);

TRUNCATE TABLE top_cards;

INSERT INTO top_cards (oracle_id, name_full, color_identity, total_decks_could_play, total_decks_with_card, rank)
WITH CardCounts AS (
  SELECT card.*, tdpc.total_decks AS total_decks_with_card, tdwci.total_decks AS total_decks_could_play FROM card
  JOIN total_decks_per_card tdpc ON card.oracle_id = tdpc.oracle_id
  JOIN total_decks_with_color_identity tdwci ON card.color_identity = tdwci.color_identity
)
SELECT
cc.oracle_id,
    cc.name_full,
    cc.color_identity,
    cc.total_decks_could_play,
    cc.total_decks_with_card,
    CASE
        WHEN cc.total_decks_could_play = 0 THEN 0 -- Avoid division by zero
        ELSE (cc.total_decks_with_card * 100.0 / cc.total_decks_could_play) -- Calculate rank using total decks with color identity
    END AS rank
FROM CardCounts cc
ORDER BY rank DESC
LIMIT 1000;
EOF
