<template>
  <div class="changelog-list">
    <li v-for="changelog in changelogs" :key="changelog.id">
      {{ changelog.score }}
    </li>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import Changelog from "@/types/Changelog";

export default defineComponent({
  props: {
    changelogs: {
      type: Array as PropType<Changelog[]>,
      required: true,
    },
  },
  setup(props) {
    console.log(props);
  },
});
</script>

<style scoped></style>
