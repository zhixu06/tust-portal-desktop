import { invoke } from "@tauri-apps/api/core";
import type { LoginResult } from "./interop";

export async function tryLogin(
  username: string,
  password: string,
  networkType: string,
): Promise<LoginResult> {
  return await invoke("try_login", {
    username,
    password,
    networkType,
  });
}
