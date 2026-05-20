export interface LaunchInfo {
  instance_id: string;
  pid: number;
  hwnd: string;
}

export interface SaveLoginInfo {
  wxid: string;
  has_avatar: boolean;
}

export interface InstanceInfo {
  id: string;
  label: string;
  pid: number;
  hwnd: string;
  data_path: string;
  status: "running" | "stopped";
  wxid: string;
  created_at: string;
}

export interface AppConfig {
  wechat_path: string;
  max_instances: number;
  auto_detect_path: boolean;
  minimize_to_tray: boolean;
  theme: "light" | "dark";
  auto_start: boolean;
}

export interface ConfigEntry {
  key: string;
  value: string;
}