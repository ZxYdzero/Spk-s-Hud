<template>
  <!-- 通知系统 -->
  <div class="notifications-container">
    <TransitionGroup name="notification">
      <div v-for="notification in notifications" :key="notification.id" class="notification-box"
        @click="showErrorDetails(notification)">
        <div class="notification-header">
          <div class="notification-icon">
            <svg viewBox="0 0 24 24">
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z" />
            </svg>
          </div>
          <span class="notification-title">系统通知</span>
          <span class="notification-time">{{ notification.time }}</span>
        </div>
        <div class="notification-content">
          {{ notification.message }}
        </div>
        <div class="notification-progress">
          <div class="notification-progress-bar" :style="{ width: `${notification.progress}%` }"></div>
        </div>
      </div>
    </TransitionGroup>
  </div>

  <!-- 错误详情模态框 -->
  <Transition name="modal">
    <div v-if="showErrorModal" class="modal-overlay" @click="closeErrorModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h3>错误详情</h3>
          <button class="modal-close" @click="closeErrorModal">×</button>
        </div>
        <div class="modal-body">
          <div class="error-info">
            <div class="error-time">
              发生时间: {{ selectedError?.time }}
            </div>
            <div class="error-message">
              {{ selectedError?.message }}
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>

  <!-- 控制菜单 -->
  <Transition name="menu">
    <div v-if="menuVisible" class="menu-container">
      <div class="menu-content">
        <!-- 菜单头部 -->
        <div class="menu-header">
          <div class="menu-title">
            <h1>Spk's Hud</h1>
            <div class="version">v0.2.1</div>
          </div>
          <div class="menu-subtitle">
            <div class="user-info">
              <div class="user-avatar">{{ userInitials }}</div>
              <div class="user-name">{{ username }}</div>
            </div>
          </div>
        </div>

        <!-- 快捷键提示 -->
        <div class="shortcut-info">
          <div class="shortcut-key">F1</div>
          <span>切换 HUD/菜单</span>
        </div>

        <!-- 操作按钮组 -->
        <div class="action-buttons">
          <button class="action-button primary" :class="{ disabled: isConnected }" @click="connect">
            <svg viewBox="0 0 24 24">
              <path
                d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z" />
            </svg>
            <span>连接服务器</span>
          </button>

          <button class="action-button warning" :class="{ disabled: !isConnected }" @click="disconnect">
            <svg viewBox="0 0 24 24">
              <path
                d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" />
            </svg>
            <span>断开连接</span>
          </button>

          <button class="action-button danger" @click="exitApp">
            <svg viewBox="0 0 24 24">
              <path
                d="M10.09 15.59L11.5 17l5-5-5-5-1.41 1.41L12.67 11H3v2h9.67l-2.58 2.59zM19 3H5c-1.11 0-2 .9-2 2v4h2V5h14v14H5v-4H3v4c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z" />
            </svg>
            <span>退出程序</span>
          </button>
        </div>

        <!-- 系统日志 -->
        <div class="log-section">
          <div class="log-header">
            <h3>系统日志</h3>
            <span class="log-count">{{ logs.length }} 条记录</span>
          </div>
          <div class="log-container">
            <div class="time-markers">
              <div class="marker" v-for="i in markerCount" :key="i"></div>
            </div>
            <div class="logs-wrapper">
              <div v-for="(log, index) in logs" :key="index" class="log-entry">
                <span class="log-time">{{ formatTime(new Date()) }}</span>
                <span class="log-text">{{ log }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Transition>

  <!-- HUD 界面 -->
  <div v-if="hudVisible" class="hud-container">
    <!-- Scoreboard -->
    <!-- Scoreboard -->
    <div class="scoreboard">
      <div class="team ct" :class="{ 'team-leading': ct_score > t_score }">
        <div class="team-name">CT</div>
        <div class="team-score">{{ ct_score }}</div>
      </div>
      <div class="versus">VS</div>
      <div class="team t" :class="{ 'team-leading': t_score > ct_score }">
        <div class="team-name">T</div>
        <div class="team-score">{{ t_score }}</div>
      </div>
      <div class="score-wave" :class="{
        'ct-leading': ct_score > t_score,
        't-leading': t_score > ct_score
      }"></div>
    </div>
    <!-- 玩家卡片部分的模板 -->
    <!-- CT Players -->
    <div class="team-players ct-team">
      <div v-for="player in ct_players" :key="player.name" class="player-card ct">
        <div class="card-content">
          <!-- 玩家基本信息 -->
          <div class="player-section">
            <div class="player-name">{{ player.name }}</div>
            <div class="player-stats">
              <div class="stat">
                <SvgKill class="stat-icon" />
                <span>{{ player.kills }}</span>
              </div>
              <div class="stat">
                <SvgDead class="stat-icon" />
                <span>{{ player.deaths }}</span>
              </div>
            </div>
          </div>

          <!-- 武器和血条 -->
          <div class="status-section">
            <div class="weapon-container">
              <SvgIcon v-if="player.active_weapon" :name="getWeaponIcon(player.active_weapon)" class="weapon-icon" />
            </div>
            <div class="health-bar">
              <div class="health-fill" :style="{
                width: `${!player.active_weapon || player.active_weapon === 'none' ? 0 : player.health}%`,
                backgroundColor: getHealthColor(player.health)
              }" />
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- T Players -->
    <div class="team-players t-team">
      <div v-for="player in t_players" :key="player.name" class="player-card t">
        <div class="card-content">
          <!-- 玩家基本信息 -->
          <div class="player-section">
            <div class="player-name">{{ player.name }}</div>
            <div class="player-stats">
              <div class="stat">
                <SvgKill class="stat-icon" />
                <span>{{ player.kills }}</span>
              </div>
              <div class="stat">
                <SvgDead class="stat-icon" />
                <span>{{ player.deaths }}</span>
              </div>
            </div>
          </div>

          <!-- 武器和血条 -->
          <div class="status-section">
            <div class="weapon-container">
              <SvgIcon v-if="player.active_weapon" :name="getWeaponIcon(player.active_weapon)" class="weapon-icon" />
            </div>
            <div class="health-bar">
              <div class="health-fill" :style="{
                width: `${!player.active_weapon || player.active_weapon === 'none' ? 0 : player.health}%`,
                backgroundColor: getHealthColor(player.health)
              }" />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/tauri";
import SvgIcon from "@/components/index.vue";
import SvgKill from "./assets/icons/kill.svg";
import SvgDead from "./assets/icons/dead.svg";
import './style/common.css';
import './style/notifications.css';
import './style/menu.css';
import './style/hud.css';

export default {
  name: 'App',
  components: {
    SvgIcon,
    SvgKill,
    SvgDead,
  },

  setup() {
    // 状态管理
    const menuVisible = ref(true);
    const hudVisible = ref(false);
    const isConnected = ref(false);
    const ct_score = ref(0);
    const t_score = ref(0);
    const ct_players = ref([]);
    const t_players = ref([]);
    const logs = ref([]);
    const notifications = ref([]);
    const showErrorModal = ref(false);
    const selectedError = ref(null);
    const username = ref('Spk');

    const scoreDifference = computed(() => Math.abs(ct_score.value - t_score.value));
    const leadingTeam = computed(() => {
      if (ct_score.value > t_score.value) return 'ct';
      if (t_score.value > ct_score.value) return 't';
      return null;
    });
    const isOnScoringStreak = (team) => {
      return (team === 'ct' && ct_score.value >= 3) ||
        (team === 't' && t_score.value >= 3);
    };
    // 计算属性
    const userInitials = computed(() => {
      return username.value.slice(0, 3).toUpperCase();
    });

    let notificationId = 0;
    let cleanupFns = [];

    // 更新当前时间
    const updateDateTime = () => {
      const now = new Date();
      const year = now.getUTCFullYear();
      const month = String(now.getUTCMonth() + 1).padStart(2, '0');
      const day = String(now.getUTCDate()).padStart(2, '0');
      const hours = String(now.getUTCHours()).padStart(2, '0');
      const minutes = String(now.getUTCMinutes()).padStart(2, '0');
      const seconds = String(now.getUTCSeconds()).padStart(2, '0');
    };

    // 错误通知系统
    const addNotification = (message) => {
      const id = notificationId++;
      const notification = {
        id,
        message,
        time: new Date().toLocaleTimeString(),
        progress: 100
      };

      notifications.value.push(notification);

      const startTime = Date.now();
      const duration = 4000;
      const updateProgress = () => {
        const elapsed = Date.now() - startTime;
        const progress = Math.max(0, 100 * (1 - elapsed / duration));

        const notificationIndex = notifications.value.findIndex(n => n.id === id);
        if (notificationIndex !== -1) {
          notifications.value[notificationIndex].progress = progress;

          if (progress > 0) {
            requestAnimationFrame(updateProgress);
          } else {
            notifications.value = notifications.value.filter(n => n.id !== id);
          }
        }
      };

      requestAnimationFrame(updateProgress);
    };

    const showErrorDetails = (notification) => {
      selectedError.value = notification;
      showErrorModal.value = true;
    };

    const closeErrorModal = () => {
      showErrorModal.value = false;
      selectedError.value = null;
    };

    // 工具函数
    const formatTime = (date) => {
      return new Intl.DateTimeFormat('zh-CN', {
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit'
      }).format(date);
    };

    const getWeaponIcon = (weaponName) => {
      return weaponName.replace(/^weapon_/, "").toLowerCase();
    };

    const getHealthColor = (health) => {
      if (!health || health <= 0) return 'transparent';
      if (health > 70) return '#4caf50';
      if (health > 30) return '#ff9800';
      return '#f44336';
    };

    // 菜单切换
    const toggleMenu = () => {
      if (isConnected.value) {
        hudVisible.value = !hudVisible.value;
        menuVisible.value = !menuVisible.value;
        invoke("ignore_cursor", { ignore: hudVisible.value });
      }
    };

    // 连接管理
    const connect = async () => {
      if (isConnected.value) return;

      try {
        await invoke("connect");
        isConnected.value = true;
        hudVisible.value = true;
        menuVisible.value = false;
        invoke("ignore_cursor", { ignore: true });
      } catch (e) {
        addNotification(e.message || "连接失败");
      }
    };

    const disconnect = async () => {
      if (!isConnected.value) return;

      try {
        await invoke("disconnect");
        isConnected.value = false;
        hudVisible.value = false;
        menuVisible.value = true;
        invoke("ignore_cursor", { ignore: false });
      } catch (e) {
        addNotification(e.message || "断开连接失败");
      }
    };

    const handleServerDisconnect = async () => {
      isConnected.value = false;
      hudVisible.value = false;
      menuVisible.value = true;
      await invoke("ignore_cursor", { ignore: false });
      addNotification("服务器连接已断开");
    };

    const exitApp = async () => {
      if (isConnected.value) {
        await disconnect();
      }
      invoke("exit_app");
    };

    // 生命周期钩子
    onMounted(() => {
      const listeners = [
        listen("update_data", (event) => {
          const data = event.payload;
          ct_score.value = data.ct_score;
          t_score.value = data.t_score;
          ct_players.value = data.players.filter((p) => p.team === "CT");
          t_players.value = data.players.filter((p) => p.team === "TERRORIST");
        }),

        listen("log", (event) => {
          logs.value.push(event.payload);
          if (logs.value.length > 100) {
            logs.value = logs.value.slice(-100);
          }
        }),

        listen("error", (event) => {
          addNotification(event.payload);
        }),

        listen("server_disconnected", handleServerDisconnect)
      ];

      Promise.all(listeners).then(unlisten => {
        cleanupFns = unlisten;
      });

      const handleKeyDown = (event) => {
        if (event.code === "F1") {
          event.preventDefault();
          toggleMenu();
        }
      };

      window.addEventListener("keydown", handleKeyDown);
      cleanupFns.push(() => window.removeEventListener("keydown", handleKeyDown));
    });

    onUnmounted(() => {
      cleanupFns.forEach(fn => fn());
    });

    return {
      // 状态
      menuVisible,
      hudVisible,
      isConnected,
      ct_score,
      t_score,
      ct_players,
      t_players,
      logs,
      notifications,
      showErrorModal,
      selectedError,
      username,
      userInitials,
      // 方法
      formatTime,
      getWeaponIcon,
      getHealthColor,
      connect,
      disconnect,
      exitApp,
      showErrorDetails,
      closeErrorModal,
    };
  }
};
</script>