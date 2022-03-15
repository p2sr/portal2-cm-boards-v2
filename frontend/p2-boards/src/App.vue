<template>
  <div class="app">
    <!-- <li v-for="changelog in changelogs" :key="changelog.id">
      {{ changelog.score }}
    </li> -->
    <ChangelogList :changelogs="changelogs" />
  </div>
</template>

<script lang="ts">
import { defineComponent, ref } from "vue";
import Changelog from "@/types/Changelog";
import ChangelogList from "@/components/ChangelogList.vue";

export default defineComponent({
  name: "App",
  compontents: { ChangelogList },
  setup() {
    const changelogs = ref<Changelog[]>([
      {
        id: 157804,
        timestamp: "2021-08-25T10:54:45",
        profile_number: "76561198295368421",
        score: 830,
        map_id: "49345",
        demo_id: 27458,
        banned: false,
        youtube_id: null,
        previous_id: 141620,
        coop_id: 1,
        post_rank: 8,
        pre_rank: 10,
        submission: false,
        note: null,
        category_id: 102,
        score_delta: -3,
        verified: true,
        admin_note: null,
        map_name: "Catapult Catch",
        user_name: "bexc",
        avatar:
          "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/ca/ca863ba646ec545eaf29f7d6b7b6c5fcd7f74d99_full.jpg",
      },
      {
        id: 157803,
        timestamp: "2021-08-25T10:49:29",
        profile_number: "76561198295368421",
        score: 806,
        map_id: "52671",
        demo_id: 27457,
        banned: false,
        youtube_id: null,
        previous_id: 154713,
        coop_id: 2,
        post_rank: 8,
        pre_rank: 8,
        submission: false,
        note: null,
        category_id: 87,
        score_delta: -2,
        verified: true,
        admin_note: null,
        map_name: "Cooperative Polarity",
        user_name: "bexc",
        avatar:
          "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/ca/ca863ba646ec545eaf29f7d6b7b6c5fcd7f74d99_full.jpg",
      },
      {
        id: 157802,
        timestamp: "2021-08-25T10:47:52",
        profile_number: "76561198084978888",
        score: 1253,
        map_id: "47746",
        demo_id: null,
        banned: false,
        youtube_id: null,
        previous_id: null,
        coop_id: null,
        post_rank: 196,
        pre_rank: null,
        submission: false,
        note: null,
        category_id: 11,
        score_delta: null,
        verified: true,
        admin_note: null,
        map_name: "Fizzler Intro",
        user_name: "Herneerius",
        avatar:
          "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/48/4879fa1c72770b2c0eaef341dc6546db13897bb5_full.jpg",
      },
      {
        id: 157801,
        timestamp: "2021-08-25T10:43:15",
        profile_number: "76561198323090545",
        score: 2023,
        map_id: "47759",
        demo_id: 27456,
        banned: false,
        youtube_id: null,
        previous_id: 157719,
        coop_id: null,
        post_rank: 25,
        pre_rank: 28,
        submission: false,
        note: null,
        category_id: 17,
        score_delta: -6,
        verified: true,
        admin_note: null,
        map_name: "Laser Relays",
        user_name: "skz",
        avatar:
          "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/24/2438fed1aef9de23e547f256f66a91615e7774be_full.jpg",
      },
      {
        id: 157800,
        timestamp: "2021-08-25T10:38:03",
        profile_number: "76561197998253566",
        score: 1256,
        map_id: "47746",
        demo_id: null,
        banned: false,
        youtube_id: null,
        previous_id: null,
        coop_id: null,
        post_rank: 199,
        pre_rank: null,
        submission: false,
        note: null,
        category_id: 11,
        score_delta: null,
        verified: true,
        admin_note: null,
        map_name: "Fizzler Intro",
        user_name: "Saxton Hale",
        avatar:
          "https://steamcdn-a.akamaihd.net/steamcommunity/public/images/avatars/2f/2f335962fbb8431ea3f5606fe3e1881dce47e63a_full.jpg",
      },
    ]);
    return { changelogs };
  },
  methods: {},
});
</script>
