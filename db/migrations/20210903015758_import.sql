-- migrate:up

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: p2boards; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA p2boards;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: categories; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.categories (
    id integer NOT NULL,
    name character varying(100) DEFAULT ''::character varying NOT NULL,
    map_id character varying(6) DEFAULT ''::character varying NOT NULL,
    rules character varying(1000) DEFAULT ''::character varying NOT NULL
);


--
-- Name: categories_id_seq; Type: SEQUENCE; Schema: p2boards; Owner: -
--

CREATE SEQUENCE p2boards.categories_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: categories_id_seq; Type: SEQUENCE OWNED BY; Schema: p2boards; Owner: -
--

ALTER SEQUENCE p2boards.categories_id_seq OWNED BY p2boards.categories.id;


--
-- Name: changelog; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.changelog (
    id bigint NOT NULL,
    "timestamp" timestamp(6) without time zone,
    profile_number character varying(50) NOT NULL,
    score integer NOT NULL,
    map_id character varying(6) DEFAULT ''::character varying NOT NULL,
    demo_id bigint,
    banned boolean DEFAULT false NOT NULL,
    youtube_id character varying(30),
    previous_id integer,
    coop_id bigint,
    post_rank integer,
    pre_rank integer,
    submission boolean DEFAULT false NOT NULL,
    note character varying(100),
    category_id integer DEFAULT 1 NOT NULL,
    score_delta integer,
    verified boolean,
    admin_note character varying(200)
);


--
-- Name: changelog_id_seq; Type: SEQUENCE; Schema: p2boards; Owner: -
--

CREATE SEQUENCE p2boards.changelog_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: changelog_id_seq; Type: SEQUENCE OWNED BY; Schema: p2boards; Owner: -
--

ALTER SEQUENCE p2boards.changelog_id_seq OWNED BY p2boards.changelog.id;


--
-- Name: chapters; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.chapters (
    id integer NOT NULL,
    chapter_name character varying(50),
    is_multiplayer boolean DEFAULT false NOT NULL,
    game_id integer DEFAULT 1 NOT NULL
);


--
-- Name: chapters_id_seq; Type: SEQUENCE; Schema: p2boards; Owner: -
--

CREATE SEQUENCE p2boards.chapters_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: chapters_id_seq; Type: SEQUENCE OWNED BY; Schema: p2boards; Owner: -
--

ALTER SEQUENCE p2boards.chapters_id_seq OWNED BY p2boards.chapters.id;


--
-- Name: coop_bundled; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.coop_bundled (
    id bigint NOT NULL,
    p_id1 character varying(50) NOT NULL,
    p_id2 character varying(50),
    p1_is_host boolean,
    cl_id1 bigint NOT NULL,
    cl_id2 bigint
);


--
-- Name: coop_bundled_id_seq; Type: SEQUENCE; Schema: p2boards; Owner: -
--

CREATE SEQUENCE p2boards.coop_bundled_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: coop_bundled_id_seq; Type: SEQUENCE OWNED BY; Schema: p2boards; Owner: -
--

ALTER SEQUENCE p2boards.coop_bundled_id_seq OWNED BY p2boards.coop_bundled.id;


--
-- Name: demos; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.demos (
    id bigint NOT NULL,
    drive_url character varying(100) NOT NULL,
    partner_name character varying(50),
    parsed_successfully boolean DEFAULT false NOT NULL,
    sar_version character varying(50),
    cl_id bigint NOT NULL
);


--
-- Name: demos_id_seq; Type: SEQUENCE; Schema: p2boards; Owner: -
--

CREATE SEQUENCE p2boards.demos_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: demos_id_seq; Type: SEQUENCE OWNED BY; Schema: p2boards; Owner: -
--

ALTER SEQUENCE p2boards.demos_id_seq OWNED BY p2boards.demos.id;


--
-- Name: games; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.games (
    id integer NOT NULL,
    game_name character varying(50) DEFAULT 'Portal 2'::character varying NOT NULL
);


--
-- Name: games_id_seq; Type: SEQUENCE; Schema: p2boards; Owner: -
--

CREATE SEQUENCE p2boards.games_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: games_id_seq; Type: SEQUENCE OWNED BY; Schema: p2boards; Owner: -
--

ALTER SEQUENCE p2boards.games_id_seq OWNED BY p2boards.games.id;


--
-- Name: maps; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.maps (
    id integer NOT NULL,
    steam_id character varying(6) DEFAULT ''::character varying NOT NULL,
    lp_id character varying(6) DEFAULT ''::character varying NOT NULL,
    name character varying(50) NOT NULL,
    chapter_id integer,
    is_public boolean DEFAULT false NOT NULL
);


--
-- Name: maps_id_seq; Type: SEQUENCE; Schema: p2boards; Owner: -
--

CREATE SEQUENCE p2boards.maps_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: maps_id_seq; Type: SEQUENCE OWNED BY; Schema: p2boards; Owner: -
--

ALTER SEQUENCE p2boards.maps_id_seq OWNED BY p2boards.maps.id;


--
-- Name: users; Type: TABLE; Schema: p2boards; Owner: -
--

CREATE TABLE p2boards.users (
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
    discord_id character varying(40)
);


--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.schema_migrations (
    version character varying(255) NOT NULL
);


--
-- Name: categories id; Type: DEFAULT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.categories ALTER COLUMN id SET DEFAULT nextval('p2boards.categories_id_seq'::regclass);


--
-- Name: changelog id; Type: DEFAULT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.changelog ALTER COLUMN id SET DEFAULT nextval('p2boards.changelog_id_seq'::regclass);


--
-- Name: chapters id; Type: DEFAULT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.chapters ALTER COLUMN id SET DEFAULT nextval('p2boards.chapters_id_seq'::regclass);


--
-- Name: coop_bundled id; Type: DEFAULT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.coop_bundled ALTER COLUMN id SET DEFAULT nextval('p2boards.coop_bundled_id_seq'::regclass);


--
-- Name: demos id; Type: DEFAULT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.demos ALTER COLUMN id SET DEFAULT nextval('p2boards.demos_id_seq'::regclass);


--
-- Name: games id; Type: DEFAULT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.games ALTER COLUMN id SET DEFAULT nextval('p2boards.games_id_seq'::regclass);


--
-- Name: maps id; Type: DEFAULT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.maps ALTER COLUMN id SET DEFAULT nextval('p2boards.maps_id_seq'::regclass);


--
-- Name: categories pk_categories_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.categories
    ADD CONSTRAINT pk_categories_id PRIMARY KEY (id);


--
-- Name: changelog pk_changelog_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.changelog
    ADD CONSTRAINT pk_changelog_id PRIMARY KEY (id);


--
-- Name: chapters pk_chapters_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.chapters
    ADD CONSTRAINT pk_chapters_id PRIMARY KEY (id);


--
-- Name: coop_bundled pk_coop_bundled_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.coop_bundled
    ADD CONSTRAINT pk_coop_bundled_id PRIMARY KEY (id);


--
-- Name: games pk_game_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.games
    ADD CONSTRAINT pk_game_id PRIMARY KEY (id);


--
-- Name: maps pk_maps_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.maps
    ADD CONSTRAINT pk_maps_id PRIMARY KEY (id);


--
-- Name: users pk_users_profile_number; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.users
    ADD CONSTRAINT pk_users_profile_number PRIMARY KEY (profile_number);


--
-- Name: demos unq_demos_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.demos
    ADD CONSTRAINT unq_demos_id UNIQUE (id);


--
-- Name: maps unq_maps_steam_id; Type: CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.maps
    ADD CONSTRAINT unq_maps_steam_id UNIQUE (steam_id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: changelog fk_changelog_categories; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.changelog
    ADD CONSTRAINT fk_changelog_categories FOREIGN KEY (category_id) REFERENCES p2boards.categories(id);


--
-- Name: changelog fk_changelog_coop_bundled; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.changelog
    ADD CONSTRAINT fk_changelog_coop_bundled FOREIGN KEY (coop_id) REFERENCES p2boards.coop_bundled(id);


--
-- Name: changelog fk_changelog_demos; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.changelog
    ADD CONSTRAINT fk_changelog_demos FOREIGN KEY (demo_id) REFERENCES p2boards.demos(id);


--
-- Name: changelog fk_changelog_maps; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.changelog
    ADD CONSTRAINT fk_changelog_maps FOREIGN KEY (map_id) REFERENCES p2boards.maps(steam_id);


--
-- Name: changelog fk_changelog_users; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.changelog
    ADD CONSTRAINT fk_changelog_users FOREIGN KEY (profile_number) REFERENCES p2boards.users(profile_number);


--
-- Name: chapters fk_chapters_game_id; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.chapters
    ADD CONSTRAINT fk_chapters_game_id FOREIGN KEY (game_id) REFERENCES p2boards.games(id);


--
-- Name: coop_bundled fk_coop_bundled_chapters_cl_id2; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.coop_bundled
    ADD CONSTRAINT fk_coop_bundled_chapters_cl_id2 FOREIGN KEY (cl_id2) REFERENCES p2boards.changelog(id);


--
-- Name: coop_bundled fk_coop_bundled_cl_id1; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.coop_bundled
    ADD CONSTRAINT fk_coop_bundled_cl_id1 FOREIGN KEY (cl_id1) REFERENCES p2boards.changelog(id);


--
-- Name: coop_bundled fk_coop_bundled_users_u1; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.coop_bundled
    ADD CONSTRAINT fk_coop_bundled_users_u1 FOREIGN KEY (p_id1) REFERENCES p2boards.users(profile_number);


--
-- Name: coop_bundled fk_coop_bundled_users_u2; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.coop_bundled
    ADD CONSTRAINT fk_coop_bundled_users_u2 FOREIGN KEY (p_id2) REFERENCES p2boards.users(profile_number);


--
-- Name: maps fk_maps_chapters; Type: FK CONSTRAINT; Schema: p2boards; Owner: -
--

ALTER TABLE ONLY p2boards.maps
    ADD CONSTRAINT fk_maps_chapters FOREIGN KEY (chapter_id) REFERENCES p2boards.chapters(id);


--
-- PostgreSQL database dump complete
--


--
-- Dbmate schema migrations
--

-- migrate:down

