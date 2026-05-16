interface Props { halted: boolean }

export function StatusBadge({ halted }: Props) {
  return (
    <span
      style={{
        display: "inline-block",
        padding: "4px 14px",
        borderRadius: 9999,
        fontWeight: 700,
        fontSize: 14,
        background: halted ? "#ef4444" : "#22c55e",
        color: "#fff",
      }}
    >
      {halted ? "🔴 HALTED" : "🟢 ACTIVE"}
    </span>
  );
}
