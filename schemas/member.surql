-- Member
DEFINE TABLE OVERWRITE member SCHEMAFULL;

DEFINE FIELD OVERWRITE global_name ON member TYPE string;
DEFINE FIELD OVERWRITE guild_name ON member TYPE option<string>;
DEFINE FIELD OVERWRITE global_avatar ON member TYPE option<string>;
DEFINE FIELD OVERWRITE guild_avatar ON member TYPE option<string>;
-- First time active with MiniBell timestamp
DEFINE FIELD OVERWRITE actived_at ON member TYPE datetime VALUE time::now() READONLY;
-- Joined in Discord guid timestamp
DEFINE FIELD OVERWRITE joined_at ON member TYPE datetime;
-- Last update information from Discord server 
DEFINE FIELD OVERWRITE updated_at ON member TYPE datetime VALUE time::now();
DEFINE FIELD OVERWRITE roles ON member TYPE array<number>;
