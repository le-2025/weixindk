import { ConfigProvider } from "antd";
import zhCN from "antd/locale/zh_CN";
import { MainLayout } from "./components/Layout/MainLayout";

function App() {
  return (
    <ConfigProvider
      locale={zhCN}
      theme={{
        token: {
          colorPrimary: "#0080ff",
          colorSuccess: "#07c160",
          borderRadius: 8,
          fontFamily: "'Inter', system-ui, -apple-system, sans-serif",
          fontSize: 14,
          colorBgLayout: "#f0f4f8",
          controlHeight: 36,
          wireframe: false,
        },
        components: {
          Button: {
            borderRadius: 8,
            fontWeight: 500,
          },
          Input: {
            borderRadius: 8,
          },
          Card: {
            borderRadius: 16,
          },
          Switch: {
            trackHeight: 22,
            trackMinWidth: 44,
          },
          Modal: {
            borderRadius: 16,
          },
          Message: {
            borderRadius: 10,
          },
        },
      }}
    >
      <MainLayout />
    </ConfigProvider>
  );
}

export default App;
