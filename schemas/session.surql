DEFINE TABLE OVERWRITE session SCHEMAFULL;

DEFINE FIELD OVERWRITE member ON session TYPE record<member> ASSERT record::exists($value);
DEFINE FIELD OVERWRITE issued_at ON session TYPE datetime READONLY;
DEFINE FIELD OVERWRITE updated_at ON session TYPE datetime;
DEFINE FIELD OVERWRITE expires_at ON session TYPE datetime;

DEFINE INDEX OVERWRITE session_member_idx ON TABLE session COLUMNS member;
