CREATE TABLE usuarios (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE clientes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    direccion TEXT NOT NULL,
    cuit_dni TEXT NOT NULL,
    telefono TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE empresa (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    nombre TEXT NOT NULL DEFAULT '',
    direccion TEXT NOT NULL DEFAULT '',
    cuit TEXT NOT NULL DEFAULT '',
    telefono TEXT NOT NULL DEFAULT '',
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO empresa (id) VALUES (1);

CREATE TABLE productos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    nombre TEXT NOT NULL,
    precio_centavos INTEGER NOT NULL,
    stock INTEGER,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE remitos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cliente_id INTEGER NOT NULL REFERENCES clientes(id),
    fecha TEXT NOT NULL DEFAULT (datetime('now')),
    total_centavos INTEGER NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE remito_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    remito_id INTEGER NOT NULL REFERENCES remitos(id) ON DELETE CASCADE,
    producto_id INTEGER REFERENCES productos(id) ON DELETE SET NULL,
    nombre_producto TEXT NOT NULL,
    precio_unitario_centavos INTEGER NOT NULL,
    cantidad INTEGER NOT NULL,
    subtotal_centavos INTEGER NOT NULL
);

CREATE INDEX idx_remitos_cliente_id ON remitos(cliente_id);
CREATE INDEX idx_remito_items_remito_id ON remito_items(remito_id);
