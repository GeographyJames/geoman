-- Features for site-boundaries collection
INSERT INTO app.features (
    project_id,
    collection_id,
    name,
    geom,
    properties,
    added_by,
    last_updated_by
) VALUES (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'site-boundaries'),
    'Manchester Development Site',
    ST_GeomFromEWKT('SRID=27700;POLYGON((384000 398000, 384500 398000, 384500 398400, 384000 398400, 384000 398000))'),
    '{"site_type": "Development", "area_sqm": 200000, "status": "Active"}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'site-boundaries'),
    'London Construction Zone',
    ST_GeomFromEWKT('SRID=27700;POLYGON((530000 180000, 530300 180000, 530300 180250, 530000 180250, 530000 180000))'),
    '{"site_type": "Construction", "area_sqm": 75000, "status": "Active"}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'site-boundaries'),
    'Birmingham Industrial Park',
    ST_GeomFromEWKT('SRID=27700;POLYGON((405000 283000, 405600 283000, 405600 283500, 405000 283500, 405000 283000))'),
    '{"site_type": "Industrial", "area_sqm": 300000, "status": "Active"}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'site-boundaries'),
    'Edinburgh Business Park',
    ST_GeomFromEWKT('SRID=27700;POLYGON((325000 673000, 325400 673000, 325400 673350, 325000 673350, 325000 673000))'),
    '{"site_type": "Business Park", "area_sqm": 140000, "status": "Planning"}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);

-- Features for access-tracks collection
INSERT INTO app.features (
    project_id,
    collection_id,
    name,
    geom,
    properties,
    added_by,
    last_updated_by
) VALUES (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'access-tracks'),
    'Pennine Way Section',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(398000 445000, 398200 445500, 398500 446000, 398700 446800)'),
    '{"track_type": "Hiking Trail", "difficulty": "Moderate", "surface": "Gravel", "length_m": 2100}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'access-tracks'),
    'Farm Access Road',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(405000 283000, 405500 283200, 406000 283500)'),
    '{"track_type": "Access Road", "surface": "Dirt", "width_m": 3.5, "length_m": 850}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'access-tracks'),
    'Coastal Path Section',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(295000 650000, 295300 650200, 295600 650500, 296000 650800)'),
    '{"track_type": "Coastal Path", "difficulty": "Easy", "surface": "Paved", "length_m": 1250}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'access-tracks'),
    'Mountain Bike Trail',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(384200 398100, 384400 398300, 384650 398450, 384800 398700, 385000 398900)'),
    '{"track_type": "Mountain Bike", "difficulty": "Hard", "surface": "Natural", "length_m": 1450}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'access-tracks'),
    'Service Track',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(530100 180100, 530200 180150, 530300 180180)'),
    '{"track_type": "Service Road", "surface": "Gravel", "width_m": 4.0, "length_m": 320}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);

-- Features for cable-routes collection
INSERT INTO app.features (
    project_id,
    collection_id,
    name,
    geom,
    properties,
    added_by,
    last_updated_by
) VALUES (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'cable-routes'),
    'Fiber Optic Main Line',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(530000 180000, 531000 181000, 532000 182000, 533000 183000)'),
    '{"cable_type": "Fiber Optic", "capacity_gbps": 100, "owner": "BT", "depth_m": 1.2, "length_m": 5200}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'cable-routes'),
    'Power Cable Route HV',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(384000 398000, 384500 398200, 385000 398500, 385500 399000)'),
    '{"cable_type": "Electrical", "voltage_kv": 11, "owner": "National Grid", "depth_m": 0.8, "length_m": 1850}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'cable-routes'),
    'Telecom Distribution Network',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(405000 283000, 405200 283100, 405400 283150, 405600 283200)'),
    '{"cable_type": "Copper", "owner": "Virgin Media", "depth_m": 0.6, "length_m": 650}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'cable-routes'),
    'Fiber Backbone Scotland',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(325000 673000, 325500 674000, 326000 675000, 326500 676000)'),
    '{"cable_type": "Fiber Optic", "capacity_gbps": 400, "owner": "BT", "depth_m": 1.5, "length_m": 4800}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'cable-routes'),
    'Low Voltage Distribution',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(530100 180100, 530250 180200, 530400 180250)'),
    '{"cable_type": "Electrical", "voltage_kv": 0.4, "owner": "UK Power Networks", "depth_m": 0.5, "length_m": 420}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
), (
    (SELECT id FROM app.projects WHERE slug = 'test-project'),
    (SELECT id FROM app.collections WHERE slug = 'cable-routes'),
    'Data Center Interconnect',
    ST_GeomFromEWKT('SRID=27700;LINESTRING(405100 283100, 405300 283200, 405500 283250, 405800 283300)'),
    '{"cable_type": "Fiber Optic", "capacity_gbps": 1000, "owner": "Equinix", "depth_m": 2.0, "length_m": 780}',
    (SELECT id FROM app.users WHERE username = 'root'),
    (SELECT id FROM app.users WHERE username = 'root')
);
