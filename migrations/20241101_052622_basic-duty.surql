INSERT INTO duty_category (id, name) VALUES
    ('ultimate', 'Ultimate'),
    ('savage', 'Savage'),
    ('extreme', 'Extreme');

INSERT INTO duty (id, name, short_name, image_url, sort, category) VALUES
    (
        '1a863f1ea3b',
        'The Unending Coil of Bahamut',
        'UCoB',
        'https://lds-img.finalfantasyxiv.com/itemicon/1a/1aa609c136b7632b4fb51293c24580f7bb50203a.png',
        0,
        duty_category:ultimate
    ),
    (
        'e6c2c586ba6',
        'The Weapon\'s Refrain',
        'UWU',
        'https://lds-img.finalfantasyxiv.com/itemicon/06/06b4b4da4b5eacee722a5f930d4d23fa880268b1.png',
        1,
        duty_category:ultimate
    ),
    (
        'b56710190e9',
        'The Epic of Alexander',
        'TEA',
        'https://lds-img.finalfantasyxiv.com/itemicon/df/df109b51274e8555932056cd985fa4229cb37505.png',
        2,
        duty_category:ultimate
    ),
    (
        'ba184b63c2d',
        'Dragonsong\'s Reprise',
        'DSR',
        'https://lds-img.finalfantasyxiv.com/itemicon/31/31986c17621c516f86a08027610bf89d4db2abbc.png',
        3,
        duty_category:ultimate
    ),
    (
        'dccacd176f4',
        'The Omega Protocol',
        'TOP',
        'https://lds-img.finalfantasyxiv.com/itemicon/07/073014b7f15f053665cb42ab750eb62498f5443b.png',
        4,
        duty_category:ultimate
    );

INSERT INTO duty_phase (duty, name, progression) VALUES
    -- UCoB
    (duty:1a863f1ea3b, 'Twintania', 0),
    (duty:1a863f1ea3b, 'Nael Deus Darnus', 1),
    (duty:1a863f1ea3b, 'Quickmarch', 2),
    (duty:1a863f1ea3b, 'Blackfire', 2.1),
    (duty:1a863f1ea3b, 'Fellruin', 2.2),
    (duty:1a863f1ea3b, 'Heavensfall', 2.3),
    (duty:1a863f1ea3b, 'Tenstrike', 2.4),
    (duty:1a863f1ea3b, 'Grand octet', 2.5),
    (duty:1a863f1ea3b, 'Adds', 3),
    (duty:1a863f1ea3b, 'Golden Bahamut', 4),
    (duty:1a863f1ea3b, 'Enrage', 5),
    -- UWU
    (duty:e6c2c586ba6, 'Garuda', 0),
    (duty:e6c2c586ba6, 'Ifrit', 1),
    (duty:e6c2c586ba6, 'Titan', 2),
    (duty:e6c2c586ba6, 'Transition', 2.5),
    (duty:e6c2c586ba6, 'Predation', 3),
    (duty:e6c2c586ba6, 'Annihilation', 4),
    (duty:e6c2c586ba6, 'Suppression', 5),
    (duty:e6c2c586ba6, 'Aetheric Boom', 5.1),
    (duty:e6c2c586ba6, 'Primal Roulette', 5.2),
    (duty:e6c2c586ba6, 'Enrage', 6),
    -- TEA
    (duty:b56710190e9, 'Living Liquid', 0),
    (duty:b56710190e9, 'Limit cut', 0.1),
    (duty:b56710190e9, 'Brute Justice & Cruise Chaser', 1),
    (duty:b56710190e9, 'Alexander Prime: Inception', 2),
    (duty:b56710190e9, 'Alexander Prime: Wormhole', 2.1),
    (duty:b56710190e9, 'Perfect Alexander', 3),
    (duty:b56710190e9, 'Enrage', 4),
    -- DSR
    (duty:ba184b63c2d, 'Vault Knights', 0),
    (duty:ba184b63c2d, 'Strength of the Ward', 1),
    (duty:ba184b63c2d, 'Sanctity of the Ward', 1.1),
    (duty:ba184b63c2d, 'Meteors', 1.2),
    (duty:ba184b63c2d, 'Nidhogg', 2),
    (duty:ba184b63c2d, 'Eyes', 3),
    (duty:ba184b63c2d, 'Intermission', 3.5),
    (duty:ba184b63c2d, 'Wrath of the Heavens', 4),
    (duty:ba184b63c2d, 'Death of the Heavens', 4.5),
    (duty:ba184b63c2d, 'Fresh', 5),
    (duty:ba184b63c2d, 'Wroth Flames', 5.1),
    (duty:ba184b63c2d, 'Dragon-King Thordan', 6),
    (duty:ba184b63c2d, 'Enrage', 7),
    -- TOP
    (duty:dccacd176f4, 'Omega', 0),
    (duty:dccacd176f4, 'Party Synergy', 1),
    (duty:dccacd176f4, 'Hello World', 2),
    (duty:dccacd176f4, 'Monitors', 3),
    (duty:dccacd176f4, 'Blue Screen', 4),
    (duty:dccacd176f4, 'Delta', 5),
    (duty:dccacd176f4, 'Sigma', 5.1),
    (duty:dccacd176f4, 'Omega', 5.2),
    (duty:dccacd176f4, 'Alpha Omega', 6),
    (duty:dccacd176f4, 'Enrage', 7);
