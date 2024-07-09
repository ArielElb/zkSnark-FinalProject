// components/Navbar.js
import Link from "next/link";
import styles from "../../styles/Navbar.module.css";

const Navbar = () => {
  return (
    <nav className={styles.nav}>
      <ul className={styles.navList}>
        <li className={styles.navItem}>
          <Link href='/'>Home</Link>
        </li>
        <li className={styles.navItem}>
          <Link href='../options'>Options</Link>
        </li>
        <li className={styles.navItem}>
          <Link href='../inputFibonachi/'>Fibonacci</Link>
        </li>
        <li className={styles.navItem}>
          <Link href='../inputMatrix/'>Matrix</Link>
        </li>
        <li className={styles.navItem}>
          <Link href='../inputPrime/'>Prime</Link>
        </li>
        <li className={styles.navItem}>
          <Link href='../information/'>information</Link>
        </li>
      </ul>
    </nav>
  );
};

export default Navbar;
