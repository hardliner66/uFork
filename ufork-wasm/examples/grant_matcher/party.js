/*jslint browser */

import hex from "../../www/hex.js";
import parseq from "../../www/parseq.js";
import requestorize from "../../www/requestors/requestorize.js";
import lazy from "../../www/requestors/lazy.js";
import ufork from "../../www/ufork.js";
import awp_device from "../../www/devices/awp_device.js";
import webrtc_transport from "../../www/transports/webrtc_transport.js";
import websockets_signaller from "../../www/transports/websockets_signaller.js";

const signaller_origin = (
    location.protocol === "https:"
    ? "wss://"
    : "ws://"
) + location.host;

function make_address(name) {
    return signaller_origin + "/connect?name=" + hex.encode(name);
}

function make_bind_info(name) {
    return (
        signaller_origin
        + "/listen?name=" + hex.encode(name)
        + "&password=uFork"
    );
}

function party(asm_url, acquaintance_names = []) {
    const pre = document.createElement("pre");
    document.body.append(pre);

    function print(...things) {
        things = things.map(function (thing) {
            return thing?.message ?? thing;
        });
        pre.textContent += "\n" + things.join(" ");
    }

    const transport = webrtc_transport(websockets_signaller(), print);
    let core;

    return parseq.sequence([
        ufork.instantiate_core(
            import.meta.resolve(
                "../../target/wasm32-unknown-unknown/debug/ufork_wasm.wasm"
            ),
            function on_wakeup() {
                const sig = core.h_run_loop(0);
                const err = core.u_fix_to_i32(sig);
                print("WAKE:", core.u_print(sig), core.u_fault_msg(err));
            },
            function on_log(log_level, ...values) {
                print(...values);
                if (log_level === ufork.LOG_DEBUG) {
                    const div = document.createElement("div");
                    div.textContent = "💸";
                    div.style.fontSize = "100px";
                    document.body.append(div);
                }
            },
            ufork.LOG_DEBUG
        ),
        parseq.parallel([
            lazy(function (the_core) {
                core = the_core;
                return core.h_import(asm_url);
            }),
            transport.generate_identity()
        ]),
        requestorize(function ([asm_module, identity]) {
            const name = transport.identity_to_name(identity);
            const address = make_address(name);
            awp_device(core, transport, [{
                identity,
                bind_info: make_bind_info(name),
                acquaintances: [
                    {name, address},
                    ...acquaintance_names.map(function (name) {
                        return {name, address: make_address(name)};
                    })
                ]
            }]);
            core.h_boot(asm_module.boot);
            const sig = core.h_run_loop(0);
            const err = core.u_fix_to_i32(sig);
            print("IDLE:", core.u_print(sig), core.u_fault_msg(err));
            return name;
        })
    ])(function callback(name, reason) {
        if (name !== undefined) {
            print("Name", hex.encode(name));
        } else {
            print(reason);
        }
    });
}

export default Object.freeze(party);
