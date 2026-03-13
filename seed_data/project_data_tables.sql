CREATE TABLE project_data.p0001_test_polygon (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    geom GEOMETRY(POLYGON, 27700)
);

INSERT INTO project_data.p0001_test_polygon (name, geom) VALUES (
    'Test Polygon',
    ST_GeomFromText('POLYGON((319288 661147, 319868 672454, 308984 669072, 319288 661147))', 27700)
);

CREATE TABLE project_data.p0001_non_spatial_table (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT
);

CREATE TABLE project_data.q0001_incorrect_prefix_letter (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    geom GEOMETRY(POLYGON, 27700)
);

CREATE TABLE project_data.pq0001_incorrect_prefix_letters (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    geom GEOMETRY(POLYGON, 27700)
);

CREATE TABLE project_data.p1_different_format (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    geom GEOMETRY(POLYGON, 27700)
);

CREATE TABLE project_data.p_no_number (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    geom GEOMETRY(POLYGON, 27700)
);

CREATE TABLE project_data.pletters_no_number (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    geom GEOMETRY(POLYGON, 27700)
);

CREATE TABLE project_data.p0001_spatial_table_with_no_epsg (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    geom GEOMETRY(POLYGON)
);

  -- Table name with multiple underscores
  CREATE TABLE project_data.p0001_layer_with_multiple_underscores (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  );

    -- Table name with no layer name part (just prefix)
  CREATE TABLE project_data.p0001_ (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  );

  -- Mixed case table name
  CREATE TABLE project_data."P0001_uppercase_prefix" (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  );

    -- Geometry column named something other than 'geom'
  CREATE TABLE project_data.p0001_different_geom_name (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geometry GEOMETRY(POLYGON, 27700)
  );

    -- Multiple geometry columns (uncommon but possible)
  CREATE TABLE project_data.p0001_multi_geom (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700),
     geom2 GEOMETRY(POINT, 27700)
  );


  CREATE TABLE project_data.p0001number_no_underscore (
      id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  );

  CREATE TABLE project_data."p0001_table-with-hyphens" (
          id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  );

    CREATE TABLE project_data."p0001_TABLE_WITH_CAPS" (
          id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  );

     CREATE TABLE project_data."p0001_table with spaces" (
          id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  ); 

       CREATE TABLE project_data."p0001 table_with_space_after_prefix" (
          id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  ); 
         CREATE TABLE project_data."p0024 3 test polygons" (
          id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
      name TEXT,
      geom GEOMETRY(POLYGON, 27700)
  ); 

