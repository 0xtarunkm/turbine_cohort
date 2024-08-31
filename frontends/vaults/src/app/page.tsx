import { Deposit } from "@/components/Deposit";
import { Navbar } from "@/components/Navbar"
import { Withdraw } from "@/components/Withdraw";

export default function Home() {
  return (
    <main className="max-w-6xl mx-auto">
      <Navbar />
      <Deposit />
      <Withdraw />
    </main>
  );
}
