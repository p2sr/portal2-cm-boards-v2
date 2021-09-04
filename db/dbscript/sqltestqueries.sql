SELECT coopbundled.time_gained, coopbundled.profile_number1, coopbundled.profile_number2, coopbundled.score FROM coopbundled INNER JOIN usersnew ON coopbundled.profile_number1=usersnew.profile_number WHERE coopbundled.banned=0 AND usersnew.banned=0 AND map_id=47856 ORDER BY coopbundled.score ASC;

SELECT coopbundled.time_gained, coopbundled.score, usersnew.boardname, u2.boardname (CASE WHEN coopbundled.profile_number2 = "" THEN "NONE" ELSE u2.boardname END) FROM coopbundled JOIN usersnew ON coopbundled.profile_number1=usersnew.profile_number JOIN usersnew AS u2 ON coopbundled.profile_numer2=u2.profile_number WHERE coopbundled.banned=0 AND usersnew.banned=0 AND u2.banned=0 AND map_id=47856 ORDER BY coopbundled.score ASC;
/*Only prints when both the names exists, adapt to handle having no boardname default to steamname*/
SELECT  coopbundled.time_gained
       ,maps.name
       ,coopbundled.score
       ,usersnew.steamname
       ,u2.steamname
FROM coopbundled
JOIN maps
ON coopbundled.map_id=maps.steam_id
JOIN usersnew
ON coopbundled.profile_number1=usersnew.profile_number
JOIN usersnew u2
ON coopbundled.profile_number2=u2.profile_number 
WHERE coopbundled.banned=0 
AND usersnew.banned=0 
AND u2.banned=0
AND map_id=52642
ORDER BY coopbundled.score ASC; 

SELECT cl.profile_number, cl.score
FROM changelog cl
	INNER JOIN (SELECT profile_number, MIN(score) as m_score
    	FROM changelog
        GROUP BY profile_number) s
     ON cl.profile_number=s.profile_number
        AND cl.score=s.m_score
WHERE
  cl.map_id = '1'
GROUP BY
  cl.profile_number, cl.score
ORDER BY 
  cl.score;


/*WIP*/
SELECT cl.profile_number, cl.map_id, cl.id, cl.score
FROM changelog cl
	INNER JOIN (SELECT profile_number, map_id, id, MIN(score) as m_score
    	FROM changelog
        GROUP BY profile_number, map_id, id) s
     ON cl.profile_number=s.profile_number
       AND cl.map_id=s.map_id
       AND cl.id=s.id
        AND cl.score=s.m_score
WHERE
  cl.map_id = '1'
GROUP BY
  cl.profile_number, cl.score, cl.id, cl.map_id
ORDER BY 
  cl.score;


SELECT  changelog.time_gained
       ,maps.name
       ,changelog.score
       ,usersnew.steamname
FROM changelog
JOIN maps
ON changelog.map_id=maps.steam_id
JOIN usersnew
ON changelog.profile_number=usersnew.profile_number
WHERE changelog.banned=0 
AND usersnew.banned=0 
AND map_id=47458
GROUP BY changelog.profile_number
ORDER BY changelog.score ASC; 


INSERT INTO usersnew


/*Changelog*/
FOREIGN KEY (`profile_number`) REFERENCES usersnew(`profile_number`),
FOREIGN KEY (`map_id`) REFERENCES maps(`steam_id`),
FOREIGN KEY (`coopid`) REFERENCES coopbundled(`id`);

/*Coopbundled*/
FOREIGN KEY (`changelogid1`) REFERENCES changelog(`id`),
FOREIGN KEY (`changelogid2`) REFERENCES changelog(`id`),
FOREIGN KEY (`profile_number1`) REFERENCES usersnew(`profile_number`),
FOREIGN KEY (`profile_number2`) REFERENCES usersnew(`profile_number`),
FOREIGN KEY (`map_id`) REFERENCES maps(`steam_id`),
FOREIGN KEY (`previous_id1`) REFERENCES changelog(`previous_id`),
FOREIGN KEY (`previous_id2`) REFERENCES changelog(`previous_id`);

/*Maps*/
FOREIGN KEY (`chapter_id`) REFERENCES chapters(`id`);   

/*scores*/
FOREIGN KEY (`map_id`) REFERENCES maps(`steam_id`),
FOREIGN KEY (`profile_number`) REFERENCES usersnew(`profile_number`),
FOREIGN KEY (`changelog_id`) REFERENCES changelog(`id`);

/*scores acts as a container for joins for the changelog entities.


ALTER TABLE scores ADD CONSTRAINT `scores_ibfk_3` FOREIGN KEY (`changelog_id`) REFERENCES `changelog` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

*/

/*
mysql> use p2boards;
Database changed
mysql> ALTER TABLE changelog
    -> ADD CONSTRAINT `changelog_ibfk_1` FOREIGN KEY (`profile_number`) REFERENCES `usersnew` (`profile_number`) ON DELETE CASCADE ON UPDATE CASCADE;
ERROR 1452 (23000): Cannot add or update a child row: a foreign key constraint fails (`p2boards`.`#sql-107_19`, CONSTRAINT `changelog_ibfk_1` FOREIGN KEY (`profile_number`) REFERENCES `usersnew` (`profile_number`) ON DELETE CASCADE ON UPDATE CASCADE)
mysql> INSERT INTO `usersnew` VALUES ('', NULL, NULL, 0, 0, NULL, NULL, NULL, NULL, 0, NULL);
Query OK, 1 row affected (0.22 sec)

mysql> ALTER TABLE changelog ADD CONSTRAINT `changelog_ibfk_1` FOREIGN KEY (`profile_number`) REFERENCES `usersnew` (`profile_number`) ON DELETE CASCADE ON UPDATE CASCADE;
ERROR 1452 (23000): Cannot add or update a child row: a foreign key constraint fails (`p2boards`.`#sql-107_19`, CONSTRAINT `changelog_ibfk_1` FOREIGN KEY (`profile_number`) REFERENCES `usersnew` (`profile_number`) ON DELETE CASCADE ON UPDATE CASCADE)
mysql> SELECT profile_number FROM changelog WHERE profile_number NOT IN (SELECT profile_number FROM usersnew);
+-------------------+
| profile_number    |
+-------------------+
| 76561197972048348 |
+-------------------+
1 row in set (0.53 sec)

mysql> INSERT INTO `usersnew` VALUES ('76561197972048348', NULL, NULL, 0, 0, NULL, NULL, NULL, NULL, 0, NULL);
Query OK, 1 row affected (0.23 sec)

mysql> ALTER TABLE changelog ADD CONSTRAINT `changelog_ibfk_1` FOREIGN KEY (`profile_number`) REFERENCES `usersnew` (`profile_number`) ON DELETE CASCADE ON UPDATE CASCADE;
Query OK, 96793 rows affected (46.48 sec)
Records: 96793  Duplicates: 0  Warnings: 0

mysql> ALTER TABLE changelog ADD CONSTRAINT `changelog_ibfk_2` FOREIGN KEY (`map_id`) REFERENCES `maps` (`steam_id`) ON DELETE CASCADE ON UPDATE CASCADE;
mysql> ALTER TABLE changelog ADD CONSTRAINT `changelog_ibfk_3` FOREIGN KEY (`coopid`) REFERENCES `coopbundled` (`id`) ON DELETE CASCADE ON UPDATE CASCADE;

mysql>*/

SELECT t.timestamp, t.score, t.steam_name, t.board_name FROM (
  SELECT DISTINCT ON (changelog.profile_number) *
  FROM "p2boards".changelog
  INNER JOIN "p2boards".users ON (users.profile_number = changelog.profile_number)
  WHERE map_id = '47763'
  AND users.banned = False
  AND changelog.banned = False
  ORDER BY changelog.profile_number, changelog.score ASC
) t
ORDER BY score;

SELECT t.timestamp,
  t.profile_number, /*Somehow make this non-ambiguous???*/
  t.score,
  t.demo_id,
  t.youtube_id,
  t.submission,
  t.note,
  t.category_id,
  CASE
    WHEN t.board_name IS NULL
      THEN t.steam_name
    WHEN t.board_name IS NOT NULL
      THEN t.board_name
  END user_name,
  t.avatar
 FROM (
  SELECT DISTINCT ON (changelog.profile_number) *
  FROM "p2boards".changelog
  INNER JOIN "p2boards".users ON (users.profile_number = changelog.profile_number)
  WHERE map_id = '47763'
  AND users.banned = False
  AND changelog.verified = True
  AND changelog.banned = False
  ORDER BY changelog.profile_number, changelog.score ASC
) t
ORDER BY score;

/*We need to make one filler entry per-coop map, and one users with a garbage profile for placeholder.*/
SELECT  c1.timestamp, 
        c1.score, 
        c1.note, 
        c2.note,
        CASE 
          WHEN p1.board_name IS NULL
            THEN p1.steam_name
          WHEN p1.board_name IS NOT NULL
            THEN p1.board_name
        END p1_username, 
        CASE 
          WHEN p2.board_name IS NULL
            THEN p2.steam_name
          WHEN p2.board_name IS NOT NULL
            THEN p2.board_name
        END p2_username ,
        c1.profile_number,
        c2.profile_number,
        c1.demo_id,
        c2.demo_id,
        c1.youtube_id,
        c2.youtube_id,
        c1.submission,
        c2.submission,
        c1.category_id,
        c2.category_id,
        p1.avatar,
        p2.avatar
FROM (SELECT * FROM 
  "p2boards".coop_bundled 
  WHERE id IN 
    (SELECT coop_id
    FROM "p2boards".changelog
    WHERE map_id='47828'
    AND coop_id IS NOT NULL)) as cb 
  INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
  INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
  INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
  INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
  WHERE p1.banned=False
  AND p2.banned=False
  AND c1.banned=False
  AND c2.banned=False
  AND c1.verified=True
  AND c2.verified=True
  ORDER BY score ASC;

SELECT c1.timestamp, c1.profile_number, c2.profile_number, c1.score, c1.demo_id, c2.demo_id, c1.youtube_id, c2.youtube_id, c1.submission, c2.submission, c1.note, c2.note, c1.category_id, c2.category_id, p1.board_name, p2.board_name, p1.avatar, p2.avatar
FROM (SELECT * FROM 
  "p2boards".coop_bundled 
  WHERE id IN 
    (SELECT coop_id
    FROM "p2boards".changelog
    WHERE map_id='47828'
    AND coop_id IS NOT NULL)) as cb 
  INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
  INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2) 
  INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
  INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
  WHERE p1.banned=False
  AND p2.banned=False
  AND c1.banned=False
  AND c2.banned=False
  AND c1.verified=True
  AND c2.verified=True
  ORDER BY score ASC;


SELECT  c1.timestamp, 
      c1.score, 
      c1.note, 
      c2.note,
      CASE 
        WHEN p1.board_name IS NULL
          THEN p1.steam_name
        WHEN p1.board_name IS NOT NULL
          THEN p1.board_name
      END p1_username, 
      CASE 
        WHEN p2.board_name IS NULL
          THEN p2.steam_name
        WHEN p2.board_name IS NOT NULL
          THEN p2.board_name
      END p2_username
FROM (SELECT * FROM 
"p2boards".coop_bundled 
WHERE id IN 
  (SELECT coop_id
  FROM "p2boards".changelog
  WHERE map_id='52777'
  AND coop_id IS NOT NULL)) as cb 
INNER JOIN "p2boards".changelog AS c1 ON (c1.id = cb.cl_id1)
INNER JOIN "p2boards".changelog AS c2 ON (c2.id = cb.cl_id2)
INNER JOIN "p2boards".users AS p1 ON (p1.profile_number = cb.p_id1)
INNER JOIN "p2boards".users AS p2 ON (p2.profile_number = cb.p_id2)
WHERE p1.banned=False
AND p2.banned=False
AND c1.banned=False
AND c2.banned=False
AND c1.verified=True
AND c2.verified=True
ORDER BY score ASC;