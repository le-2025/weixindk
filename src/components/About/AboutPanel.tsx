import {
  CodeOutlined,
  GlobalOutlined,
  QqOutlined,
  WechatOutlined,
  HeartOutlined,
  SafetyCertificateOutlined,
} from "@ant-design/icons";

const AboutPanel = () => {
  return (
    <div className="animate-fade-in" style={{ maxWidth: 640, margin: "0 auto" }}>
      <div className="mb-8">
        <h2 style={{ fontSize: 22, fontWeight: 700, color: "var(--color-text-primary)", marginBottom: 4 }}>
          关于
        </h2>
        <p style={{ color: "var(--color-text-secondary)", fontSize: 14 }}>
          了解更多关于械式微信多开器的信息
        </p>
      </div>

      <div
        className="glass-card p-6 mb-4"
        style={{ background: "rgba(255, 255, 255, 0.95)" }}
      >
        <div className="flex items-center gap-4 mb-5">
          <div
            className="w-14 h-14 rounded-2xl flex items-center justify-center flex-shrink-0"
            style={{
              background: "linear-gradient(135deg, #00d4ff 0%, #0080ff 100%)",
              boxShadow: "0 8px 24px rgba(0, 128, 255, 0.25)",
            }}
          >
            <CodeOutlined style={{ color: "#fff", fontSize: 28 }} />
          </div>
          <div>
            <h3 className="gradient-text" style={{ fontSize: 20, fontWeight: 700, letterSpacing: "-0.02em" }}>
              械式微信多开器
            </h3>
            <p style={{ color: "var(--color-text-secondary)", fontSize: 13, marginTop: 2 }}>
              版本 0.2.0 · 免费开源工具
            </p>
          </div>
        </div>

        <div
          className="rounded-xl p-4 mb-5"
          style={{
            background: "linear-gradient(135deg, rgba(0, 128, 255, 0.04) 0%, rgba(0, 212, 255, 0.06) 100%)",
            border: "1px solid rgba(0, 128, 255, 0.1)",
          }}
        >
          <p style={{ color: "var(--color-text-primary)", fontSize: 14, lineHeight: 1.8 }}>
            械式微信多开器是一款免费的 Windows 桌面工具，通过独立数据目录隔离技术实现微信多开，
            让您可以在同一台电脑上同时登录多个微信账号。无需修改微信本体，安全可靠。
          </p>
        </div>

        <div className="flex items-center gap-2 mb-1">
          <SafetyCertificateOutlined style={{ color: "#07c160", fontSize: 14 }} />
          <span style={{ fontSize: 13, fontWeight: 500, color: "var(--color-text-primary)" }}>
            安全声明
          </span>
        </div>
        <p style={{ color: "var(--color-text-secondary)", fontSize: 13, lineHeight: 1.7, paddingLeft: 22 }}>
          本工具不修改微信程序本体，不注入任何代码，不收集用户隐私数据。
          仅通过 Windows 系统级别的数据目录隔离实现多开功能。
        </p>
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
            <HeartOutlined style={{ color: "#7c3aed", fontSize: 14 }} />
          </div>
          <span className="font-semibold text-sm" style={{ color: "var(--color-text-primary)" }}>
            作者信息
          </span>
        </div>

        <div className="flex flex-col gap-4">
          <div className="flex items-center gap-3">
            <div
              className="w-9 h-9 rounded-lg flex items-center justify-center flex-shrink-0"
              style={{
                background: "rgba(0, 128, 255, 0.08)",
                border: "1px solid rgba(0, 128, 255, 0.12)",
              }}
            >
              <CodeOutlined style={{ color: "#0080ff", fontSize: 15 }} />
            </div>
            <div>
              <div style={{ fontSize: 12, color: "var(--color-text-secondary)", marginBottom: 2 }}>作者</div>
              <div style={{ fontSize: 14, fontWeight: 600, color: "var(--color-text-primary)" }}>aaaa</div>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div
              className="w-9 h-9 rounded-lg flex items-center justify-center flex-shrink-0"
              style={{
                background: "rgba(0, 128, 255, 0.08)",
                border: "1px solid rgba(0, 128, 255, 0.12)",
              }}
            >
              <GlobalOutlined style={{ color: "#0080ff", fontSize: 15 }} />
            </div>
            <div>
              <div style={{ fontSize: 12, color: "var(--color-text-secondary)", marginBottom: 2 }}>个人主页</div>
              <a
                href="http://www.xieshi.中国"
                target="_blank"
                rel="noopener noreferrer"
                style={{ fontSize: 14, fontWeight: 500, color: "#0080ff" }}
              >
                www.xieshi.中国
              </a>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div
              className="w-9 h-9 rounded-lg flex items-center justify-center flex-shrink-0"
              style={{
                background: "rgba(0, 128, 255, 0.08)",
                border: "1px solid rgba(0, 128, 255, 0.12)",
              }}
            >
              <QqOutlined style={{ color: "#0080ff", fontSize: 15 }} />
            </div>
            <div>
              <div style={{ fontSize: 12, color: "var(--color-text-secondary)", marginBottom: 2 }}>QQ</div>
              <div style={{ fontSize: 14, fontWeight: 500, color: "var(--color-text-primary)" }}>229458084</div>
            </div>
          </div>

          <div className="flex items-center gap-3">
            <div
              className="w-9 h-9 rounded-lg flex items-center justify-center flex-shrink-0"
              style={{
                background: "rgba(7, 193, 96, 0.08)",
                border: "1px solid rgba(7, 193, 96, 0.12)",
              }}
            >
              <WechatOutlined style={{ color: "#07c160", fontSize: 15 }} />
            </div>
            <div>
              <div style={{ fontSize: 12, color: "var(--color-text-secondary)", marginBottom: 2 }}>微信</div>
              <div style={{ fontSize: 14, fontWeight: 500, color: "var(--color-text-primary)" }}>Red---eyes</div>
            </div>
          </div>
        </div>
      </div>

      <div
        className="glass-card p-6 mb-4"
        style={{ background: "rgba(255, 255, 255, 0.95)" }}
      >
        <div className="flex items-center gap-2.5 mb-4">
          <div
            className="w-8 h-8 rounded-lg flex items-center justify-center"
            style={{
              background: "linear-gradient(135deg, rgba(7, 193, 96, 0.1) 0%, rgba(5, 163, 78, 0.15) 100%)",
              border: "1px solid rgba(7, 193, 96, 0.12)",
            }}
          >
            <SafetyCertificateOutlined style={{ color: "#07c160", fontSize: 14 }} />
          </div>
          <span className="font-semibold text-sm" style={{ color: "var(--color-text-primary)" }}>
            免责声明
          </span>
        </div>

        <div style={{ fontSize: 13, color: "var(--color-text-secondary)", lineHeight: 1.8 }}>
          <p className="mb-2">
            1. 本工具仅供学习和研究使用，严禁用于任何违法违规用途。
          </p>
          <p className="mb-2">
            2. 本工具不修改微信程序本体，不破解任何功能限制，仅利用系统级数据目录隔离实现多开。
          </p>
          <p className="mb-2">
            3. 使用本工具产生的任何直接或间接损失，作者不承担任何责任。
          </p>
          <p>
            4. 微信是腾讯公司的注册商标，本工具与腾讯公司无任何关联。
          </p>
        </div>
      </div>

      <div
        className="text-center py-4"
        style={{ color: "var(--color-text-secondary)", fontSize: 12 }}
      >
        <p>© 2026 aaaa · www.xieshi.中国</p>
        <p className="mt-1">Built with Tauri + React + Rust</p>
      </div>
    </div>
  );
};

export default AboutPanel;
