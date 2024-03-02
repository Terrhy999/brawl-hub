#!/usr/bin/env bash

sudo -i -u postgres -H -- psql -d brawlhub -h localhost << EOF
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
    SELECT
        card.*,
        COUNT(DISTINCT deck.id) AS total_decks_could_play,
        COUNT(DISTINCT decklist.deck_id) AS total_decks_with_card
    FROM card
    JOIN deck ON deck.color_identity @> card.color_identity
    LEFT JOIN decklist ON card.oracle_id = decklist.oracle_id
    GROUP BY card.oracle_id
)
SELECT
    cc.oracle_id,
    cc.name_full,
    cc.color_identity,
    cc.total_decks_could_play,
    cc.total_decks_with_card,
    CASE
        WHEN cc.total_decks_could_play = 0 THEN 0 -- Avoid division by zero
        ELSE (cc.total_decks_with_card * 100.0 / cc.total_decks_could_play)
    END AS rank
FROM CardCounts cc
ORDER BY rank DESC
LIMIT 1000;
EOF
