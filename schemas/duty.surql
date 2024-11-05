DEFINE TABLE OVERWRITE duty_category SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON duty_category TYPE string;


DEFINE TABLE OVERWRITE duty SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON duty TYPE string;
DEFINE FIELD OVERWRITE short_name ON duty TYPE option<string>;
DEFINE FIELD OVERWRITE image_url ON duty TYPE string;
DEFINE FIELD OVERWRITE sort ON duty TYPE number; -- This is a sort field for ordering the duties in the UI
DEFINE FIELD OVERWRITE category ON duty TYPE record<duty_category>;


DEFINE TABLE OVERWRITE duty_phase SCHEMAFULL;
DEFINE FIELD OVERWRITE duty ON duty_phase TYPE record<duty>;
DEFINE FIELD OVERWRITE name ON duty_phase TYPE string;
DEFINE FIELD OVERWRITE progression ON duty_phase TYPE number;


-- Duty view
DEFINE TABLE OVERWRITE duty_view TYPE NORMAL AS
SELECT *,
    (
        SELECT *
        FROM duty_phase WHERE duty = $parent.id
        ORDER BY progression
    ) as phases
FROM duty;

DEFINE TABLE OVERWRITE duty_category_view TYPE NORMAL AS
SELECT *,
    (
        SELECT *
        FROM duty_view WHERE category = $parent.id
        ORDER BY sort
    ) as duties
FROM duty_category;