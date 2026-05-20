import { useState } from "react";
import { WechatOutlined, SettingOutlined, CodeOutlined, InfoCircleOutlined } from "@ant-design/icons";
import { ControlPanel } from "../Instance/ControlPanel";
import SettingsPanel from "../Settings/SettingsPanel";
import AboutPanel from "../About/AboutPanel";

const navItems = [
  { key: "control", icon: <WechatOutlined />, label: "多开控制" },
  { key: "settings", icon: <SettingOutlined />, label: "设置" },
  { key: "about", icon: <InfoCircleOutlined />, label: "关于" },
];

export function MainLayout() {
  const [current, setCurrent] = useState("control");

  return (
    <div className="flex h-screen overflow-hidden">
      <aside
        className="flex flex-col w-[220px] flex-shrink-0"
        style={{
          background: "linear-gradient(180deg, #0f1729 0%, #131d38 50%, #0f1729 100%)",
          borderRight: "1px solid rgba(42, 58, 106, 0.4)",
        }}
      >
        <div className="px-5 pt-6 pb-5">
          <div className="flex items-center gap-3 mb-1">
            <div
              className="w-9 h-9 rounded-xl flex items-center justify-center"
              style={{
                background: "linear-gradient(135deg, #00d4ff 0%, #0080ff 100%)",
                boxShadow: "0 4px 16px rgba(0, 212, 255, 0.3)",
              }}
            >
              <CodeOutlined style={{ color: "#fff", fontSize: 16 }} />
            </div>
            <div>
              <div className="text-white text-sm font-semibold tracking-wide">械式</div>
              <div
                className="text-xs font-medium"
                style={{ color: "rgba(0, 212, 255, 0.7)" }}
              >
                微信多开器
              </div>
            </div>
          </div>
        </div>

        <div className="px-3 mb-2">
          <div
            className="text-xs font-medium uppercase tracking-widest px-3 mb-2"
            style={{ color: "rgba(148, 163, 184, 0.5)" }}
          >
            导航
          </div>
        </div>

        <nav className="flex-1 px-3">
          {navItems.map((item) => (
            <div
              key={item.key}
              className={`sidebar-nav-item ${current === item.key ? "active" : ""}`}
              onClick={() => setCurrent(item.key)}
            >
              <span style={{ fontSize: 16, display: "flex", alignItems: "center" }}>
                {item.icon}
              </span>
              <span>{item.label}</span>
            </div>
          ))}
        </nav>

        <div className="px-4 pb-5">
          <div
            className="rounded-xl p-3"
            style={{
              background: "rgba(0, 212, 255, 0.05)",
              border: "1px solid rgba(0, 212, 255, 0.1)",
            }}
          >
            <div className="text-xs font-medium" style={{ color: "rgba(0, 212, 255, 0.8)" }}>
              v0.2.0
            </div>
            <div className="text-xs mt-1" style={{ color: "rgba(148, 163, 184, 0.5)" }}>
              Powered by Tauri
            </div>
          </div>
        </div>
      </aside>

      <main
        className="flex-1 overflow-auto mesh-bg"
        style={{ background: "var(--color-bg-primary)" }}
      >
        <div className="p-6 animate-fade-in" key={current}>
          {current === "control" && <ControlPanel />}
          {current === "settings" && <SettingsPanel />}
          {current === "about" && <AboutPanel />}
        </div>
      </main>
    </div>
  );
}
