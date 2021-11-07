DROP DATABASE IF EXISTS `testing`;

CREATE DATABASE `testing` CHARSET utf8mb4 COLLATE utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `testing`.`algorithm` (
    `id` BIGINT NOT NULL,
    `name` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `display_name` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `location` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `image` BIGINT NOT NULL,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    UNIQUE KEY unique_name (`name`)  
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE utf8mb4_general_ci;


CREATE TABLE IF NOT EXISTS `testing`.`image` (
    `id` BIGINT NOT NULL,
    `name` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `image` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `testing`.`trainset` (
    `id` BIGINT NOT NULL,
    `name` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `location` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE utf8mb4_general_ci;

CREATE TABLE IF NOT EXISTS `testing`.`testset` (
    `id` BIGINT NOT NULL,
    `name` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `location` VARCHAR(255) CHARSET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE utf8mb4_general_ci;
