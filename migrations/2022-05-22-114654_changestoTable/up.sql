-- Your SQL goes here
DROP TABLE car, country, car_model, customer, manufacturer, rented_car, staff;

CREATE TABLE country (
    country_id varchar(2) PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE customer (
	customer_id serial PRIMARY KEY,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	email TEXT NOT NULL,
	phone_number TEXT NOT NULL
);

CREATE TABLE manufacturer (
	manufacturer_id serial PRIMARY KEY,
	name TEXT NOT NULL,
	country_id VARCHAR(2) NOT NULL,
	website TEXT NOT NULL
);

CREATE TABLE tableware (
	tableware_id serial PRIMARY KEY,
	manufacturer_id integer NOT NULL,
	name TEXT NOT NULL,
	"type" TEXT NOT NULL,
	main_material TEXT NOT NULL,
	main_colour TEXT NOT NULL
);

CREATE TABLE sold_tableware (
	sold_tableware_id serial PRIMARY KEY,
	customer_id integer NOT NULL,
	tableware_id integer NOT NULL,
	staff_id integer NOT NULL,
	date DATE NOT NULL,
	amount integer NOT NULL
);

CREATE TABLE staff (
	staff_id serial PRIMARY KEY,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	email TEXT NOT NULL,
	phone_number TEXT NOT NULL
);

ALTER TABLE manufacturer ADD CONSTRAINT manufacturer_fk0 FOREIGN KEY (country_id) REFERENCES country(country_id);
ALTER TABLE tableware ADD CONSTRAINT tableware_fk0 FOREIGN KEY (manufacturer_id) REFERENCES manufacturer(manufacturer_id);
ALTER TABLE sold_tableware ADD CONSTRAINT sold_tableware_fk0 FOREIGN KEY (customer_id) REFERENCES customer(customer_id);
ALTER TABLE sold_tableware ADD CONSTRAINT sold_tableware_fk1 FOREIGN KEY (tableware_id) REFERENCES tableware(tableware_id);
ALTER TABLE sold_tableware ADD CONSTRAINT sold_tableware_fk2 FOREIGN KEY (staff_id) REFERENCES staff(staff_id);
