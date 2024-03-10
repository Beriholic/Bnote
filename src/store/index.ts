import { defineStore } from "pinia";
import { ref } from "vue";
import { useTheme } from "vuetify";

export const useThemeStore = defineStore(
  "theme",
  () => {
    const theme_mode = ref("classic");

    const theme = useTheme();
    const toggleTheme = () => {
      theme_mode.value = theme_mode.value === "classic" ? "dark" : "classic";
      theme.global.name.value =
        theme_mode.value === "classic" ? "light" : "dark";
    };

    return {
      theme_mode,
      toggleTheme,
    };
  },
  {
    persist: true,
  }
);
