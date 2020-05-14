-- MySQL Workbench Forward Engineering

SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

-- -----------------------------------------------------
-- Schema labs
-- -----------------------------------------------------

-- -----------------------------------------------------
-- Schema labs
-- -----------------------------------------------------
CREATE SCHEMA IF NOT EXISTS `labs` DEFAULT CHARACTER SET utf8 ;
USE `labs` ;

-- -----------------------------------------------------
-- Table `labs`.`amostras`
-- -----------------------------------------------------
CREATE TABLE IF NOT EXISTS `labs`.`amostras` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `nome` VARCHAR(40) NULL,
  `matriz` VARCHAR(40) NULL,
  `dopante` VARCHAR(40) NULL,
  `autor` VARCHAR(160) NULL,
  `local` VARCHAR(160) NULL,
  PRIMARY KEY (`id`),
  UNIQUE INDEX `id_UNIQUE` (`id` ASC) VISIBLE)
ENGINE = InnoDB;


SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;
