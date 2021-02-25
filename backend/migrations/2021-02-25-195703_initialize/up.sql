-- Your SQL goes here

CREATE TABLE `changelog` (
  `id` int NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `time_gained` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  `profile_number` varchar(50) NOT NULL DEFAULT '',
  `score` int NOT NULL,
  `map_id` varchar(6) NOT NULL DEFAULT '',
  `wr_gain` int NOT NULL DEFAULT '0',
  `has_demo` int DEFAULT '0',
  `banned` int NOT NULL DEFAULT '0',
  `youtube_id` varchar(30) DEFAULT NULL,
  `previous_id` int DEFAULT NULL,
  `coopid` int DEFAULT NULL,
  `post_rank` int DEFAULT NULL,
  `pre_rank` int DEFAULT NULL,
  `submission` int NOT NULL DEFAULT '0',
  `note` varchar(100) DEFAULT NULL,
  `category` varchar(100) DEFAULT 'any%'
);