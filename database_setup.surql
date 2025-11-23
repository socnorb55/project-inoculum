DEFINE TABLE starter_feeding_log SCHEMAFULL;
DEFINE FIELD flour_amount ON TABLE starter_feeding_log TYPE float;
DEFINE FIELD starter_amount ON TABLE starter_feeding_log TYPE float;
DEFINE FIELD timestamp ON TABLE starter_feeding_log TYPE datetime;
DEFINE FIELD water_amount ON TABLE starter_feeding_log TYPE float;
DEFINE FIELD water_temp ON TABLE starter_feeding_log TYPE float;

DEFINE TABLE doughs SCHEMAFULL;
DEFINE FIELD fat ON TABLE doughs TYPE float;
DEFINE FIELD flour ON TABLE doughs TYPE float;
DEFINE FIELD hydration ON TABLE doughs TYPE float;
DEFINE FIELD leaven ON TABLE doughs TYPE float;
DEFINE FIELD name ON TABLE doughs TYPE string;
DEFINE FIELD salt ON TABLE doughs TYPE float;
DEFINE FIELD start_timestamp ON TABLE doughs TYPE datetime;
DEFINE FIELD status ON TABLE doughs TYPE string;
DEFINE FIELD sugar ON TABLE doughs TYPE float;
DEFINE FIELD total_weight ON TABLE doughs TYPE float;
DEFINE FIELD update_timestamp ON TABLE doughs TYPE datetime;
DEFINE FIELD water ON TABLE doughs TYPE float;

DEFINE TABLE dough_statuses SCHEMAFULL;
DEFINE FIELD dough_name ON TABLE dough_statuses TYPE string;
DEFINE FIELD dough_status ON TABLE dough_statuses TYPE string;
DEFINE FIELD timestamp ON TABLE dough_statuses TYPE datetime;