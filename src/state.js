import { emit as emitEvent, listen } from "@tauri-apps/api/event"
import { create } from "zustand";

function emit(key, value) {
    return emitEvent('global_state_change', { key, value });
}

const getGlobalState = create(() => ({
    n: 4,
    setN: (n) => emit('n', n),
    k: 0,
    setK: (k) => emit('k', k),
    content: "ABCD",
    setContent: (content) => emit('content', content),
    sequence: [],
    rods: 3,
    setRods: (rods) => emit('rods', rods),
    plates: 4,
    setPlates: (plates) => emit('plates', plates),
    fps: 2,
    setFps: (fps) => emit('fps', fps),
    video_path: "",
    max_frame: -1,
    current_frame: 0
}));

export async function subscribeState() {
    const unsubscribeState = await listen('global_state_sync', (e) => {
        getGlobalState.setState({ [e.payload.key]: e.payload.value });
    });

    return async () => {
        unsubscribeState();
    }
}

export default getGlobalState;