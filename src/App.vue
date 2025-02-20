<template>
  <div v-if="menuVisible" class="menu-overlay">
    <div class="menu">
      <h2>控制菜单</h2>
      <p>F1 - 显示/隐藏菜单</p>
      <p>F2 - 显示/隐藏 HUD</p>
      <button @click="exitApp">退出</button>
    </div>

  </div>
  <div v-if="hudVisible" class="hud-container">
    <div class="scoreboard">
      <div class="team">
        <div class="team-name ct">CT</div>
        <div class="team-score ct">{{ ct_score }}</div>
      </div>
      <div class="versus">VS</div>
      <div class="team">
        <div class="team-name t">T</div>
        <div class="team-score t">{{ t_score }}</div>
      </div>
    </div>

    <div class="team ct-team compact">
      <div v-for="player in ct_players" :key="player.name" class="ct_compact-player">
        <span class="player-name">{{ player.name }}</span>
        <span class="player-stats">
          <SvgKill style="width: 16px;"></SvgKill> {{ player.kills }} <SvgDead style="width: 14px;"></SvgDead> {{
            player.deaths }}
        </span>
        <SvgIcon class="ct_weapon" :name='getWeaponIcon(player.active_weapon)'></SvgIcon>
        <div class="health-bar-container">
          <div class="health-bar" :style="{ height: (player.health <= 1 ? 0 : player.health) + '%', backgroundColor: 'red' }"></div>
        </div>
      </div>
    </div>

    <div class="team t-team compact">
      <div v-for="player in t_players" :key="player.name" class="t_compact-player">
        <SvgIcon class="t_weapon" :name='getWeaponIcon(player.active_weapon)'></SvgIcon>
        <span class="player-stats">
          <SvgKill style="width: 16px;"></SvgKill> {{ player.kills }} <SvgDead style="width: 14px;"></SvgDead> {{
            player.deaths }}
        </span>
        <span class="player-name">{{ player.name }}</span>
        <div class="health-bar-container">
          <div class="health-bar" :style="{ height: (player.health <= 1 ? 0 : player.health) + '%', backgroundColor: 'red' }"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import SvgIcon from "@/components/index.vue";
import SvgKill from "./assets/icons/kill.svg";
import SvgDead from "./assets/icons/dead.svg";

export default {
  components: {
    SvgIcon,
    SvgKill,
    SvgDead,
  },
  setup() {
    const menuVisible = ref(true);
    const hudVisible = ref(false);
    const ct_score = ref(0);
    const t_score = ref(0);
    const ct_players = ref([]);
    const t_players = ref([]);

    const getWeaponIcon = (weaponName) => {
      if (!weaponName) return "";
      return `${weaponName.replace(/^weapon_/, "").toLowerCase()}`;
    };

    const toggleHUD = () => {
      hudVisible.value = !hudVisible.value;
      invoke("ignore_cursor", { ignore: hudVisible.value });
    };

    const toggleMenu = () => {
      menuVisible.value = !menuVisible.value;
    };

    const exitApp = () => {
      invoke("exit_app");
    };

    onMounted(() => {
      listen("update_data", (event) => {
        const data = event.payload;
        ct_score.value = data.ct_score;
        t_score.value = data.t_score;
        ct_players.value = data.players.filter((p) => p.team === "CT");
        t_players.value = data.players.filter((p) => p.team === "TERRORIST");
      });

      window.addEventListener("keydown", (event) => {
        if (event.code === "F1") toggleMenu();
        if (event.code === "F2") toggleHUD();
      });
    });

    return {
      menuVisible,
      hudVisible,
      ct_score,
      t_score,
      ct_players,
      t_players,
      exitApp,
      getWeaponIcon,
    };
  },
};
</script>

<style scoped>
.titlebar {
  background-color: rgb(0, 0, 0 / 0%);
}

.menu-overlay {
  position: fixed;
  top: 0;
  left: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  z-index: 1000;
}

body {
  /* background-color: rgba(255, 255, 255, 0); */
  overflow: hidden;
  margin: 0;
  padding: 0;
  background: transparent;
}

.menu {
  background: white;
  padding: 20px;
  border-radius: 10px;
  text-align: center;
}

/* 调整原有样式 */
.hud-container {
  display: block;
  margin-top: 0;
}

html {
  margin: 0;
  padding: 0;
  background: transparent !important;
  overflow: hidden;
}

button {
  margin-top: 10px;
  padding: 10px;
  border: none;
  background: red;
  color: white;
  cursor: pointer;
  border-radius: 5px;
}

.scoreboard {
  position: absolute;
  top: 4%;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(145deg, rgba(15, 15, 15, 0.95), rgba(30, 30, 30, 0.85));
  padding: 10px 30px; /* 减少内边距 */
  border-radius: 12px;
  box-shadow: 0 0 20px rgba(255, 255, 255, 0.15), inset 0 0 8px rgba(255, 255, 255, 0.05);
  backdrop-filter: blur(8px);
  transition: all 0.3s ease-in-out;
}

.team {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 5px 15px;
  width: 240px;
  border-radius: 8px;
  transition: transform 0.2s ease-in-out;
}

.team:hover {
  transform: scale(1.05);
}

.team-name {
  font-size: 1.4rem;
  font-weight: bold;
  text-transform: uppercase;
  margin-bottom: 5px;
  letter-spacing: 1px;
}

.team-score {
  font-size: 3rem;
  font-weight: bold;
  text-shadow: 0 0 15px rgba(255, 255, 255, 0.6), 0 0 25px rgba(255, 255, 255, 0.3);
  animation: pulseGlow 2s infinite alternate ease-in-out;
}

.ct {
  color: #00aaff;
  text-shadow: 0 0 15px rgba(0, 170, 255, 0.8), 0 0 30px rgba(0, 170, 255, 0.5);
}

.t {
  color: #ff9900;
  text-shadow: 0 0 15px rgba(255, 153, 0, 0.8), 0 0 30px rgba(255, 153, 0, 0.5);
}

.versus {
  font-size: 1.8rem;
  font-weight: bold;
  color: #fff;
  text-shadow: 0 0 8px rgba(255, 255, 255, 0.8), 0 0 16px rgba(255, 255, 255, 0.5);
  margin: 0 15px;
}

@keyframes pulseGlow {
  0% {
    text-shadow: 0 0 15px rgba(255, 255, 255, 0.6), 0 0 25px rgba(255, 255, 255, 0.3);
  }
  100% {
    text-shadow: 0 0 20px rgba(255, 255, 255, 1), 0 0 40px rgba(255, 255, 255, 0.5);
  }
}

.team.compact {
  width: 300px;
  padding: 15px;
  margin: 20px;
  background: rgba(0, 0, 0, 0) !important;
}

.ct-team.compact {
  position: absolute;
  left: 1%;
  top: 45%;
}

.t-team.compact {
  position: absolute;
  right: 1%;
  top: 45%;
}

.ct_compact-player {
  width: 300px;
  display: flex;
  justify-content: space-between;
  padding: 12px 0;
  color: white;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  background-color: rgba(0, 170, 255, 0.7);
  margin-bottom: 15px;
  border-radius: 20px; 
  overflow: hidden;
  box-shadow: 0 0 15px rgba(0, 170, 255, 0.8);
}

.t_compact-player {
  width: 300px;
  display: flex;
  justify-content: space-between;
  padding: 10px 0;
  color: white;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  background-color: rgba(255, 153, 0, 0.7);
  margin-bottom: 15px;
  border-radius: 20px;
  overflow: hidden;
  box-shadow: 0 0 15px rgba(255, 153, 0, 0.8);;
}

.player-name {
  width: 100px;
  font-size: 15px;
  font-weight: bold;
  text-transform: uppercase;
  text-shadow: 0 0 15px rgba(255, 107, 243, 0.8);
  margin-bottom: 5px;
  margin: auto;
}

.player-stats {
  width: 50px;
  font-size: 20px;
  margin: auto;
  color: #000000;
}

.ct_weapon {
  margin: auto;
  float: left;
  width: 100px;
  height: 32px;
}

.t_weapon {
  margin: auto;
  float: left;
  width: 100px;
  height: 32px;
}
.health-bar-container {
  width: 12px;
  height: 60px;
  background: linear-gradient(to bottom, rgba(255, 255, 255, 0.3), rgba(0, 0, 0, 0.3));
  border-radius: 6px;
  border: 1px solid rgba(255, 255, 255, 0.5);
  position: relative;
  overflow: hidden;
  box-shadow: 0 0 8px rgba(255, 0, 0, 0.5);
  margin-right:10px;
}

.health-bar {
  width: 100%;
  position: absolute;
  bottom: 0;
  background: linear-gradient(to top, red, darkred);
  transition: height 0.3s ease-in-out;
  border-radius: 6px;
}

</style>