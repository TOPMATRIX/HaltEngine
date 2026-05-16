import { useEffect, useState, useCallback } from "react";
import * as StellarSdk from "@stellar/stellar-sdk";

const RPC_URL = import.meta.env.VITE_SOROBAN_RPC_URL ?? "https://soroban-testnet.stellar.org";
const CONTRACT_ID = import.meta.env.VITE_GUARDIAN_PROXY_CONTRACT_ID ?? "";

export interface ContractState {
  isHalted: boolean;
  threshold: number;
  loading: boolean;
  error: string | null;
}

export function useContractState(): ContractState & { refresh: () => void } {
  const [state, setState] = useState<ContractState>({
    isHalted: false,
    threshold: 0,
    loading: true,
    error: null,
  });

  const fetch = useCallback(async () => {
    if (!CONTRACT_ID) {
      setState((s) => ({ ...s, loading: false, error: "VITE_GUARDIAN_PROXY_CONTRACT_ID not set" }));
      return;
    }
    setState((s) => ({ ...s, loading: true, error: null }));
    try {
      const server = new StellarSdk.rpc.Server(RPC_URL);
      const contract = new StellarSdk.Contract(CONTRACT_ID);

      const [haltedRes, threshRes] = await Promise.all([
        server.simulateTransaction(
          new StellarSdk.TransactionBuilder(
            await server.getAccount("GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN"),
            { fee: "100", networkPassphrase: StellarSdk.Networks.TESTNET }
          ).addOperation(contract.call("is_halted")).setTimeout(30).build()
        ),
        server.simulateTransaction(
          new StellarSdk.TransactionBuilder(
            await server.getAccount("GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN"),
            { fee: "100", networkPassphrase: StellarSdk.Networks.TESTNET }
          ).addOperation(contract.call("get_threshold")).setTimeout(30).build()
        ),
      ]);

      const isHalted = StellarSdk.scValToNative((haltedRes as StellarSdk.rpc.Api.SimulateTransactionSuccessResponse).result!.retval) as boolean;
      const threshold = StellarSdk.scValToNative((threshRes as StellarSdk.rpc.Api.SimulateTransactionSuccessResponse).result!.retval) as number;

      setState({ isHalted, threshold, loading: false, error: null });
    } catch (e) {
      setState((s) => ({ ...s, loading: false, error: String(e) }));
    }
  }, []);

  useEffect(() => { fetch(); }, [fetch]);
  return { ...state, refresh: fetch };
}
