CREATE DATABASE IF NOT EXISTS graphql;
USE graphql;


DROP TABLE IF EXISTS `user`;
SET character_set_client = utf8mb4 ;
CREATE TABLE `user` (
  `id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `email` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `email` (`email`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


DROP TABLE IF EXISTS `product`;
SET character_set_client = utf8mb4 ;
CREATE TABLE `product` (
  `id` varchar(255) NOT NULL,
  `user_id` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `price` decimal(10,0) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `product_fk0` (`user_id`),
  CONSTRAINT `product_fk0` FOREIGN KEY (`user_id`) REFERENCES `user` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


INSERT INTO user (id, name, email)
VALUES
  (1, 'adam', 'a@mail.com'),
  (2, 'bob', 'b@mail.com'),
  (3, 'craig', 'c@mail.com'),
  (4, 'dan', 'd@mail.com'),
  (5, 'ernie', 'e@mail.com');

INSERT INTO product (id, user_id, name, price)
VALUES
  (1, 1, 'book', 10),
  (2, 1, 'table', 20),
  (3, 1, 'hammer', 5),
  (4, 2, 'book', 10),
  (5, 2, 'hammer', 5),
  (6, 3, 'book', 10),
  (7, 3, 'house', 1000),
  (8, 3, 'pool', 50),
  (9, 3, 'window', 30),
  (10, 3, 'soap', 2),
  (11, 4, 'paper', 1),
  (12, 4, 'scissors', 3);
