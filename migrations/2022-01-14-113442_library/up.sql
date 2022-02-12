CREATE TABLE customer (
	driver_license_id serial PRIMARY KEY,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	birth_date DATE NOT NULL,
	email TEXT NOT NULL,
	phone_number TEXT NOT NULL
);

CREATE TABLE car_model (
	car_model_id serial PRIMARY KEY,
	model_name TEXT NOT NULL,
	manufacturer_id integer NOT NULL,
	release_year integer NOT NULL
);

CREATE TABLE manufacturer (
	manufacturer_id serial PRIMARY KEY,
	name TEXT NOT NULL,
	country TEXT NOT NULL,
	website TEXT NOT NULL
);

CREATE TABLE car (
	plate_number TEXT PRIMARY KEY,
	car_model_id integer NOT NULL,
	available bool NOT NULL,
	condition TEXT NOT NULL,
	price_per_day FLOAT4 NOT NULL
);

CREATE TABLE rented_car (
	rented_car_id serial PRIMARY KEY,
	staff_id integer NOT NULL,
	plate_number TEXT NOT NULL,
	customer_id integer NOT NULL,
	rent_date DATE NOT NULL,
	return_date DATE NOT NULL,
	returned bool NOT NULL,
	comment TEXT NOT NULL
);

CREATE TABLE staff (
	staff_id serial PRIMARY KEY,
	first_name TEXT NOT NULL,
	last_name TEXT NOT NULL,
	email TEXT NOT NULL,
	password TEXT NOT NULL
);

ALTER TABLE car_model ADD CONSTRAINT car_model_fk0 FOREIGN KEY (manufacturer_id) REFERENCES manufacturer(manufacturer_id);
ALTER TABLE car ADD CONSTRAINT car_fk0 FOREIGN KEY (car_model_id) REFERENCES car_model(car_model_id);
ALTER TABLE rented_car ADD CONSTRAINT rented_car_fk0 FOREIGN KEY (staff_id) REFERENCES staff(staff_id);
ALTER TABLE rented_car ADD CONSTRAINT rented_car_fk1 FOREIGN KEY (plate_number) REFERENCES car(plate_number);
ALTER TABLE rented_car ADD CONSTRAINT rented_car_fk2 FOREIGN KEY (customer_id) REFERENCES customer(driver_license_id);




