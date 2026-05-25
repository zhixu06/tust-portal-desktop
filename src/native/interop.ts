export interface LogEntry {
  timestamp: string;
  message: string;
}

export interface NetworkStatus {
  wifi_ssid: string | null;
  local_ipv4: string | null;
  local_ipv6: string | null;
  is_tust_network: boolean;
}

export interface LoginResult {
  success: boolean;
  message: string;
  logs: string[];
}

export interface Credentials {
  username: string;
  password: string;
  network_type: string;
}
