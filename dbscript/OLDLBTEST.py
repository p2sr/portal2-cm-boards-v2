import json
import os

# Add isObsolete?
class Time:
    def __init__(self, ID = None, note = None,
    submission = None, changelogID = None,
    playerRank = None, scoreRank = None, 
    score = None, date = None,
    hasDemo = False, youtubeID = None,
    boardname = None, avatar = None,
    mapname = None, gm = "Portal 2",
    pfn = None, wr = False, cid = None,
    mid = None, preR = None, imp = None,
    rImp = None, preP = None, postP = None,
    impP = None, pscore = None, catID = None, 
    cat = "Any%", ban = False, demoID = None):
        self.runID = ID                         # Unique ID for the run                 (int)      
        self.changelogID = changelogID          # Sequential identifier based off age   (int)
        self.profile_number = pfn               # Profile # associated with SteamID     (int)
        self.submission = submission            # Steam or user submission              (boolean)
        self.wr_gain = wr                       # Was this time WR?                     (boolean)    
        self.boardname = boardname              # Player's name on the board            (string)
        self.avatar = avatar                    # URL for player's profile picture      (string)
        self.chapterID = cid                    # ID of the given chapter               (int)
        self.mapID = mid                        # Associated with SteamLB               (int)

        self.playerRank = playerRank            # Rank of the player                    (int)
        self.pre_rank = preR                    # Rank of your old time                 (int)
        self.post_rank = scoreRank              # Rank of yout new time                 (int)
        self.improvement = imp                  # Score improved by ms                  (int)
        self.rank_improvement = rImp            # Num of ranks improved                 (int)
        self.pre_points = postP                 # Num of points with old time           (float)
        self.post_points = preP                 # Num of points with new time           (float)
        self.point_improvement = impP           # Point delta                           (float)

        # TODO Move to subclass
        self.hasDemo = hasDemo                  # If a demo exists.                     (boolean)          

        # TODO Decide on stored score format
        # Runs a script to change score formatting
        if(score != None):
            self.score = self.scoreFormatting(score)# Score of the time                     (int)       # TODO keep as int?
        self.previous_score = pscore            # Old score                             (int)       # TODO keep as int?  

        # User or admin mutable
        self.comment = note                     # User created comments                 (string)
        self.date = date                        # Date of the time                      (string)
        self.categoryID = catID                 # Category ID                           (int)
        self.category = cat                     # Category name                         (string)
        self.banned = ban                       # Is this time banned?                  (boolean)
        self.chamberName = mapname              # Map name                              (string)
        self.gamemode = gm                      # Game mode                             (string)
        # TODO Move to subclasses
        self.youtubeID = youtubeID              # Youtube URL watch string              (string)
        self.demoID = demoID                    # ID of the demo                        (string)    # TODO keep as int?

    # Debug string representation for output checks
    def __str__(self):
        return f"{self.boardname} - {self.score} - {self.category}"
    # Debug string representation for output checks
    def __repr__(self):
        return f"{self.boardname} - {self.score} - {self.category}"
    def scoreFormatting(self, score):
        if(len(score) == 5):
            score = score[:3] + '.' + score[3:]
            score = score[:1] + ':' + score[1:]            
        elif(len(score) == 4):
            if(int(score[0]) >= 6):
                score = (int(score)-6000)+10000
                score = str(score)[:3] + '.' + str(score)[3:]
                score = score[:1] + ':' + score[1:]
            else:
                score = score[:2] + '.' + score[2:]
        elif(len(score) == 3):
            score = score[:1] + '.' + score[1:]
        return score

class SpTime(Time):
    def __init__(self, ID = None, note = None,
    submission = None, changelogID = None,
    playerRank = None, scoreRank = None, 
    score = None, date = None,
    hasDemo = False, youtubeID = None,
    boardname = None, avatar = None,
    mapname = None, gm = "Portal 2",
    pfn = None, wr = False, cid = None,
    mid = None, preR = None, imp = None,
    rImp = None, preP = None, postP = None,
    impP = None, pscore = None, catID = None, 
    cat = "Any%", ban = False, demoID = None):
        super().__init__(ID, note, submission,
        changelogID, playerRank, scoreRank, score, 
        date, hasDemo, youtubeID, boardname, avatar,
        mapname, gm, pfn, wr, cid, mid, preR, imp, 
        rImp, preP, postP, impP, pscore, catID, cat, 
        ban, demoID)
        self.isCoop = False             # Keeps the status  (boolean)

class CoopTime(Time):
    def __init__(self, ID = None, note = None,
    submission = None, changelogID = None,
    playerRank = None, scoreRank = None, 
    score = None, date = None,
    hasDemo = False, youtubeID = None,
    boardname = None, avatar = None,
    mapname = None, gm = "Portal 2",
    pfn = None, wr = False, cid = None,
    mid = None, preR = None, imp = None,
    rImp = None, preP = None, postP = None,
    impP = None, pscore = None, catID = None, 
    cat = "Any%", ban = False, demoID = None,
    partnerID = None, IDPair = None, partnerDemo = None):
        super().__init__(ID, note, submission,
        changelogID, playerRank, scoreRank, score, 
        date, hasDemo, youtubeID, boardname, avatar,
        mapname, gm, pfn, wr, cid, mid, preR, imp, 
        rImp, preP, postP, impP, pscore, catID, cat, 
        ban, demoID)
        self.isCoop = True              # Keeps the status  (boolean)

def getChapterName(chapterID):
    chapterID = int(chapterID)
    if(chapterID == 1):
        return "Team Building"
    elif(chapterID == 2):
        return "Mass and Velocity"
    elif(chapterID == 3):
        return "Hard Light"
    elif(chapterID == 4):
        return "Excursion Funnels"
    elif(chapterID == 5):
        return "Mobility Gels"
    elif(chapterID == 6):
        return "Art Therapy"
    elif(chapterID == 7):
        return "The Courtesy Call"
    elif(chapterID == 8):
        return "The Cold Boot"
    elif(chapterID == 9):
        return "The Return"
    elif(chapterID == 10):
        return "The Surprise"
    elif(chapterID == 11):
        return "The Escape"
    elif(chapterID == 12):
        return "The Fall"
    elif(chapterID == 13):
        return "The Reunion"
    elif(chapterID == 14):
        return "The Itech"
    elif(chapterID == 15):
        return "The Part Where..."

def parseMapPageJSON(filename = "portalgun.json", mapName = "Portal Gun", gm = "SP"):
    runIDs = []
    runs = []
    with open(filename) as fp:
        data = json.load(fp)
    for ID in data:
        runIDs.append(ID)
    for runID in runIDs:
        temp = Time(ID = runID, 
        note = data[runID]["scoreData"]['note'], 
        submission = data[runID]["scoreData"]['submission'], 
        changelogID = data[runID]["scoreData"]['changelogId'], 
        playerRank = data[runID]["scoreData"]['playerRank'],
        scoreRank = data[runID]["scoreData"]['scoreRank'],
        score = data[runID]["scoreData"]['score'],
        date = data[runID]["scoreData"]['date'],
        hasDemo = data[runID]["scoreData"]['hasDemo'],
        youtubeID = data[runID]["scoreData"]['youtubeID'],
        boardname = data[runID]["userData"]['boardname'],
        avatar = data[runID]["userData"]['avatar'],
        mapname = mapName,
        gm = gm)
        runs.append(temp)
    return runs

def parseChangelogJSON(filename = "changelog.json"):
    runs = []
    with open(filename) as fp:
        data = json.load(fp)
    for count, dictionary in enumerate(data):
        chapterID = dictionary["chapterId"]
        if int(chapterID) > 6:
            temp = createSP(dictionary)
        else:
            temp = createCoop(dictionary)
        runs.append(temp)
    return runs

def createSP(dictionary):
    temp = SpTime(boardname = dictionary["player_name"],
        avatar = dictionary["avatar"],
        pfn = dictionary["profile_number"],
        score = dictionary["score"],
        changelogID = dictionary["id"],
        preR = dictionary["pre_rank"],
        playerRank = dictionary["post_rank"],
        wr = dictionary["wr_gain"],
        date = dictionary["time_gained"],
        hasDemo = dictionary["hasDemo"],
        youtubeID = dictionary["youtubeID"],
        note = dictionary["note"],
        ban = dictionary["banned"],
        submission = dictionary["submission"],
        pscore = dictionary["previous_score"],
        mapname = dictionary["chamberName"],
        cid = dictionary["chapterId"],
        mid = dictionary["mapid"],
        imp = dictionary["improvement"],
        rImp = dictionary["rank_improvement"],
        preP = dictionary["pre_points"],
        postP = dictionary["post_point"],
        impP = dictionary["point_improvement"],)
    return temp

def createCoop(dictionary):
    temp = CoopTime(boardname = dictionary["player_name"],
        avatar = dictionary["avatar"],
        pfn = dictionary["profile_number"],
        score = dictionary["score"],
        changelogID = dictionary["id"],
        preR = dictionary["pre_rank"],
        playerRank = dictionary["post_rank"],
        wr = dictionary["wr_gain"],
        date = dictionary["time_gained"],
        hasDemo = dictionary["hasDemo"],
        youtubeID = dictionary["youtubeID"],
        note = dictionary["note"],
        ban = dictionary["banned"],
        submission = dictionary["submission"],
        pscore = dictionary["previous_score"],
        mapname = dictionary["chamberName"],
        cid = dictionary["chapterId"],
        mid = dictionary["mapid"],
        imp = dictionary["improvement"],
        rImp = dictionary["rank_improvement"],
        preP = dictionary["pre_points"],
        postP = dictionary["post_point"],
        impP = dictionary["point_improvement"])
    return temp

def main():
    os.system("cls")
    clData = parseChangelogJSON(filename = "changelog.1.17.2021.json")
    #mapData = parseMapPageJSON("lasercrusher.json", "Laser Crusher", "Coop")
    #print(f'''\nMap: {mapData[0].chamberName}\n\nGameMode: {mapData[0].gamemode}\n\nRunner Name - Time  - Category\n''')
    #for run in mapData:
    #    print(run)
    fp = open("changelog1.10.2021.txt", "w")
    fp1 = open("Admin_Alerts.txt", "w")
    fp2 = open("NotReal.txt", "w")
    for run in clData:
        if run.isCoop == True:
            coopStatus = "Coop"
        else:
            coopStatus = "SP"
        try:
            if int(run.playerRank) <= 40:
                if(int(run.hasDemo) == 1 or run.youtubeID != None):
                    try:
                        fp.write(f"{run.boardname} - {run.score} - {run.chamberName} - {coopStatus}\n")
                    except:
                        fp.write(f"dumbassname - {run.score} - {run.chamberName} - {coopStatus}\n")
                else:
                    try:
                        fp1.write(f"{run.boardname} - {run.score} - {run.chamberName} - {coopStatus} - NEEDS DEMO OR VIDEO PROOF\n")
                    except:
                        fp1.write(f"dumbassname - {run.score} - {run.chamberName} - {coopStatus} - NEEDS DEMO OR VIDEO PROOF\n")
        except:
            fp2.write(f"{run.boardname} - {run.score} - {run.chamberName} - {run.playerRank} - Not top 200\n")

main()


#newinsert = "INSERT INTO coopbundled (time_gained, profile_number1, score, map_id, wr_gain, has_demo1, banned, youtube_id1, previous_id1, changelogid1, changelogid2, id, post_rank1, pre_rank1, submission1, note1) VALUES ('%s', '%s', %s, '%s', %s, %s, %s, '%s', %s, %s, %s, %s, %s, %s, %s, '%s')" % (value.time_gained, value.profile_number, value.score, value.map_id, value.wr_gain, value.has_demo, value.banned, value.youtube_id, temp.previous_id, value.id, newid, count, value.post_rank, value.pre_rank, value.submission, value.note)
