import { Button } from "antd";
import { StopOutlined, DesktopOutlined } from "@ant-design/icons";
import type { InstanceInfo } from "../../types";

interface Props {
  instance: InstanceInfo;
  onTerminate: (id: string) => void;
}

export function InstanceCard({ instance, onTerminate }: Props) {
  const isRunning = instance.status === "running";

  return (
    <div className="instance-card">
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
            <span
              className="font-semibold text-sm"
              style={{ color: "var(--color-text-primary)" }}
            >
              {instance.label}
            </span>
            {instance.pid > 0 && (
              <div className="mt-1">
                <span style={{ color: "var(--color-text-secondary)", fontSize: 12 }}>
                  PID: {instance.pid}
                </span>
              </div>
            )}
          </div>
        </div>

        <div className="flex items-center gap-2 flex-shrink-0">
          <div
            className="flex items-center gap-1.5 px-2.5 py-1 rounded-lg"
            style={{
              background: isRunning ? "rgba(7, 193, 96, 0.08)" : "rgba(148, 163, 184, 0.08)",
              border: isRunning
                ? "1px solid rgba(7, 193, 96, 0.12)"
                : "1px solid rgba(148, 163, 184, 0.12)",
            }}
          >
            <span
              className={`status-dot ${isRunning ? "status-dot-running" : "status-dot-stopped"}`}
              style={{ width: 6, height: 6 }}
            />
            <span
              style={{
                color: isRunning ? "#07c160" : "#94a3b8",
                fontSize: 12,
                fontWeight: 500,
              }}
            >
              {isRunning ? "运行中" : "已停止"}
            </span>
          </div>

          {isRunning && (
            <Button
              danger
              size="small"
              icon={<StopOutlined />}
              onClick={() => onTerminate(instance.id)}
              style={{ borderRadius: 8, fontWeight: 500 }}
            >
              终止
            </Button>
          )}
        </div>
      </div>
    </div>
  );
}
