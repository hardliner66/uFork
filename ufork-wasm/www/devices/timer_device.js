// Installs the timer device.

/*jslint browser, devel */

import ufork from "../ufork.js";

function timer_device(core) {
    const timer_map = Object.create(null);
    core.h_install(
        [[
            ufork.TIMER_DEV_OFS,
            core.u_ptr_to_cap(core.u_ramptr(ufork.TIMER_DEV_OFS))
        ]],
        {
            host_start_timer(delay, stub) { // (i32, i32) -> nil
                if (core.u_is_fix(delay)) {
                    const quad = core.u_read_quad(stub);
                    const evt = quad.y;  // stub carries pre-allocated event
                    timer_map[stub] = setTimeout(function () {
                        delete timer_map[stub];
                        core.h_release_stub(stub);
                        core.h_event_inject(evt);
                        core.h_wakeup(ufork.TIMER_DEV_OFS);
                    }, core.u_fix_to_i32(delay));
                    if (core.u_trace !== undefined) {
                        core.u_trace("host_start_timer", timer_map[stub]);
                    }
                }
            },
            host_stop_timer(stub) { // (i32) -> bool
                const id = timer_map[stub];
                if (id !== undefined) {
                    clearTimeout(id);
                    delete timer_map[stub];
                    if (core.u_trace !== undefined) {
                        core.u_trace("host_stop_timer", id);
                    }
                    return true;
                }
                return false;
            }
        }
    );
}

export default Object.freeze(timer_device);
