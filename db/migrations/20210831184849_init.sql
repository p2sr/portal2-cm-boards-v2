-- migrate:up

CREATE SCHEMA IF NOT EXISTS "p2boards";

CREATE  TABLE "p2boards".categories ( 
	id                   serial  NOT NULL ,
	name                 varchar(100) DEFAULT '' NOT NULL ,
	map_id               varchar(6) DEFAULT '' NOT NULL ,
	rules                varchar(1000) DEFAULT '' NOT NULL ,
	CONSTRAINT pk_categories_id PRIMARY KEY ( id )
 );

CREATE  TABLE "p2boards".games ( 
	id                   serial  NOT NULL ,
	game_name            varchar(50) DEFAULT 'Portal 2' NOT NULL ,
	CONSTRAINT pk_game_id PRIMARY KEY ( id )
 );

CREATE  TABLE "p2boards".users ( 
	profile_number       varchar(50) DEFAULT '' NOT NULL ,
	board_name           varchar(50)   ,
	steam_name           varchar(50)   ,
	banned               boolean DEFAULT false NOT NULL ,
	registered           integer DEFAULT 0 NOT NULL ,
	avatar               varchar(200)   ,
	twitch               varchar(100)   ,
	youtube              varchar(100)   ,
	title                varchar(200)   ,
	"admin"              integer DEFAULT 0 NOT NULL ,
	donation_amount      varchar(11)   ,
	discord_id           varchar(40)   ,
	CONSTRAINT pk_users_profile_number PRIMARY KEY ( profile_number )
 );

CREATE  TABLE "p2boards".chapters ( 
	id                   serial  NOT NULL ,
	chapter_name         varchar(50)   ,
	is_multiplayer       boolean DEFAULT false NOT NULL ,
	game_id              integer DEFAULT 1 NOT NULL ,
	CONSTRAINT pk_chapters_id PRIMARY KEY ( id ),
	CONSTRAINT fk_chapters_game_id FOREIGN KEY ( game_id ) REFERENCES "p2boards".games( id )   
 );

CREATE  TABLE "p2boards".maps ( 
	id                   serial  NOT NULL ,
	steam_id             varchar(6) DEFAULT '' NOT NULL ,
	lp_id                varchar(6) DEFAULT '' NOT NULL ,
	name                 varchar(50)  NOT NULL ,
	chapter_id           integer   ,
	is_public            boolean DEFAULT false NOT NULL ,
	CONSTRAINT pk_maps_id PRIMARY KEY ( id ),
	CONSTRAINT unq_maps_steam_id UNIQUE ( steam_id ) ,
	CONSTRAINT fk_maps_chapters FOREIGN KEY ( chapter_id ) REFERENCES "p2boards".chapters( id )   
 );

CREATE  TABLE "p2boards".changelog ( 
	id                   bigserial  NOT NULL ,
	"timestamp"          timestamp(6)   ,
	profile_number       varchar(50)  NOT NULL ,
	score                integer  NOT NULL ,
	map_id               varchar(6) DEFAULT '' NOT NULL ,
	demo_id              bigint   ,
	banned               boolean DEFAULT false NOT NULL ,
	youtube_id           varchar(30)   ,
	previous_id          integer   ,
	coop_id              bigint   ,
	post_rank            integer   ,
	pre_rank             integer   ,
	submission           boolean DEFAULT false NOT NULL ,
	note                 varchar(100)   ,
	category_id          integer DEFAULT 1 NOT NULL ,
	score_delta          integer   ,
	verified             boolean   ,
	admin_note           varchar(200)   ,
	CONSTRAINT pk_changelog_id PRIMARY KEY ( id )
 );

CREATE  TABLE "p2boards".coop_bundled ( 
	id                   bigserial  NOT NULL ,
	p_id1                varchar(50)  NOT NULL ,
	p_id2                varchar(50)   ,
	p1_is_host           boolean   ,
	cl_id1               bigint  NOT NULL ,
	cl_id2               bigint   ,
	CONSTRAINT pk_coop_bundled_id PRIMARY KEY ( id )
 );

CREATE  TABLE "p2boards".demos ( 
	id                   bigserial  NOT NULL ,
	drive_url            varchar(100)  NOT NULL ,
	partner_name         varchar(50)   ,
	parsed_successfully  boolean DEFAULT false NOT NULL ,
	sar_version          varchar(50)   ,
	cl_id                bigint  NOT NULL ,
	CONSTRAINT unq_demos_id UNIQUE ( id ) 
 );

ALTER TABLE "p2boards".changelog ADD CONSTRAINT fk_changelog_users FOREIGN KEY ( profile_number ) REFERENCES "p2boards".users( profile_number );
ALTER TABLE "p2boards".changelog ADD CONSTRAINT fk_changelog_maps FOREIGN KEY ( map_id ) REFERENCES "p2boards".maps( steam_id );
ALTER TABLE "p2boards".changelog ADD CONSTRAINT fk_changelog_coop_bundled FOREIGN KEY ( coop_id ) REFERENCES "p2boards".coop_bundled( id );
ALTER TABLE "p2boards".changelog ADD CONSTRAINT fk_changelog_demos FOREIGN KEY ( demo_id ) REFERENCES "p2boards".demos( id );
ALTER TABLE "p2boards".changelog ADD CONSTRAINT fk_changelog_categories FOREIGN KEY ( category_id ) REFERENCES "p2boards".categories( id );
ALTER TABLE "p2boards".coop_bundled ADD CONSTRAINT fk_coop_bundled_cl_id1 FOREIGN KEY ( cl_id1 ) REFERENCES "p2boards".changelog( id );
ALTER TABLE "p2boards".coop_bundled ADD CONSTRAINT fk_coop_bundled_chapters_cl_id2 FOREIGN KEY ( cl_id2 ) REFERENCES "p2boards".changelog( id );
ALTER TABLE "p2boards".coop_bundled ADD CONSTRAINT fk_coop_bundled_users_u1 FOREIGN KEY ( p_id1 ) REFERENCES "p2boards".users( profile_number );
ALTER TABLE "p2boards".coop_bundled ADD CONSTRAINT fk_coop_bundled_users_u2 FOREIGN KEY ( p_id2 ) REFERENCES "p2boards".users( profile_number );

-- migrate:down


ALTER TABLE "p2boards".changelog DROP CONSTRAINT fk_changelog_users;
ALTER TABLE "p2boards".changelog DROP CONSTRAINT fk_changelog_maps;
ALTER TABLE "p2boards".changelog DROP CONSTRAINT fk_changelog_coop_bundled;
ALTER TABLE "p2boards".changelog DROP CONSTRAINT fk_changelog_demos;
ALTER TABLE "p2boards".changelog DROP CONSTRAINT fk_changelog_categories;
ALTER TABLE "p2boards".coop_bundled DROP CONSTRAINT fk_coop_bundled_cl_id1;
ALTER TABLE "p2boards".coop_bundled DROP CONSTRAINT fk_coop_bundled_chapters_cl_id2;
ALTER TABLE "p2boards".coop_bundled DROP CONSTRAINT fk_coop_bundled_users_u1;
ALTER TABLE "p2boards".coop_bundled DROP CONSTRAINT fk_coop_bundled_users_u2;
ALTER TABLE "p2boards".chapters DROP CONSTRAINT fk_chapters_game_id;
ALTER TABLE "p2boards".maps DROP CONSTRAINT fk_maps_chapters;

DROP  TABLE "p2boards".categories; 
DROP  TABLE "p2boards".games; 
DROP  TABLE "p2boards".users; 
DROP  TABLE "p2boards".chapters; 
DROP  TABLE "p2boards".maps; 
DROP  TABLE "p2boards".changelog; 
DROP  TABLE "p2boards".coop_bundled; 
DROP  TABLE "p2boards".demos; 
DROP SCHEMA IF EXISTS "p2boards";