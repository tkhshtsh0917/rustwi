CREATE TABLE tweets
(
    id        serial primary key,
    message   text        not null,
    posted_at timestamptz not null
);
INSERT INTO tweets VALUES (1, '始めてのツイート', '2023-01-02 03:04:05');
INSERT INTO tweets VALUES (2, '久しぶりのツイート', '2023-06-07 08:09:10');
