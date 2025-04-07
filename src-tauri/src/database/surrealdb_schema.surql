-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: Artifact_library
-- ------------------------------

DEFINE TABLE Artifact_library TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD artifact_id ON Artifact_library TYPE int PERMISSIONS FULL;
DEFINE FIELD artifact_name ON Artifact_library TYPE string PERMISSIONS FULL;
DEFINE FIELD artifact_type ON Artifact_library TYPE int PERMISSIONS FULL;
DEFINE FIELD result ON Artifact_library TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD update_time ON Artifact_library TYPE datetime PERMISSIONS FULL;

DEFINE INDEX artifact_id ON Artifact_library FIELDS artifact_id UNIQUE;


-- ------------------------------
-- TABLE DATA: Artifact_library
-- ------------------------------


-- ------------------------------
-- TABLE: Hole_library
-- ------------------------------

DEFINE TABLE Hole_library TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD action1 ON Hole_library TYPE array<float> PERMISSIONS FULL;
DEFINE FIELD action2 ON Hole_library TYPE array<float> PERMISSIONS FULL;
DEFINE FIELD action3 ON Hole_library TYPE object PERMISSIONS FULL;
DEFINE FIELD action4 ON Hole_library TYPE object PERMISSIONS FULL;
DEFINE FIELD artifact_id ON Hole_library TYPE int PERMISSIONS FULL;
DEFINE FIELD artifact_name ON Hole_library TYPE string PERMISSIONS FULL;
DEFINE FIELD depth ON Hole_library TYPE float PERMISSIONS FULL;
DEFINE FIELD depth_result ON Hole_library TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD diameter ON Hole_library TYPE float PERMISSIONS FULL;
DEFINE FIELD diameter_orig_path ON Hole_library TYPE string PERMISSIONS FULL;
DEFINE FIELD diameter_result_path ON Hole_library TYPE string PERMISSIONS FULL;
DEFINE FIELD dimeter_result ON Hole_library TYPE float PERMISSIONS FULL;
DEFINE FIELD face_id ON Hole_library TYPE int PERMISSIONS FULL;
DEFINE FIELD hole_id ON Hole_library TYPE int PERMISSIONS FULL;
DEFINE FIELD luowen_orig_path ON Hole_library TYPE string PERMISSIONS FULL;
DEFINE FIELD luowen_result ON Hole_library TYPE option<bool> PERMISSIONS FULL;
DEFINE FIELD luowen_result_path ON Hole_library TYPE string PERMISSIONS FULL;
DEFINE FIELD update_time ON Hole_library TYPE datetime PERMISSIONS FULL;

DEFINE INDEX artifact_id ON Hole_library FIELDS artifact_id;


-- ------------------------------
-- TABLE DATA: Hole_library
-- ------------------------------


-- ------------------------------
-- TABLE: Log
-- ------------------------------

DEFINE TABLE Log TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD artifact_id ON Log TYPE int PERMISSIONS FULL;
DEFINE FIELD artifact_name ON Log TYPE string PERMISSIONS FULL;
DEFINE FIELD level ON Log TYPE int PERMISSIONS FULL;
DEFINE FIELD message ON Log TYPE string PERMISSIONS FULL;
DEFINE FIELD role ON Log TYPE int PERMISSIONS FULL;
DEFINE FIELD update_time ON Log TYPE datetime PERMISSIONS FULL;

DEFINE INDEX artifact_id ON Log FIELDS artifact_id;


-- ------------------------------
-- TABLE DATA: Log
-- ------------------------------


-- ------------------------------
-- TABLE: Run_log
-- ------------------------------

DEFINE TABLE Run_log TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD level ON Run_log TYPE int PERMISSIONS FULL;
DEFINE FIELD message ON Run_log TYPE string PERMISSIONS FULL;
DEFINE FIELD role ON Run_log TYPE int PERMISSIONS FULL;
DEFINE FIELD update_time ON Run_log TYPE datetime PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: Run_log
-- ------------------------------


