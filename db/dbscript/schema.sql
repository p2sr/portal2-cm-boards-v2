--
-- PostgreSQL database dump
--

-- Dumped from database version 13.4
-- Dumped by pg_dump version 13.4

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', 'public', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: evidence_requirements; Type: TABLE;
--

CREATE TABLE evidence_requirements (
    id integer NOT NULL,
    rank integer NOT NULL,
    demo boolean,
    video boolean,
    active boolean,
    "timestamp" timestamp(6) without time zone,
    closed_timestamp timestamp(6) without time zone
);

--
-- Name: evidence_requirements_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE evidence_requirements_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: evidence_requirements_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE evidence_requirements_id_seq OWNED BY evidence_requirements.id;


--
-- Name: categories; Type: TABLE;
--

CREATE TABLE categories (
    id integer NOT NULL,
    name character varying(100) DEFAULT ''::character varying NOT NULL,
    map_id character varying(6) DEFAULT ''::character varying NOT NULL,
    rules_id integer,
    updated timestamp(6) without time zone
);

-- Trigger to update `updated` when a row is updated.
 
CREATE OR REPLACE FUNCTION update_updated_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated = now(); 
   RETURN NEW;
END;
$$ language 'plpgsql';

    CREATE TRIGGER update_categories_updated BEFORE UPDATE
    ON categories FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_column();


--
-- Name: categories_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE categories_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: categories_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE categories_id_seq OWNED BY categories.id;

--
-- Name: category_rules; Type: TABLE;
--

CREATE TABLE category_rules (
    id integer NOT NULL,
    rules character varying(2000),
    external_link character varying(200),
    is_active boolean,
    updated timestamp(6) without time zone
);

    CREATE TRIGGER update_category_rules_updated BEFORE UPDATE
    ON category_rules FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_column();

--
-- Name: category_rules_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE category_rules_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

--
-- Name: category_rules_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE category_rules_id_seq OWNED BY category_rules.id;

--
-- Name: changelog; Type: TABLE;
--

CREATE TABLE changelog (
    id bigint NOT NULL,
    "timestamp" timestamp(6) without time zone,
    profile_number character varying(50) NOT NULL,
    score integer NOT NULL,
    map_id character varying(6) DEFAULT ''::character varying NOT NULL,
    demo_id bigint,
    banned boolean DEFAULT false NOT NULL,
    youtube_id character varying(30),
    previous_id bigint,
    coop_id bigint,
    post_rank integer,
    pre_rank integer,
    submission boolean DEFAULT false NOT NULL,
    note character varying(100),
    category_id integer DEFAULT 1 NOT NULL,
    score_delta integer,
    verified boolean,
    admin_note character varying(200),
    updated timestamp(6) without time zone
);

-- Trigger to update `updated` when a row is updated.

    CREATE TRIGGER update_changelog_updated BEFORE UPDATE
    ON changelog FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_column();

--
-- Name: changelog_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE changelog_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: changelog_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE changelog_id_seq OWNED BY changelog.id;


--
-- Name: chapters; Type: TABLE;
--

CREATE TABLE chapters (
    id integer NOT NULL,
    chapter_name character varying(50),
    is_multiplayer boolean DEFAULT false NOT NULL,
    game_id integer DEFAULT 1 NOT NULL
);


--
-- Name: chapters_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE chapters_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: chapters_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE chapters_id_seq OWNED BY chapters.id;


--
-- Name: coop_bundled; Type: TABLE;
--

CREATE TABLE coop_bundled (
    id bigint NOT NULL,
    p_id1 character varying(50) NOT NULL,
    p_id2 character varying(50),
    p1_is_host boolean,
    cl_id1 bigint NOT NULL,
    cl_id2 bigint,
    updated timestamp(6) without time zone
);

-- Trigger to update `updated` when a row is updated.
 
    CREATE TRIGGER update_coop_bundled_updated BEFORE UPDATE
    ON coop_bundled FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_column();

--
-- Name: coop_bundled_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE coop_bundled_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: coop_bundled_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE coop_bundled_id_seq OWNED BY coop_bundled.id;


--
-- Name: demos; Type: TABLE;
--

CREATE TABLE demos (
    id bigint NOT NULL,
    file_id character varying(100) NOT NULL,
    partner_name character varying(50),
    parsed_successfully boolean DEFAULT false NOT NULL,
    sar_version character varying(50),
    cl_id bigint NOT NULL,
    updated timestamp(6) without time zone
);


-- Trigger to update `updated` when a row is updated.
 
    CREATE TRIGGER update_demos_updated BEFORE UPDATE
    ON demos FOR EACH ROW EXECUTE PROCEDURE 
    update_updated_column();

--
-- Name: demos_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE demos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: demos_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE demos_id_seq OWNED BY demos.id;


--
-- Name: games; Type: TABLE;
--

CREATE TABLE games (
    id integer NOT NULL,
    game_name character varying(50) DEFAULT 'Portal 2'::character varying NOT NULL
);


--
-- Name: games_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE games_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: games_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE games_id_seq OWNED BY games.id;


--
-- Name: maps; Type: TABLE;
--

CREATE TABLE maps (
    id integer NOT NULL,
    steam_id character varying(6) DEFAULT ''::character varying NOT NULL,
    lp_id character varying(6) DEFAULT ''::character varying NOT NULL,
    name character varying(50) NOT NULL,
    chapter_id integer,
    default_cat_id integer,
    is_public boolean DEFAULT false NOT NULL
);


--
-- Name: maps_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE maps_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: maps_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE maps_id_seq OWNED BY maps.id;


-- MTRIGGERS TEST:

--
-- Name: mtriggers; Type: TABLE;
--

CREATE TABLE mtriggers (
    id integer NOT NULL,
    map_id character varying(6) NOT NULL,
    category_id integer NOT NULL,
    name character varying(64),
    description character varying(500)
);

--
-- Name: mtriggers_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE mtriggers_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: mtriggers_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE mtriggers_id_seq OWNED BY mtriggers.id;

--
-- Name: mtriggers; Type: TABLE;
--

CREATE TABLE mtrigger_entries (
    id integer NOT NULL,
    mtrigger_id integer NOT NULL,
    changelog_id bigint NOT NULL,
    "time" integer NOT NULL
);

--
-- Name: mtrigger_entries_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE mtrigger_entries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: mtrigger_entries_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE mtrigger_entries_id_seq OWNED BY mtrigger_entries.id;


--
-- Name: countries; Type: TABLE;
--

CREATE TABLE countries (
    id integer NOT NULL,
    iso character varying(2) NOT NULL,
    name character varying(80) NOT NULL,
    nicename character varying(80) NOT NULL,
    iso3 character varying(3) DEFAULT NULL,
    numcode integer DEFAULT NULL,
    phonecode integer NOT NULL
);

--
-- Name: countries_id_seq; Type: SEQUENCE;
--

CREATE SEQUENCE countries_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: countries_id_seq; Type: SEQUENCE OWNED BY;
--

ALTER SEQUENCE countries_id_seq OWNED BY countries.id;


--
-- Name: users; Type: TABLE;
--

CREATE TABLE users (
    profile_number character varying(50) DEFAULT ''::character varying NOT NULL,
    board_name character varying(50),
    steam_name character varying(50),
    banned boolean DEFAULT false NOT NULL,
    registered integer DEFAULT 0 NOT NULL,
    avatar character varying(200),
    twitch character varying(100),
    youtube character varying(100),
    title character varying(200),
    admin integer DEFAULT 0 NOT NULL,
    donation_amount character varying(11),
    discord_id character varying(40),
    auth_hash character varying(64),
    country_id integer
);

--
-- Name: evidence_requirements id; Type: DEFAULT;
--

ALTER TABLE ONLY evidence_requirements ALTER COLUMN id SET DEFAULT nextval('evidence_requirements_id_seq'::regclass);

--
-- Name: categories id; Type: DEFAULT;
--

ALTER TABLE ONLY categories ALTER COLUMN id SET DEFAULT nextval('categories_id_seq'::regclass);

--
-- Name: category_rules id; Type: DEFAULT;
--

ALTER TABLE ONLY category_rules ALTER COLUMN id SET DEFAULT nextval('category_rules_id_seq'::regclass);


--
-- Name: changelog id; Type: DEFAULT;
--

ALTER TABLE ONLY changelog ALTER COLUMN id SET DEFAULT nextval('changelog_id_seq'::regclass);


--
-- Name: chapters id; Type: DEFAULT;
--

ALTER TABLE ONLY chapters ALTER COLUMN id SET DEFAULT nextval('chapters_id_seq'::regclass);


--
-- Name: coop_bundled id; Type: DEFAULT;
--

ALTER TABLE ONLY coop_bundled ALTER COLUMN id SET DEFAULT nextval('coop_bundled_id_seq'::regclass);


--
-- Name: demos id; Type: DEFAULT;
--

ALTER TABLE ONLY demos ALTER COLUMN id SET DEFAULT nextval('demos_id_seq'::regclass);


--
-- Name: games id; Type: DEFAULT;
--

ALTER TABLE ONLY games ALTER COLUMN id SET DEFAULT nextval('games_id_seq'::regclass);


--
-- Name: maps id; Type: DEFAULT;
--

ALTER TABLE ONLY maps ALTER COLUMN id SET DEFAULT nextval('maps_id_seq'::regclass);

--
-- Name: countries id; Type: DEFAULT;
--

ALTER TABLE ONLY countries ALTER COLUMN id SET DEFAULT nextval('countries_id_seq'::regclass);

--
-- Name: mtriggers id; Type: DEFAULT;
--

ALTER TABLE ONLY mtriggers ALTER COLUMN id SET DEFAULT nextval('mtriggers_id_seq'::regclass);

--
-- Name: mtrigger_entries id; Type: DEFAULT;
--

ALTER TABLE ONLY mtrigger_entries ALTER COLUMN id SET DEFAULT nextval('mtrigger_entries_id_seq'::regclass);



-- --
-- -- Name: categories_id_seq; Type: SEQUENCE SET;
-- --

-- SELECT pg_catalog.setval('categories_id_seq', 109, false);


-- --
-- -- Name: changelog_id_seq; Type: SEQUENCE SET;
-- --

-- SELECT pg_catalog.setval('changelog_id_seq', 157886, true);


-- --
-- -- Name: chapters_id_seq; Type: SEQUENCE SET;
-- --

-- SELECT pg_catalog.setval('chapters_id_seq', 16, false);


-- --
-- -- Name: coop_bundled_id_seq; Type: SEQUENCE SET;
-- --

-- SELECT pg_catalog.setval('coop_bundled_id_seq', 33186, false);


-- --
-- -- Name: demos_id_seq; Type: SEQUENCE SET;
-- --

-- SELECT pg_catalog.setval('demos_id_seq', 27503, true);


-- --
-- -- Name: games_id_seq; Type: SEQUENCE SET;
-- --

-- SELECT pg_catalog.setval('games_id_seq', 2, false);


-- --
-- -- Name: maps_id_seq; Type: SEQUENCE SET;
-- --

-- SELECT pg_catalog.setval('maps_id_seq', 109, false);

--
-- Name: evidence_requirements pk_evidence_requirements_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY evidence_requirements
    ADD CONSTRAINT pk_evidence_requirements_id PRIMARY KEY (id);

--
-- Name: categories pk_categories_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY categories
    ADD CONSTRAINT pk_categories_id PRIMARY KEY (id);

--
-- Name: category_rules pk_category_rules_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY category_rules
    ADD CONSTRAINT pk_category_rules_id PRIMARY KEY (id);

--
-- Name: countries pk_countries_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY countries
    ADD CONSTRAINT pk_countries_id PRIMARY KEY (id);

--
-- Name: changelog pk_changelog_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY changelog
    ADD CONSTRAINT pk_changelog_id PRIMARY KEY (id);


--
-- Name: chapters pk_chapters_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY chapters
    ADD CONSTRAINT pk_chapters_id PRIMARY KEY (id);


--
-- Name: coop_bundled pk_coop_bundled_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY coop_bundled
    ADD CONSTRAINT pk_coop_bundled_id PRIMARY KEY (id);


--
-- Name: games pk_game_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY games
    ADD CONSTRAINT pk_game_id PRIMARY KEY (id);


--
-- Name: maps pk_maps_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY maps
    ADD CONSTRAINT pk_maps_id PRIMARY KEY (id);


--
-- Name: mtriggers pk_mtriggers_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY mtriggers
    ADD CONSTRAINT pk_mtriggers_id PRIMARY KEY (id);

--
-- Name: mtrigger_entries pk_mtrigger_entries_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY mtrigger_entries
    ADD CONSTRAINT pk_mtrigger_entries_id PRIMARY KEY (id);

--
-- Name: users pk_users_profile_number; Type: CONSTRAINT;
--

ALTER TABLE ONLY users
    ADD CONSTRAINT pk_users_profile_number PRIMARY KEY (profile_number);


--
-- Name: demos unq_demos_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY demos
    ADD CONSTRAINT unq_demos_id UNIQUE (id);


--
-- Name: maps unq_maps_steam_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY maps
    ADD CONSTRAINT unq_maps_steam_id UNIQUE (steam_id);


--
-- Name: mtriggers unq_mtriggers_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY mtriggers
    ADD CONSTRAINT unq_mtriggers_id UNIQUE (id);


--
-- Name: mtrigger_entries unq_mtrigger_entries_id; Type: CONSTRAINT;
--

ALTER TABLE ONLY mtrigger_entries
    ADD CONSTRAINT unq_mtrigger_entries_id UNIQUE (id);


--
-- Name: categories fk_categories_category_rules; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY categories
    ADD CONSTRAINT fk_categories_category_rules FOREIGN KEY (rules_id) REFERENCES category_rules(id);


--
-- Name: changelog fk_changelog_categories; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY changelog
    ADD CONSTRAINT fk_changelog_categories FOREIGN KEY (category_id) REFERENCES categories(id);


--
-- Name: changelog fk_changelog_coop_bundled; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY changelog
    ADD CONSTRAINT fk_changelog_coop_bundled FOREIGN KEY (coop_id) REFERENCES coop_bundled(id);


--
-- Name: changelog fk_changelog_demos; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY changelog
    ADD CONSTRAINT fk_changelog_demos FOREIGN KEY (demo_id) REFERENCES demos(id);


--
-- Name: changelog fk_changelog_maps; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY changelog
    ADD CONSTRAINT fk_changelog_maps FOREIGN KEY (map_id) REFERENCES maps(steam_id);


--
-- Name: changelog fk_changelog_users; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY changelog
    ADD CONSTRAINT fk_changelog_users FOREIGN KEY (profile_number) REFERENCES users(profile_number);


--
-- Name: chapters fk_chapters_game_id; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY chapters
    ADD CONSTRAINT fk_chapters_game_id FOREIGN KEY (game_id) REFERENCES games(id);


--
-- Name: coop_bundled fk_coop_bundled_chapters_cl_id2; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY coop_bundled
    ADD CONSTRAINT fk_coop_bundled_chapters_cl_id2 FOREIGN KEY (cl_id2) REFERENCES changelog(id);


--
-- Name: coop_bundled fk_coop_bundled_cl_id1; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY coop_bundled
    ADD CONSTRAINT fk_coop_bundled_cl_id1 FOREIGN KEY (cl_id1) REFERENCES changelog(id);


--
-- Name: coop_bundled fk_coop_bundled_users_u1; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY coop_bundled
    ADD CONSTRAINT fk_coop_bundled_users_u1 FOREIGN KEY (p_id1) REFERENCES users(profile_number);


--
-- Name: coop_bundled fk_coop_bundled_users_u2; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY coop_bundled
    ADD CONSTRAINT fk_coop_bundled_users_u2 FOREIGN KEY (p_id2) REFERENCES users(profile_number);


--
-- Name: maps fk_maps_chapters; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY maps
    ADD CONSTRAINT fk_maps_chapters FOREIGN KEY (chapter_id) REFERENCES chapters(id);

--
-- Name: users fk_users_countries; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY users
    ADD CONSTRAINT fk_users_countries FOREIGN KEY (country_id) REFERENCES countries(id);

--
-- Name: mtriggers fk_mtriggers_maps; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY mtriggers
    ADD CONSTRAINT fk_mtriggers_maps FOREIGN KEY (map_id) REFERENCES maps(steam_id);

--
-- Name: mtriggers fk_mtriggers_categories; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY mtriggers
    ADD CONSTRAINT fk_mtriggers_categories FOREIGN KEY (category_id) REFERENCES categories(id);


--
-- Name: mtrigger_entries fk_mtrigger_entries_mtriggers; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY mtrigger_entries
    ADD CONSTRAINT fk_mtrigger_entries_mtriggers FOREIGN KEY (mtrigger_id) REFERENCES mtriggers(id);

--
-- Name: mtrigger_entries fk_mtrigger_entries_changelog; Type: FK CONSTRAINT;
--

ALTER TABLE ONLY mtrigger_entries
    ADD CONSTRAINT fk_mtrigger_entries_changelog FOREIGN KEY (changelog_id) REFERENCES changelog(id);



--
-- PostgreSQL database dump complete
--

