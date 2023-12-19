DROP TABLE IF EXISTS todos;
DROP TABLE IF EXISTS orders;

CREATE TABLE orders (
   id INT PRIMARY KEY,
   region_id INT,
   gift_name VARCHAR(50),
   quantity INT
);

CREATE TABLE todos (
  id serial PRIMARY KEY,
  note TEXT NOT NULL
);
