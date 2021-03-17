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

,
  FOREIGN KEY (`profile_number`) REFERENCES usersnew(`profile_number`),
  FOREIGN KEY (`map_id`) REFERENCES maps(`map_id`)