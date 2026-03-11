-- Add migration script here
CREATE TABLE public.layer_styles (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    f_table_catalog character varying COLLATE pg_catalog."default",
    f_table_schema character varying COLLATE pg_catalog."default",
    f_table_name character varying COLLATE pg_catalog."default",
    f_geometry_column character varying COLLATE pg_catalog."default",
    stylename text NOT NULL UNIQUE COLLATE pg_catalog."default",
    styleqml xml,
    stylesld xml,
    useasdefault boolean,
    description text COLLATE pg_catalog."default",
    owner character varying(63) NOT NULL COLLATE pg_catalog."default" DEFAULT CURRENT_USER,
    ui xml,
    update_time timestamp without time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    type character varying COLLATE pg_catalog."default"
);
