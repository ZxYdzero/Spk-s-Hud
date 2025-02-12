#pragma semicolon 1
#pragma newdecls required
#pragma dynamic 131072

#include <sourcemod>
#include <sdktools>
#include <socket>
#include <json>


#define PLUGIN_NAME "[Spk's Hud]"

Handle PORT;
Handle passwd;
Handle g_socket = INVALID_HANDLE;
Handle g_clientSocket = INVALID_HANDLE;
Handle g_timer = INVALID_HANDLE;

public Plugin myinfo = {
    name = "Spk's Hud",
    author = "Spk",
    description = "",
    version = "1.0",
    url = ""
};

public void OnPluginStart() {

    PORT = CreateConVar("sm_spkhud_port", "32456", "作为默认打开的端口");
    passwd = CreateConVar("sm_spkhud_passwd", "", "密码,默认为空");
    g_socket = SocketCreate(SOCKET_TCP, OnServerError);
    if(SocketBind(g_socket, "0.0.0.0", GetConVarInt(PORT)))
    {
        SocketListen(g_socket, OnIncomingConnection);
        PrintToServer("%s 服务器开始监听端口 %d", PLUGIN_NAME, GetConVarInt(PORT));
    }
}

public Action SendGameData(Handle timer) {
    JSON_Object data = new JSON_Object();
    
    // 获取比分
    data.SetInt("ct_score", GetTeamScore(3));
    data.SetInt("t_score", GetTeamScore(2));

    // 获取玩家数据
    JSON_Array players = new JSON_Array();
    for (int i = 1; i <= MaxClients; i++) {
        if (IsClientInGame(i) && !IsFakeClient(i)) {
            JSON_Object player = new JSON_Object();

            // 玩家名称
            char player_name[32];
            GetClientName(i, player_name, sizeof(player_name));
            player.SetString("name", player_name);

            // KDA
            player.SetInt("kills", GetClientFrags(i));
            player.SetInt("deaths", GetClientDeaths(i));

            // 队伍名称
            char team_name[32];
            GetTeamName(GetClientTeam(i), team_name, sizeof(team_name));
            player.SetString("team", team_name);

            // 血量和护甲
            player.SetInt("health", GetClientHealth(i));
            player.SetInt("armor", GetClientArmor(i));

            // 获取当前持枪
            int activeWeapon = GetEntPropEnt(i, Prop_Send, "m_hActiveWeapon");
            if (activeWeapon > MaxClients) {
                char weaponName[32];
                GetEdictClassname(activeWeapon, weaponName, sizeof(weaponName));
                player.SetString("active_weapon", weaponName);
            } else {
                player.SetString("active_weapon", "none");
            }

            // 获取所有持有的武器 暂时不用
            // JSON_Array weapons = new JSON_Array();
            // for (int slot = 0; slot < 5; slot++) {  // CSGO / CSS 一般有 5 个武器槽位
            //     int weapon = GetPlayerWeaponSlot(i, slot);
            //     if (weapon > MaxClients) {
            //         char weaponName[32];
            //         GetEdictClassname(weapon, weaponName, sizeof(weaponName));
            //         JSON_Object weaponObject = new JSON_Object();
            //         weaponObject.SetString("weapon", weaponName);
            //         weapons.PushObject(weaponObject);  // 将 weaponObject 添加到 weapons 数组
            //     }
            // }
            // 将 weapons 数组添加到 player 对象
            // player.SetObject("weapons", weapons);

            // 将 player 对象添加到 players 数组
            players.PushObject(player);  // 将 player 添加到 players 数组

        }
    }

    // 将 players 数组添加到 data 对象
    data.SetObject("players", players);

    // 序列化并发送
    char buffer[2048];
    data.Encode(buffer, sizeof(buffer));
    SocketSend(g_clientSocket, buffer, strlen(buffer));
    delete data;

    return Plugin_Continue;
}

public void OnIncomingConnection(Handle socket, Handle newSocket, const char[] remoteIP, int remotePort, any arg)
{
    PrintToServer("%s 收到来自 %s:%d 的新连接", PLUGIN_NAME, remoteIP, remotePort);
    g_clientSocket = newSocket;
    
    // 设置新socket回调
    SocketSetReceiveCallback(newSocket, OnServerReceive);
    SocketSetDisconnectCallback(newSocket, OnClientDisconnected);
    SocketSetErrorCallback(newSocket, OnClientError);
}

public void OnServerReceive(Handle socket, const char[] receiveData, const int dataSize, any arg)
{
    char[] buffer = new char[dataSize+1];
    strcopy(buffer, dataSize+1, receiveData);
    TrimString(buffer);
    
    // 获取配置密码
    char cvPasswd[64];
    GetConVarString(passwd, cvPasswd, sizeof(cvPasswd));
    PrintToServer("%s 客户端验证中", PLUGIN_NAME);
    PrintToServer("%s 客户端密码: %s", PLUGIN_NAME, buffer);
    // 验证阶段处理
    if (g_timer == INVALID_HANDLE)
    {
        if (StrEqual(buffer, cvPasswd))
        {
            PrintToServer("%s 客户端验证成功", PLUGIN_NAME);
            g_timer = CreateTimer(0.1, SendGameData, _, TIMER_REPEAT);
            SocketSend(socket, "AUTH_SUCCESS", 12);
        }
        else
        {
            PrintToServer("%s 密码错误 关闭连接", PLUGIN_NAME);
            SocketSend(socket, "AUTH_FAIL", 9);
            CloseHandle(socket);
            g_clientSocket = INVALID_HANDLE;
        }
    }
}
// 客户端断开回调
public void OnClientDisconnected(Handle socket, any arg)
{
    PrintToServer("%s 客户端断开连接", PLUGIN_NAME);
    if (g_clientSocket == socket) {
        CloseHandle(g_timer);
        g_timer = INVALID_HANDLE;
        g_clientSocket = INVALID_HANDLE;
    }
    CloseHandle(socket);
}

// 错误处理回调
public void OnServerError(Handle socket, const int errorType, const int errorNum, any arg)
{
    PrintToServer("%s 服务器错误: %d (errno: %d)", PLUGIN_NAME, errorType, errorNum);
    CloseHandle(socket);
}

public void OnPluginEnd()
{
    CloseHandle(g_socket);
}

// 错误处理回调
public void OnClientError(Handle socket, const int errorType, const int errorNum, any arg)
{
    PrintToServer("%s 发生错误: %d (errno: %d)", PLUGIN_NAME, errorType, errorNum);
    CloseHandle(socket);
}