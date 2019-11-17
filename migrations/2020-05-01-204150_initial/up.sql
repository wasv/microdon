-- List of all actors encountered.
CREATE TABLE actors     (
  id       VARCHAR(2048) NOT NULL PRIMARY KEY, -- URL to actor.
  username TEXT          NOT NULL, -- Display name for actor.
  profile  VARCHAR(2048) NOT NULL -- URL to human readable profile.
);

-- List of actors I follow.
CREATE TABLE following  (
  actor    VARCHAR(2048) NOT NULL PRIMARY KEY REFERENCES actors(id),
  since    TIMESTAMP     NOT NULL -- Time and date I followed actor.
);

-- List of actors following me.
CREATE TABLE followers  (
  actor    VARCHAR(2048) NOT NULL PRIMARY KEY REFERENCES actors(id),
  since    TIMESTAMP     NOT NULL -- Time and date actor followed me
);

-- List of activities recieved.
CREATE TABLE outbox (
  id       VARCHAR(2048) NOT NULL PRIMARY KEY, -- URL to incoming activity.
  actor    VARCHAR(2048) NOT NULL REFERENCES actors(id), -- Actor that published activity.
  payload  JSON -- Content of entire activity.
);

-- List of activities published.
CREATE TABLE inbox (
  id       VARCHAR(2048) NOT NULL PRIMARY KEY, -- URL for published activity.
--  actor    VARCHAR(2048) NOT NULL REFERENCES actors(id), -- Not needed, always me.
  payload  JSON -- Content of entire activity.
);
