-- Features for site-boundaries collection
WITH inserted_features AS (
    INSERT INTO app.project_features (
        project_id,
        collection_id,
        name,
        properties,
        added_by,
        last_updated_by,
        is_primary
    ) VALUES
    (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'site boundaries'),
        'Manchester Development Site',
        '{"site_type": "Development", "area_sqm": 200000, "status": "Active"}',
        0, 0,
        true
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'site boundaries'),
        'London Construction Zone',
        '{"site_type": "Construction", "area_sqm": 75000, "status": "Active"}',
        0, 0,
        false
        
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'site boundaries'),
        'Birmingham Industrial Park',
        '{"site_type": "Industrial", "area_sqm": 300000, "status": "Active"}',
        0, 0, 
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'site boundaries'),
        'Edinburgh Business Park',
        '{"site_type": "Business Park", "area_sqm": 140000, "status": "Planning"}',
         0, 0,
        false
    )
    RETURNING id, collection_id, name
)
INSERT INTO app.feature_objects (collection_id, project_feature_id, geom)
SELECT
    collection_id,
    id,
    CASE name
        WHEN 'Manchester Development Site' THEN ST_GeomFromEWKT('SRID=27700;POLYGON((384000 398000, 384500 398000, 384500 398400, 384000 398400, 384000 398000))')
        WHEN 'London Construction Zone' THEN ST_GeomFromEWKT('SRID=27700;POLYGON((530000 180000, 530300 180000, 530300 180250, 530000 180250, 530000 180000))')
        WHEN 'Birmingham Industrial Park' THEN ST_GeomFromEWKT('SRID=27700;POLYGON((405000 283000, 405600 283000, 405600 283500, 405000 283500, 405000 283000))')
        WHEN 'Edinburgh Business Park' THEN ST_GeomFromEWKT('SRID=27700;POLYGON((325000 673000, 325400 673000, 325400 673350, 325000 673350, 325000 673000))')
    END
FROM inserted_features;

-- Features for access-tracks collection
WITH inserted_features AS (
    INSERT INTO app.project_features (
        project_id,
        collection_id,
        name,
        properties,
        added_by,
        last_updated_by,
        is_primary
    ) VALUES
    (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'access tracks'),
        'Pennine Way Section',
        '{"track_type": "Hiking Trail", "difficulty": "Moderate", "surface": "Gravel", "length_m": 2100}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        true
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'access tracks'),
        'Farm Access Road',
        '{"track_type": "Access Road", "surface": "Dirt", "width_m": 3.5, "length_m": 850}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'access tracks'),
        'Coastal Path Section',
        '{"track_type": "Coastal Path", "difficulty": "Easy", "surface": "Paved", "length_m": 1250}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'access tracks'),
        'Mountain Bike Trail',
        '{"track_type": "Mountain Bike", "difficulty": "Hard", "surface": "Natural", "length_m": 1450}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'access tracks'),
        'Service Track',
        '{"track_type": "Service Road", "surface": "Gravel", "width_m": 4.0, "length_m": 320}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    )
    RETURNING id, collection_id, name
)
INSERT INTO app.feature_objects (collection_id, project_feature_id, geom)
SELECT
    collection_id,
    id,
    CASE name
        WHEN 'Pennine Way Section' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(398000 445000, 398200 445500, 398500 446000, 398700 446800)')
        WHEN 'Farm Access Road' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(405000 283000, 405500 283200, 406000 283500)')
        WHEN 'Coastal Path Section' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(295000 650000, 295300 650200, 295600 650500, 296000 650800)')
        WHEN 'Mountain Bike Trail' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(384200 398100, 384400 398300, 384650 398450, 384800 398700, 385000 398900)')
        WHEN 'Service Track' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(530100 180100, 530200 180150, 530300 180180)')
    END
FROM inserted_features;

-- Features for cable-routes collection
WITH inserted_features AS (
    INSERT INTO app.project_features (
        project_id,
        collection_id,
        name,
        properties,
        added_by,
        last_updated_by,
        is_primary
    ) VALUES
    (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'cable routes'),
        'Fiber Optic Main Line',
        '{"cable_type": "Fiber Optic", "capacity_gbps": 100, "owner": "BT", "depth_m": 1.2, "length_m": 5200}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        true
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'cable routes'),
        'Power Cable Route HV',
        '{"cable_type": "Electrical", "voltage_kv": 11, "owner": "National Grid", "depth_m": 0.8, "length_m": 1850}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'cable routes'),
        'Telecom Distribution Network',
        '{"cable_type": "Copper", "owner": "Virgin Media", "depth_m": 0.6, "length_m": 650}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'cable routes'),
        'Fiber Backbone Scotland',
        '{"cable_type": "Fiber Optic", "capacity_gbps": 400, "owner": "BT", "depth_m": 1.5, "length_m": 4800}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'cable routes'),
        'Low Voltage Distribution',
        '{"cable_type": "Electrical", "voltage_kv": 0.4, "owner": "UK Power Networks", "depth_m": 0.5, "length_m": 420}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        (SELECT id FROM app.collections WHERE title = 'cable routes'),
        'Data Center Interconnect',
        '{"cable_type": "Fiber Optic", "capacity_gbps": 1000, "owner": "Equinix", "depth_m": 2.0, "length_m": 780}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        false
    )
    RETURNING id, collection_id, name
)
INSERT INTO app.feature_objects (collection_id, project_feature_id, geom)
SELECT
    collection_id,
    id,
    CASE name
        WHEN 'Fiber Optic Main Line' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(530000 180000, 531000 181000, 532000 182000, 533000 183000)')
        WHEN 'Power Cable Route HV' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(384000 398000, 384500 398200, 385000 398500, 385500 399000)')
        WHEN 'Telecom Distribution Network' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(405000 283000, 405200 283100, 405400 283150, 405600 283200)')
        WHEN 'Fiber Backbone Scotland' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(325000 673000, 325500 674000, 326000 675000, 326500 676000)')
        WHEN 'Low Voltage Distribution' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(530100 180100, 530250 180200, 530400 180250)')
        WHEN 'Data Center Interconnect' THEN ST_GeomFromEWKT('SRID=27700;LINESTRING(405100 283100, 405300 283200, 405500 283250, 405800 283300)')
    END
FROM inserted_features;

-- Features for site-boundaries collection
WITH inserted_features AS (
    INSERT INTO app.project_features (
        project_id,
        collection_id,
        name,
        properties,
        added_by,
        last_updated_by,
        is_primary
    ) VALUES
 (
        (SELECT id FROM app.projects WHERE name = 'Test Project2'),
        (SELECT id FROM app.collections WHERE title = 'site boundaries'),
        'London Construction Zone',
        '{"site_type": "Construction", "area_sqm": 75000, "status": "Active"}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        true
        
    )
    RETURNING id, collection_id, name
)
INSERT INTO app.feature_objects (collection_id, project_feature_id, geom)
SELECT
    collection_id,
    id,
    CASE name
        WHEN 'London Construction Zone' THEN ST_GeomFromEWKT('SRID=27700;POLYGON((530000 180000, 530300 180000, 530300 180250, 530000 180250, 530000 180000))')
    END
FROM inserted_features;

-- Features for site-boundaries collection
WITH inserted_features AS (
    INSERT INTO app.project_features (
        project_id,
        collection_id,
        name,
        properties,
        added_by,
        last_updated_by,
        is_primary
    ) VALUES
 (
        (SELECT id FROM app.projects WHERE name = 'Test Project3'),
        (SELECT id FROM app.collections WHERE title = 'site boundaries'),
        'Birmingham Industrial Park',
        '{"site_type": "Industrial", "area_sqm": 300000, "status": "Active"}',
        (SELECT id FROM app.users WHERE username = 'root'),
        (SELECT id FROM app.users WHERE username = 'root'),
        true
    )
        
    
    RETURNING id, collection_id, name
)
INSERT INTO app.feature_objects (collection_id, project_feature_id, geom)
SELECT
    collection_id,
    id,
    CASE name
       WHEN 'Birmingham Industrial Park' THEN ST_GeomFromEWKT('SRID=27700;POLYGON((405000 283000, 405600 283000, 405600 283500, 405000 283500, 405000 283000))')
    END
FROM inserted_features;