-- List of all actors encountered.
--   For future expansion.
CREATE TABLE actors      (
  id       VARCHAR(2048) NOT NULL PRIMARY KEY -- URL of actor.
);

-- List of follow relationships.
CREATE TABLE followings  (
  -- Actor being followed.
  target    VARCHAR(2048) NOT NULL PRIMARY KEY REFERENCES actors(id),
  -- Actor following target.
  follower  VARCHAR(2048) NOT NULL REFERENCES actors(id), 
  -- Time and date relationship encountered.
  since     TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- List of objects seen.
CREATE TABLE objects     (
  id        VARCHAR(2048) NOT NULL PRIMARY KEY, -- URL for published activity.
  objtype   VARCHAR(8)    NOT NULL, -- Activity Type (Create, Update, Delete, Follow, ...)
  author    VARCHAR(2048) NOT NULL REFERENCES actors(id), -- Actor that published object.
  published TIMESTAMP, -- Time that object was created (optional)
  contents  JSON -- Content of entire object.
);

-- List of activities seen.
CREATE TABLE activities  (
  id        VARCHAR(2048) NOT NULL PRIMARY KEY, -- URL of activity.
  acttype   VARCHAR(8)    NOT NULL, -- Activity Type (Create, Update, Delete, Follow, ...)
  author    VARCHAR(2048) NOT NULL REFERENCES actors(id), -- Not needed, always me.
  published TIMESTAMP, -- Time that activity was created (optional)
  object    VARCHAR(2048) NOT NULL REFERENCES objects(id), -- Actor that published activity.
  contents  JSON -- Content of entire activity.
);
