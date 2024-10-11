# -- Roles
DEFINE TABLE OVERWRITE job_role SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON job_role TYPE string;
DEFINE FIELD OVERWRITE number ON job_role TYPE number;

-- Sub Roles
DEFINE TABLE OVERWRITE job_sub_role SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON job_sub_role TYPE string;
DEFINE FIELD OVERWRITE number ON job_sub_role TYPE number;
DEFINE FIELD OVERWRITE role ON job_sub_role TYPE record<job_role>;

-- Jobs
DEFINE TABLE OVERWRITE job SCHEMAFULL;
DEFINE FIELD OVERWRITE name ON job TYPE string;
DEFINE FIELD OVERWRITE number ON job TYPE number;
DEFINE FIELD OVERWRITE sub_role ON job TYPE record<job_sub_role>;

-- Role view
DEFINE TABLE OVERWRITE job_role_view TYPE NORMAL AS
SELECT *, id.id() as code,
    (
        SELECT *, id.id() as code,
            (
                SELECT *, id.id() as code
                FROM job WHERE sub_role = $parent.id ORDER BY number
            ) as jobs
        FROM job_sub_role WHERE role = $parent.id ORDER BY number
    ) as sub_roles
FROM job_role;