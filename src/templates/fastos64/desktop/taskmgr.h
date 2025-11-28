/* ═══════════════════════════════════════════════════════════════════════════
 * FASTOS 64-BIT - TASK MANAGER (Administrador de Tareas)
 * ═══════════════════════════════════════════════════════════════════════════
 * Archivo: taskmgr.h
 * Descripción: Administrador de tareas estilo Windows 11
 * Autor: Eddi Andreé Salazar Matos
 * ═══════════════════════════════════════════════════════════════════════════
 */

#ifndef TASKMGR_H
#define TASKMGR_H

#include "../types64.h"
#include "window.h"

/* ═══════════════════════════════════════════════════════════════════════════
 * CONSTANTES
 * ═══════════════════════════════════════════════════════════════════════════
 */

#define TASKMGR_WIDTH           800
#define TASKMGR_HEIGHT          600

#define MAX_PROCESSES           64
#define MAX_PROCESS_NAME        32

/* Tabs */
typedef enum {
    TAB_PROCESSES,
    TAB_PERFORMANCE,
    TAB_APP_HISTORY,
    TAB_STARTUP,
    TAB_USERS,
    TAB_DETAILS,
    TAB_SERVICES,
    TAB_COUNT
} TaskMgrTab;

/* ═══════════════════════════════════════════════════════════════════════════
 * ESTRUCTURAS
 * ═══════════════════════════════════════════════════════════════════════════
 */

typedef struct {
    uint32_t        pid;
    char            name[MAX_PROCESS_NAME];
    uint32_t        cpu_percent;        /* 0-100 */
    uint32_t        memory_mb;
    uint32_t        disk_percent;
    uint32_t        network_kbps;
    uint32_t        gpu_percent;
    bool            is_system;
    bool            is_background;
} ProcessInfo;

typedef struct {
    /* CPU */
    uint32_t        cpu_usage;          /* 0-100 */
    uint32_t        cpu_speed_mhz;
    uint32_t        cpu_processes;
    uint32_t        cpu_threads;
    uint32_t        cpu_handles;
    uint32_t        cpu_uptime_sec;
    
    /* Historial CPU (últimos 60 segundos) */
    uint8_t         cpu_history[60];
    uint32_t        cpu_history_index;
    
} CPUInfo;

typedef struct {
    /* Memoria */
    uint64_t        total_mb;
    uint64_t        used_mb;
    uint64_t        available_mb;
    uint64_t        cached_mb;
    uint32_t        usage_percent;
    
    /* Historial */
    uint8_t         mem_history[60];
    uint32_t        mem_history_index;
    
} MemoryInfo;

typedef struct {
    /* GPU */
    char            name[64];
    uint32_t        usage_percent;
    uint32_t        vram_total_mb;
    uint32_t        vram_used_mb;
    uint32_t        temperature_c;
    uint32_t        fan_percent;
    
    /* Historial */
    uint8_t         gpu_history[60];
    uint32_t        gpu_history_index;
    
} GPUInfo;

typedef struct {
    /* Disco */
    char            name[32];
    uint64_t        total_gb;
    uint64_t        used_gb;
    uint32_t        read_kbps;
    uint32_t        write_kbps;
    uint32_t        usage_percent;
    
} DiskInfo;

typedef struct {
    /* Estado */
    Window*         window;
    TaskMgrTab      current_tab;
    bool            compact_mode;
    
    /* Procesos */
    ProcessInfo     processes[MAX_PROCESSES];
    uint32_t        process_count;
    int32_t         selected_process;
    
    /* Hardware Info */
    CPUInfo         cpu_info;
    MemoryInfo      memory_info;
    GPUInfo         gpu_info;
    DiskInfo        disk_info;
    
    /* Sorting */
    int32_t         sort_column;
    bool            sort_ascending;
    
    /* Scroll */
    int32_t         scroll_offset;
    
} TaskManager;

/* ═══════════════════════════════════════════════════════════════════════════
 * FUNCIONES
 * ═══════════════════════════════════════════════════════════════════════════
 */

/* Inicialización */
void taskmgr_init(void);
void taskmgr_shutdown(void);

/* Ventana */
Window* taskmgr_open(void);
void taskmgr_close(void);
bool taskmgr_is_open(void);

/* Renderizado */
void taskmgr_render(Window* win);
void taskmgr_render_tabs(Window* win, int32_t x, int32_t y);
void taskmgr_render_processes(Window* win, int32_t x, int32_t y, int32_t w, int32_t h);
void taskmgr_render_performance(Window* win, int32_t x, int32_t y, int32_t w, int32_t h);
void taskmgr_render_cpu_graph(Window* win, int32_t x, int32_t y, int32_t w, int32_t h);
void taskmgr_render_memory_graph(Window* win, int32_t x, int32_t y, int32_t w, int32_t h);
void taskmgr_render_gpu_graph(Window* win, int32_t x, int32_t y, int32_t w, int32_t h);

/* Eventos */
void taskmgr_handle_click(Window* win, int32_t x, int32_t y);
void taskmgr_handle_key(Window* win, uint8_t key);

/* Procesos */
void taskmgr_update_processes(void);
void taskmgr_kill_process(uint32_t pid);
void taskmgr_set_priority(uint32_t pid, int32_t priority);

/* Hardware */
void taskmgr_update_cpu(void);
void taskmgr_update_memory(void);
void taskmgr_update_gpu(void);
void taskmgr_update_disk(void);

/* Tab */
void taskmgr_set_tab(TaskMgrTab tab);
TaskMgrTab taskmgr_get_tab(void);

/* Sorting */
void taskmgr_sort_by(int32_t column);

TaskManager* taskmgr_get(void);

#endif /* TASKMGR_H */

