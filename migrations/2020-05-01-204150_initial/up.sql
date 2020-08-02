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
  since     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- List of objects seen.
CREATE TABLE objects     (
  id        VARCHAR(2048) NOT NULL PRIMARY KEY, -- URL for published object.
  objtype   VARCHAR(8)    NOT NULL, -- Object Type (Note, Photo, Announce, ...)
  author    VARCHAR(2048) NOT NULL REFERENCES actors(id), -- Actor that published object.
  published TIMESTAMP     NOT NULL, -- Time that object was created
  contents  JSON NOT NULL -- Content of entire object.
);

-- List of activities seen.
CREATE TABLE activities  (
  id        VARCHAR(2048) NOT NULL PRIMARY KEY, -- URL of activity.
  acttype   VARCHAR(8)    NOT NULL, -- Activity Type (Create, Update, Delete, Follow, ...)
  author    VARCHAR(2048) NOT NULL REFERENCES actors(id), -- Not needed, always me.
  published TIMESTAMP     NOT NULL, -- Time that activity was created
  object    VARCHAR(2048) NOT NULL REFERENCES objects(id), -- Actor that published activity.
  contents  JSON NOT NULL -- Content of entire activity.
);
