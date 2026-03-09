CREATE TABLE app.figure_layers(
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    user_id INTEGER NOT NULL REFERENCES app.users(id),
    figure_id INTEGER NOT NULL REFERENCES app.figures(id),
    style_id INTEGER REFERENCES public.layer_styles(id),
    name TEXT NOT NULL,
    layer_order INTEGER NOT NULL,
    properties JSONB NOT NULL,
    source JSONB NOT NULL,
    added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    added_by INTEGER NOT NULL REFERENCES app.users(id),
      CONSTRAINT unique_layer_order_per_figure
      UNIQUE (figure_id, layer_order),
      CONSTRAINT unique_layer_name_per_figure UNIQUE (figure_id, name)

);

CREATE INDEX ON app.figure_layers(figure_id);