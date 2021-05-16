CREATE TABLE vendors (
  id INT GENERATED ALWAYS AS IDENTITY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  phone VARCHAR NOT NULL,
  PRIMARY KEY(id)
);
CREATE TABLE users (
  id INT GENERATED ALWAYS AS IDENTITY,
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  ucid VARCHAR NOT NULL,
  PRIMARY KEY(id)
);
CREATE TABLE points (
  id INT GENERATED ALWAYS AS IDENTITY,
  user_id INT NOT NULL,
  vendor_id INT NOT NULL,
  amount INT,
  PRIMARY KEY(id),
  CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id),
  CONSTRAINT fk_vendor_id FOREIGN KEY(vendor_id) REFERENCES vendors(id)
);
-- insert some data
INSERT INTO vendors(name, email, password, phone)
VALUES (
    'PHO',
    'pho@email.com',
    '6b3a55e0261b0304143f805a24924d0c1c44524821305f31d9277843b8a10f4e',
    '5875875858'
  );
INSERT INTO vendors(name, email, password, phone)
VALUES (
    'Ramen',
    'ramen@email.com',
    '6b3a55e0261b0304143f805a24924d0c1c44524821305f31d9277843b8a10f4e',
    '5875875858'
  );
INSERT INTO users(name, email, password, ucid)
VALUES (
    'Brenda Galoc',
    'brenda@email.com',
    '6b3a55e0261b0304143f805a24924d0c1c44524821305f31d9277843b8a10f4e',
    '30018900'
  );
INSERT INTO users(name, email, password, ucid)
VALUES (
    'Artem Golovin',
    'artem@email.com',
    '6b3a55e0261b0304143f805a24924d0c1c44524821305f31d9277843b8a10f4e',
    '30018900'
  );
INSERT INTO points(user_id, vendor_id, amount)
VALUES (1, 1, 0);
INSERT INTO points(user_id, vendor_id, amount)
VALUES (1, 2, 30);