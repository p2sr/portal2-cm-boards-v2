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

/*scores acts as a container for joins for the changelog entities.*/

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