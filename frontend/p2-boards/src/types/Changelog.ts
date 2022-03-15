interface Changelog {
  id: number;
  timestamp?: string | null; //Date.parse(changelogs[0].timestamp)
  profile_number: string;
  score: number;
  map_id: string;
  demo_id?: number | null;
  banned: boolean;
  youtube_id?: string | null;
  previous_id?: number | null;
  coop_id?: number | null;
  post_rank?: number | null;
  pre_rank?: number | null;
  submission: boolean;
  note?: string | null;
  category_id: number;
  score_delta?: number | null;
  verified?: boolean | null;
  admin_note?: string | null;
  map_name: string;
  user_name: string;
  avatar: string;
}

export default Changelog;
