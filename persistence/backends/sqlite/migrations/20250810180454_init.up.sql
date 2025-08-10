CREATE TABLE IF NOT EXISTS "decks" (
    "id" BLOB
        PRIMARY KEY,
    "name" VARCHAR NOT NULL,
    "active" BOOLEAN NOT NULL DEFAULT TRUE
);

INSERT INTO decks
    (
        "id",
        "name",
        "active"
    )
    VALUES
        (
            X'019899fbb71a730f846f478e315ebd34',
            'Zombify',
            1
        );

CREATE TABLE IF NOT EXISTS "colors" (
    "id" BLOB
        PRIMARY KEY,
    "name" VARCHAR NOT NULL
);

INSERT INTO colors(
    id,
    name
)
    VALUES
        (
            X'01989559ac1b78c08b88d15ffe406b29',
            'Red'
        ),
        (
            X'0198955a46fc7a118f0d0d3b7c2cdc97',
            'Blue'
        ),
        (
            X'0198955a656d778eaa4c852d6cdb1dd6',
            'Black'
        ),
        (
            X'0198955a7fa1731aa46e75b10cca5bf0',
            'White'
        ),
        (
            X'0198955a9f437299b28378d2cb8f08b3',
            'Green'
        ),
        (
            X'0198955ae72c4344a5bd09fb1e014012',
            'Colorless'
        );

CREATE TABLE IF NOT EXISTS "players" (
    "id" BLOB
        PRIMARY KEY,
    "name" VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS "deck_colors" (
    "deck_id" BLOB NOT NULL,
    "color_id" BLOB NOT NULL,
    PRIMARY KEY ("deck_id", "color_id"),
    CONSTRAINT fk_deck_colors_deck
        FOREIGN KEY ("deck_id")
            REFERENCES "decks" ("id")
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    CONSTRAINT fk_deck_colors_color
        FOREIGN KEY ("color_id")
            REFERENCES "colors" ("id")
            ON UPDATE CASCADE
            ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "deck_colors_deck_id_idx" ON "deck_colors" ("deck_id");

CREATE INDEX IF NOT EXISTS "deck_colors_color_id_idx" ON "deck_colors" ("color_id");

CREATE TABLE IF NOT EXISTS "matches" (
    "id" BLOB
        PRIMARY KEY,
    "start" DATETIME NOT NULL DEFAULT (DATETIME('now'))
);

CREATE INDEX IF NOT EXISTS "matches_start_idx" ON "matches" ("start");

CREATE TABLE IF NOT EXISTS "participants" (
    "id" BLOB
        PRIMARY KEY,
    "match_id" BLOB NOT NULL,
    "player_id" BLOB NOT NULL,
    "deck_id" BLOB NOT NULL,
    "placement" INTEGER,
    "placement_time" DATETIME,
    CONSTRAINT fk_participant_deck
        FOREIGN KEY ("deck_id")
            REFERENCES "decks" ("id")
            ON UPDATE CASCADE
            ON DELETE RESTRICT,
    CONSTRAINT fk_participant_match
        FOREIGN KEY ("match_id")
            REFERENCES "matches" ("id")
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    CONSTRAINT fk_participant_player
        FOREIGN KEY ("player_id")
            REFERENCES "players" ("id")
            ON UPDATE CASCADE
            ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS "participants_match_id_idx" ON "participants" ("match_id");

CREATE INDEX IF NOT EXISTS "participants_player_id_idx" ON "participants" ("player_id");

CREATE INDEX IF NOT EXISTS "participants_deck_id_idx" ON "participants" ("deck_id");

CREATE INDEX IF NOT EXISTS "participants_placement_idx" ON "participants" ("placement");

CREATE TABLE IF NOT EXISTS "deck_mutations" (
    "id" BLOB
        PRIMARY KEY,
    "deck_id" BLOB NOT NULL,
    -- -1 - disbanded
    -- 1 - minor
    -- 2 - major
    -- 3 - rework
    "type" INTEGER NOT NULL,
    "timestamp" DATETIME NOT NULL DEFAULT (DATETIME('now')),
    CONSTRAINT fk_deck_mutations_deck
        FOREIGN KEY ("deck_id")
            REFERENCES "decks" ("id")
            ON UPDATE CASCADE
            ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "deck_mutations_deck_id_idx" ON "deck_mutations" ("deck_id");

CREATE INDEX IF NOT EXISTS "deck_mutations_type_idx" ON "deck_mutations" ("type");

CREATE INDEX IF NOT EXISTS "deck_mutations_timestamp_idx" ON "deck_mutations" ("timestamp");

CREATE TABLE IF NOT EXISTS "deck_owners" (
    "player_id" BLOB NOT NULL,
    "deck_id" BLOB NOT NULL,
    PRIMARY KEY ("player_id", "deck_id"),
    CONSTRAINT fk_deck_owners_player
        FOREIGN KEY ("player_id")
            REFERENCES "players" ("id")
            ON UPDATE CASCADE
            ON DELETE CASCADE,
    CONSTRAINT fk_deck_owners_deck
        FOREIGN KEY ("deck_id")
            REFERENCES "decks" ("id")
            ON UPDATE CASCADE
            ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS "deck_owners_player_id_idx" ON "deck_owners" ("player_id");

CREATE INDEX IF NOT EXISTS "deck_owners_deck_id_idx" ON "deck_owners" ("deck_id");
