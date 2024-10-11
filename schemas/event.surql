DEFINE TABLE OVERWRITE event SCHEMAFULL;

DEFINE FIELD OVERWRITE title ON event TYPE string;
DEFINE FIELD OVERWRITE description ON event TYPE option<string>;
DEFINE FIELD OVERWRITE start_at ON event TYPE datetime;
DEFINE FIELD OVERWRITE deadline_at ON event TYPE option<datetime>;
DEFINE FIELD OVERWRITE status ON event TYPE string;
DEFINE FIELD OVERWRITE created_at ON event TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON event TYPE datetime VALUE time::now() READONLY;
-- Slots
DEFINE FIELD OVERWRITE slots ON event TYPE array<object>;
DEFINE FIELD OVERWRITE slots.*.jobs ON event TYPE array<record<job>> VALUE <set>($value);
DEFINE FIELD OVERWRITE slots.*.jobs.* ON event TYPE record<job> ASSERT record::exists($value);
