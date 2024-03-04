WITH CommanderDecks AS (
    SELECT
        deck.commander AS commander_id,
        COUNT(DISTINCT deck.id) AS total_commander_decks
    FROM
        deck
    GROUP BY
        deck.commander
    HAVING
        COUNT(DISTINCT deck.id) >= 5
),
CardInput AS (
    SELECT card.oracle_id
    FROM card
    WHERE card.slug = 'elvish-mystic'
)
SELECT
    cd.commander_id,
    cd.total_commander_decks,
    COUNT(DISTINCT deck.id) AS decks_with_input_card,
    (COUNT(DISTINCT deck.id) * 100 / cd.total_commander_decks) AS rank,
    card.*
FROM
    CommanderDecks cd
JOIN
    deck ON cd.commander_id = deck.commander
JOIN
    decklist ON deck.id = decklist.deck_id
JOIN
    CardInput ci ON decklist.oracle_id = ci.oracle_id
JOIN
    card ON cd.commander_id = card.oracle_id
GROUP BY
    card.oracle_id,
    card.name_full,
    card.name_front,
    card.name_back,
    card.slug,
    card.scryfall_uri,
    card.layout,
    card.rarity,
    card.lowest_rarity,
    card.lang,
    card.mana_cost_combined,
    card.mana_cost_front,
    card.mana_cost_back,
    card.cmc,
    card.type_line_full,
    card.type_line_front,
    card.type_line_back,
    card.oracle_text,
    card.oracle_text_back,
    card.colors,
    card.colors_back,
    card.color_identity,
    card.is_legal,
    card.is_legal_commander,
    card.is_rebalanced,
    card.image_small,
    card.image_normal,
    card.image_large,
    card.image_art_crop,
    card.image_border_crop,
    card.image_small_back,
    card.image_normal_back,
    card.image_large_back,
    card.image_art_crop_back,
    card.image_border_crop_back,
    cd.commander_id,
    cd.total_commander_decks
ORDER BY
    rank DESC;

---

WITH CommanderDecks AS (
    SELECT
        deck.commander AS commander_id,
        COUNT(DISTINCT deck.id) AS total_commander_decks
    FROM
        deck
    GROUP BY
        deck.commander
)
SELECT
    cd.commander_id,
    cd.total_commander_decks
FROM
    CommanderDecks cd
WHERE
    cd.total_commander_decks < 2;
