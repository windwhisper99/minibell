-- 
-- Event schema
DEFINE TABLE OVERWRITE event SCHEMAFULL;

-- Info
DEFINE FIELD OVERWRITE title ON event TYPE string;
DEFINE FIELD OVERWRITE description ON event TYPE option<string>;
DEFINE FIELD OVERWRITE status ON event TYPE string;

-- Access
DEFINE FIELD OVERWRITE host ON event TYPE option<record<member>>;

-- Slots
DEFINE FIELD OVERWRITE slots ON event TYPE array<object>;
DEFINE FIELD OVERWRITE slots.*.jobs ON event TYPE array<record<job>> VALUE <set>($value);
DEFINE FIELD OVERWRITE slots.*.jobs.* ON event TYPE record<job>;

-- Schedule
DEFINE FIELD OVERWRITE start_at ON event TYPE datetime;
DEFINE FIELD OVERWRITE deadline_at ON event TYPE option<datetime>;
-- Duration on minutes
DEFINE FIELD OVERWRITE duration ON event TYPE number;

-- Timestamp
DEFINE FIELD OVERWRITE created_at ON event TYPE datetime VALUE time::now() READONLY;
DEFINE FIELD OVERWRITE updated_at ON event TYPE datetime VALUE time::now();
DEFINE FIELD OVERWRITE published_at ON event TYPE option<datetime>;


-- 
-- Event log schema
DEFINE TABLE OVERWRITE event_log SCHEMAFULL;

DEFINE FIELD OVERWRITE event ON event_log TYPE record<event>;
DEFINE FIELD OVERWRITE type ON event_log TYPE string;
DEFINE FIELD OVERWRITE content ON event_log TYPE option<object>;

-- Timestamp
DEFINE FIELD OVERWRITE at ON event_log TYPE datetime VALUE time::now() READONLY;
