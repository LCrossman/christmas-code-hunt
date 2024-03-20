DROP TABLE IF EXISTS todos;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS regions;

CREATE TABLE regions (
   id INT PRIMARY KEY,
   name VARCHAR(50)
);

CREATE TABLE orders (
   id INT PRIMARY KEY,
   region_id INT,
   gift_name VARCHAR(50),
   quantity INT,
   FOREIGN KEY (region_id) REFERENCES regions(id)
);

CREATE TABLE todos (
  id serial PRIMARY KEY,
  note TEXT NOT NULL
);
