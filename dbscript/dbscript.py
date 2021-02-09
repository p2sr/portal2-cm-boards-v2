import mysql.connector
changelogIDs = []
coopdict = {}

with open(".secret") as fp:
    mysql_password = fp.read()

class changelog:
    def __init__(self, time_gained,
    profile_number, score, map_id, wr_gain, 
    has_demo, banned, youtube_id, previous_id, 
    id, coopid, post_rank, pre_rank, submission, note):
        self.time_gained = time_gained
        self.profile_number = profile_number
        self.score = score
        self.map_id = map_id
        self.wr_gain = wr_gain
        self.has_demo = has_demo
        self.banned = banned
        self.youtube_id = youtube_id
        self.previous_id = previous_id
        self.id = id
        self.coopid = coopid
        self.post_rank = post_rank
        self.pre_rank = pre_rank
        self.submission = submission
        self.note = note
    def __str__(self):
        return f"{self.id} - {self.time_gained} - {self.profile_number} - {self.score} - {self. note}"
def main():
    mydb = mysql.connector.connect(
        host="localhost",
        user="root",
        password=mysql_password,
        database="testdb"
    )
    mycursor = mydb.cursor()
    testquery = "SELECT * FROM changelog WHERE"
    #Grabs all coop entries from the database
    grabcoop = "SELECT changelog.time_gained, changelog.profile_number, changelog.score, changelog.map_id, changelog.wr_gain, changelog.has_demo, changelog.banned, changelog.youtube_id, changelog.previous_id, changelog.id, changelog.coopid, changelog.post_rank, changelog.pre_rank,changelog.submission, changelog.note FROM changelog LEFT JOIN maps ON maps.steam_id=changelog.map_id WHERE maps.is_coop=1"
    mycursor.execute(grabcoop)
    myresult = mycursor.fetchall()
    # Adds every coop changelog entry into a class object, and inserts it into a dictionary with id as the key
    for i, x in enumerate(myresult):
        temp = changelog(*x)
        coopdict[temp.id] = temp
        changelogIDs.append(temp.id)
    # Create the coop entries
    count = 1
    while len(changelogIDs) != 0:
        value = coopdict[changelogIDs[len(changelogIDs)-1]]
        #print(value.time_gained)
        if value.time_gained != None:
            #value will be the most current object
            querystring = f"SELECT * FROM changelog WHERE time_gained=\"{value.time_gained}\" AND score={value.score} AND map_id={value.map_id}"
            #print(querystring)
            mycursor.execute(querystring)
            myresult = mycursor.fetchall()
            # Both exist, insert both into `coopbundled`
            if len(myresult) == 2:            
                temp = changelog(*myresult[0])
                temp2 = changelog(*myresult[1])
                # String to insert new coopbundled
                #newinsert = f"INSERT INTO coopbundled (time_gained, profile_number1, profile_number2, score, map_id, wr_gain, has_demo1, has_demo2, banned, youtube_id1, youtube_id2, previous_id1, previous_id2, changelogid1, changelogid2, id, post_rank1, post_rank2, pre_rank1, pre_rank2, submission1, submission2, note1, note2) VALUES ('{value.time_gained}', '{temp.profile_number}', '{temp2.profile_number}', {value.score}, '{value.map_id}', {value.wr_gain}, {temp.has_demo}, {temp2.has_demo}, {value.banned}, '{temp.youtube_id}', '{temp2.youtube_id}', {temp.previous_id}, {temp2.previous_id},{temp.id}, {temp2.id}, {temp.previous_id}, {temp2.previous_id}, {count}, {temp.post_rank}, {temp2.post_rank}, {temp.pre_rank}, {temp2.pre_rank}, {temp.submission}, {temp2.submission}, '{temp.note}', {temp2.note})"
                #TODO Fix this dumb shit
                mycursor.execute("INSERT INTO coopbundled (time_gained, profile_number1, profile_number2, score, map_id, wr_gain, has_demo1, has_demo2, banned, youtube_id1, youtube_id2, previous_id1, previous_id2, changelogid1, changelogid2, id, post_rank1, post_rank2, pre_rank1, pre_rank2, submission1, submission2, note1, note2) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)", (value.time_gained, temp.profile_number, temp2.profile_number, value.score, value.map_id, value.wr_gain, temp.has_demo, temp2.has_demo, value.banned, temp.youtube_id, temp2.youtube_id, temp.previous_id, temp2.previous_id, temp.id, temp2.id, temp.previous_id, temp2.previous_id, count, temp.post_rank, temp2.post_rank, temp.pre_rank, temp2.pre_rank, temp.submission, temp2.submission, temp.note, temp2.note,))
                # Update the coopids for both changelog entities
                update1 = f"UPDATE changelog SET coopid={count} WHERE id={temp.id}"
                update2 = f"UPDATE changelog SET coopid={count} WHERE id={temp2.id}"
                mycursor.execute(update1)
                mycursor.execute(update2)
                count = count + 1
                # Pop both from the list of IDS
                changelogIDs.remove(temp.id)
                changelogIDs.remove(temp2.id)
            # Only one exists, create new changelog entity, insert it and the existing one into the coop bundle.
            else:
                #print(myresult[0])
                temp1 = changelog(*myresult[0])
                blankcl = f"INSERT INTO changelog (time_gained, score, map_id, wr_gain, banned, submission, note) VALUES ('{temp.time_gained}', {temp.score}, '{temp.map_id}', {temp.wr_gain}, {temp.banned}, 1, 'TMP_COOP_PLACEHOLDER')"
                mycursor.execute(blankcl)
                newid = mycursor.lastrowid
                print(newid)
                #newinsert = f"INSERT INTO coopbundled (time_gained, profile_number1, score, map_id, wr_gain, has_demo1, banned, youtube_id1, previous_id1, changelogid1, changelogid2, id, post_rank1,  pre_rank1, submission1, note1) VALUES ('{value.time_gained}', '{value.profile_number}', {value.score}, '{value.map_id}', {value.wr_gain}, {value.has_demo}, {value.banned}, '{value.youtube_id}', {temp.previous_id}, {value.id}, {newid}, {count}, {value.post_rank}, {value.pre_rank}, {value.submission}, '{value.note}')"
                #mycursor.execute("INSERT INTO coopbundled (time_gained, profile_number1, score, map_id, wr_gain, has_demo1, banned, youtube_id1, previous_id1, changelogid1, changelogid2, id, post_rank1, pre_rank1, submission1, note1) VALUES ('%s', '%s', %s, '%s', %s, %s, %s, '%s', %s, %s, %s, %s, %s, %s, %s, '%s')", (value.time_gained, value.profile_number, value.score, value.map_id, value.wr_gain, value.has_demo, value.banned, value.youtube_id, temp.previous_id, value.id, newid, count, value.post_rank, value.pre_rank, value.submission, value.note,))
                mycursor.execute("INSERT INTO coopbundled (time_gained, profile_number1, score, map_id, wr_gain, has_demo1, banned, youtube_id1, previous_id1, changelogid1, changelogid2, id, post_rank1, pre_rank1, submission1, note1) VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s, %s)", (value.time_gained, value.profile_number, value.score, value.map_id, value.wr_gain, value.has_demo, value.banned, value.youtube_id, temp.previous_id, value.id, newid, count, value.post_rank, value.pre_rank, value.submission, value.note))
                update1 = f"UPDATE changelog SET coopid={count} WHERE id={value.id}"
                update2 = f"UPDATE changelog SET coopid={count} WHERE id={newid}"
                mycursor.execute(update1)
                mycursor.execute(update2)
                changelogIDs.remove(temp1.id)
                count = count + 1
        #TODO handle None timestamps
        else:
            print("Ignoring NULL time")

    print("Finished coupling")
    #for ind in coopdict:
    #    if coopdict[ind].note != None and "fuck" in coopdict[ind].note:
    #        print(coopdict[ind].note)
    #for i, y in enumerate(changelogList):
    #    print(y.note)
    #print(i)
main()
#TODO Fix key structure on coopbundled
# Step 3, Go through dictionary, grab the ID, Datestamp, score and map
# Step 4, Query based on those attributes, find any matches
# IF TWO MATCHES
    # Step 5, Grab both IDS, insert their info into a new bundled coop
    # Step 6, Take the bundled coop ID, and add it to both changelogs.
    # Step 7, Take both changelog ids, remove from dictionary.
# IF ONLY THE ONE TIME
    # Step 8, Create new changelog for a blank, matching time.
# RETURN TO STEP 5
'''done'''
# Step 1, Add a new column to changelog that holds the coop time.
# Step 2, Keep a dictionary of all coop times (key=ID)

'''
individual coop time 1 cooptableID
individual coop time 1 cooptableID
individual coop time 2 
individual coop time 3 

new table for combined coop:
partnered-coop-time uuid1 uuid2 comm1 comm1 yt1 yt2 dem1 dem2 booleanforblue

# Algorithm steps
New coop time comes in, same date stamp, map and score. Bundle together in new coop table with relivant info.
Use the coop table to compute the rankings, update the rankings on the changelog appropriately

individual coop time where only one partner PBs cooptableID
filler changelog entity with empty user cooptableID


INSERT INTO `maps` (`id`, `steam_id`, `lp_id`, `name`, `type`, `chapter_id`, `is_coop`, `is_public`) VALUES
(5, '45467', '45466', 'Laser Crusher', 'time', 1, 1, 1),
(6, '46362', '46361', 'Behind the Scenes', 'time', 1, 1, 1),
'''