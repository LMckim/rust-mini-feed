CREATE TABLE TEST_FH(
    `ID` INT NOT NULL AUTO_INCREMENT,
    `SEQ` BIGINT,
    `DATE` DATE,
    `TIME` TIME,
    `SYMBOL` VARCHAR(16),
    `VOLUME` BIGINT,
    `PRICE` DECIMAL(12,2),
    `VENUE` CHAR(3),
    PRIMARY KEY(`ID`)
)ENGINE=InnoDB CHARSET=utf8mb4