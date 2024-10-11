INSERT INTO job_role (number, id, name) VALUES
    (0, 'tank', 'Tank'),
    (1, 'healer', 'Healer'),
    (2, 'dps', 'DPS');

INSERT INTO job_sub_role (number, id, name, role) VALUES
    (0, 'tank', 'Tank', job_role:tank),
    (1, 'pure', 'Pure Healer', job_role:healer),
    (2, 'shield', 'Shield Healer', job_role:healer),
    (3, 'melee', 'Melee DPS', job_role:dps),
    (4, 'ranged', 'Ranged DPS', job_role:dps),
    (5, 'caster', 'Caster DPS', job_role:dps);

INSERT INTO job (number, id, name, sub_role) VALUES
    (0, 'pld', 'Paladin', job_sub_role:tank),
    (1, 'war', 'Warrior', job_sub_role:tank),
    (2, 'drg', 'Dragoon', job_sub_role:melee),
    (3, 'mnk', 'Monk', job_sub_role:melee),
    (4, 'brd', 'Bard', job_sub_role:ranged),
    (5, 'blm', 'Black Mage', job_sub_role:caster),
    (6, 'smn', 'Summoner', job_sub_role:caster),
    (7, 'sch', 'Scholar', job_sub_role:shield),
    (8, 'whm', 'White Mage', job_sub_role:pure),
    (9, 'nin', 'Ninja', job_sub_role:melee),
    (10, 'drk', 'Dark Knight', job_sub_role:tank),
    (11, 'mch', 'Machinist', job_sub_role:ranged),
    (12, 'ast', 'Astrologian', job_sub_role:pure),
    (13, 'sam', 'Samurai', job_sub_role:melee),
    (14, 'rdm', 'Red Mage', job_sub_role:caster),
    (15, 'gnb', 'Gunbreaker', job_sub_role:tank),
    (16, 'dnc', 'Dancer', job_sub_role:ranged),
    (17, 'rpr', 'Reaper', job_sub_role:melee),
    (18, 'sge', 'Sage', job_sub_role:shield),
    (19, 'vpr', 'Viper', job_sub_role:melee),
    (20, 'pct', 'Pictomancer', job_sub_role:caster);
