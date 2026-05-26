import { invoke } from "@tauri-apps/api/core";
import type { Credentials } from "@/native/types";

export async function loadCredentials(): Promise<Credentials | null> {
  return (await invoke("load_credentials")) as Credentials | null;
}

export async function saveCredentials(
  username: string,
  password: string,
  networkType: string,
): Promise<void> {
  await invoke("save_credentials", {
    username,
    password,
    networkType,
  });
}
