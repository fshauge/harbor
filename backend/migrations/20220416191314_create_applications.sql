CREATE TABLE applications (
    id serial PRIMARY KEY,
    name text NOT NULL,
    repository text NOT NULL,
    branch text NOT NULL,
    created_at timestamp NOT NULL DEFAULT now(),
    updated_at timestamp NOT NULL DEFAULT now()
);

CREATE TRIGGER set_updated_at BEFORE UPDATE ON applications
FOR EACH ROW EXECUTE PROCEDURE set_updated_at();
