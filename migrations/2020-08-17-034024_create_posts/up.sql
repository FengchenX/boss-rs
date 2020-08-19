-- Your SQL goes here
CREATE TABLE posts (
  id INTEGER AUTO_INCREMENT PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  body TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE `job` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `creator` bigint(20) DEFAULT NULL,
  `create_at` bigint(20) DEFAULT NULL,
  `update_at` bigint(20) DEFAULT NULL,
  `delete_at` bigint(20) DEFAULT NULL,
  `progress` double DEFAULT NULL,
  `status` varchar(255) CHARACTER SET utf8 DEFAULT NULL,
  `file` varchar(255) CHARACTER SET utf8 DEFAULT NULL,
  PRIMARY KEY (`id`) USING BTREE,
  KEY `update_at` (`update_at`)
) ENGINE=InnoDB AUTO_INCREMENT=5738 DEFAULT CHARSET=latin1 ROW_FORMAT=DYNAMIC;