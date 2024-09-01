import { Inter } from "next/font/google";
import Navbar from "./components/Navbar"; // Adjust the import path as needed
import styles from "../styles/main.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata = {
  title: "zkSnark",
  description:
    "zkSnark is a zero-knowledge proof system for proving the knowledge of a secret without revealing the secret itself.",
};

export default function RootLayout({ children }) {
  return (
    <html lang='en'>
      <head>
        <link rel="icon" href="/logo.jpg" />
        <title>{metadata.title}</title>
        <meta name="description" content={metadata.description} />
      </head>
      <body className={inter.className} id="body">
        <Navbar />
        {children}
      </body>
    </html>
  );
}
