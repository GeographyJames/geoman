CREATE TABLE app.supported_crs (
    srid integer PRIMARY KEY REFERENCES spatial_ref_sys(srid)
)