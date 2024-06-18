CREATE TABLE `track_genres`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`track_id` INTEGER NOT NULL,
	`genre_id` INTEGER NOT NULL,
	FOREIGN KEY (`track_id`) REFERENCES `tracks`(`id`),
	FOREIGN KEY (`genre_id`) REFERENCES `genres`(`id`)
);

CREATE TABLE `record_labels`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);

CREATE TABLE `album_artists`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`album_id` INTEGER NOT NULL,
	`artist_id` INTEGER NOT NULL,
	FOREIGN KEY (`album_id`) REFERENCES `albums`(`id`),
	FOREIGN KEY (`artist_id`) REFERENCES `artists`(`id`)
);

CREATE TABLE `genres`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL
);

CREATE TABLE `tracks`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`album_id` INTEGER,
	`path` TEXT,
	`filesize` INTEGER NOT NULL,
	`track_number` INTEGER,
	`disc_number` INTEGER,
	`disc_title` TEXT,
	`content_group` TEXT,
	`title` TEXT NOT NULL,
	`subtitle` TEXT,
	`year` INTEGER,
	`release_date` TEXT,
	`bpm` TEXT,
	`length` INTEGER,
	`initial_key` TEXT,
	`language` TEXT,
	`label_id` INTEGER,
	`original_title` TEXT,
	`added_at` TEXT,
	`art_id` INTEGER,
	`fallback_artist_id` INTEGER,
	FOREIGN KEY (`album_id`) REFERENCES `albums`(`id`),
	FOREIGN KEY (`label_id`) REFERENCES `record_labels`(`id`),
	FOREIGN KEY (`art_id`) REFERENCES `art`(`id`),
	FOREIGN KEY (`fallback_artist_id`) REFERENCES `artists`(`id`)
);

CREATE TABLE `track_artists`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`track_id` INTEGER,
	`artist_id` INTEGER,
	FOREIGN KEY (`track_id`) REFERENCES `tracks`(`id`),
	FOREIGN KEY (`artist_id`) REFERENCES `artists`(`id`)
);

CREATE TABLE `albums`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`year` INTEGER,
	`title` TEXT NOT NULL,
	`art_id` INTEGER,
	FOREIGN KEY (`art_id`) REFERENCES `art`(`id`)
);

CREATE TABLE `artists`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL,
	`art_id` INTEGER,
	FOREIGN KEY (`art_id`) REFERENCES `art`(`id`)
);

CREATE TABLE `art`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`hash` DOUBLE NOT NULL,
	`data` BINARY NOT NULL
);
