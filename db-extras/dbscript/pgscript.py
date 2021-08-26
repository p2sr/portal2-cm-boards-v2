from re import L
import mysql.connector
from mysql.connector.errors import Error
import psycopg2

changelogIDs = []
coopdict = {}

num_demos = 1
all_demo_objects = []

category_values = {}
map_name = {}

sp_map_ids = [
    47458,47455,47452,47106,47735,47736,47738,47742,
    47744,47465,47746,47748,47751,47752,47755,47756,
    47759,47760,47763,47764,47766,47768,47770,47773,
    47774,47776,47779,47780,47783,47784,47787,47468,
    47469,47472,47791,47793,47795,47798,47800,47802,
    47804,47806,47808,47811,47813,47815,47817,47819,
    47821,47824,47456,62761,62758,62763,62759,62765,
    62767,62771,88350,62776
    ]

coop_map_ids = [
    47741,47825,47828,47829,45467,46362,47831,47833,
    47835,47837,47840,47841,47844,47845,47848,47849,
    47854,47856,47858,47861,52642,52660,52662,52663,
    52665,52667,52671,52687,52689,52691,52777,52694,
    52711,52714,52715,52717,52735,52738,52740,49341,
    49343,49345,49347,49349,49351,52757,52759,48287
    ]
  
def get_bool(val):
    if val == 1:
        return True
    elif val == 0:
        return False
    else:
        raise Error

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
        self.is_multiplayer = get_bool(is_multiplayer)
    
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
        self.is_public = get_bool(is_public)
    
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
        self.banned = get_bool(banned)
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
    youtube, title, admin, donation_amount, discord_id):
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
        self.discord_id = discord_id
    
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

class PgSQLChangelog:
    def __init__(self, time_gained,
    profile_number, score, map_id, 
    has_demo, banned, youtube_id, previous_id, 
    id_, coopid, post_rank, pre_rank, submission, note, pending):
        self.verified = self.is_verified(pending)
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
        self.admin_note = None

    def __str__(self):
        return f"{self.id} - {self.time_gained} - {self.profile_number} - {self.score} - {self.note}"

    def make_new_demo_id(self, has_demo):
        if has_demo == 0:
            return None
        elif has_demo == 1:
            # The drive_url is a combination of map name, score, profile_number and the demo_id
            # demo_id is serial, but we want to work around a weird issue with psycopg2 not resetting the serial.
            map_name_temp = map_name.get(self.map_id).replace(" ", "")
            drive_url = f"{map_name_temp}_{self.score}_{self.profile_number}_{num_demos}.dem"
            temp = PgSQLDemos(num_demos, drive_url, None, self.is_verified, "", self.id)
            all_demo_objects.append(temp)
            num_demos += 1
            return num_demos-1
    
    def get_category_id(self, map_id):
        return category_values[int(map_id)]

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
    def __init__(self, id_, p_id1, p_id2, p1_is_host, cl_id1, cl_id2):
        self.id = id_
        self.p_id1 = p_id1
        self.p_id2 = p_id2
        self.p1_is_host = p1_is_host 
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
    categories(pg_cursor)
    games(pg_cursor)
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
            pg_cursor.execute("""INSERT INTO 
                \"p2boards\".coop_bundled 
                (p_id1, p_id2, p1_id_host, cl_id1, cl_id2) 
                VALUES (%s, %s, %s, %s, %s);""",
                (temp.profile_number, temp2.profile_number, None, temp.id, temp2.id))
            # TODO: Update both changelog entries to have a coop_bundled ID
            # TODO: Figure out if this works
            result = pg_cursor.fetchone()
            result = PgSQLCoopBundled(*result)
            print(result)
            pg_cursor.execute("""
            
            """)
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
def categories(pg_cursor):
    # We want to create 108 any% cateogies for all 108 base-maps
    id_ = 1
    for map in sp_map_ids:
        pg_cursor.execute("""INSERT INTO
            \"p2boards\".categories
            (id, name, map_id)
            VALUES (%s, %s, %s);""",
            (id_, "any%", map))
        category_values[map] = id_
        id_ += 1
    for map in coop_map_ids:
        pg_cursor.execute("""INSERT INTO
            \"p2boards\".categories
            (id, name, map_id)
            VALUES (%s, %s, %s);""",
            (id_, "any%", map))
        category_values[map] = id_
        id_ += 1
    #pg_cursor.execute("""SELECT * FROM
    #    \"p2boards\".categories""")
    #print(pg_cursor.fetchall())

def games(pg_cursor):
    # We just create game "Portal 2" for now.
    pg_cursor.execute("""INSERT INTO \"p2boards\".games (id, game_name) VALUES (%s, %s);""",(1, "Portal 2"))
    # pg_cursor.execute("""SELECT * FROM
    #     \"p2boards\".games""")
    # print(pg_cursor.fetchall())

def users(mysql_cursor, pg_cursor):
    # Keep all user data, add `None` for discord_id 
    mysql_cursor.execute("SELECT * FROM usersnew")
    all_users = mysql_cursor.fetchall()
    all_users_object = []
    for user in all_users:
        temp = MySQLUsersnew(*user)
        all_users_object.append(temp)
    for user in all_users_object:
        pg_cursor.execute("""INSERT INTO
            \"p2boards\".users
            (profile_number, board_name, steam_name, 
            banned, registered, avatar, twitch, youtube, 
            title, admin, donation_amount, discord_id)
            VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s);""",
            (user.profile_number, user.boardname, user.steamname, 
            user.banned, user.registered, user.avatar, user.twitch,
            user.youtube, user.title, user.admin, user.donation_amount,
            None))
    # pg_cursor.execute("""SELECT * FROM
    #     \"p2boards\".users""")
    # print(pg_cursor.fetchall())    

def chapters(mysql_cursor, pg_cursor):
    # Set game_id = 1
    mysql_cursor.execute("SELECT * FROM chapters")
    all_chapters = mysql_cursor.fetchall()
    all_chapters_object = []
    for chapter in all_chapters:
        temp = MySQLChapter(*chapter)
        all_chapters_object.append(temp)
    for chapter in all_chapters_object:
        pg_cursor.execute("""INSERT INTO
            \"p2boards\".chapters
            (id, chapter_name, is_multiplayer, game_id)
            VALUES (%s, %s, %s, %s);""",
            (chapter.id, chapter.chapter_name, chapter.is_multiplayer, 1)) 
            #This should be the ID of game, which should be 1          ^
    # pg_cursor.execute("""SELECT * FROM
    #     \"p2boards\".chapters""")
    # print(pg_cursor.fetchall())   

def maps(mysql_cursor, pg_cursor):
    # Trimmed down, otherwise the same
    mysql_cursor.execute("SELECT * FROM maps")
    all_maps = mysql_cursor.fetchall()
    all_maps_object = []
    for map in all_maps:
        temp = MySQLMap(*map)
        all_maps_object.append(temp)
        # Add map_id & name to dictionary for later use
        map_name[temp.id] = temp.name
    for map in all_maps_object:
        pg_cursor.execute("""INSERT INTO
            \"p2boards\".maps
            (id, steam_id, lp_id, name, chapter_id, is_public)
            VALUES (%s, %s, %s, %s, %s, %s);""",
            (map.id, map.steam_id, map.lp_id, map.name, map.chapter_id, map.is_public)) 
    # pg_cursor.execute("""SELECT * FROM
    #     \"p2boards\".maps""")
    # print(pg_cursor.fetchall())   

#Demo creation will happen here.
def changelog(mysql_cursor, pg_cursor):
    # `coop_id` & `admin_note` now exists
    # Calculate `score_delta`
    # Invert `pending`
    # Class constructor *should* handle all of this logic for us.
    mysql_cursor.execute("SELECT * FROM changelog")
    all_changelogs = mysql_cursor.fetchall()
    all_changelogs_object = []
    for changelog in all_changelogs:
        temp = MySQLChangelog(*changelog)
        temp = PgSQLChangelog(temp.time_gained, temp.profile_number, temp.score, temp.map_id,
            temp.has_demo, temp.banned, temp.youtube_id, temp.previous_id, temp.id, None, 
            temp.post_rank, temp.pre_rank, temp.submission, temp.note, temp.pending)
        all_changelogs_object.append(temp)
    for changelog in all_changelogs_object:
        pg_cursor.execute("""INSERT INTO
            \"p2boards\".changelog
            ()
            VALUES (%s, %s, %s, %s, %s, %s);""",
            ())      

main()