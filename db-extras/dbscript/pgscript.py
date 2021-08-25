from re import L
import mysql.connector
import psycopg2

changelogIDs = []
coopdict = {}

with open(".secret") as fp:
    db_password = fp.read()

# Game
class PgSQLGame:
    def __init__(self, id_, name):
        self.id = id_
        self.name = name
    
    def __str__(self):
        return f"{self.id} - {self.name}"

# Chapters
class MySQLChapter:
    def __init__(self, id_, chapter_name, is_multiplayer):
        self.id = id_
        self.chapter_name = chapter_name
        self.is_multiplayer = is_multiplayer
    
    def __str__(self):
        return f"{self.id} - {self.chapter_name} - {self.is_multiplayer}"

class PgSQLChapter:
    def __init__(self, id_, chapter_name, is_multiplayer, game_id):
        self.id = id_
        self.chapter_name = chapter_name
        self.is_multiplayer = is_multiplayer
        self.game_id = 1    # 1 for default game (Portal 2)

    def __str__(self):
        return f"{self.id} - {self.chapter_name} - {self.is_multiplayer} - {self.game_id}"

# Maps
class MySQLMap:
    def __init__(self, id_, steam_id, lp_id, name, type_, chapter_id, is_coop, is_public):
        self.id = id_
        self.steam_id = steam_id
        self.lp_id = lp_id
        self.name = name 
        self.type = type_
        self.chapter_id = chapter_id 
        self.is_coop = is_coop 
        self.is_public = is_public
    
    def __str__(self):
        return f"{self.id} - {self.steam_id} - {self.name} - {self.is_coop}"

class PGSQLMap:
    def __init__(self, id_, steam_id, lp_id, name, chapter_id, is_public):
        self.id = id_
        self.steam_id = steam_id
        self.lp_id = lp_id
        self.name = name 
        self.chapter_id = chapter_id 
        self.is_public = is_public
    
    def __str__(self):
        return f"{self.id} - {self.steam_id} - {self.name}"

# Categories
class PgSQLCategories:
    def __init__(self, id_, name, map_id, rules):
        self.id = id_
        self.name = name
        self.map_id = map_id
        self.rules = rules

# Users
class MySQLUsersnew:
    def __init__(self, profile_number, boardname, 
    steamname, banned, registered, avatar, twitch, 
    youtube, title, admin, donation_amount):
        self.profile_number = profile_number
        self.boardname = boardname
        self.steamname = steamname
        self.banned = banned
        self.registered = registered
        self.avatar = avatar
        self.twitch = twitch
        self.youtube = youtube
        self.title = title
        self.admin = admin
        self.donation_amount = donation_amount
    
    def __str__(self):
        return f"{self.profile_number} - {self.steamname} - {self.banned}"

class PgSQLUsers:
    def __init__(self, profile_number, board_name, 
    steam_name, banned, registered, avatar, twitch, 
    youtube, title, admin, donation_amount):
        self.profile_number = profile_number
        self.board_name = board_name
        self.steam_name = steam_name
        self.banned = banned
        self.registered = registered
        self.avatar = avatar
        self.twitch = twitch
        self.youtube = youtube
        self.title = title
        self.admin = admin
        self.donation_amount = donation_amount
        self.discord_id = None
    
    def __str__(self):
        return f"{self.profile_number} - {self.steamname} - {self.banned}"



# Changelogs
class MySQLChangelog:
    def __init__(self, time_gained,
    profile_number, score, map_id, wr_gain, 
    has_demo, banned, youtube_id, previous_id, 
    id_, post_rank, pre_rank, submission, note, pending):
        self.time_gained = time_gained
        self.profile_number = profile_number
        self.score = score
        self.map_id = map_id
        self.wr_gain = wr_gain
        self.has_demo = has_demo
        self.banned = banned
        self.youtube_id = youtube_id
        self.previous_id = previous_id
        self.id = id_
        self.post_rank = post_rank
        self.pre_rank = pre_rank
        self.submission = submission
        self.note = note
        self.pending = pending
    def __str__(self):
        return f"{self.id} - {self.time_gained} - {self.profile_number} - {self.score} - {self.note}"

class PSQLChangelog:
    def __init__(self, time_gained,
    profile_number, score, map_id, 
    has_demo, banned, youtube_id, previous_id, 
    id_, coopid, post_rank, pre_rank, submission, note, pending):
        self.id = id_
        self.timestamp = time_gained
        self.profile_number = profile_number
        self.score = score
        self.map_id = map_id
        self.demo_id = self.make_new_demo_id(has_demo)
        self.banned = banned
        self.youtube_id = youtube_id
        self.previous_id = previous_id
        self.coop_id = coopid
        self.post_rank = post_rank
        self.pre_rank = pre_rank
        self.submission = submission
        self.note = note
        self.category_id = self.get_category_id(map_id)
        self.score_delta = self.get_score_delta(previous_id, score)
        self.verified = self.is_verified(pending)
        self.admin_note = None

    def __str__(self):
        return f"{self.id} - {self.time_gained} - {self.profile_number} - {self.score} - {self.note}"

    def make_new_demo_id(self, has_demo):
        if has_demo == 1:
            return None
        else:
            # TODO: Make a new row in `demos`
            raise NotImplementedError
    
    def get_cateogry_id(self, map_id):
        # TODO: Query for the category_id for `any%` on the given map.
        raise NotImplementedError

    def is_verified(self, pending):
        if pending == 1:
            return 0
        elif pending == 0:
            return 1
    
    def get_score_delta(self, previous_id, score):
        # TODO: Select `score` for previous time from MySQL.
        raise NotImplementedError

# Coop bundled
class PgSQLCoopBundled:
    def __init__(self, id_, p_id1, p_id2, cl_id1, cl_id2):
        self.id = id_
        self.p_id1 = p_id1
        self.p_id2 = p_id2
        self.p1_is_host = None # Won't know by default
        self.cl_id1 = cl_id1
        self.cl_id2 = cl_id2

    def __str__(self):
        return f"{self.id} - {self.p_id1} / {self.p_id2} - {self.cl_id1} / {self.cl_id2}"

# Demos
class PgSQLDemos:
    def __init__(self, id_, drive_url, partner_name, parsed_successfully, sar_version, cl_id):
        self.id = id_ 
        self.drive_url = drive_url
        self.partner_name = partner_name
        self.parsed_successfully = parsed_successfully
        self.sar_version = sar_version
        self.cl_id = cl_id
    
    def __str__(self):
        return f"{self.id}"

def main():
    mysql_conn = mysql.connector.connect(
        host="localhost",
        user="root",
        password=db_password,
        database="p2boardsOriginal",
        autocommit=False,
    )
    pg_conn = psycopg2.connect(
        dbname="p2boards",
        user="djbates", # TODO: Allow this to be kept in a .secret or .env (pass as cl arg?)
        password=db_password
    )
    pg_cursor = pg_conn.cursor()
    mysql_cursor = mysql_conn.cursor()
    categories(mysql_cursor, pg_cursor)
    games(mysql_cursor, pg_cursor)
    users(mysql_cursor, pg_cursor)
    chapters(mysql_cursor, pg_cursor)
    maps(mysql_cursor, pg_cursor)
    changelog(mysql_cursor, pg_cursor) #Demo creation will happen here.
    coop_bundled(mysql_cursor, pg_cursor)

def coop_bundled(mysql_cursor, pg_cursor):
    get_all_coop = """SELECT
    changelog.time_gained, changelog.profile_number, changelog.score, changelog.map_id, changelog.wr_gain,
    changelog.has_demo, changelog.banned, changelog.youtube_id, changelog.previous_id, changelog.id,
    changelog.post_rank, changelog.pre_rank, changelog.submission,
    changelog.note, changelog.pending 
    FROM changelog 
    LEFT JOIN maps 
    ON maps.steam_id=changelog.map_id 
    LEFT JOIN usersnew
    ON usersnew.profile_number=changelog.profile_number
    WHERE maps.is_coop=1
    AND usersnew.banned=0
    AND changelog.banned=0
    AND changelog.time_gained IS NOT NULL"""
    mysql_cursor.execute(get_all_coop)
    all_coop = mysql_cursor.fetchall()
    # Adds every coop changelog entry into a class object, and inserts it into a dictionary with id as the key
    for i, x in enumerate(all_coop):
        temp = MySQLChangelog(*x)
        #print(f"Adding key {temp.id}")
        coopdict[temp.id] = temp
        changelogIDs.append(temp.id)
    # Our query handles checking for banned users, changelog entries, and NULL timestamps
    count = 1
    while len(changelogIDs) != 0:
        index = len(changelogIDs)-1
        value = coopdict[changelogIDs[index]]
        #
        get_matching_times = f"SELECT * FROM changelog WHERE time_gained=\"{value.time_gained}\" AND score={value.score} AND map_id={value.map_id}"
        #print(get_matching_times)
        mysql_cursor.execute(get_matching_times)
        matching_times = mysql_cursor.fetchall()
        if len(matching_times) == 2:
            temp = MySQLChangelog(*matching_times[0])
            temp2 = MySQLChangelog(*matching_times[1])
            # TODO: Create coop_bundled
            # TODO: Update both changelog entries to have a coop_bundled ID
            # These updates should happen on the PG side. I will use the MySQL connection to parse coop information,
            # but the actual coop_bundle should be created on PG.
            raise NotImplementedError
        elif len(matching_times) == 1:
            temp = MySQLChangelog(*matching_times[0])
            # TODO: Set the values for cl_id2 & p_id2 to NULL. 
            # Insert coopbundle to PG, and update PG changelog for cl_id1
            raise NotImplementedError
        else: # There are more than 2 times.
            temp = MySQLChangelog(*matching_times[0])
            print(f"{temp}") #DEBUG: This case shouldn't be reached.
        #
        del changelogIDs[index]
    
# NEW BLOCK
def categories(mysql_cursor, pg_cursor):
    # We want to create 108 any% cateogies for all 108 base-maps
    raise NotImplementedError

def games(mysql_cursor, pg_cursor):
    # We just create game "Portal 2" for now.
    raise NotImplementedError

def users(mysql_cursor, pg_cursor):
    # Keep all user data, add `None` for discord_id 
    raise NotImplementedError

def chapters(mysql_cursor, pg_cursor):
    # Set game_id = 1
    raise NotImplementedError

def maps(mysql_cursor, pg_cursor):
    # Trimmed down, otherwise the same
    raise NotImplementedError

#Demo creation will happen here.
def changelog(mysql_cursor, pg_cursor):
    # `coop_id` & `admin_note` now exists
    # Calculate `score_delta`
    # Invert `pending`
    # Class constructor *should* handle all of this logic for us.
    raise NotImplementedError    
     

main()