import { Inter } from "next/font/google";
import Navbar from "./components/Navbar"; // Adjust the import path as needed

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "zkSnark",
  description:
    "zkSnark is a zero-knowledge proof system for proving the knowledge of a secret without revealing the secret itself.",
};

export default function RootLayout({ children }) {
  return (
    <html lang='en'>
      <body className={inter.className} style={{ overflow: "hidden" }}>
        <Navbar />
        {children}
      </body>
    </html>
  );
}
