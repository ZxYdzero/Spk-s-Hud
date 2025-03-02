<template>
  <div v-if="menuVisible" class="menu-overlay">
    <div class="menu">
      <h2>控制菜单</h2>
      <p>F1 - 切换菜单和Hud显示</p>

      <!-- 新增连接按钮 -->
      <button @click="connect">连接</button>
      <button @click="disconnect">断开</button>
      <button @click="exitApp">退出</button>
      <!-- 日志显示窗口 -->
      <div class="log-window">
        <h3>日志</h3>
        <div v-for="(log, index) in logs" :key="index" class="log-entry">
          {{ log }}
        </div>
      </div>
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
          <SvgKill style="width: 16px;"></SvgKill> {{ player.kills }} 
          <SvgDead style="width: 14px;"></SvgDead> {{ player.deaths }}
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
          <SvgKill style="width: 16px;"></SvgKill> {{ player.kills }} 
          <SvgDead style="width: 14px;"></SvgDead> {{ player.deaths }}
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
import './style/main.css'

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
    const logs = ref([]);
    const isConnect = ref(false);

    const getWeaponIcon = (weaponName) => {
      if (!weaponName) return "";
      return weaponName.replace(/^weapon_/, "").toLowerCase();
    };

    const toggleMenu = () => {
      hudVisible.value = !hudVisible.value;
      menuVisible.value = !menuVisible.value;
      invoke("ignore_cursor", { ignore: hudVisible.value });
    };
    
    const toggleHud = () => {
      hudVisible.value = !hudVisible.value;
      menuVisible.value = !menuVisible.value;
    }
    const exitApp = () => {
      invoke("exit_app");
    };

    const connect = async () => {
      await invoke("connect");
    };

    const disconnect = async () => {
      await invoke("disconnect");
    };

    onMounted(() => {
      listen("update_data", (event) => {
        const data = event.payload;
        ct_score.value = data.ct_score;
        t_score.value = data.t_score;
        ct_players.value = data.players.filter((p) => p.team === "CT");
        t_players.value = data.players.filter((p) => p.team === "TERRORIST");
      });

      listen("log", (event) => {
        logs.value.push(event.payload);
      });

      window.addEventListener("keydown", (event) => {
        if (event.code === "F1") toggleMenu();
      });
    });

    return {
      menuVisible,
      hudVisible,
      ct_score,
      t_score,
      ct_players,
      t_players,
      logs,
      exitApp,
      getWeaponIcon,
      connect,
      disconnect,
    };
  },
};
</script>
