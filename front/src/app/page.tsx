import Header from "@/components/Header";
import { LiveMap } from "@/components/LiveMap";
import Image from "next/image";

export default function Home() {
  return (
    <>
    
      <Header />
      <div>
        <LiveMap />
      </div>
    </>
  );
}
