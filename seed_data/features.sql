-- Features for site-boundaries collection
INSERT INTO app.project_features (
    project_id,
    collection_id,
    name,
    properties,
    added_by,
    last_updated_by,
    is_primary,
    geom
)
SELECT
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'site boundaries'),
    name,
    jsonb_build_object('area_ellipsoidal_m2', ST_Area(ST_Transform(geom, 4326)::geography)),
    0, 0,
    is_primary,
    geom
FROM (VALUES
    (true,  'Manchester Development Site',  ST_GeomFromEWKT('SRID=27700;POLYGON((384000 398000, 384500 398000, 384500 398400, 384000 398400, 384000 398000))')),
    (false, 'London Construction Zone',     ST_GeomFromEWKT('SRID=27700;POLYGON((530000 180000, 530300 180000, 530300 180250, 530000 180250, 530000 180000))')),
    (false, 'Birmingham Industrial Park',   ST_GeomFromEWKT('SRID=27700;POLYGON((405000 283000, 405600 283000, 405600 283500, 405000 283500, 405000 283000))')),
    (false, 'Edinburgh Business Park',      ST_GeomFromEWKT('SRID=27700;POLYGON((325000 673000, 325400 673000, 325400 673350, 325000 673350, 325000 673000))'))
) AS t(is_primary, name, geom);

-- Features for access-tracks collection
INSERT INTO app.project_features (
    project_id,
    collection_id,
    name,
    properties,
    added_by,
    last_updated_by,
    is_primary,
    geom
) VALUES
(
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'access tracks'),
    'Pennine Way Section',
    '{}',
    0, 0,
    true,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(398000 445000, 398200 445500, 398500 446000, 398700 446800)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'access tracks'),
    'Farm Access Road',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(405000 283000, 405500 283200, 406000 283500)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'access tracks'),
    'Coastal Path Section',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(295000 650000, 295300 650200, 295600 650500, 296000 650800)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'access tracks'),
    'Mountain Bike Trail',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(384200 398100, 384400 398300, 384650 398450, 384800 398700, 385000 398900)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'access tracks'),
    'Service Track',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(530100 180100, 530200 180150, 530300 180180)')
);

-- Features for cable-routes collection
INSERT INTO app.project_features (
    project_id,
    collection_id,
    name,
    properties,
    added_by,
    last_updated_by,
    is_primary,
    geom
) VALUES
(
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'cable routes'),
    'Fiber Optic Main Line',
    '{}',
    0, 0,
    true,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(530000 180000, 531000 181000, 532000 182000, 533000 183000)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'cable routes'),
    'Power Cable Route HV',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(384000 398000, 384500 398200, 385000 398500, 385500 399000)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'cable routes'),
    'Telecom Distribution Network',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(405000 283000, 405200 283100, 405400 283150, 405600 283200)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'cable routes'),
    'Fiber Backbone Scotland',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(325000 673000, 325500 674000, 326000 675000, 326500 676000)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'cable routes'),
    'Low Voltage Distribution',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(530100 180100, 530250 180200, 530400 180250)')
), (
    (SELECT id FROM app.projects WHERE name = 'Test Project'),
    (SELECT id FROM app.collections WHERE title = 'cable routes'),
    'Data Center Interconnect',
    '{}',
    0, 0,
    false,
    ST_GeomFromEWKT('SRID=27700;LINESTRING(405100 283100, 405300 283200, 405500 283250, 405800 283300)')
);

-- Features for Test Project2 site-boundaries collection
INSERT INTO app.project_features (
    project_id,
    collection_id,
    name,
    properties,
    added_by,
    last_updated_by,
    is_primary,
    geom
)
SELECT
    (SELECT id FROM app.projects WHERE name = 'Test Project2'),
    (SELECT id FROM app.collections WHERE title = 'site boundaries'),
    name,
    jsonb_build_object('area_ellipsoidal_m2', ST_Area(ST_Transform(geom, 4326)::geography)),
    0, 0,
    is_primary,
    geom
FROM (VALUES
    (true, 'London Construction Zone', ST_GeomFromEWKT('SRID=27700;POLYGON((530000 180000, 530300 180000, 530300 180250, 530000 180250, 530000 180000))'))
) AS t(is_primary, name, geom);

-- Features for Test Project3 site-boundaries collection
INSERT INTO app.project_features (
    project_id,
    collection_id,
    name,
    properties,
    added_by,
    last_updated_by,
    is_primary,
    geom
)
SELECT
    (SELECT id FROM app.projects WHERE name = 'Test Project3'),
    (SELECT id FROM app.collections WHERE title = 'site boundaries'),
    name,
    jsonb_build_object('area_ellipsoidal_m2', ST_Area(ST_Transform(geom, 4326)::geography)),
    0, 0,
    is_primary,
    geom
FROM (VALUES
    (true, 'Birmingham Industrial Park', ST_GeomFromEWKT('SRID=27700;POLYGON((405000 283000, 405600 283000, 405600 283500, 405000 283500, 405000 283000))'))
) AS t(is_primary, name, geom);
