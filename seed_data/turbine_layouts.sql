WITH layouts AS (
    INSERT INTO app.turbine_layouts (project_id, name, is_primary, added_by, last_updated_by)
    VALUES (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        'Manchester Layout A',
        true,
        0, 0
    ), (
        (SELECT id FROM app.projects WHERE name = 'Test Project'),
        'Manchester Layout B',
        false,
        0, 0
    )
    RETURNING id, name
)
INSERT INTO app.turbines (layout_id, turbine_number, hub_height_mm, rotor_diameter_mm, geom)
VALUES
    -- Manchester Layout A (5 turbines, 100m hub, 130m rotor)
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout A'), 1, 100000, 130000, ST_GeomFromEWKT('SRID=27700;POINT(384100 397600)')),
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout A'), 2, 100000, 130000, ST_GeomFromEWKT('SRID=27700;POINT(384500 397600)')),
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout A'), 3, 100000, 130000, ST_GeomFromEWKT('SRID=27700;POINT(384900 397600)')),
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout A'), 4, 100000, 130000, ST_GeomFromEWKT('SRID=27700;POINT(384300 397950)')),
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout A'), 5, 100000, 130000, ST_GeomFromEWKT('SRID=27700;POINT(384700 397950)')),

    -- Manchester Layout B (4 turbines, 120m hub, 150m rotor)
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout B'), 1, 120000, 150000, ST_GeomFromEWKT('SRID=27700;POINT(384050 397550)')),
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout B'), 2, 120000, 150000, ST_GeomFromEWKT('SRID=27700;POINT(384550 397550)')),
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout B'), 3, 120000, 150000, ST_GeomFromEWKT('SRID=27700;POINT(385050 397550)')),
    ((SELECT id FROM layouts WHERE name = 'Manchester Layout B'), 4, 120000, 150000, ST_GeomFromEWKT('SRID=27700;POINT(384300 397900)'));
