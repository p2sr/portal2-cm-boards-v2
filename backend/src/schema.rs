//use diesel::sql_types::*;

table! {
    changelog (id) {
        time_gained -> Nullable<Timestamp>,
        profile_number -> Varchar,
        score -> Integer,
        map_id -> Varchar,
        wr_gain -> Integer,
        has_demo -> Nullable<Integer>,
        banned -> Integer,
        youtube_id -> Nullable<Varchar>,
        previous_id -> Nullable<Integer>,
        id -> Integer,
        coopid -> Nullable<Integer>,
        post_rank -> Nullable<Integer>,
        pre_rank -> Nullable<Integer>,
        submission -> Integer,
        note -> Nullable<Varchar>,
        category -> Nullable<Varchar>,
    }
}

table! {
    chapters (id) {
        id -> Unsigned<Integer>,
        chapter_name -> Nullable<Varchar>,
        is_multiplayer -> Integer,
    }
}

table! {
    coopbundled (id) {
        time_gained -> Nullable<Timestamp>,
        profile_number1 -> Varchar,
        profile_number2 -> Varchar,
        score -> Integer,
        map_id -> Varchar,
        wr_gain -> Integer,
        is_blue -> Nullable<Integer>,
        has_demo1 -> Nullable<Integer>,
        has_demo2 -> Nullable<Integer>,
        banned -> Integer,
        youtube_id1 -> Nullable<Varchar>,
        youtube_id2 -> Nullable<Varchar>,
        previous_id1 -> Nullable<Integer>,
        previous_id2 -> Nullable<Integer>,
        changelogid1 -> Integer,
        changelogid2 -> Integer,
        id -> Integer,
        post_rank1 -> Nullable<Integer>,
        post_rank2 -> Nullable<Integer>,
        pre_rank1 -> Nullable<Integer>,
        pre_rank2 -> Nullable<Integer>,
        submission1 -> Integer,
        submission2 -> Integer,
        note1 -> Nullable<Varchar>,
        note2 -> Nullable<Varchar>,
        category -> Nullable<Varchar>,
    }
}

table! {
    exceptions (map_id, legit_score, curl) {
        map_id -> Varchar,
        legit_score -> Integer,
        curl -> Integer,
    }
}

table! {
    leastportals (steam_id) {
        steam_id -> Varchar,
        portals -> Integer,
    }
}

table! {
    leastportals_exceptions (map_id, profile_number) {
        map_id -> Varchar,
        profile_number -> Varchar,
    }
}

table! {
    maps (id) {
        id -> Integer,
        steam_id -> Varchar,
        lp_id -> Varchar,
        name -> Nullable<Varchar>,
        #[sql_name = "type"]
        type_ -> Varchar,
        chapter_id -> Nullable<Unsigned<Integer>>,
        is_coop -> Integer,
        is_public -> Integer,
    }
}

table! {
    scores (changelog_id) {
        profile_number -> Varchar,
        map_id -> Varchar,
        changelog_id -> Integer,
    }
}

table! {
    singlesegment (id) {
        id -> Integer,
        updated -> Varchar,
        datatable -> Mediumtext,
    }
}

table! {
    usersnew (profile_number) {
        profile_number -> Varchar,
        boardname -> Nullable<Varchar>,
        steamname -> Nullable<Varchar>,
        banned -> Integer,
        registered -> Integer,
        avatar -> Nullable<Varchar>,
        twitch -> Nullable<Varchar>,
        youtube -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        admin -> Integer,
        donation_amount -> Nullable<Varchar>,
    }
}

joinable!(maps -> chapters (chapter_id));

allow_tables_to_appear_in_same_query!(
    changelog,
    chapters,
    coopbundled,
    exceptions,
    leastportals,
    leastportals_exceptions,
    maps,
    scores,
    singlesegment,
    usersnew,
);