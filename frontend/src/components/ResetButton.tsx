import { useState } from "react";
import { submitReset } from "../lib/stellar";

interface Props { onReset: () => void }

export function ResetButton({ onReset }: Props) {
  const [secret, setSecret] = useState("");
  const [busy, setBusy]     = useState(false);
  const [txHash, setTxHash] = useState<string | null>(null);
  const [error, setError]   = useState<string | null>(null);

  async function handleReset() {
    if (!secret) return;
    setBusy(true); setError(null); setTxHash(null);
    try {
      const hash = await submitReset(secret);
      setTxHash(hash);
      onReset();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div style={{ border: "2px solid #22c55e", borderRadius: 8, padding: 20, maxWidth: 480 }}>
      <h2 style={{ color: "#22c55e", marginTop: 0 }}>🔓 Reset Circuit Breaker</h2>
      <label style={{ display: "block", marginBottom: 16 }}>
        <span style={{ fontSize: 12, fontWeight: 600 }}>Admin Secret Key</span>
        <input
          type="password"
          value={secret}
          onChange={(e) => setSecret(e.target.value)}
          placeholder="S..."
          style={{ display: "block", width: "100%", marginTop: 4, padding: "6px 8px", borderRadius: 4, border: "1px solid #ccc" }}
        />
      </label>
      <button
        onClick={handleReset}
        disabled={busy || !secret}
        style={{
          background: "#22c55e", color: "#fff", border: "none",
          borderRadius: 6, padding: "10px 24px", fontWeight: 700,
          fontSize: 15, cursor: busy ? "not-allowed" : "pointer", opacity: busy ? 0.7 : 1,
        }}
      >
        {busy ? "Submitting…" : "✅ RESET"}
      </button>
      {txHash && <p style={{ color: "#22c55e", marginTop: 12, fontSize: 13 }}>✅ tx: {txHash}</p>}
      {error  && <p style={{ color: "#ef4444", marginTop: 12, fontSize: 13 }}>❌ {error}</p>}
    </div>
  );
}
