import { invoke } from "@tauri-apps/api/core";
import type { LogEntry } from "@/native/types";

export async function getLogs(): Promise<LogEntry[]> {
  return await invoke("get_logs");
}
