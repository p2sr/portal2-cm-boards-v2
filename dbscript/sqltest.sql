SELECT coopbundled.time_gained, coopbundled.profile_number1, coopbundled.profile_number2, coopbundled.score FROM coopbundled INNER JOIN usersnew ON coopbundled.profile_number1=usersnew.profile_number WHERE coopbundled.banned=0 AND usersnew.banned=0 AND map_id=47856 ORDER BY coopbundled.score ASC;


SELECT coopbundled.time_gained, coopbundled.score, usersnew.boardname, u2.boardname (CASE WHEN coopbundled.profile_number2 = "" THEN "NONE" ELSE u2.boardname END) FROM coopbundled JOIN usersnew ON coopbundled.profile_number1=usersnew.profile_number JOIN usersnew AS u2 ON coopbundled.profile_numer2=u2.profile_number WHERE coopbundled.banned=0 AND usersnew.banned=0 AND u2.banned=0 AND map_id=47856 ORDER BY coopbundled.score ASC;

INSERT INTO usersnew