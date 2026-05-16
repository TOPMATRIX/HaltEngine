import * as StellarSdk from "@stellar/stellar-sdk";

const RPC_URL     = import.meta.env.VITE_SOROBAN_RPC_URL ?? "https://soroban-testnet.stellar.org";
const CONTRACT_ID = import.meta.env.VITE_GUARDIAN_PROXY_CONTRACT_ID ?? "";
const NETWORK     = import.meta.env.VITE_NETWORK_PASSPHRASE ?? StellarSdk.Networks.TESTNET;

/** Sign and submit a `reset()` call using the admin secret key. */
export async function submitReset(adminSecret: string): Promise<string> {
  const server  = new StellarSdk.rpc.Server(RPC_URL);
  const keypair = StellarSdk.Keypair.fromSecret(adminSecret);
  const account = await server.getAccount(keypair.publicKey());
  const contract = new StellarSdk.Contract(CONTRACT_ID);

  const tx = new StellarSdk.TransactionBuilder(account, { fee: "100", networkPassphrase: NETWORK })
    .addOperation(contract.call("reset"))
    .setTimeout(30)
    .build();

  const prepared = await server.prepareTransaction(tx);
  prepared.sign(keypair);
  const result = await server.sendTransaction(prepared);
  return result.hash;
}

/** Sign and submit a `trigger_halt(current_balance)` call using the guardian secret key. */
export async function submitManualHalt(guardianSecret: string, currentBalance: bigint): Promise<string> {
  const server   = new StellarSdk.rpc.Server(RPC_URL);
  const keypair  = StellarSdk.Keypair.fromSecret(guardianSecret);
  const account  = await server.getAccount(keypair.publicKey());
  const contract = new StellarSdk.Contract(CONTRACT_ID);

  const tx = new StellarSdk.TransactionBuilder(account, { fee: "100", networkPassphrase: NETWORK })
    .addOperation(contract.call("trigger_halt", StellarSdk.nativeToScVal(currentBalance, { type: "i128" })))
    .setTimeout(30)
    .build();

  const prepared = await server.prepareTransaction(tx);
  prepared.sign(keypair);
  const result = await server.sendTransaction(prepared);
  return result.hash;
}
