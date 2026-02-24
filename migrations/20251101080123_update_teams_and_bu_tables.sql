ALTER TABLE app.business_units ADD COLUMN added_by integer NOT NULL REFERENCES app.users(id) DEFAULT 0;
ALTER TABLE app.business_units ADD COLUMN last_updated_by integer NOT NULL REFERENCES app.users(id) DEFAULT 0;

ALTER TABLE app.teams ADD COLUMN added_by integer NOT NULL REFERENCES app.users(id) DEFAULT 0;
ALTER TABLE app.teams ADD COLUMN last_updated_by integer NOT NULL REFERENCES app.users(id) DEFAULT 0;

ALTER TABLE app.business_units ALTER COLUMN added_by DROP DEFAULT;
ALTER TABLE app.business_units ALTER COLUMN last_updated_by DROP DEFAULT; 

ALTER TABLE app.teams ALTER COLUMN added_by DROP DEFAULT;
ALTER TABLE app.teams ALTER COLUMN last_updated_by DROP DEFAULT;