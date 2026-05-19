import { invoke } from "@tauri-apps/api/core";
import type { LaunchInfo, InstanceInfo } from "../types";

export const tauriApi = {
  launchWechat: (label?: string) =>
    invoke<LaunchInfo>("launch_wechat", { label: label ?? null }),

  getWechatInstances: () =>
    invoke<InstanceInfo[]>("get_wechat_instances"),

  updateInstanceLabel: (instanceId: string, label: string) =>
    invoke<void>("update_instance_label", { instanceId, label }),

  terminateInstance: (instanceId: string) =>
    invoke<void>("terminate_instance", { instanceId }),

  getWechatPath: () =>
    invoke<string>("get_wechat_path"),

  syncInstances: () =>
    invoke<InstanceInfo[]>("sync_instances"),

  getAppConfig: () =>
    invoke<Record<string, string>>("get_app_config"),

  saveAppConfig: (key: string, value: string) =>
    invoke<void>("save_app_config", { key, value }),
};