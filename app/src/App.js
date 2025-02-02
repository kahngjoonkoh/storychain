// App.jsx
import { useState, useMemo } from "react";
import { useAnchorWallet, useWallet } from "@solana/wallet-adapter-react";
import { Connection, PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { Program, AnchorProvider } from "@project-serum/anchor";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";
import {
  PhantomWalletAdapter,
  SolflareWalletAdapter,
} from "@solana/wallet-adapter-wallets";
import "@solana/wallet-adapter-react-ui/styles.css";
import IDL from "./idl/storychain.json"

// Replace with your program ID
const PROGRAM_ID = IDL.address;

// Replace with your network URL
const network = "https://api.devnet.solana.com";

function App() {
  const [loading, setLoading] = useState(false);
  const wallet = useAnchorWallet();
  const { connected } = useWallet();

  const wallets = useMemo(
    () => [new PhantomWalletAdapter(), new SolflareWalletAdapter()],
    []
  );

  const getProvider = () => {
    if (!wallet) return null;
    const connection = new Connection(network, "confirmed");
    const provider = new AnchorProvider(connection, wallet, {
      preflightCommitment: "confirmed",
    });
    return provider;
  };

  const initializeProgram = async () => {
    try {
      setLoading(true);
      const provider = getProvider();
      if (!provider) {
        throw new Error("Provider is null");
      }

      // Replace with your program IDL
      const program = new Program(IDL, PROGRAM_ID, provider);

      // Your program interaction logic here
      // Example:
      // await program.methods.yourMethod().accounts({...}).rpc();

      setLoading(false);
    } catch (error) {
      console.error("Error:", error);
      setLoading(false);
    }
  };

  return (
    <ConnectionProvider endpoint={network}>
      <WalletProvider wallets={wallets} autoConnect>
        <div className="App">
          <header className="App-header">
            <h1>Solana dApp</h1>
            <WalletMultiButton />
            {connected && (
              <button
                onClick={initializeProgram}
                disabled={loading}
                style={{
                  padding: "10px 20px",
                  margin: "20px",
                  cursor: loading ? "not-allowed" : "pointer",
                }}
              >
                {loading ? "Processing..." : "Initialize Program"}
              </button>
            )}
          </header>
        </div>
      </WalletProvider>
    </ConnectionProvider>
  );
}

export default App;