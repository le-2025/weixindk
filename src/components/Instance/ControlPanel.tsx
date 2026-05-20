import { Button, Input, message, Modal } from "antd";
import {
  PlusOutlined,
  ReloadOutlined,
  EditOutlined,
  CheckOutlined,
  CloseOutlined,
  ThunderboltOutlined,
  StopOutlined,
  PlayCircleOutlined,
  DeleteOutlined,
  DesktopOutlined,
} from "@ant-design/icons";
import { useEffect, useState } from "react";
import { useWechat } from "../../hooks/useWechat";

export function ControlPanel() {
  const { instances, loading, launchNewInstance, refreshInstances, syncInstances, updateLabel, terminateInstance, relaunchInstance, deleteInstance } = useWechat();
  const [launchLabel, setLaunchLabel] = useState("");
  const [editingId, setEditingId] = useState<string | null>(null);
  const [editLabel, setEditLabel] = useState("");

  useEffect(() => {
    refreshInstances();
  }, []);

  const handleLaunch = async (label?: string) => {
    try {
      await launchNewInstance(label);
      message.success("微信启动成功");
      setLaunchLabel("");
    } catch (err) {
      message.error(`启动失败: ${err}`);
    }
  };

  const handleRelaunch = async (instanceId: string) => {
    try {
      await relaunchInstance(instanceId);
      message.success("微信启动成功");
    } catch (err) {
      message.error(`启动失败: ${err}`);
    }
  };

  const handleRefresh = async () => {
    try {
      await syncInstances();
      message.success("已刷新状态");
    } catch (err) {
      message.error(`刷新失败: ${err}`);
    }
  };

  const handleTerminate = (id: string, label: string) => {
    Modal.confirm({
      title: "确认终止实例",
      content: `确定要终止「${label}」吗？微信进程将被强制关闭。`,
      okText: "终止",
      okType: "danger",
      cancelText: "取消",
      centered: true,
      onOk: async () => {
        try {
          await terminateInstance(id);
          message.success("实例已终止");
        } catch (err) {
          message.error(`终止失败: ${err}`);
        }
      },
    });
  };

  const handleDelete = (id: string, label: string) => {
    Modal.confirm({
      title: "确认删除实例",
      content: `确定要删除「${label}」吗？该实例的登录记录将被清除。`,
      okText: "删除",
      okType: "danger",
      cancelText: "取消",
      centered: true,
      onOk: async () => {
        try {
          await deleteInstance(id);
          message.success("实例已删除");
        } catch (err) {
          message.error(`删除失败: ${err}`);
        }
      },
    });
  };

  const startEdit = (id: string, currentLabel: string) => {
    setEditingId(id);
    setEditLabel(currentLabel);
  };

  const saveEdit = async () => {
    if (!editingId) return;
    try {
      await updateLabel(editingId, editLabel);
      setEditingId(null);
      message.success("名称已更新");
    } catch (err) {
      message.error(`更新失败: ${err}`);
    }
  };

  const cancelEdit = () => {
    setEditingId(null);
  };

  const runningInstances = instances.filter((i) => i.status === "running");
  const hasAnyInstances = instances.length > 0;

  if (!hasAnyInstances) {
    return (
      <div className="flex flex-col items-center justify-center" style={{ minHeight: "calc(100vh - 48px)" }}>
        <div className="animate-fade-in-up flex flex-col items-center">
          <div
            className="animate-float"
            style={{
              width: 96,
              height: 96,
              borderRadius: 24,
              background: "linear-gradient(135deg, #07c160 0%, #05a34e 50%, #048f42 100%)",
              display: "flex",
              alignItems: "center",
              justifyContent: "center",
              marginBottom: 32,
              boxShadow: "0 12px 40px rgba(7, 193, 96, 0.25), 0 0 0 8px rgba(7, 193, 96, 0.06)",
            }}
          >
            <DesktopOutlined style={{ fontSize: 40, color: "#fff" }} />
          </div>

          <h2
            className="gradient-text"
            style={{ fontSize: 28, fontWeight: 700, marginBottom: 8, letterSpacing: "-0.02em" }}
          >
            微信多开器
          </h2>
          <p style={{ color: "var(--color-text-secondary)", marginBottom: 40, fontSize: 15, fontWeight: 400 }}>
            点击下方按钮，打开新的微信登录窗口
          </p>

          <div className="flex items-center gap-3 mb-6">
            <Input
              placeholder="输入名称（如：工作号）"
              value={launchLabel}
              onChange={(e) => setLaunchLabel(e.target.value)}
              onPressEnter={() => handleLaunch(launchLabel || undefined)}
              style={{
                width: 200,
                height: 44,
                borderRadius: 12,
                fontSize: 14,
              }}
            />
            <Button
              type="primary"
              size="large"
              icon={<PlusOutlined />}
              onClick={() => handleLaunch(launchLabel || undefined)}
              loading={loading}
              className="btn-glow"
              style={{
                height: 44,
                paddingInline: 28,
                borderRadius: 12,
                background: "linear-gradient(135deg, #07c160 0%, #05a34e 100%)",
                border: "none",
                boxShadow: "0 4px 20px rgba(7, 193, 96, 0.3)",
                fontWeight: 600,
                fontSize: 15,
              }}
            >
              打开新微信
            </Button>
          </div>

          <div
            className="flex items-center gap-2"
            style={{ color: "var(--color-text-secondary)", fontSize: 13 }}
          >
            <ThunderboltOutlined style={{ color: "#00d4ff" }} />
            <span>支持同时运行多个独立微信实例</span>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="animate-fade-in">
      <div className="flex items-center justify-between mb-6">
        <div>
          <h2 style={{ fontSize: 22, fontWeight: 700, color: "var(--color-text-primary)", marginBottom: 4 }}>
            所有实例
          </h2>
          <div className="flex items-center gap-3">
            <span style={{ color: "var(--color-text-secondary)", fontSize: 14 }}>
              共 {instances.length} 个
              {runningInstances.length > 0 && ` · ${runningInstances.length} 个运行中`}
            </span>
          </div>
        </div>

        <div className="flex items-center gap-2">
          <Input
            placeholder="名称"
            value={launchLabel}
            onChange={(e) => setLaunchLabel(e.target.value)}
            onPressEnter={() => handleLaunch(launchLabel || undefined)}
            style={{ width: 140, height: 36, borderRadius: 10, fontSize: 13 }}
          />
          <Button
            type="primary"
            icon={<PlusOutlined />}
            onClick={() => handleLaunch(launchLabel || undefined)}
            loading={loading}
            style={{
              height: 36,
              borderRadius: 10,
              background: "linear-gradient(135deg, #07c160 0%, #05a34e 100%)",
              border: "none",
              boxShadow: "0 2px 12px rgba(7, 193, 96, 0.25)",
              fontWeight: 500,
            }}
          >
            再开一个
          </Button>
          <Button
            icon={<ReloadOutlined />}
            onClick={handleRefresh}
            style={{ height: 36, borderRadius: 10, fontWeight: 500 }}
          >
            刷新
          </Button>
        </div>
      </div>

      <div className="flex flex-col gap-3">
        {instances.map((item, index) => {
          const isRunning = item.status === "running";
          return (
            <div
              key={item.id}
              className="instance-card animate-fade-in-up"
              style={{ animationDelay: `${index * 60}ms`, animationFillMode: "both" }}
            >
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-3 flex-1 min-w-0">
                  <div
                    className="w-10 h-10 rounded-xl flex items-center justify-center flex-shrink-0"
                    style={{
                      background: isRunning
                        ? "linear-gradient(135deg, rgba(7, 193, 96, 0.1) 0%, rgba(5, 163, 78, 0.15) 100%)"
                        : "linear-gradient(135deg, rgba(148, 163, 184, 0.1) 0%, rgba(100, 116, 139, 0.1) 100%)",
                      border: isRunning
                        ? "1px solid rgba(7, 193, 96, 0.15)"
                        : "1px solid rgba(148, 163, 184, 0.15)",
                    }}
                  >
                    <DesktopOutlined
                      style={{
                        color: isRunning ? "#07c160" : "#94a3b8",
                        fontSize: 18,
                      }}
                    />
                  </div>

                  <div className="flex-1 min-w-0">
                    {editingId === item.id ? (
                      <div className="flex items-center gap-2">
                        <Input
                          value={editLabel}
                          onChange={(e) => setEditLabel(e.target.value)}
                          onPressEnter={saveEdit}
                          size="small"
                          style={{ width: 160, borderRadius: 8 }}
                          autoFocus
                        />
                        <Button
                          size="small"
                          type="text"
                          icon={<CheckOutlined />}
                          onClick={saveEdit}
                          style={{ color: "#07c160" }}
                        />
                        <Button
                          size="small"
                          type="text"
                          icon={<CloseOutlined />}
                          onClick={cancelEdit}
                          style={{ color: "#999" }}
                        />
                      </div>
                    ) : (
                      <div
                        className="flex items-center gap-2 cursor-pointer group"
                        onClick={() => startEdit(item.id, item.label)}
                      >
                        <span
                          className="font-semibold text-sm truncate"
                          style={{ color: "var(--color-text-primary)" }}
                        >
                          {item.label}
                        </span>
                        <EditOutlined
                          className="opacity-0 group-hover:opacity-100 transition-opacity"
                          style={{ color: "#bbb", fontSize: 12 }}
                        />
                      </div>
                    )}
                    <div className="flex items-center gap-3 mt-1">
                      {isRunning && (
                        <span style={{ color: "var(--color-text-secondary)", fontSize: 12 }}>
                          PID: {item.pid}
                        </span>
                      )}
                      <span style={{ color: "var(--color-text-secondary)", fontSize: 12 }}>
                        {new Date(item.created_at).toLocaleTimeString("zh-CN", {
                          hour: "2-digit",
                          minute: "2-digit",
                        })}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-2 flex-shrink-0">
                  {isRunning ? (
                    <>
                      <div
                        className="flex items-center gap-1.5 px-2.5 py-1 rounded-lg"
                        style={{
                          background: "rgba(7, 193, 96, 0.08)",
                          border: "1px solid rgba(7, 193, 96, 0.12)",
                        }}
                      >
                        <span className="status-dot status-dot-running" style={{ width: 6, height: 6 }} />
                        <span style={{ color: "#07c160", fontSize: 12, fontWeight: 500 }}>运行中</span>
                      </div>
                      <Button
                        danger
                        size="small"
                        icon={<StopOutlined />}
                        onClick={() => handleTerminate(item.id, item.label)}
                        style={{ borderRadius: 8, fontWeight: 500 }}
                      >
                        终止
                      </Button>
                    </>
                  ) : (
                    <>
                      <Button
                        type="primary"
                        size="small"
                        icon={<PlayCircleOutlined />}
                        onClick={() => handleRelaunch(item.id)}
                        loading={loading}
                        style={{
                          borderRadius: 8,
                          fontWeight: 500,
                          background: "linear-gradient(135deg, #07c160 0%, #05a34e 100%)",
                          border: "none",
                          boxShadow: "0 2px 8px rgba(7, 193, 96, 0.25)",
                        }}
                      >
                        启动
                      </Button>
                      <Button
                        size="small"
                        icon={<DeleteOutlined />}
                        onClick={() => handleDelete(item.id, item.label)}
                        style={{ borderRadius: 8, fontWeight: 500, color: "#94a3b8" }}
                      />
                    </>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
