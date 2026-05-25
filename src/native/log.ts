import { invoke } from "@tauri-apps/api/core";
import type { LogEntry } from "./interop";

export async function getLogs(): Promise<LogEntry[]> {
  return await invoke("get_logs");
}
