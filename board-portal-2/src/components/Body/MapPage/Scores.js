import Score from "./Score";

function Scores({ mapData, mapType }) {
  const getName = (item) => {
    if (mapType === "sp") {
      return item.map_data.boardname
        ? item.map_data.boardname
        : item.map_data.steamname;
    } else if (mapType === "coop") {
      if (item.map_data.boardname2 || item.map_data.steamname2) {
        return `${
          item.map_data.boardname1
            ? item.map_data.boardname1
            : item.map_data.steamname1
        } & ${
          item.map_data.boardname2
            ? item.map_data.boardname2
            : item.map_data.steamname2
        }`;
      } else {
        return item.map_data.boardname1
          ? item.map_data.boardname1
          : item.map_data.steamname1;
      }
    }
  };

  const getAvatar = (item) => {
    if (mapType === "sp") {
      return [item.map_data.avatar];
    } else if (mapType === "coop") {
      if (item.map_data.avatar2) {
        return [item.map_data.avatar1, item.map_data.avatar2];
      } else {
        return [item.map_data.avatar1];
      }
    }
  };

  return (
    <>
      {mapData.map((item) => {
        return (
          <Score
            key={`${item.map_data.score}${item.rank}`}
            rank={item.rank}
            avatar={getAvatar(item)}
            name={getName(item)}
            time={item.map_data.time_gained}
            score={item.map_data.score}
          />
        );
      })}
    </>
  );
}

export default Scores;
