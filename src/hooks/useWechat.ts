import { useState, useCallback, useEffect } from "react";
import { tauriApi } from "../services/tauri";
import { InstanceInfo } from "../types";
import { listen } from "@tauri-apps/api/event";

export function useWechat() {
  const [instances, setInstances] = useState<InstanceInfo[]>([]);
  const [loading, setLoading] = useState(false);

  const refreshInstances = useCallback(async () => {
    try {
      const list = await tauriApi.getWechatInstances();
      setInstances(list);
    } catch (err) {
      console.error("刷新实例列表失败:", err);
    }
  }, []);

  const launchNewInstance = useCallback(async (label?: string) => {
    setLoading(true);
    try {
      const result = await tauriApi.launchWechat(label);
      await refreshInstances();
      return result;
    } finally {
      setLoading(false);
    }
  }, [refreshInstances]);

  const syncInstances = useCallback(async () => {
    try {
      const list = await tauriApi.syncInstances();
      setInstances(list);
    } catch (err) {
      console.error("同步实例失败:", err);
    }
  }, []);

  const updateLabel = useCallback(async (instanceId: string, label: string) => {
    await tauriApi.updateInstanceLabel(instanceId, label);
    await refreshInstances();
  }, [refreshInstances]);

  const terminateInstance = useCallback(async (instanceId: string) => {
    await tauriApi.terminateInstance(instanceId);
    await refreshInstances();
  }, [refreshInstances]);

  const relaunchInstance = useCallback(async (instanceId: string) => {
    setLoading(true);
    try {
      const result = await tauriApi.relaunchWechat(instanceId);
      await refreshInstances();
      return result;
    } finally {
      setLoading(false);
    }
  }, [refreshInstances]);

  const deleteInstance = useCallback(async (instanceId: string) => {
    await tauriApi.deleteInstance(instanceId);
    await refreshInstances();
  }, [refreshInstances]);

  useEffect(() => {
    refreshInstances();
    const unlisten = listen("wechat-process-exited", () => {
      refreshInstances();
    });
    return () => { unlisten.then(fn => fn()); };
  }, [refreshInstances]);

  return {
    instances, loading,
    launchNewInstance, refreshInstances, syncInstances, updateLabel, terminateInstance, relaunchInstance, deleteInstance,
  };
}
