import Score from "./Score";

function Scores({ mapData, mapType }) {
  const getName = (item) => {
    if (mapType === "sp") {
      return item.map_data.user_name
    } else if (mapType === "coop") {
      if (item.map_data.user_name2) {
        return `${item.map_data.user_name1} & ${item.map_data.user_name2}`;
      } else {
        return item.map_data.user_name1
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
