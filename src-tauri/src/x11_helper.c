#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdint.h>
#include <strings.h>
#include <xcb/xcb.h>
#include <xcb/xcb_ewmh.h>
#include <xcb/xcb_icccm.h>

void arch_island_set_skip_taskbar(uint32_t target_pid) {
    if (target_pid == 0) {
        target_pid = (uint32_t)getpid();
    }

    const char *dpy = getenv("DISPLAY");
    if (!dpy || dpy[0] == '\0') {
        dpy = ":0";
    }

    xcb_connection_t *conn = xcb_connect(dpy, NULL);
    if (!conn || xcb_connection_has_error(conn)) return;

    xcb_ewmh_connection_t ewmh;
    xcb_intern_atom_cookie_t *cookie = xcb_ewmh_init_atoms(conn, &ewmh);
    if (!xcb_ewmh_init_atoms_replies(&ewmh, cookie, NULL)) {
        xcb_disconnect(conn);
        return;
    }

    xcb_get_property_cookie_t cl_cookie = xcb_ewmh_get_client_list(&ewmh, 0);
    xcb_ewmh_get_windows_reply_t clients;
    if (xcb_ewmh_get_client_list_reply(&ewmh, cl_cookie, &clients, NULL)) {
        for (unsigned int i = 0; i < clients.windows_len; i++) {
            xcb_window_t win = clients.windows[i];
            uint32_t pid = 0;
            xcb_get_property_cookie_t pid_cookie = xcb_ewmh_get_wm_pid(&ewmh, win);
            int match = 0;
            if (xcb_ewmh_get_wm_pid_reply(&ewmh, pid_cookie, &pid, NULL)) {
                if (pid == target_pid) {
                    match = 1;
                }
            }

            if (!match) {
                xcb_get_property_cookie_t cky = xcb_icccm_get_wm_class(conn, win);
                xcb_icccm_get_wm_class_reply_t wcr;
                if (xcb_icccm_get_wm_class_reply(conn, cky, &wcr, NULL)) {
                    if ((wcr.class_name && strcasecmp(wcr.class_name, "arch-island") == 0) ||
                        (wcr.instance_name && strcasecmp(wcr.instance_name, "arch-island") == 0)) {
                        match = 1;
                    }
                    xcb_icccm_get_wm_class_reply_wipe(&wcr);
                }
            }

            if (match) {
                xcb_atom_t states[] = {
                    ewmh._NET_WM_STATE_STICKY,
                    ewmh._NET_WM_STATE_SKIP_TASKBAR,
                    ewmh._NET_WM_STATE_SKIP_PAGER
                };
                xcb_ewmh_set_wm_state(&ewmh, win, 3, states);
            }
        }
        xcb_ewmh_get_windows_reply_wipe(&clients);
    }
    xcb_flush(conn);
    xcb_disconnect(conn);
}
