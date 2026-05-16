import { useContractState } from "./hooks/useContractState";
import { StatusBadge } from "./components/StatusBadge";
import { PanicButton } from "./components/PanicButton";
import { ResetButton } from "./components/ResetButton";

export default function App() {
  const { isHalted, threshold, loading, error, refresh } = useContractState();

  return (
    <div style={{ fontFamily: "system-ui, sans-serif", maxWidth: 600, margin: "40px auto", padding: "0 16px" }}>
      <h1 style={{ fontSize: 24, marginBottom: 4 }}>⚙️ HaltEngine</h1>
      <p style={{ color: "#666", marginTop: 0, marginBottom: 24 }}>
        Automated Circuit Breaker &amp; Emergency Console
      </p>

      <section style={{ marginBottom: 32 }}>
        <h2 style={{ fontSize: 16, marginBottom: 8 }}>Contract Status</h2>
        {loading && <p>Loading…</p>}
        {error   && <p style={{ color: "#ef4444" }}>⚠ {error}</p>}
        {!loading && !error && (
          <div style={{ display: "flex", alignItems: "center", gap: 16 }}>
            <StatusBadge halted={isHalted} />
            <span style={{ fontSize: 13, color: "#555" }}>
              Drain threshold: <strong>{(threshold / 100).toFixed(0)}%</strong>
            </span>
            <button onClick={refresh} style={{ fontSize: 12, cursor: "pointer" }}>↻ Refresh</button>
          </div>
        )}
      </section>

      <section style={{ display: "flex", flexDirection: "column", gap: 24 }}>
        <PanicButton onHalted={refresh} />
        <ResetButton onReset={refresh} />
      </section>

      <footer style={{ marginTop: 48, fontSize: 11, color: "#aaa" }}>
        Contract: {import.meta.env.VITE_GUARDIAN_PROXY_CONTRACT_ID || "not configured"}
      </footer>
    </div>
  );
}
