import { Button, InputNumber, Switch, Form, Input, message, Spin } from "antd";
import {
  SaveOutlined,
  FolderOutlined,
  AppstoreOutlined,
  MinusCircleOutlined,
  LoadingOutlined,
} from "@ant-design/icons";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { AppConfig } from "../../types";

const SettingsPanel = () => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);
  const [initializing, setInitializing] = useState(true);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    try {
      const config = (await invoke("get_app_config")) as Record<string, string>;

      form.setFieldsValue({
        wechat_path: config.wechat_path || "",
        max_instances: Number(config.max_instances) || 5,
        minimize_to_tray: config.minimize_to_tray !== "false",
      });
    } catch (err) {
      console.error("Failed to load config:", err);
      message.error("加载配置失败");
    } finally {
      setInitializing(false);
    }
  };

  const onFinish = async (values: Partial<AppConfig>) => {
    setLoading(true);
    try {
      const entries = Object.entries(values).filter(
        ([, value]) => value !== undefined,
      );

      for (const [key, value] of entries) {
        const stringValue = typeof value === "boolean" ? String(value) : String(value);
        if (key === "minimize_to_tray") {
          await invoke("set_minimize_to_tray", { enabled: value as boolean });
        } else {
          await invoke("save_app_config", { key, value: stringValue });
        }
      }

      message.success("配置已保存");
    } catch (err) {
      console.error("Failed to save config:", err);
      message.error("保存配置失败");
    } finally {
      setLoading(false);
    }
  };

  if (initializing) {
    return (
      <div className="flex items-center justify-center" style={{ minHeight: "calc(100vh - 48px)" }}>
        <Spin indicator={<LoadingOutlined style={{ fontSize: 32, color: "#0080ff" }} spin />} />
      </div>
    );
  }

  return (
    <div className="animate-fade-in" style={{ maxWidth: 640, margin: "0 auto" }}>
      <div className="mb-8">
        <h2 style={{ fontSize: 22, fontWeight: 700, color: "var(--color-text-primary)", marginBottom: 4 }}>
          应用设置
        </h2>
        <p style={{ color: "var(--color-text-secondary)", fontSize: 14 }}>
          自定义应用行为与偏好设置
        </p>
      </div>

      <Form
        form={form}
        layout="vertical"
        onFinish={onFinish}
        initialValues={{ max_instances: 5 }}
        requiredMark={false}
      >
        <div
          className="glass-card p-6 mb-4"
          style={{ background: "rgba(255, 255, 255, 0.95)" }}
        >
          <div className="flex items-center gap-2.5 mb-5">
            <div
              className="w-8 h-8 rounded-lg flex items-center justify-center"
              style={{
                background: "linear-gradient(135deg, rgba(0, 128, 255, 0.1) 0%, rgba(0, 212, 255, 0.15) 100%)",
                border: "1px solid rgba(0, 128, 255, 0.12)",
              }}
            >
              <FolderOutlined style={{ color: "#0080ff", fontSize: 14 }} />
            </div>
            <span className="font-semibold text-sm" style={{ color: "var(--color-text-primary)" }}>
              路径配置
            </span>
          </div>

          <Form.Item
            name="wechat_path"
            label="微信安装路径"
            extra="微信可执行文件的完整路径，如 D:\WeChat\WeChat.exe"
          >
            <Input
              placeholder="例如: D:\WeChat\WeChat.exe"
              style={{ borderRadius: 10 }}
            />
          </Form.Item>
        </div>

        <div
          className="glass-card p-6 mb-4"
          style={{ background: "rgba(255, 255, 255, 0.95)" }}
        >
          <div className="flex items-center gap-2.5 mb-5">
            <div
              className="w-8 h-8 rounded-lg flex items-center justify-center"
              style={{
                background: "linear-gradient(135deg, rgba(124, 58, 237, 0.1) 0%, rgba(139, 92, 246, 0.15) 100%)",
                border: "1px solid rgba(124, 58, 237, 0.12)",
              }}
            >
              <AppstoreOutlined style={{ color: "#7c3aed", fontSize: 14 }} />
            </div>
            <span className="font-semibold text-sm" style={{ color: "var(--color-text-primary)" }}>
              实例管理
            </span>
          </div>

          <Form.Item
            name="max_instances"
            label="最大多开数量"
            extra="允许同时运行的最大微信实例数量"
          >
            <InputNumber
              min={1}
              max={20}
              style={{ width: "100%", borderRadius: 10 }}
            />
          </Form.Item>
        </div>

        <div
          className="glass-card p-6 mb-6"
          style={{ background: "rgba(255, 255, 255, 0.95)" }}
        >
          <div className="flex items-center gap-2.5 mb-5">
            <div
              className="w-8 h-8 rounded-lg flex items-center justify-center"
              style={{
                background: "linear-gradient(135deg, rgba(7, 193, 96, 0.1) 0%, rgba(5, 163, 78, 0.15) 100%)",
                border: "1px solid rgba(7, 193, 96, 0.12)",
              }}
            >
              <MinusCircleOutlined style={{ color: "#07c160", fontSize: 14 }} />
            </div>
            <span className="font-semibold text-sm" style={{ color: "var(--color-text-primary)" }}>
              窗口行为
            </span>
          </div>

          <Form.Item
            name="minimize_to_tray"
            label="关闭时最小化到系统托盘"
            valuePropName="checked"
            extra="关闭后程序仍在托盘运行，可通过托盘图标重新打开"
          >
            <Switch />
          </Form.Item>
        </div>

        <Form.Item className="mb-0">
          <Button
            type="primary"
            htmlType="submit"
            loading={loading}
            icon={<SaveOutlined />}
            className="btn-glow"
            style={{
              height: 44,
              borderRadius: 12,
              fontWeight: 600,
              fontSize: 15,
              background: "linear-gradient(135deg, #0080ff 0%, #0066cc 100%)",
              border: "none",
              boxShadow: "0 4px 20px rgba(0, 128, 255, 0.25)",
            }}
          >
            保存配置
          </Button>
        </Form.Item>
      </Form>
    </div>
  );
};

export default SettingsPanel;
