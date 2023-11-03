#!/usr/bin/env bash

sudo -i -u postgres -H -- psql -d brawlhub -h localhost -c "CREATE TABLE IF NOT EXISTS card (
    oracle_id uuid NOT NULL PRIMARY KEY,
    name text NOT NULL,
    lang text NOT NULL,
    scryfall_uri text NOT NULL,
    layout text NOT NULL,
    mana_cost text,
    cmc real NOT NULL,
    type_line text NOT NULL,
    oracle_text text,
    colors char(1)[],
    color_identity char(1)[] NOT NULL,
    is_legal bool NOT NULL,
    is_legal_commander bool NOT NULL,
    rarity text NOT NULL,
    image_small text NOT NULL,
    image_normal text NOT NULL,
    image_large text NOT NULL,
    image_art_crop text NOT NULL,
    image_border_crop text NOT NULL,
    is_alchemy bool NOT NULL DEFAULT false,
    slug text NOT NULL
);
CREATE TABLE IF NOT EXISTS deck (
    id SERIAL PRIMARY KEY,
    deck_id int UNIQUE,
    url text NOT NULL,
    username text NOT NULL,
    date_created bigint NOT NULL,
    date_updated bigint NOT NULL,
    commander uuid REFERENCES card(oracle_id) NOT NULL,
    companion uuid REFERENCES card(oracle_id)
);
CREATE TABLE IF NOT EXISTS decklist (
    oracle_id uuid REFERENCES card(oracle_id),
    deck_id int REFERENCES deck(id),
    is_companion bool NOT NULL DEFAULT false,
    is_commander bool NOT NULL DEFAULT false,
    quantity integer NOT NULL,
    PRIMARY KEY (oracle_id, deck_id)
)"