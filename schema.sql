CREATE TABLE temporal_metadata (transaction_time TSTZRANGE NOT NULL);

INSERT INTO
    temporal_metadata (transaction_time)
VALUES
    ('[2010-01-01 14:30, 2010-01-01 15:30]');
