import { useState } from "react";
import { submitManualHalt } from "../lib/stellar";

interface Props { onHalted: () => void }

export function PanicButton({ onHalted }: Props) {
  const [secret, setSecret]   = useState("");
  const [balance, setBalance] = useState("");
  const [busy, setBusy]       = useState(false);
  const [txHash, setTxHash]   = useState<string | null>(null);
  const [error, setError]     = useState<string | null>(null);

  async function handlePanic() {
    if (!secret || !balance) return;
    setBusy(true); setError(null); setTxHash(null);
    try {
      const hash = await submitManualHalt(secret, BigInt(balance));
      setTxHash(hash);
      onHalted();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div style={{ border: "2px solid #ef4444", borderRadius: 8, padding: 20, maxWidth: 480 }}>
      <h2 style={{ color: "#ef4444", marginTop: 0 }}>⚡ Manual Panic Button</h2>
      <p style={{ fontSize: 13, color: "#666" }}>
        Immediately triggers an on-chain halt via the GuardianProxy contract.
      </p>
      <label style={{ display: "block", marginBottom: 8 }}>
        <span style={{ fontSize: 12, fontWeight: 600 }}>Guardian Secret Key</span>
        <input
          type="password"
          value={secret}
          onChange={(e) => setSecret(e.target.value)}
          placeholder="S..."
          style={{ display: "block", width: "100%", marginTop: 4, padding: "6px 8px", borderRadius: 4, border: "1px solid #ccc" }}
        />
      </label>
      <label style={{ display: "block", marginBottom: 16 }}>
        <span style={{ fontSize: 12, fontWeight: 600 }}>Current Balance (stroops)</span>
        <input
          type="number"
          value={balance}
          onChange={(e) => setBalance(e.target.value)}
          placeholder="e.g. 5000000000"
          style={{ display: "block", width: "100%", marginTop: 4, padding: "6px 8px", borderRadius: 4, border: "1px solid #ccc" }}
        />
      </label>
      <button
        onClick={handlePanic}
        disabled={busy || !secret || !balance}
        style={{
          background: "#ef4444", color: "#fff", border: "none",
          borderRadius: 6, padding: "10px 24px", fontWeight: 700,
          fontSize: 15, cursor: busy ? "not-allowed" : "pointer", opacity: busy ? 0.7 : 1,
        }}
      >
        {busy ? "Submitting…" : "🚨 TRIGGER HALT"}
      </button>
      {txHash && <p style={{ color: "#22c55e", marginTop: 12, fontSize: 13 }}>✅ tx: {txHash}</p>}
      {error  && <p style={{ color: "#ef4444", marginTop: 12, fontSize: 13 }}>❌ {error}</p>}
    </div>
  );
}
