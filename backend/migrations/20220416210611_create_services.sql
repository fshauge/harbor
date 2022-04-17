CREATE TABLE services (
    id serial PRIMARY KEY,
    application_id integer NOT NULL REFERENCES applications(id),
    name text NOT NULL,
    image text NOT NULL,
    build_context text NOT NULL,
    container_id text,
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now()
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON services
FOR EACH ROW EXECUTE PROCEDURE set_updated_at();
