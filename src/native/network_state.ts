import { invoke } from "@tauri-apps/api/core";
import type { NetworkStatus } from "@/native/types";

export interface StatusState {
  paused: boolean;
  ignoreSsid: boolean;
  networkStatus: NetworkStatus;
}

export async function refreshStatus(): Promise<StatusState> {
  const [paused, ignoreSsid, networkStatus] = await Promise.all([
    invoke<boolean>("get_auto_login_paused"),
    invoke<boolean>("get_ignore_ssid"),
    invoke<NetworkStatus>("check_network_status"),
  ]);
  return { paused, ignoreSsid, networkStatus };
}

export async function setAutoLoginPaused(paused: boolean): Promise<void> {
  await invoke("set_auto_login_paused", { paused });
}

export async function setIgnoreSsid(ignore: boolean): Promise<void> {
  await invoke("set_ignore_ssid", { ignore });
}
