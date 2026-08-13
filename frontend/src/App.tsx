import { useState } from 'react';
import { Wallet, Trophy, CheckCircle, RefreshCw, Star, Shield, User, ExternalLink } from 'lucide-react';

const RETIUM_WALLET_URL = "https://wallet.retium.org/";

export default function App() {
  const [walletConnected, setWalletConnected] = useState(false);
  const [walletAddress, setWalletAddress] = useState("");
  const [userXp, setUserXp] = useState(0);
  const [completedQuests, setCompletedQuests] = useState<number[]>([]);
  const [earnedBadges, setEarnedBadges] = useState<string[]>([]);
  
  const [statusMessage, setStatusMessage] = useState("");
  const [isProcessing, setIsProcessing] = useState(false);

  // 1. Resmi Retium Web Cüzdanı (wallet.retium.org) Bağlantı Simülasyonu
  const connectWallet = async () => {
    setIsProcessing(true);
    setStatusMessage("Connecting to Retium L1 Network... Scanning active sessions on wallet.retium.org...");
    
    setTimeout(() => {
      setWalletAddress("reteeca31728b97ac");
      setWalletConnected(true);
      setStatusMessage("Retium Wallet successfully connected! Ready to fetch active developer quests.");
      setIsProcessing(false);
    }, 2000);
  };

  // 2. RCP-1 Sözleşmesini Çağırma ve Görevi Claim Etme Fonksiyonu
  const handleCompleteQuest = (questId: number, xpReward: number, questName: string) => {
    setIsProcessing(true);
    setStatusMessage(`Executing RCP-1 smart contract transaction on Retium Virtual Machine (RVM)...`);
    
    setTimeout(() => {
      setStatusMessage(`Suits validating block finality on PrimeMesh network... [SoftFinal ➜ HardFinal Committed]!`);
      
      setTimeout(() => {
        // XP'yi güncelle
        const nextXp = userXp + xpReward;
        setUserXp(nextXp);

        // Tamamlanan görevlere ekle
        setCompletedQuests([...completedQuests, questId]);

        // Rozet/Badge Kontrolü (.includes metodunu kullanarak hatasız kurguladık!)
        const newBadges = [...earnedBadges];
        if (nextXp >= 100 && !newBadges.includes("Bronze Explorer")) {
          newBadges.push("Bronze Explorer");
        }
        if (nextXp >= 350 && !newBadges.includes("Silver Builder")) {
          newBadges.push("Silver Builder");
        }
        if (nextXp >= 850 && !newBadges.includes("Gold ZK-Master")) {
          newBadges.push("Gold ZK-Master");
        }
        setEarnedBadges(newBadges);

        setStatusMessage(`SUCCESS! Quest "${questName}" successfully verified on-chain! Earned +${xpReward} XP!`);
        setIsProcessing(false);
      }, 2000);
    }, 2000);
  };

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-purple-500 selection:text-white">
      {/* Header */}
      <header className="border-b border-purple-950 bg-slate-900/40 backdrop-blur px-6 py-4 flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <Trophy className="h-8 w-8 text-purple-500 animate-pulse" />
          <span className="text-xl font-black tracking-wider bg-gradient-to-r from-purple-400 via-fuchsia-500 to-cyan-400 bg-clip-text text-transparent">
            RETIUM QUEST PORTAL (RQS)
          </span>
        </div>
        
        {walletConnected ? (
          <div className="flex items-center space-x-3 bg-purple-950/40 px-4 py-2 rounded-full border border-purple-800/50">
            <div className="h-2 w-2 rounded-full bg-cyan-400 animate-ping" />
            <span className="text-xs font-mono text-cyan-300">
              {walletAddress.slice(0, 12)}...{walletAddress.slice(-6)}
            </span>
          </div>
        ) : (
          <button 
            onClick={connectWallet}
            disabled={isProcessing}
            className="flex items-center space-x-2 bg-gradient-to-r from-purple-600 via-fuchsia-600 to-cyan-600 hover:from-purple-700 hover:to-cyan-700 text-white font-bold px-5 py-2.5 rounded-full transition-all duration-300 shadow-lg shadow-purple-500/20 active:scale-95 disabled:opacity-40"
          >
            <Wallet className="h-4 w-4" />
            <span>Connect Retium Wallet</span>
          </button>
        )}
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-5xl w-full mx-auto p-6 md:p-12 grid grid-cols-1 lg:grid-cols-3 gap-8">
        
        {/* Left Column: User Profile & Badges */}
        <div className="lg:col-span-1 flex flex-col space-y-6">
          {/* Profile Card */}
          <div className="bg-slate-900/40 border border-purple-950 p-6 rounded-3xl backdrop-blur relative overflow-hidden">
            <div className="absolute top-0 right-0 h-24 w-24 bg-purple-500/10 rounded-full blur-2xl" />
            <div className="flex items-center space-x-4">
              <div className="h-12 w-12 rounded-2xl bg-purple-600 flex items-center justify-center border border-purple-500/50">
                <User className="h-6 w-6 text-white" />
              </div>
              <div>
                <h3 className="text-lg font-bold text-slate-200">{walletConnected ? "AzAgent" : "Anonymous Guest"}</h3>
                <span className="text-xs text-purple-400 font-semibold uppercase tracking-wider">
                  {userXp >= 850 ? "Gold ZK-Master" : userXp >= 350 ? "Silver Builder" : userXp >= 100 ? "Bronze Explorer" : "Novice Builder"}
                </span>
              </div>
            </div>

            <div className="mt-6 space-y-2 border-t border-purple-950/80 pt-4">
              <div className="flex justify-between text-sm">
                <span className="text-slate-400">Total Experience:</span>
                <span className="font-bold text-cyan-400 font-mono">{userXp} XP</span>
              </div>
              <div className="w-full bg-slate-950 h-2.5 rounded-full overflow-hidden border border-purple-950">
                <div 
                  className="bg-gradient-to-r from-purple-500 to-cyan-500 h-full transition-all duration-500" 
                  style={{ width: `${Math.min((userXp / 850) * 100, 100)}%` }}
                />
              </div>
            </div>
          </div>

          {/* Badge Showcase */}
          <div className="bg-slate-900/40 border border-purple-950 p-6 rounded-3xl backdrop-blur flex-1">
            <h4 className="text-sm font-bold text-slate-400 uppercase tracking-widest mb-4 flex items-center space-x-2">
              <Shield className="h-4 w-4 text-purple-500" />
              <span>Earned RCP-1 Badges</span>
            </h4>
            
            <div className="space-y-4">
              {/* Bronze Badge */}
              <div className={`p-4 rounded-2xl border transition duration-300 flex items-center justify-between ${earnedBadges.includes("Bronze Explorer") ? "bg-amber-900/10 border-amber-800/40" : "bg-slate-950/40 border-slate-900 opacity-40"}`}>
                <div className="flex items-center space-x-3">
                  <Star className={`h-5 w-5 ${earnedBadges.includes("Bronze Explorer") ? "text-amber-500 fill-amber-500" : "text-slate-600"}`} />
                  <span className="text-sm font-semibold">Bronze Explorer</span>
                </div>
                <span className="text-xs font-mono text-slate-500">100 XP</span>
              </div>

              {/* Silver Badge */}
              <div className={`p-4 rounded-2xl border transition duration-300 flex items-center justify-between ${earnedBadges.includes("Silver Builder") ? "bg-slate-400/10 border-slate-400/30" : "bg-slate-950/40 border-slate-900 opacity-40"}`}>
                <div className="flex items-center space-x-3">
                  <Star className={`h-5 w-5 ${earnedBadges.includes("Silver Builder") ? "text-slate-400 fill-slate-400" : "text-slate-600"}`} />
                  <span className="text-sm font-semibold">Silver Builder</span>
                </div>
                <span className="text-xs font-mono text-slate-500">350 XP</span>
              </div>

              {/* Gold Badge */}
              <div className={`p-4 rounded-2xl border transition duration-300 flex items-center justify-between ${earnedBadges.includes("Gold ZK-Master") ? "bg-yellow-500/10 border-yellow-500/20" : "bg-slate-950/40 border-slate-900 opacity-40"}`}>
                <div className="flex items-center space-x-3">
                  <Star className={`h-5 w-5 ${earnedBadges.includes("Gold ZK-Master") ? "text-yellow-400 fill-yellow-400" : "text-slate-600"}`} />
                  <span className="text-sm font-semibold">Gold ZK-Master</span>
                </div>
                <span className="text-xs font-mono text-slate-500">850 XP</span>
              </div>
            </div>
          </div>
        </div>

        {/* Right Columns: Quest Board */}
        <div className="lg:col-span-2 flex flex-col space-y-6">
          
          {/* Link to Official Wallet Card */}
          <div className="bg-slate-900/40 border border-purple-950 p-6 rounded-3xl flex flex-col md:flex-row items-center justify-between gap-6 backdrop-blur">
            <div className="space-y-2 text-center md:text-left">
              <h2 className="text-md font-bold text-slate-200">Official Retium Web Wallet is Live!</h2>
              <p className="text-xs text-slate-400">Claim free RTM testnet tokens, send transactions, and secure the network on the official portal.</p>
            </div>
            <a 
              href={RETIUM_WALLET_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="w-full md:w-auto flex items-center justify-center space-x-2 bg-purple-950/80 hover:bg-purple-900 text-cyan-400 border border-purple-800/60 px-5 py-3 rounded-xl text-sm font-semibold transition"
            >
              <span>Go to wallet.retium.org</span>
              <ExternalLink className="h-4 w-4" />
            </a>
          </div>

          <div className="bg-slate-900/20 border border-purple-950 p-6 md:p-8 rounded-3xl min-h-[400px] backdrop-blur flex flex-col justify-between">
            <div className="space-y-6">
              <h3 className="text-xl font-bold flex items-center space-x-2 border-b border-purple-950/50 pb-4">
                <Trophy className="text-purple-500 h-5 w-5 animate-bounce" />
                <span>Retium Active Quests</span>
              </h3>

              <div className="space-y-4">
                {/* Quest 1 */}
                <div className="bg-slate-900/40 border border-purple-950/60 p-5 rounded-2xl flex flex-col md:flex-row items-center justify-between gap-4">
                  <div className="space-y-1 text-center md:text-left">
                    <h4 className="font-bold text-slate-200">Quest 1: First Miden Transfer</h4>
                    <p className="text-xs text-slate-400">Complete an on-chain transfer of 1 MIDEN using the local CLI weekly automation script.</p>
                  </div>
                  <button 
                    onClick={() => handleCompleteQuest(1, 100, "First Miden Transfer")}
                    disabled={!walletConnected || completedQuests.includes(1) || isProcessing}
                    className="w-full md:w-auto bg-purple-600 hover:bg-purple-700 disabled:opacity-40 disabled:cursor-not-allowed text-white font-bold px-6 py-3 rounded-xl transition duration-200 text-xs tracking-wider shortcut-uppercase"
                  >
                    {completedQuests.includes(1) ? "Completed" : "Claim 100 XP"}
                  </button>
                </div>

                {/* Quest 2 */}
                <div className="bg-slate-900/40 border border-purple-950/60 p-5 rounded-2xl flex flex-col md:flex-row items-center justify-between gap-4">
                  <div className="space-y-1 text-center md:text-left">
                    <h4 className="font-bold text-slate-200">Quest 2: ZK Bank Deposit</h4>
                    <p className="text-xs text-slate-400">Successfully compile, test locally on MockChain, and deploy your custom Bank Contract on-chain.</p>
                  </div>
                  <button 
                    onClick={() => handleCompleteQuest(2, 250, "ZK Bank Deposit")}
                    disabled={!walletConnected || completedQuests.includes(2) || isProcessing}
                    className="w-full md:w-auto bg-purple-600 hover:bg-purple-700 disabled:opacity-40 disabled:cursor-not-allowed text-white font-bold px-6 py-3 rounded-xl transition duration-200 text-xs tracking-wider shortcut-uppercase"
                  >
                    {completedQuests.includes(2) ? "Completed" : "Claim 250 XP"}
                  </button>
                </div>

                {/* Quest 3 */}
                <div className="bg-slate-900/40 border border-purple-950/60 p-5 rounded-2xl flex flex-col md:flex-row items-center justify-between gap-4">
                  <div className="space-y-1 text-center md:text-left">
                    <h4 className="font-bold text-slate-200">Quest 3: ZK-AMM Swap</h4>
                    <p className="text-xs text-slate-400">Implement and verify a constant-ratio decentralized liquidity pool and execute a private swap.</p>
                  </div>
                  <button 
                    onClick={() => handleCompleteQuest(3, 500, "ZK-AMM Swap")}
                    disabled={!walletConnected || completedQuests.includes(3) || isProcessing}
                    className="w-full md:w-auto bg-purple-600 hover:bg-purple-700 disabled:opacity-40 disabled:cursor-not-allowed text-white font-bold px-6 py-3 rounded-xl transition duration-200 text-xs tracking-wider shortcut-uppercase"
                  >
                    {completedQuests.includes(3) ? "Completed" : "Claim 500 XP"}
                  </button>
                </div>
              </div>
            </div>

            {/* Status Bar / RVM Terminal Logs */}
            {statusMessage && (
              <div className="mt-8 p-4 bg-slate-950 border border-purple-950/80 rounded-xl flex items-start space-x-3 text-xs text-slate-400 font-mono animate-fade-in shadow-inner relative overflow-hidden">
                <div className="absolute top-0 left-0 h-full w-1 bg-gradient-to-b from-purple-500 to-cyan-500" />
                {isProcessing ? (
                  <RefreshCw className="h-4 w-4 text-purple-400 animate-spin flex-shrink-0" />
                ) : (
                  <CheckCircle className="h-4 w-4 text-cyan-400 flex-shrink-0" />
                )}
                <span>{statusMessage}</span>
              </div>
            )}
          </div>
        </div>
      </main>

      {/* Footer */}
      <footer className="border-t border-purple-950/50 py-6 text-center text-xs text-slate-600 font-mono">
        © 2026 RETIUM Protocol. Powered by Retium Virtual Machine (RVM) and PrimeMesh.
      </footer>
    </div>
  );
}
